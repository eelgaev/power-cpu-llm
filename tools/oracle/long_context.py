#!/usr/bin/env python3
"""Independent long-context oracle for DiffusionGemma (test tooling; engine output is never read).

    RUN="numactl --physcpubind=CPUS --membind=NODE env OPENBLAS_NUM_THREADS=N python3"
    $RUN long_context.py MODEL PROMPT_IDS.json OUT_DIR --work WORK_DIR     # resumable via WORK_DIR

Long prompts exercise what the 256-position fixtures cannot: sliding-window eviction, global attention over
thousands of keys and RoPE at large positions. Cache semantics follow the pinned transformers source
(cache_utils.py DynamicSlidingWindowLayer at bd970fda): sliding-attention layers keep only the last
`sliding_window - 1` = 1023 tokens after every update; the decoder attends (bidirectionally, no further mask) to
that retained cache plus the canvas; global layers keep every token. The encoder runs causally in 256-token
chunks (chunking does not change results under these semantics). Positions are explicit (the reference's
`decode`/`encode_extend` derive them from layer 0's cache length, which is wrong once layer 0 is truncated).

The prompt token IDs are an INPUT (produced by the engine tokenizer, which is validated exactly against the HF
tokenizer separately, docs/tokenizer.md); all expected tensors come from reference.py + full_canvas/model.py.
"""
import argparse
import importlib.util
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
import produce  # noqa: E402
import reference  # noqa: E402
from full_canvas.model import BatchedModel  # noqa: E402

_spec = importlib.util.spec_from_file_location("full_canvas_script", os.path.join(HERE, "full_canvas.py"))
fcs = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(fcs)

WINDOW, CHUNK, CANVAS = 1024, 256, 256
SELECTED = [0, 1, 2, 7, 15, 31, 63, 127, 128, 191, 254, 255]   # canvas indices
CACHE_LAYERS = [0, 5, 11, 23, 28, 29]


def canvas_rule():
    ids = [(7001 + i * 6151) % 262144 for i in range(CANVAS)]
    for i in range(60, 64):
        ids[i] = 9259
    return ids


CANVAS_RULE = "ids[i] = (7001 + 6151*i) % 262144; ids[60..64] = 9259"


class LongModel(BatchedModel):
    def extend(self, ids, caches, start, layer_traces=None):
        """Causal encoder chunk at positions start..; returns final-normed hidden and NEW caches
        (sliding layers truncated to the last WINDOW - 1 tokens, as DynamicSlidingWindowLayer.update)."""
        positions = np.arange(start, start + len(ids))
        x = self.embed(ids)
        new = []
        for layer in range(30):
            info = {} if layer_traces is not None else None
            x, cache = self.layer(layer, x, positions, caches[layer] if caches else None, encoder=True, trace=info)
            if self.layer_types[layer] == "sliding_attention":
                cache = {k: cache[k][-(WINDOW - 1):] for k in ("k", "v", "pos")}
            new.append(cache)
            if layer_traces is not None:
                layer_traces.append(dict(info, output=x))
        return self.rmsnorm(x, self.vec("output_norm.weight")), new

    def decode_at(self, ids, caches, start, layer_traces=None, extra=None):
        """Bidirectional decoder at positions start.. over the (already windowed) read-only caches."""
        positions = np.arange(start, start + len(ids))
        embeds = self.embed(ids)
        x = self.self_condition(embeds, np.zeros_like(embeds))
        if extra is not None:
            extra.update(embeds=embeds, self_conditioned=x)
        for layer in range(30):
            info = {} if layer_traces is not None else None
            x, _ = self.layer(layer, x, positions, caches[layer], encoder=False, trace=info)
            if layer_traces is not None:
                layer_traces.append(dict(info, output=x))
        return self.rmsnorm(x, self.vec("output_norm.weight"))


