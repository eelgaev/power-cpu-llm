#!/usr/bin/env python3
"""Produce independent DiffusionGemma golden traces + manifest.

    numactl --physcpubind=CPUS --membind=NODE env OPENBLAS_NUM_THREADS=N ... \\
      python3 produce.py diffusiongemma-26B-A4B-it-Q4_K_M.gguf OUT_DIR [--stages blocks,full]

Every case is computed twice: `f64` (the golden values written to disk) and
`f32` storage emulation. Their difference is recorded per entry as the expected
deviation of a correct F32-activation / F64-reduction engine.
"""
import argparse
import hashlib
import json
import os
import platform
import resource
import sys
import time

import numpy as np

import ggufref
import reference

REVISIONS = {
    "google/diffusiongemma-26B-A4B-it": "f7f5b7f5fa82ffc52addd066915886d497f5517b",
    "huggingface/transformers": "bd970fda476f204f9852fb551aa07a4b3fbc5540",
    "unsloth/diffusiongemma-26B-A4B-it-GGUF": "f4183a2c7a354128d02545752303c4354d165bf0",
    "danielhanchen/llama.cpp PR 24423": "3dac51dc80b1400929266453c6ced6748ccf6862",
}
PINNED_SOURCE_SHA256 = {
    "modeling_diffusion_gemma.py": "3a114c48ccd270577c6caf4f04009442f706aea4d5cb0a48f08beaa0c2b602ae",
    "modeling_rope_utils.py": "9592ba129bf723ed3fa54afcc48ddd3d40d2dac2947ce5cc37803c496ca320e8",
    "generation_diffusion_gemma.py": "19eb26c4415d0c7b1431327f542a0abd8134ca5d4954f6d2301a58256c447348",
}
ENCODER_IDS = [2]  # <bos>
CANVAS_A = [42]
CANVAS_C = [42, 9259]
SC_TEMPERATURE = 0.8
TOPK_LOGITS = 64


def synthetic_hidden(n):
    """Deterministic block input, rows t = 0..n-1, columns i = 0..2815."""
    i = np.arange(reference.HIDDEN, dtype=np.float64)[None, :]
    t = np.arange(n, dtype=np.float64)[:, None]
    return 0.5 * np.sin(0.013 * (i + 1) * (t + 1)) + 0.25 * np.cos(0.007 * i + t)


def sha256_file(path, limit=None):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        remaining = limit
        while True:
            block = f.read(1 << 20 if remaining is None else min(1 << 20, remaining))
            if not block:
                break
            h.update(block)
            if remaining is not None:
                remaining -= len(block)
                if remaining == 0:
                    break
    return h.hexdigest()


class Writer:
    """Little-endian float32 blobs grouped per file, described in the manifest."""

    def __init__(self, out_dir):
        self.out_dir = out_dir
        self.entries = []
        self.files = {}

    def add(self, file, name, golden, other=None, **meta):
        golden = np.asarray(golden, dtype=np.float64)
        data = golden.astype("<f4")
        assert np.all(np.isfinite(data)), name
        buf = self.files.setdefault(file, bytearray())
        entry = {"name": name, "file": file, "offset": len(buf), "shape": list(data.shape), "dtype": "<f4",
                 "rms": float(np.sqrt(np.mean(golden ** 2))), "max_abs": float(np.max(np.abs(golden)))}
        if other is not None:
            diff = np.abs(np.asarray(other, dtype=np.float64) - golden)
            entry["f32_storage_deviation"] = {
                "max_abs": float(diff.max()), "rms": float(np.sqrt(np.mean(diff ** 2))),
                "max_rel_to_entry_max": float(diff.max() / max(entry["max_abs"], 1e-30))}
        entry.update(meta)
        buf += data.tobytes()
        self.entries.append(entry)

    def flush(self):
        for file, buf in self.files.items():
            path = os.path.join(self.out_dir, file)
            os.makedirs(os.path.dirname(path), exist_ok=True)
            with open(path, "wb") as f:
                f.write(buf)
        return {f: {"bytes": len(b), "sha256": hashlib.sha256(bytes(b)).hexdigest()} for f, b in self.files.items()}


def router_json(traces):
    return [{"layer": i, "top_ids": t["top_ids"].tolist(),
             "top_weights": [[float(w) for w in row] for row in t["top_weights"]]} for i, t in enumerate(traces)]


