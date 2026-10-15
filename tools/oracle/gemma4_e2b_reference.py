#!/usr/bin/env python3
"""Independent reference for the DENSE Gemma 4 with per-layer embeddings and shared K/V
(gemma-4-E2B-it, GGUF arch `gemma4`). Test tooling: engine output is never read.

    RUN="numactl --physcpubind=CPUS --membind=NODE env OPENBLAS_NUM_THREADS=N python3"
    $RUN gemma4_e2b_reference.py MODEL PROMPT_IDS.json OUT_DIR

Equations: transformers @ bd970fda476f204f9852fb551aa07a4b3fbc5540,
src/transformers/models/gemma4/modeling_gemma4.py (sha256
ccab8e2dd80b71e9ca34e2c87291e17c40a27c755006e554da2ebf70d6616916), cited as `G:<line>`:
  * Gemma4TextModel.forward / get_per_layer_inputs / project_per_layer_inputs   G:1593-1821
    per-layer input = (RMSNorm(per_layer_model_proj(embeds) * hidden**-0.5) + table[id] * sqrt(ple)) * 2**-0.5
  * Gemma4TextDecoderLayer.forward (dense: enable_moe_block false)              G:1405-1466
    x += post_attn_norm(attn(input_norm(x))); x += post_ffw_norm(mlp(pre_ffw_norm(x)));
    x += post_per_layer_input_norm(proj(gelu(gate(x)) * per_layer_input)); x *= layer_scalar
  * Gemma4TextAttention (q/k norms, v_norm without scale, scaling 1.0)          G:1183-1298
    shared-K/V layers (index >= layers - num_kv_shared_layers) attend over the key/value states
    of the LAST earlier layer of the same type, as returned by its cache update in the same
    forward pass (before sliding-window truncation), with their own mask        G:1205-1276
  * Gemma4TextMLP: down(gelu_tanh(gate(x)) * up(x))                             G:1074-1091
RoPE: reference.py (default for sliding layers, proportional 0.25 of 512 for full layers; the same
transformers formulas). Cache semantics as long_context.py: sliding layers keep the last
`sliding_window - 1` tokens after each update; the prompt is encoded causally in 256-token chunks.
Arithmetic: GGUF float32 dequantized weights (ggufref.py), float64 activations and reductions.
Prompt IDs are an INPUT (engine tokenizer + GGUF chat template, validated separately).
"""
import json
import math
import os
import sys
import time

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import ggufref  # noqa: E402
import produce  # noqa: E402
import reference as R  # noqa: E402
import long_context as lc  # noqa: E402

fcs = lc.fcs
CHUNK = 256