def save(path, caches, done, extra=None):
    data = {"done": np.array(done)}
    for i, c in enumerate(caches):
        for k in ("k", "v", "pos"):
            data[f"c{i}.{k}"] = c[k]
    if extra:
        data.update(extra)
    np.savez(path + ".tmp.npz", **data)
    os.replace(path + ".tmp.npz", path)


def load(path):
    with np.load(path, allow_pickle=False) as z:
        caches = [{k: z[f"c{i}.{k}"] for k in ("k", "v", "pos")} for i in range(30)]
        rest = {k: z[k] for k in z.files if not k.startswith("c")}
    return caches, int(rest.pop("done")), rest


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("model")
    ap.add_argument("prompt_ids")
    ap.add_argument("out_dir")
    ap.add_argument("--work", required=True)
    # f32: round every stored intermediate to binary32 (reference.py storage
    # mode), to measure the noise floor of an engine with F32 activations.
    ap.add_argument("--storage", choices=["f64", "f32"], default="f64")
    args = ap.parse_args()
    out_dir = os.path.abspath(args.out_dir)
    if os.path.exists(out_dir):
        sys.exit(f"refusing to overwrite existing {out_dir}")
    os.makedirs(args.work, exist_ok=True)
    t0 = time.time()
    log = lambda msg: print(f"[{time.time() - t0:8.1f}s] {msg}", flush=True)  # noqa: E731
    ids = [int(i) for i in json.load(open(args.prompt_ids))]
    total = len(ids)
    assert total > 2 * WINDOW, "the long-context case must exceed the sliding window"
    g = ggufref.GGUF(args.model)
    m = LongModel(g, args.storage)
    ckpt = os.path.join(args.work, "encoder.npz")
    last_chunk_start = (total - 1) // CHUNK * CHUNK
    if os.path.exists(ckpt):
        caches, done, rest = load(ckpt)
        log(f"resumed after {done} prompt tokens")
    else:
        caches, done, rest = None, 0, {}
    while done < total:
        n = min(CHUNK, total - done)
        traces = [] if done == last_chunk_start else None
        hidden, caches = m.extend(ids[done:done + n], caches, done, layer_traces=traces)
        done += n
        if traces is not None:
            rest = {"enc_hidden": hidden, **fcs.flatten_traces(traces, "enc")}
        if done % (8 * CHUNK) == 0 or done == total:
            save(ckpt, caches, done, rest)
        log(f"encoded {done}/{total}")
    log("decoder canvas")
    dec_path = os.path.join(args.work, "decoder.npz")
    canvas = canvas_rule()
    if os.path.exists(dec_path):
        with np.load(dec_path, allow_pickle=False) as z:
            dec = {k: z[k] for k in z.files}
    else:
        traces, extra = [], {}
        hidden = m.decode_at(canvas, caches, total, layer_traces=traces, extra=extra)
        dec = {**fcs.flatten_traces(traces, "t"), "hidden": hidden, "logits": m.logits(hidden),
               "embeds": extra["embeds"], "sc": extra["self_conditioned"]}
        np.savez(dec_path + ".tmp.npz", **dec)
        os.replace(dec_path + ".tmp.npz", dec_path)

    log("writing artifacts")
    partial = out_dir + ".partial"
    os.makedirs(partial, exist_ok=True)
    blobs = fcs.Blobs(partial)
    sel = np.array(SELECTED)
    summary = {}
    blobs.add("long_decode.inputs.bin", "long_decode.embedding.selected", dec["embeds"][sel])
    blobs.add("long_decode.inputs.bin", "long_decode.self_conditioned.selected", dec["sc"][sel])
    blobs.add("long_decode.inputs.bin", "long_decode.final.selected", dec["hidden"][sel])
    summary["long_decode"] = {"routing_margin": fcs.add_layers(blobs, "long_decode", fcs.unflatten_traces(dec, "t"), sel),
                              "logits": fcs.add_logits(blobs, "long_decode", dec["logits"])}
    n_last = total - last_chunk_start
    enc_sel = np.array(sorted({0, 1, n_last // 2, n_last - 2, n_last - 1}))
    blobs.add("long_encode.inputs.bin", "long_encode.final.selected", rest["enc_hidden"][enc_sel])
    summary["long_encode"] = {"routing_margin": fcs.add_layers(blobs, "long_encode", fcs.unflatten_traces(rest, "enc"), enc_sel)}
    cache_positions = {}
    for layer in CACHE_LAYERS:
        c = caches[layer]
        pos = c["pos"]
        if m.layer_types[layer] == "sliding_attention":
            wanted = [int(pos[0]), int(pos[len(pos) // 2]), int(pos[-1])]
        else:
            wanted = sorted({0, 1, WINDOW - 1, WINDOW, total // 2, total - WINDOW, total - 1})
        index = [int(np.nonzero(pos == p)[0][0]) for p in wanted]
        blobs.add("cache.bin", f"long_cache.{layer}.k.selected", c["k"][index])
        blobs.add("cache.bin", f"long_cache.{layer}.v.selected", c["v"][index])
        cache_positions[str(layer)] = wanted
    files = blobs.flush()
    st = os.stat(args.model)
    manifest = {
        "schema": "power-llm long-context golden v1",
        "complete": True,
        "model": {"path": args.model, "bytes": st.st_size, "general.name": g.metadata.get("general.name"),
                  "header_sha256": produce.sha256_file(args.model, g.data_start)},
        "sources": {"revisions": produce.REVISIONS, "pinned_file_sha256": produce.PINNED_SOURCE_SHA256,
                    "cache_semantics": "transformers cache_utils.py DynamicSlidingWindowLayer @ bd970fda (sha256 "
                                       "e518af31e419837ab980816677b8cbce13c1b81d7f3f46adcadfc988d0b1fb15): sliding "
                                       "layers keep the last sliding_window-1 tokens; decoder attends retained cache + canvas"},
        "tooling": {"python": platform.python_version(), "numpy": np.__version__,
                    "cpus": sorted(os.sched_getaffinity(0)),
                    "scripts_sha256": {s: produce.sha256_file(os.path.join(HERE, s)) for s in
                                       ["reference.py", "ggufref.py", "full_canvas.py", "full_canvas/model.py",
                                        "long_context.py"]}},
        "precision": ("weights: GGUF float32 dequantization; activations/reductions float64; blobs float32 (ids int32)"
                      + ("; every stored intermediate rounded to float32 (storage=f32 noise-floor run)"
                         if args.storage == "f32" else "")),
        "tolerance_contract": fcs.TOLERANCE,
        "cases": {
            "prompt": {"ids_file": os.path.basename(args.prompt_ids), "tokens": total, "encoder_chunk": CHUNK,
                       "ids": ids, "ids_source": "engine tokenizer + GGUF chat template (validated separately)"},
            "long_encode": {"last_chunk_start": last_chunk_start, "selected_chunk_indices": enc_sel.tolist()},
            "long_decode": {"decoder_ids": canvas, "rule": CANVAS_RULE, "positions": f"{total}..{total + CANVAS - 1}",
                            "self_conditioning": "none (zero soft embedding)"},
            "cache": {"layers": CACHE_LAYERS, "positions": cache_positions,
                      "retained_per_layer": [len(c["pos"]) for c in caches]},
        },
        "selection": {"positions": SELECTED},
        "summary": summary,
        "files": files,
        "entries": blobs.entries,
        "resources": {"wall_seconds": time.time() - t0,
                      "peak_rss_mib": resource.getrusage(resource.RUSAGE_SELF).ru_maxrss / 1024},
    }
    with open(os.path.join(partial, "manifest.json"), "w") as f:
        json.dump(manifest, f, indent=1)
    os.rename(partial, out_dir)
    log(f"done: {out_dir}")


if __name__ == "__main__":
    main()
