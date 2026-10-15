"""Independent NumPy forward pass for DiffusionGemma-26B-A4B-it text (test tooling).

Equations follow transformers @ bd970fda476f204f9852fb551aa07a4b3fbc5540,
src/transformers/models/diffusion_gemma/modeling_diffusion_gemma.py (cited as
`M:<line>`) and modeling_rope_utils.py (`R:<line>`). Weights come from
ggufref.py's own dequantizers; nothing from the engine is used.

Arithmetic: weights are the GGUF's float32 dequantized values; activations and
all reductions are float64. RoPE inverse frequencies and cos/sin are computed in
float32 as the official module forces (M:145). With `storage="f32"`, every
stored intermediate is additionally rounded to float32 to emulate an engine that
keeps F32 activations with F64 reductions; comparing the two modes gives the
expected noise floor.
"""
import math

import numpy as np

HIDDEN, HEADS, EXPERTS, TOP_K, MOE_FF = 2816, 16, 128, 8, 704
EPS = 1e-6  # config.rms_norm_eps (GGUF stores the same f32 value)
SOFTCAP = 30.0
SLIDING_WINDOW = 1024
EMBED_SCALE = float(np.float32(math.sqrt(HIDDEN)))  # M:912 torch.tensor(hidden**0.5) is float32


def gelu_tanh(x):  # gelu_pytorch_tanh (activations.py GELUTanh)
    return 0.5 * x * (1.0 + np.tanh(math.sqrt(2.0 / math.pi) * (x + 0.044715 * x ** 3)))


def softmax(x, axis=-1):
    z = np.exp(x - np.max(x, axis=axis, keepdims=True))
    return z / np.sum(z, axis=axis, keepdims=True)