def router_agreement(a, b):
    return all(np.array_equal(x["top_ids"], y["top_ids"]) for x, y in zip(a, b))


def top_logits(logits, k=TOPK_LOGITS):
    out = []
    for row in logits:
        ids = np.argsort(-row, kind="stable")[:k]
        out.append({"ids": ids.tolist(), "scores": [float(row[i]) for i in ids]})
    return out


def stage_blocks(models, w, manifest):
    x = synthetic_hidden(2)
    w.add("blocks.f32", "blocks.input", x)
    cases = []
    for layer in (0, 5):
        for mode in ("encoder", "decoder"):
            res = {}
            for key, m in models.items():
                info = {}
                out, cache = m.layer(layer, x, np.array([0, 1]), None, encoder=(mode == "encoder"), trace=info)
                res[key] = dict(info, output=out, cache=cache)
            g, f = res["f64"], res["f32"]
            prefix = f"blocks.layer.{layer}.{mode}"
            for part in ("post_attention", "shared", "routed", "output", "router_probs", "top_weights"):
                w.add("blocks.f32", f"{prefix}.{part}", g[part], f[part])
            if mode == "encoder":
                w.add("blocks.f32", f"{prefix}.cache_k", g["cache"]["k"], f["cache"]["k"])
                w.add("blocks.f32", f"{prefix}.cache_v", g["cache"]["v"], f["cache"]["v"])
            cases.append({"layer": layer, "mode": mode, "layer_type": models["f64"].layer_types[layer],
                          "top_ids": g["top_ids"].tolist(),
                          "top_ids_agree_f32_storage": bool(np.array_equal(g["top_ids"], f["top_ids"]))})
    manifest["blocks"] = {
        "input": "blocks.input: x[t, i] = 0.5*sin(0.013*(i+1)*(t+1)) + 0.25*cos(0.007*i + t), t in {0,1}, i in 0..2815",
        "positions": [0, 1],
        "encoder_mode": "causal attention from an empty cache, enc_layer_output_scale",
        "decoder_mode": "bidirectional attention over the two rows only (empty cache), layer_output_scale",
        "parts": {"post_attention": "residual + post_attention_norm(attn)", "shared": "post_ffw_norm_1(shared MLP)",
                  "routed": "post_ffw_norm_2(sum of weighted experts)", "output": "block output after layer scalar",
                  "router_probs": "softmax over 128 experts", "top_weights": "renormalized top-8 x per-expert scale",
                  "cache_k": "[t, kv_head, d] after k_norm + RoPE", "cache_v": "[t, kv_head, d] after v_norm"},
        "cases": cases,
    }