class E2B:
    def __init__(self, g):
        self.g, self._cache = g, {}
        md = g.metadata
        assert md["general.architecture"] == "gemma4"
        P = "gemma4."
        self.n = int(md[P + "block_count"])
        self.hidden = int(md[P + "embedding_length"])
        self.heads = int(md[P + "attention.head_count"])
        self.kv_heads = int(md[P + "attention.head_count_kv"])
        self.window = int(md[P + "attention.sliding_window"])
        self.ple = int(md[P + "embedding_length_per_layer_input"])
        self.eps = float(md[P + "attention.layer_norm_rms_epsilon"])
        self.softcap = float(md[P + "final_logit_softcapping"])
        self.types = ["sliding_attention" if s else "full_attention" for s in md[P + "attention.sliding_window_pattern"]]
        self.first_shared = self.n - int(md[P + "attention.shared_kv_layers"])
        # G:1207-1210 store_full_length_kv: the last non-shared layer of each type.
        self.source = {t: max(i for i in range(self.first_shared) if self.types[i] == t) for t in set(self.types)}
        # reference.py's RoPE tables are the transformers formulas for exactly these parameters.
        assert (int(md[P + "attention.key_length_swa"]), int(md[P + "attention.key_length"])) == (256, 512)
        assert float(md[P + "rope.freq_base_swa"]) == 10000.0 and float(md[P + "rope.freq_base"]) == 1e6
        self.vocab = g.tensors["token_embd.weight"].dims[1]

    def vec(self, name):
        if name not in self._cache:
            self._cache[name] = self.g.rows(name)[0].astype(np.float64)
        return self._cache[name]

    def mat(self, name):
        if name not in self._cache:
            self._cache[name] = self.g.matrix(name)
        return self._cache[name]

    def has(self, name):
        return name in self.g.tensors

    def rmsnorm(self, x, weight=None):  # G:206-215 (direct scale)
        y = x * np.power(np.mean(x * x, axis=-1, keepdims=True) + self.eps, -0.5)
        return y if weight is None else y * weight

    def linear(self, name, x):
        return x @ self.mat(name).astype(np.float64).T

    def embed(self, ids):  # Gemma4TextScaledWordEmbedding, embed_scale = hidden**0.5 (float32 tensor)
        rows = np.stack([self.g.rows("token_embd.weight", int(i), 1)[0] for i in ids]).astype(np.float64)
        return rows * float(np.float32(math.sqrt(self.hidden)))

    def per_layer_inputs(self, ids, embeds):  # G:1757-1821
        n = len(ids)
        table = np.stack([self.g.rows("per_layer_token_embd.weight", int(i), 1)[0] for i in ids]).astype(np.float64)
        token = (table * float(np.float32(math.sqrt(self.ple)))).reshape(n, self.n, self.ple)
        proj = self.linear("per_layer_model_proj.weight", embeds) * self.hidden ** -0.5
        proj = self.rmsnorm(proj.reshape(n, self.n, self.ple), self.vec("per_layer_proj_norm.weight"))
        return (proj + token) * 2.0 ** -0.5

    def attention(self, layer, h, positions, cache, shared):
        lt = self.types[layer]
        sliding = lt == "sliding_attention"
        d = 256 if sliding else 512
        n = h.shape[0]
        p = f"blk.{layer}."
        cos, sin = R.rope_cos_sin(lt, positions)
        q = self.linear(p + "attn_q.weight", h).reshape(n, self.heads, d)
        q = R.apply_rope(self.rmsnorm(q, self.vec(p + "attn_q_norm.weight")), cos, sin)
        new_cache = None
        if layer >= self.first_shared:  # G:1257-1261
            keys, values, key_pos = shared[lt]
        else:
            k_raw = self.linear(p + "attn_k.weight", h).reshape(n, self.kv_heads, d)
            v_raw = (self.linear(p + "attn_v.weight", h).reshape(n, self.kv_heads, d)
                     if self.has(p + "attn_v.weight") else k_raw)
            k = R.apply_rope(self.rmsnorm(k_raw, self.vec(p + "attn_k_norm.weight")), cos, sin)
            v = self.rmsnorm(v_raw)  # v_norm, with_scale=False
            keys = np.concatenate([cache["k"], k]) if cache is not None else k
            values = np.concatenate([cache["v"], v]) if cache is not None else v
            key_pos = np.concatenate([cache["pos"], positions]) if cache is not None else np.asarray(positions)
            if layer == self.source[lt]:  # G:1275-1276 full (untruncated) states of this pass
                shared[lt] = (keys, values, key_pos)
            keep = slice(-(self.window - 1), None) if sliding else slice(None)
            new_cache = {"k": keys[keep], "v": values[keep], "pos": key_pos[keep]}
        rep = self.heads // self.kv_heads
        out = np.empty((n, self.heads, d))
        for head in range(self.heads):
            kv = head // rep
            scores = keys[:, kv, :] @ q[:, head, :].T  # scaling 1.0
            for i in range(n):
                allowed = key_pos <= positions[i]
                if sliding:
                    allowed &= key_pos > positions[i] - self.window
                w = R.softmax(np.where(allowed, scores[:, i], -np.inf))
                out[i, head] = w @ values[:, kv, :]
        return self.linear(p + "attn_output.weight", out.reshape(n, self.heads * d)), new_cache

    def layer(self, layer, x, positions, cache, per_layer, shared):  # G:1405-1466
        p = f"blk.{layer}."
        residual = x
        a, new_cache = self.attention(layer, self.rmsnorm(x, self.vec(p + "attn_norm.weight")), positions, cache, shared)
        x = residual + self.rmsnorm(a, self.vec(p + "post_attention_norm.weight"))
        residual = x
        m = self.rmsnorm(x, self.vec(p + "ffn_norm.weight"))
        m = self.linear(p + "ffn_down.weight",
                        R.gelu_tanh(self.linear(p + "ffn_gate.weight", m)) * self.linear(p + "ffn_up.weight", m))
        x = residual + self.rmsnorm(m, self.vec(p + "post_ffw_norm.weight"))
        residual = x
        g = R.gelu_tanh(self.linear(p + "inp_gate.weight", x)) * per_layer
        x = residual + self.rmsnorm(self.linear(p + "proj.weight", g), self.vec(p + "post_norm.weight"))
        return x * self.vec(p + "layer_output_scale.weight")[0], new_cache

    def extend(self, ids, caches, start, outputs=None):
        positions = np.arange(start, start + len(ids))
        x = self.embed(ids)
        ple = self.per_layer_inputs(ids, x)
        shared, new = {}, []
        for layer in range(self.n):
            x, cache = self.layer(layer, x, positions, caches[layer] if caches else None, ple[:, layer, :], shared)
            new.append(cache)
            if outputs is not None:
                outputs.append(x)
        return self.rmsnorm(x, self.vec("output_norm.weight")), new

    def logits(self, hidden, chunk=16384):  # tied head + softcap
        raw = np.empty((hidden.shape[0], self.vocab))
        for s in range(0, self.vocab, chunk):
            raw[:, s:s + chunk] = hidden @ self.g.rows("token_embd.weight", s, min(chunk, self.vocab - s)).astype(np.float64).T
        return self.softcap * np.tanh(raw / self.softcap)