def inv_freq(layer_type):
    """float32, as torch computes base ** (arange / dim) in float32."""
    f32 = np.float32
    if layer_type == "sliding_attention":  # M:124-132 default RoPE, dim = head_dim 256
        exps = np.arange(0, 256, 2, dtype=np.int64).astype(f32) / f32(256)
        return (f32(1.0) / (f32(10000.0) ** exps)).astype(f32)
    # R:227-247 proportional: head_dim 512, partial 0.25 -> 64 rotated angles, 192 zero frequencies
    rope_angles = int(0.25 * 512 // 2)
    exps = np.arange(0, 2 * rope_angles, 2, dtype=np.int64).astype(f32) / f32(512)
    rotated = (f32(1.0) / (f32(1000000.0) ** exps)).astype(f32)
    return np.concatenate([rotated, np.zeros(256 - rope_angles, dtype=f32)])


def rope_cos_sin(layer_type, positions):
    """M:141-149: freqs = inv_freq * pos (f32), emb = cat(freqs, freqs), cos/sin in f32."""
    freqs = inv_freq(layer_type)[None, :] * np.asarray(positions, dtype=np.float32)[:, None]
    emb = np.concatenate([freqs, freqs], axis=-1).astype(np.float32)
    return np.cos(emb).astype(np.float64), np.sin(emb).astype(np.float64)


def apply_rope(x, cos, sin):
    """x [n, heads, d]; M:204-230 rotate_half."""
    half = x.shape[-1] // 2
    rotated = np.concatenate([-x[..., half:], x[..., :half]], axis=-1)
    return x * cos[:, None, :] + rotated * sin[:, None, :]


class Model:
    def __init__(self, gguf, storage="f64"):
        self.g = gguf
        assert storage in ("f64", "f32")
        self.storage = storage
        md = gguf.metadata
        self.layer_types = ["sliding_attention" if s else "full_attention"
                            for s in md["diffusion-gemma.attention.sliding_window_pattern"]]
        self.kv_heads = [int(n) for n in md["diffusion-gemma.attention.head_count_kv"]]
        assert len(self.layer_types) == 30 and md["diffusion-gemma.embedding_length"] == HIDDEN
        self._cache = {}

    # ---- weights -----------------------------------------------------------
    def vec(self, name):
        if name not in self._cache:
            self._cache[name] = self.g.rows(name)[0].astype(np.float64)
        return self._cache[name]

    def mat(self, name):
        """Cached as the exact float32 weights; products upcast them to float64."""
        if name not in self._cache:
            self._cache[name] = self.g.matrix(name)
        return self._cache[name]

    def expert(self, layer, e):
        """Dequantized on demand (not cached: 128 experts x 30 layers would not fit politely)."""
        gu = self.g.rows(f"blk.{layer}.ffn_gate_up_exps.weight", e * 2 * MOE_FF, 2 * MOE_FF)
        down = self.g.rows(f"blk.{layer}.ffn_down_exps.weight", e * HIDDEN, HIDDEN)
        return gu.astype(np.float64), down.astype(np.float64)

    def embedding_rows(self, ids):
        return np.stack([self.g.rows("token_embd.weight", int(i), 1)[0] for i in ids]).astype(np.float64)

    def st(self, x):
        """Storage rounding of an intermediate result (identity in f64 mode)."""
        return x.astype(np.float32).astype(np.float64) if self.storage == "f32" else x

    # ---- primitives ----------------------------------------------------------
    def rmsnorm(self, x, weight=None):  # M:163-172, direct scale (no 1 + w)
        y = x * np.power(np.mean(x * x, axis=-1, keepdims=True) + EPS, -0.5)
        return self.st(y if weight is None else y * weight)

    def linear(self, name, x):  # x [n, in] -> [n, out]
        return self.st(x @ self.mat(name).astype(np.float64).T)

    # ---- attention (M:320-373 encoder, M:422-475 decoder) ---------------------
    def attention(self, layer, h, positions, cache, causal):
        lt = self.layer_types[layer]
        sliding = lt == "sliding_attention"
        d = 256 if sliding else 512
        kvh = self.kv_heads[layer]
        n = h.shape[0]
        p = f"blk.{layer}."
        cos, sin = rope_cos_sin(lt, positions)
        q = self.linear(p + "attn_q.weight", h).reshape(n, HEADS, d)
        q = self.st(apply_rope(self.rmsnorm(q, self.vec(p + "attn_q_norm.weight")), cos, sin))
        k_raw = self.linear(p + "attn_k.weight", h).reshape(n, kvh, d)
        v_raw = self.linear(p + "attn_v.weight", h).reshape(n, kvh, d) if sliding else k_raw  # M:341
        k = self.st(apply_rope(self.rmsnorm(k_raw, self.vec(p + "attn_k_norm.weight")), cos, sin))
        v = self.rmsnorm(v_raw)  # v_norm without scale (M:318)
        keys = np.concatenate([cache["k"], k]) if cache is not None else k
        values = np.concatenate([cache["v"], v]) if cache is not None else v
        key_pos = np.concatenate([cache["pos"], positions]) if cache is not None else np.asarray(positions)
        rep = HEADS // kvh  # repeat_kv: query head j uses kv head j // rep (M:233-242)
        out = np.empty((n, HEADS, d))
        for head in range(HEADS):
            kv = head // rep
            scores = keys[:, kv, :] @ q[:, head, :].T  # [keys, n], scaling 1.0 (M:299)
            for i in range(n):
                allowed = np.ones(len(key_pos), dtype=bool)
                if causal:  # encoder: causal, sliding layers additionally windowed
                    allowed &= key_pos <= positions[i]
                    if sliding:
                        allowed &= key_pos > positions[i] - SLIDING_WINDOW
                w = softmax(np.where(allowed, scores[:, i], -np.inf))
                out[i, head] = w @ values[:, kv, :]
        new_cache = {"k": keys, "v": values, "pos": key_pos}
        return self.linear(p + "attn_output.weight", self.st(out.reshape(n, HEADS * d))), new_cache

    # ---- router + experts (M:535-596) ------------------------------------------
    def route(self, layer, residual):
        p = f"blk.{layer}."
        x = self.rmsnorm(residual) * self.vec(p + "ffn_gate_inp.scale") * HIDDEN ** -0.5
        logits = self.st(self.st(x) @ self.mat(p + "ffn_gate_inp.weight").astype(np.float64).T)
        probs = softmax(logits)
        ids = np.argsort(-probs, axis=-1, kind="stable")[:, :TOP_K]  # topk: descending
        top = np.take_along_axis(probs, ids, axis=-1)
        weights = top / top.sum(axis=-1, keepdims=True)  # renormalize first (M:551)
        weights = weights * self.vec(p + "ffn_down_exps.scale")[ids]  # then per-expert scale (M:554)
        return probs, ids, self.st(weights)

    def experts(self, layer, x, ids, weights):
        out = np.zeros_like(x)
        for e in sorted(set(int(i) for i in ids.reshape(-1))):  # ascending expert IDs (M:582-594)
            gu, down = self.expert(layer, e)
            for t in range(x.shape[0]):
                slots = np.nonzero(ids[t] == e)[0]
                if len(slots) == 0:
                    continue
                g = self.st(gu @ x[t])
                hidden = self.st(gelu_tanh(g[:MOE_FF]) * g[MOE_FF:])  # chunk(2): gate first
                out[t] += self.st(down @ hidden) * weights[t, slots[0]]
        return self.st(out)

    # ---- decoder layer (M:626-670 encoder / M:701-745 decoder) ------------------
    def layer(self, layer, x, positions, cache, encoder, trace=None):
        p = f"blk.{layer}."
        residual = x
        h = self.rmsnorm(x, self.vec(p + "attn_norm.weight"))
        a, new_cache = self.attention(layer, h, positions, cache, causal=encoder)
        x = self.st(residual + self.rmsnorm(a, self.vec(p + "post_attention_norm.weight")))
        post_attention = x
        residual = x
        m = self.rmsnorm(x, self.vec(p + "ffn_norm.weight"))
        m = self.linear(p + "ffn_down.weight", self.st(gelu_tanh(self.linear(p + "ffn_gate.weight", m))
                                                       * self.linear(p + "ffn_up.weight", m)))
        h1 = self.rmsnorm(m, self.vec(p + "post_ffw_norm_1.weight"))
        probs, ids, weights = self.route(layer, residual)  # routing uses the un-normed residual
        e_in = self.rmsnorm(residual, self.vec(p + "pre_ffw_norm_2.weight"))
        h2 = self.rmsnorm(self.experts(layer, e_in, ids, weights), self.vec(p + "post_ffw_norm_2.weight"))
        x = self.st(residual + self.rmsnorm(self.st(h1 + h2), self.vec(p + "post_ffw_norm.weight")))
        scale = self.vec(p + ("enc_layer_output_scale.weight" if encoder else "layer_output_scale.weight"))[0]
        x = self.st(x * scale)
        if trace is not None:
            trace.update(post_attention=post_attention, shared=h1, routed=h2, router_probs=probs,
                         top_ids=ids, top_weights=weights)
        return x, new_cache

    # ---- full model ----------------------------------------------------------
    def embed(self, ids):  # M:758 lookup * embed_scale
        return self.st(self.embedding_rows(ids) * EMBED_SCALE)

    def encode(self, ids, layer_traces=None):
        """Causal encoder from an empty cache; positions 0..n-1 (M:927-988)."""
        positions = np.arange(len(ids))
        x = self.embed(ids)
        caches = []
        for layer in range(30):
            info = {} if layer_traces is not None else None
            x, cache = self.layer(layer, x, positions, None, encoder=True, trace=info)
            caches.append(cache)
            if layer_traces is not None:
                layer_traces.append(dict(info, output=x))
        return self.rmsnorm(x, self.vec("output_norm.weight")), caches

    def soft_embedding(self, processed_logits, scaled=True, chunk=16384):
        """M:1278-1281: softmax(f32 in reference; f64 here) @ E * embed_scale."""
        probs = softmax(np.asarray(processed_logits, dtype=np.float64))
        out = np.zeros((probs.shape[0], HIDDEN))
        for s in range(0, 262144, chunk):
            out += probs[:, s:s + chunk] @ self.g.rows("token_embd.weight", s, chunk).astype(np.float64)
        return self.st(out * EMBED_SCALE if scaled else out)

    def self_condition(self, embeds, soft):  # M:812-824
        normed = self.rmsnorm(soft, self.vec("self_cond_pre_norm.weight"))
        signal = self.linear("self_cond_down.weight", self.st(gelu_tanh(self.linear("self_cond_gate.weight", normed))
                                                              * self.linear("self_cond_up.weight", normed)))
        return self.rmsnorm(self.st(embeds + signal))  # post_norm without scale

    def decode(self, ids, caches, processed_logits=None, scaled_soft=True, layer_traces=None, extra=None):
        """Bidirectional decoder over the read-only encoder cache (M:1243-1328)."""
        cache_len = len(caches[0]["pos"])
        positions = np.arange(cache_len, cache_len + len(ids))
        embeds = self.embed(ids)
        soft = (np.zeros_like(embeds) if processed_logits is None
                else self.soft_embedding(processed_logits, scaled=scaled_soft))
        x = self.self_condition(embeds, soft)
        if extra is not None:
            extra.update(embeds=embeds, soft=soft, self_conditioned=x)
        for layer in range(30):
            info = {} if layer_traces is not None else None
            x, _ = self.layer(layer, x, positions, caches[layer], encoder=False, trace=info)
            if layer_traces is not None:
                layer_traces.append(dict(info, output=x))
        return self.rmsnorm(x, self.vec("output_norm.weight"))

    def logits(self, hidden, chunk=16384):
        """Tied unscaled output head + softcap (M:1666-1670)."""
        raw = np.empty((hidden.shape[0], 262144))
        for s in range(0, 262144, chunk):
            raw[:, s:s + chunk] = hidden @ self.g.rows("token_embd.weight", s, chunk).astype(np.float64).T
        return SOFTCAP * np.tanh(self.st(raw) / SOFTCAP)