def stage_full(models, w, manifest):
    runs = {}
    for key, m in models.items():
        t0 = time.time()
        enc_traces = []
        enc_final, caches = m.encode(ENCODER_IDS, layer_traces=enc_traces)
        # Case A: zero self-conditioning, canvas [42] at position 1.
        dec_a, extra_a = [], {}
        hid_a = m.decode(CANVAS_A, caches, None, layer_traces=dec_a, extra=extra_a)
        logits_a = m.logits(hid_a)
        runs[key] = dict(enc=enc_traces, enc_final=enc_final, caches=caches, dec_a=dec_a, extra_a=extra_a,
                         hid_a=hid_a, logits_a=logits_a, seconds_a=time.time() - t0)
    g, f = runs["f64"], runs["f32"]
    # Case B inputs derive from the GOLDEN case-A logits as stored (float32), for both modes.
    processed_b = g["logits_a"].astype(np.float32).astype(np.float64) / SC_TEMPERATURE
    canvas_b = [int(np.argmax(g["logits_a"][0]))]
    for key, m in models.items():
        t0 = time.time()
        r = runs[key]
        assert all(len(c["pos"]) == len(ENCODER_IDS) for c in r["caches"])  # decoder never extends the cache
        dec_b, extra_b = [], {}
        r["hid_b"] = m.decode(canvas_b, r["caches"], processed_b, layer_traces=dec_b, extra=extra_b)
        r["dec_b"], r["extra_b"], r["logits_b"] = dec_b, extra_b, m.logits(r["hid_b"])
        dec_c = []
        r["hid_c"] = m.decode(CANVAS_C, r["caches"], None, layer_traces=dec_c)
        r["dec_c"], r["logits_c"] = dec_c, m.logits(r["hid_c"])
        r["seconds_bc"] = time.time() - t0
    # Diagnostic only: the engine's documented UNSCALED soft-embedding convention.
    unscaled_logits = models["f64"].logits(models["f64"].decode(canvas_b, g["caches"], processed_b, scaled_soft=False))

    w.add("encoder.f32", "encoder.embedding", models["f64"].embed(ENCODER_IDS), models["f32"].embed(ENCODER_IDS))
    for i in range(30):
        w.add("encoder.f32", f"encoder.layer.{i}", g["enc"][i]["output"], f["enc"][i]["output"])
    w.add("encoder.f32", "encoder.final", g["enc_final"], f["enc_final"])
    for i in range(30):
        w.add("cache.f32", f"encoder.cache.{i}.k", g["caches"][i]["k"], f["caches"][i]["k"])
        w.add("cache.f32", f"encoder.cache.{i}.v", g["caches"][i]["v"], f["caches"][i]["v"])
    for case, canvas in (("a", CANVAS_A), ("b", canvas_b), ("c", CANVAS_C)):
        file = f"decoder_{case}.f32"
        if case != "c":
            ex, exf = g[f"extra_{case}"], f[f"extra_{case}"]
            w.add(file, f"decoder_{case}.embedding", ex["embeds"], exf["embeds"])
            w.add(file, f"decoder_{case}.soft_embedding", ex["soft"], exf["soft"])
            w.add(file, f"decoder_{case}.self_conditioned", ex["self_conditioned"], exf["self_conditioned"])
        for i in range(30):
            w.add(file, f"decoder_{case}.layer.{i}", g[f"dec_{case}"][i]["output"], f[f"dec_{case}"][i]["output"])
        w.add(file, f"decoder_{case}.final", g[f"hid_{case}"], f[f"hid_{case}"])
        if case != "c":
            w.add(f"logits_{case}.f32", f"decoder_{case}.logits", g[f"logits_{case}"], f[f"logits_{case}"])

    def logit_summary(key):
        gl, fl = g[key], f[key]
        return {"top": top_logits(gl), "argmax_agree_f32_storage": bool(np.array_equal(gl.argmax(-1), fl.argmax(-1))),
                "f32_storage_max_abs": float(np.abs(gl - fl).max())}

    diag = np.abs(unscaled_logits - g["logits_b"])
    manifest["full"] = {
        "encoder": {"ids": ENCODER_IDS, "positions": [0], "attention": "causal; cache = post-norm/RoPE K and normed V",
                    "router": router_json(g["enc"]), "router_top_ids_agree_f32_storage": router_agreement(g["enc"], f["enc"])},
        "decoder_a": {"ids": CANVAS_A, "positions": [1], "self_conditioning": "none (zero soft embedding)",
                      "router": router_json(g["dec_a"]), "router_top_ids_agree_f32_storage": router_agreement(g["dec_a"], f["dec_a"]),
                      "logits": logit_summary("logits_a")},
        "decoder_b": {"ids": canvas_b, "positions": [1],
                      "canvas_rule": "argmax of golden decoder_a.logits",
                      "self_conditioning": f"processed logits = float32(golden decoder_a.logits) / {SC_TEMPERATURE}; "
                                           "soft = softmax(processed) @ token_embd * sqrt(2816) (M:1278-1281)",
                      "router": router_json(g["dec_b"]), "router_top_ids_agree_f32_storage": router_agreement(g["dec_b"], f["dec_b"]),
                      "logits": logit_summary("logits_b")},
        "decoder_c": {"ids": CANVAS_C, "positions": [1, 2], "self_conditioning": "none",
                      "attention": "bidirectional over cache + both canvas positions",
                      "router": router_json(g["dec_c"]), "router_top_ids_agree_f32_storage": router_agreement(g["dec_c"], f["dec_c"]),
                      "logits": logit_summary("logits_c"),
                      "note": f"full logits not stored for this case; top-{TOPK_LOGITS} per position"},
        "cache_unchanged_by_decoder": True,
        "diagnostic_unscaled_soft_embedding": {
            "what": "decoder_b recomputed with soft = softmax @ token_embd WITHOUT sqrt(2816) (engine docs convention)",
            "logits_max_abs_diff": float(diag.max()), "logits_rms_diff": float(np.sqrt(np.mean(diag ** 2))),
            "argmax_same": bool(unscaled_logits.argmax() == g["logits_b"].argmax()),
            "vs_f32_storage_max_abs": float(np.abs(g["logits_b"] - f["logits_b"]).max())},
        "seconds": {k: {"a": runs[k]["seconds_a"], "b_c": runs[k]["seconds_bc"]} for k in runs},
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("model")
    ap.add_argument("out_dir")
    ap.add_argument("--stages", default="blocks,full")
    args = ap.parse_args()
    t0 = time.time()
    g = ggufref.GGUF(args.model)
    models = {"f64": reference.Model(g, "f64"), "f32": reference.Model(g, "f32")}
    here = os.path.dirname(os.path.abspath(__file__))
    st = os.stat(args.model)
    manifest = {
        "schema": "power-llm numerical golden v1",
        "model": {"path": args.model, "bytes": st.st_size, "mtime": int(st.st_mtime),
                  "general.name": g.metadata.get("general.name"), "tensors": len(g.tensors),
                  "data_start": g.data_start,
                  "header_sha256": sha256_file(args.model, g.data_start),
                  "identity_note": "sha256 covers metadata + tensor directory only; the weight blob is not re-hashed"},
        "sources": {"revisions": REVISIONS, "pinned_file_sha256": PINNED_SOURCE_SHA256},
        "tooling": {"python": platform.python_version(), "numpy": np.__version__,
                    "blas": "OpenBLAS (numpy build); float64 GEMV",
                    "scripts_sha256": {n: sha256_file(os.path.join(here, n))
                                       for n in ("ggufref.py", "reference.py", "produce.py", "crosscheck_dequant.py")}},
        "arithmetic": {
            "weights": "GGUF float32 dequantization (independent NumPy decoders, bit-identical to gguf-py on sampled rows)",
            "golden": "float64 activations and reductions, stored as float32",
            "f32_storage_deviation": "|golden - same computation with every intermediate rounded to float32|; "
                                     "expected order of a correct F32-storage/F64-reduction engine",
            "rms_norm": "x * (mean(x^2) + 1e-6)^-0.5 * w (direct weight, no 1+w)",
            "rope": "float32 inv_freq/cos/sin; NeoX rotate_half; local theta 1e4 dim 256; global theta 1e6, "
                    "64 rotated pairs (i, i+256), remaining 192 frequencies exactly 0",
            "attention": "scale 1.0; GQA query head j -> kv head j // (16/kv_heads); global V = raw K projection, "
                         "v_norm unscaled; encoder causal (+1024 window on sliding layers); decoder bidirectional, "
                         "cache read-only",
            "moe": "router on un-normed residual: rmsnorm(no scale) * router.scale * 2816^-0.5 -> softmax(128) -> top-8 "
                   "-> renormalize -> * per_expert_scale; experts on pre_ffw_norm_2(residual); gate_up rows "
                   "[gate 704 | up 704]; gelu_tanh; ascending expert accumulation",
            "layer_scalar": "encoder: enc_layer_output_scale; decoder: layer_output_scale",
            "embedding": f"token_embd row * float32(sqrt(2816)) = {reference.EMBED_SCALE!r}",
            "self_conditioning": "soft = softmax(processed logits) @ token_embd * sqrt(2816); "
                                 "post_norm(embeds + down(gelu(gate(pre_norm(soft))) * up(...))) without scale",
            "logits": "tied token_embd (unscaled) on final-normed hidden, 30 * tanh(x / 30)",
        },
    }
    w = Writer(args.out_dir)
    stages = args.stages.split(",")
    if "blocks" in stages:
        stage_blocks(models, w, manifest)
    if "full" in stages:
        stage_full(models, w, manifest)
    manifest["files"] = w.flush()
    manifest["entries"] = w.entries
    manifest["resources"] = {"wall_seconds": time.time() - t0,
                             "peak_rss_mib": resource.getrusage(resource.RUSAGE_SELF).ru_maxrss / 1024,
                             "cpus": os.sched_getaffinity(0) and sorted(os.sched_getaffinity(0)),
                             "argv": sys.argv}
    with open(os.path.join(args.out_dir, "manifest.json"), "w") as f:
        json.dump(manifest, f, indent=1)
        f.write("\n")
    print(json.dumps({"seconds": manifest["resources"]["wall_seconds"], "files": manifest["files"]}, indent=1))


if __name__ == "__main__":
    main()