def main():
    model_path, ids_path, out_dir = sys.argv[1:4]
    out_dir = os.path.abspath(out_dir)
    if os.path.exists(out_dir):
        sys.exit(f"refusing to overwrite {out_dir}")
    t0 = time.time()
    ids = [int(i) for i in json.load(open(ids_path))]
    g = ggufref.GGUF(model_path)
    m = E2B(g)
    caches, layer_rows, hidden_all = None, [[] for _ in range(m.n)], []
    for start in range(0, len(ids), CHUNK):
        outputs = []
        hidden, caches = m.extend(ids[start:start + CHUNK], caches, start, outputs)
        for layer, rows in enumerate(outputs):
            layer_rows[layer].append(rows)
        hidden_all.append(hidden)
        print(f"[{time.time() - t0:7.1f}s] encoded {start + len(hidden)}/{len(ids)}", flush=True)
    layers = np.stack([np.concatenate(r) for r in layer_rows])
    hidden = np.concatenate(hidden_all)
    logits = m.logits(hidden[-1:])
    partial = out_dir + ".partial"
    os.makedirs(partial)
    blobs = fcs.Blobs(partial)
    blobs.add("prompt.inputs.bin", "prompt.final.selected", hidden)
    blobs.add("prompt.layers.bin", "prompt.layers.selected", layers)
    summary = {"logits": fcs.add_logits(blobs, "prompt", logits, [0])}
    files = blobs.flush()
    manifest = {
        "schema": "power-llm gemma4 dense/PLE prompt golden v1", "complete": True,
        "model": {"path": model_path, "general.name": g.metadata.get("general.name"),
                  "header_sha256": produce.sha256_file(model_path, g.data_start)},
        "sources": {"transformers": "bd970fda476f204f9852fb551aa07a4b3fbc5540",
                    "modeling_gemma4.py_sha256": "ccab8e2dd80b71e9ca34e2c87291e17c40a27c755006e554da2ebf70d6616916"},
        "precision": "weights: GGUF float32 dequantization (ggufref); activations/reductions float64",
        "tolerance_contract": fcs.TOLERANCE,
        "cases": {"prompt": {"ids": ids, "tokens": len(ids), "encoder_chunk": CHUNK,
                             "logits": "position len(ids)-1 only (stored as logits row 0)",
                             "routing": "none (dense model)"}},
        "selection": {"positions": list(range(len(ids)))},
        "summary": summary, "files": files, "entries": blobs.entries,
        "resources": {"wall_seconds": time.time() - t0},
    }
    json.dump(manifest, open(os.path.join(partial, "manifest.json"), "w"), indent=1)
    os.rename(partial, out_dir)
    print(f"done {out_dir}")


if __name__ == "__main__":
    main()
