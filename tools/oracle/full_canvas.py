#!/usr/bin/env python3
"""Independent 256-position + canvas-extension oracle for DiffusionGemma (test tooling).

    RUN="numactl --physcpubind=CPUS --membind=NODE env OPENBLAS_NUM_THREADS=N python3"
    $RUN full_canvas.py --self-test MODEL                       # ~2 min: batched experts + old golden case A
    $RUN full_canvas.py MODEL OUT_DIR --work WORK_DIR           # full run, resumable via WORK_DIR checkpoints
    $RUN full_canvas/verify.py OUT_DIR                          # containment/size/shape/finite/hash checks

Expected values come only from the independent NumPy reference (reference.py +
full_canvas/model.py) over the GGUF via ggufref.py. Engine output is never read.
OUT_DIR must not exist; blobs are written to OUT_DIR.partial, the manifest last,
then the directory is renamed.
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

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import ggufref  # noqa: E402
import produce  # noqa: E402  (only for sha256_file / revisions; not modified)
import reference  # noqa: E402
from full_canvas.model import BatchedModel  # noqa: E402

VOCAB, CANVAS = 262144, 256
SELECTED = [0, 1, 2, 7, 15, 31, 63, 127, 128, 191, 254, 255]
TOPK_LOGITS, TOPK_ROUTER = 16, 10
CACHE_LAYERS = [0, 5, 11, 23, 29]
CACHE_POSITIONS = [0, 1, 2, 127, 128, 129, 255, 256]  # positions within the extended (257-token) cache
FULL_LOGIT_ROWS = [0, 255]  # complete 262144-wide rows stored for case 1 only
TEMPERATURE = 0.8
TOLERANCE = {"atol": 1e-4, "rtol": 1e-4, "rule": "|engine - expected| <= atol + rtol*|expected| per value"}


def canvas_case1():
    """Deterministic, independently specified first canvas (no RNG)."""
    ids = [(1000 + i * 7919) % VOCAB for i in range(CANVAS)]
    for i in range(40, 44):
        ids[i] = 9259            # repeated run
    for i in range(126, 130):
        ids[i] = 42              # repeated across the 127/128 boundary
    for i in range(253, 256):
        ids[i] = 108             # repeated at the end
    return ids


def canvas_case3():
    ids = [(5003 + i * 104729) % VOCAB for i in range(CANVAS)]
    for i in range(0, 3):
        ids[i] = 1902
    ids[127] = ids[128] = 236743
    ids[254] = ids[255] = 9259
    return ids


CANVAS_RULES = {
    "case1": "ids[i] = (1000 + 7919*i) % 262144; ids[40..44] = 9259; ids[126..130] = 42; ids[253..256] = 108",
    "case3": "ids[i] = (5003 + 104729*i) % 262144; ids[0..3] = 1902; ids[127] = ids[128] = 236743; "
             "ids[254] = ids[255] = 9259",
}


class Blobs:
    """Little-endian blobs grouped per file; finite floats; recorded with shape/dtype/offset."""

    def __init__(self, out_dir):
        self.out_dir, self.entries, self.files = out_dir, [], {}

    def add(self, file, name, array, dtype="<f4", **meta):
        data = np.ascontiguousarray(np.asarray(array).astype(dtype))
        if data.dtype.kind == "f":
            assert np.all(np.isfinite(data)), name
        buf = self.files.setdefault(file, bytearray())
        entry = {"name": name, "file": file, "offset": len(buf), "shape": list(data.shape), "dtype": dtype}
        if data.dtype.kind == "f":
            a = np.asarray(array, dtype=np.float64)
            entry.update(rms=float(np.sqrt(np.mean(a ** 2))), max_abs=float(np.max(np.abs(a))))
        entry.update(meta)
        buf += data.tobytes()
        self.entries.append(entry)

    def flush(self):
        out = {}
        for file, buf in self.files.items():
            with open(os.path.join(self.out_dir, file), "wb") as f:
                f.write(buf)
            out[file] = {"bytes": len(buf), "sha256": hashlib.sha256(bytes(buf)).hexdigest()}
        return out


def logit_summary(logits):
    """Exhaustive over all 262144 columns per position: argmax, top-k, margin, logsumexp."""
    top = np.argsort(-logits, axis=1, kind="stable")[:, :TOPK_LOGITS]
    scores = np.take_along_axis(logits, top, axis=1)
    m = logits.max(axis=1, keepdims=True)
    lse = (m[:, 0] + np.log(np.exp(logits - m).sum(axis=1)))
    return top, scores, lse


def add_logits(blobs, prefix, logits, full_rows=()):
    top, scores, lse = logit_summary(logits)
    blobs.add("logits.bin", f"{prefix}.logits.top_ids", top, "<i4")
    blobs.add("logits.bin", f"{prefix}.logits.top_scores", scores)
    blobs.add("logits.bin", f"{prefix}.logits.logsumexp", lse)
    for p in full_rows:
        blobs.add("logits.bin", f"{prefix}.logits.full_row.{p}", logits[p])
    margin = scores[:, 0] - scores[:, 1]
    return {"argmax": top[:, 0].tolist(), "min_top1_top2_margin": float(margin.min()),
            "positions_with_margin_below_1e-3": [int(i) for i in np.nonzero(margin < 1e-3)[0]],
            "exhaustive": "argmax/top-16/logsumexp over all 262144 columns for every position",
            "full_rows": list(full_rows)}


def add_layers(blobs, prefix, traces, rows):
    """Selected rows of every layer output, plus routing for ALL rows and layers."""
    blobs.add(f"{prefix}.layers.bin", f"{prefix}.layers.selected", np.stack([t["output"][rows] for t in traces]))
    ids = np.stack([t["top_ids"] for t in traces])                   # [30, n, 8]
    weights = np.stack([t["top_weights"] for t in traces])           # [30, n, 8]
    probs = np.stack([t["router_probs"] for t in traces])            # [30, n, 128]
    order = np.argsort(-probs, axis=2, kind="stable")[:, :, :TOPK_ROUTER]
    top_probs = np.take_along_axis(probs, order, axis=2)
    assert np.array_equal(order[:, :, :8], ids)
    blobs.add(f"{prefix}.routing.bin", f"{prefix}.routing.top8_ids", ids, "<i4")
    blobs.add(f"{prefix}.routing.bin", f"{prefix}.routing.top8_weights", weights)
    blobs.add(f"{prefix}.routing.bin", f"{prefix}.routing.top10_ids", order, "<i4")
    blobs.add(f"{prefix}.routing.bin", f"{prefix}.routing.top10_probs", top_probs)
    gap = top_probs[:, :, 7] - top_probs[:, :, 8]
    worst = np.unravel_index(np.argmin(gap), gap.shape)
    return {"rank8_minus_rank9_prob_min": float(gap.min()), "at_layer_position": [int(worst[0]), int(worst[1])],
            "count_below_1e-6": int((gap < 1e-6).sum())}


def checkpoint(work, name, compute):
    path = os.path.join(work, name + ".npz")
    if os.path.exists(path):
        with np.load(path, allow_pickle=False) as z:
            return {k: z[k] for k in z.files}, True
    t0 = time.time()
    data = compute()
    data["_seconds"] = np.array(time.time() - t0)
    np.savez(path + ".tmp.npz", **data)
    os.replace(path + ".tmp.npz", path)
    return data, False


def flatten_traces(traces, prefix):
    keys = ("output", "top_ids", "top_weights", "router_probs")
    return {f"{prefix}.{k}": np.stack([t[k] for t in traces]) for k in keys}


def unflatten_traces(data, prefix):
    keys = ("output", "top_ids", "top_weights", "router_probs")
    return [{k: data[f"{prefix}.{k}"][i] for k in keys} for i in range(30)]


def cache_arrays(caches, prefix):
    out = {}
    for i, c in enumerate(caches):
        out[f"{prefix}.{i}.k"], out[f"{prefix}.{i}.v"], out[f"{prefix}.{i}.pos"] = c["k"], c["v"], c["pos"]
    return out


def cache_list(data, prefix):
    return [{"k": data[f"{prefix}.{i}.k"], "v": data[f"{prefix}.{i}.v"], "pos": data[f"{prefix}.{i}.pos"]}
            for i in range(30)]


def self_test(model_path):
    g = ggufref.GGUF(model_path)
    ref, batched = reference.Model(g, "f64"), BatchedModel(g, "f64")
    rng = np.random.default_rng(7)
    x = rng.standard_normal((5, reference.HIDDEN))
    _, ids, weights = ref.route(0, x)
    a, b = ref.experts(0, x, ids, weights), batched.experts(0, x, ids, weights)
    rel = float(np.abs(a - b).max() / np.abs(a).max())
    assert rel < 1e-12, rel
    # Reproduce the existing independent golden decoder_a (BOS encoder, canvas [42]).
    golden_dir = os.path.join(HERE, "..", "..", "..", "fixtures", "numerical")
    manifest = json.load(open(os.path.join(golden_dir, "manifest.json")))
    def golden(name):
        e = next(e for e in manifest["entries"] if e["name"] == name)
        raw = np.fromfile(os.path.join(golden_dir, e["file"]), dtype="<f4")
        return raw[e["offset"] // 4: e["offset"] // 4 + int(np.prod(e["shape"]))].reshape(e["shape"])
    _, caches = batched.encode([2])
    hidden = batched.decode([42], caches)
    final_err = float(np.abs(hidden.astype(np.float32) - golden("decoder_a.final")).max())
    logits = batched.logits(hidden)
    logit_err = float(np.abs(logits.astype(np.float32) - golden("decoder_a.logits")).max())
    _, extended = batched.encode_extend([42, 9259], caches)
    full_enc, _ = batched.encode([2, 42, 9259])
    ext_ok = all(np.allclose(e["k"], f["k"], atol=1e-9, rtol=1e-9) for e, f in zip(extended, ref.encode([2, 42, 9259])[1]))
    print(json.dumps({"batched_experts_max_rel": rel, "decoder_a_final_max_abs": final_err,
                      "decoder_a_logits_max_abs": logit_err, "encode_extend_matches_one_pass_cache": ext_ok}))
    assert final_err < 1e-5 and logit_err < 1e-4 and ext_ok
    print("SELF-TEST OK")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("model")
    ap.add_argument("out_dir", nargs="?")
    ap.add_argument("--work", help="checkpoint directory (resumable)")
    ap.add_argument("--self-test", action="store_true")
    args = ap.parse_args()
    if args.self_test:
        return self_test(args.model)
    out_dir = os.path.abspath(args.out_dir)
    if os.path.exists(out_dir):
        sys.exit(f"refusing to overwrite existing {out_dir}")
    partial = out_dir + ".partial"
    os.makedirs(partial, exist_ok=True)
    work = os.path.abspath(args.work or out_dir + ".work")
    os.makedirs(work, exist_ok=True)
    t_start = time.time()
    g = ggufref.GGUF(args.model)
    m = BatchedModel(g, "f64")
    c1, c3 = canvas_case1(), canvas_case3()
    log = lambda msg: print(f"[{time.time() - t_start:8.1f}s] {msg}", flush=True)  # noqa: E731

    log("prompt encoder [2]")
    prompt, cached = checkpoint(work, "prompt", lambda: cache_arrays(m.encode([2])[1], "cache"))
    prompt_caches = cache_list(prompt, "cache")

    def case1():
        traces, extra = [], {}
        hidden = m.decode(c1, prompt_caches, None, layer_traces=traces, extra=extra)
        return {**flatten_traces(traces, "t"), "hidden": hidden, "logits": m.logits(hidden),
                "embeds": extra["embeds"], "sc": extra["self_conditioned"]}
    log("case 1: 256-position decoder, zero self-conditioning")
    r1, cached = checkpoint(work, "case1", case1)
    a1 = [int(i) for i in np.argmax(r1["logits"], axis=1)]
    processed = (r1["logits"].astype(np.float64) / TEMPERATURE).astype(np.float32)

    def case2():
        traces, extra = [], {}
        hidden = m.decode(a1, prompt_caches, processed.astype(np.float64), layer_traces=traces, extra=extra)
        return {**flatten_traces(traces, "t"), "hidden": hidden, "logits": m.logits(hidden),
                "embeds": extra["embeds"], "soft": extra["soft"], "sc": extra["self_conditioned"]}
    log("case 2: decoder on case-1 argmax with processed self-conditioning")
    r2, _ = checkpoint(work, "case2", case2)

    def case3_encode():
        traces = []
        hidden, ext = m.encode_extend(a1, prompt_caches, layer_traces=traces)
        return {**flatten_traces(traces, "t"), "hidden": hidden, **cache_arrays(ext, "cache")}
    log("case 3a: causal encoder extension with the 256 case-1 argmax IDs at positions 1..256")
    r3e, _ = checkpoint(work, "case3_encode", case3_encode)
    ext_caches = cache_list(r3e, "cache")

    def case3_decode():
        traces, extra = [], {}
        hidden = m.decode(c3, ext_caches, None, layer_traces=traces, extra=extra)
        return {**flatten_traces(traces, "t"), "hidden": hidden, "logits": m.logits(hidden),
                "embeds": extra["embeds"], "sc": extra["self_conditioned"]}
    log("case 3b: decoder at positions 257..512 over the extended cache, zero self-conditioning")
    r3d, _ = checkpoint(work, "case3_decode", case3_decode)
    for c in ext_caches:  # decoder must not have modified the extended cache
        assert len(c["pos"]) == 257

    log("writing artifacts")
    sel = np.array(SELECTED)
    blobs = Blobs(partial)
    summary = {}
    for name, r, extra_keys in (("case1", r1, ("embeds", "sc")), ("case2", r2, ("embeds", "soft", "sc")),
                                ("case3_decode", r3d, ("embeds", "sc"))):
        for k in extra_keys:
            blobs.add(f"{name}.inputs.bin", f"{name}.{ {'embeds': 'embedding', 'soft': 'soft_embedding', 'sc': 'self_conditioned'}[k] }.selected", r[k][sel])
        blobs.add(f"{name}.inputs.bin", f"{name}.final.selected", r["hidden"][sel])
        routing = add_layers(blobs, name, unflatten_traces(r, "t"), sel)
        logits = add_logits(blobs, name, r["logits"], FULL_LOGIT_ROWS if name == "case1" else ())
        summary[name] = {"routing_margin": routing, "logits": logits}
    # Processed self-conditioning evidence (input to case 2), exhaustive per position.
    top, scores, lse = logit_summary(processed.astype(np.float64))
    blobs.add("logits.bin", "case2.processed_sc.top_ids", top, "<i4")
    blobs.add("logits.bin", "case2.processed_sc.top_scores", scores)
    blobs.add("logits.bin", "case2.processed_sc.logsumexp", lse)
    # Encoder extension: selected rows (cache positions 1 + p for canvas index p) and all routing.
    routing = add_layers(blobs, "case3_encode", unflatten_traces(r3e, "t"), sel)
    blobs.add("case3_encode.inputs.bin", "case3_encode.final.selected", r3e["hidden"][sel])
    summary["case3_encode"] = {"routing_margin": routing}
    for i in range(30):
        blobs.add("cache.bin", f"prompt_cache.{i}.k", prompt_caches[i]["k"])
        blobs.add("cache.bin", f"prompt_cache.{i}.v", prompt_caches[i]["v"])
    cpos = np.array(CACHE_POSITIONS)
    for i in CACHE_LAYERS:
        blobs.add("cache.bin", f"extended_cache.{i}.k.selected", ext_caches[i]["k"][cpos])
        blobs.add("cache.bin", f"extended_cache.{i}.v.selected", ext_caches[i]["v"][cpos])
    files = blobs.flush()
    total = sum(f["bytes"] for f in files.values())
    assert total <= 25 * 1024 * 1024, total

    st = os.stat(args.model)
    scripts = ["reference.py", "ggufref.py", "produce.py", "full_canvas.py", "full_canvas/model.py", "full_canvas/verify.py"]
    manifest = {
        "schema": "power-llm full-canvas golden v1",
        "complete": False,
        "model": {"path": args.model, "bytes": st.st_size, "mtime": int(st.st_mtime),
                  "general.name": g.metadata.get("general.name"), "data_start": g.data_start,
                  "header_sha256": produce.sha256_file(args.model, g.data_start),
                  "identity_note": "sha256 covers metadata + tensor directory; weight blob not re-hashed"},
        "sources": {"revisions": produce.REVISIONS, "pinned_file_sha256": produce.PINNED_SOURCE_SHA256,
                    "equations": "docs/validation.md (unchanged reference.py)"},
        "tooling": {"python": platform.python_version(), "numpy": np.__version__,
                    "blas": "OpenBLAS (numpy build), OPENBLAS_NUM_THREADS=" + os.environ.get("OPENBLAS_NUM_THREADS", "?"),
                    "cpus": sorted(os.sched_getaffinity(0)),
                    "scripts_sha256": {s: produce.sha256_file(os.path.join(HERE, s)) for s in scripts}},
        "precision": "weights: GGUF float32 dequantization; activations/reductions float64; blobs float32 (ids int32). "
                     "Expert products batched per expert (same equations/accumulation order, BLAS grouping only).",
        "tolerance_contract": TOLERANCE,
        "cases": {
            "prompt": {"encoder_ids": [2], "positions": [0]},
            "case1": {"decoder_ids": c1, "rule": CANVAS_RULES["case1"], "positions": "1..256",
                      "self_conditioning": "none (zero soft embedding)"},
            "case2": {"decoder_ids": a1, "rule": "argmax of the independent case-1 logits (first index on ties)",
                      "positions": "1..256", "self_conditioning":
                      f"processed = float32(float64(case-1 softcapped logits) / {TEMPERATURE}); "
                      "soft = softmax(processed) @ token_embd * float32(sqrt(2816))"},
            "case3": {"encoder_extension_ids": a1, "encoder_positions": "1..256 (causal, after the [2] prompt cache)",
                      "decoder_ids": c3, "rule": CANVAS_RULES["case3"], "decoder_positions": "257..512",
                      "self_conditioning": "none (fresh zero)",
                      "cache": {"total_tokens": 257, "retained_per_layer": [len(c["pos"]) for c in ext_caches],
                                "retention_note": "257 <= sliding window 1024: local layers retain every position; "
                                                  "global layers retain all; nothing evicted or truncated",
                                "decoder_left_cache_unchanged": True}},
        },
        "selection": {"positions": SELECTED, "position_meaning": "canvas index p (absolute position = cache length + p)",
                      "cache_layers": CACHE_LAYERS, "cache_positions": CACHE_POSITIONS,
                      "full_logit_rows": {"case1": FULL_LOGIT_ROWS}},
        "routing_policy": "top-8 by descending router probability; ties broken toward the lower expert ID (stable "
                          "argsort); weights = renormalized top-8 x per-expert scale; top10 probs give the rank-8/9 margin",
        "summary": summary,
        "files": files,
        "total_bytes": total,
        "entries": blobs.entries,
        "resources": {"wall_seconds": time.time() - t_start,
                      "case_seconds": {k: float(v["_seconds"]) for k, v in
                                       (("case1", r1), ("case2", r2), ("case3_encode", r3e), ("case3_decode", r3d))
                                       if "_seconds" in v},
                      "peak_rss_mib": resource.getrusage(resource.RUSAGE_SELF).ru_maxrss / 1024},
    }
    manifest["complete"] = True
    tmp = os.path.join(partial, "manifest.json.tmp")
    with open(tmp, "w") as f:
        json.dump(manifest, f, indent=1)
        f.write("\n")
    os.replace(tmp, os.path.join(partial, "manifest.json"))  # manifest LAST
    os.rename(partial, out_dir)
    log(f"done: {total} bytes in {out_dir}")


if __name__ == "__main__":
    main()
