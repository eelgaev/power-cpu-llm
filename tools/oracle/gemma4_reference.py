#!/usr/bin/env python3
"""Independent reference for the AUTOREGRESSIVE Gemma 4 MoE (gemma-4-26B-A4B, GGUF arch `gemma4`).

    RUN="numactl --physcpubind=CPUS --membind=NODE env OPENBLAS_NUM_THREADS=N python3"
    $RUN gemma4_reference.py MODEL PROMPT_IDS.json OUT_DIR

Gemma 4 shares DiffusionGemma's layer equations (reference.py, unchanged); this subclass only reads the
`gemma4.*` metadata and uses Gemma 4's single `layer_output_scale` (it has no encoder/decoder split). The
prompt is encoded causally in 256-token chunks with transformers' sliding-window cache semantics (as
long_context.py). Recorded: every layer's output and routing at every prompt position, the final normed
hidden rows and the full logit row of the last position. Prompt IDs are an INPUT (engine tokenizer).
"""
import json
import os
import sys
import time

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import ggufref  # noqa: E402
import produce  # noqa: E402
import long_context as lc  # noqa: E402

fcs = lc.fcs


class Gemma4Model(lc.LongModel):
    def __init__(self, gguf, storage="f64"):
        self.g, self.storage, self._cache = gguf, storage, {}
        md = gguf.metadata
        assert md["general.architecture"] == "gemma4"
        self.layer_types = ["sliding_attention" if s else "full_attention"
                            for s in md["gemma4.attention.sliding_window_pattern"]]
        self.kv_heads = [int(n) for n in md["gemma4.attention.head_count_kv"]]
        assert len(self.layer_types) == 30 and md["gemma4.embedding_length"] == 2816

    def vec(self, name):
        # One causal layer scale in Gemma 4 (the reference's encoder asks for `enc_`).
        return super().vec(name.replace("enc_layer_output_scale", "layer_output_scale"))


def main():
    model_path, ids_path, out_dir = sys.argv[1:4]
    out_dir = os.path.abspath(out_dir)
    if os.path.exists(out_dir):
        sys.exit(f"refusing to overwrite {out_dir}")
    t0 = time.time()
    ids = [int(i) for i in json.load(open(ids_path))]
    g = ggufref.GGUF(model_path)
    m = Gemma4Model(g, "f64")
    caches, traces_all, hidden_all = None, [], []
    for start in range(0, len(ids), lc.CHUNK):
        traces = []
        hidden, caches = m.extend(ids[start:start + lc.CHUNK], caches, start, layer_traces=traces)
        traces_all.append(traces)
        hidden_all.append(hidden)
        print(f"[{time.time() - t0:7.1f}s] encoded {start + len(hidden)}/{len(ids)}", flush=True)
    # Concatenate chunks per layer.
    keys = ("output", "top_ids", "top_weights", "router_probs")
    merged = [{k: np.concatenate([chunk[layer][k] for chunk in traces_all]) for k in keys} for layer in range(30)]
    hidden = np.concatenate(hidden_all)
    logits = m.logits(hidden[-1:])
    partial = out_dir + ".partial"
    os.makedirs(partial)
    blobs = fcs.Blobs(partial)
    every = np.arange(len(ids))
    blobs.add("prompt.inputs.bin", "prompt.final.selected", hidden)
    summary = {"routing_margin": fcs.add_layers(blobs, "prompt", merged, every),
               "logits": fcs.add_logits(blobs, "prompt", logits, [0])}
    files = blobs.flush()
    manifest = {
        "schema": "power-llm gemma4 prompt golden v1", "complete": True,
        "model": {"path": model_path, "general.name": g.metadata.get("general.name"),
                  "header_sha256": produce.sha256_file(model_path, g.data_start)},
        "precision": "weights: GGUF float32 dequantization (Q4_0 via ggufref); activations/reductions float64",
        "tolerance_contract": fcs.TOLERANCE,
        "cases": {"prompt": {"ids": ids, "tokens": len(ids), "encoder_chunk": lc.CHUNK,
                             "logits": "position len(ids)-1 only (stored as logits row 0)"}},
        "selection": {"positions": every.tolist()},
        "summary": summary, "files": files, "entries": blobs.entries,
        "resources": {"wall_seconds": time.time() - t0},
    }
    json.dump(manifest, open(os.path.join(partial, "manifest.json"), "w"), indent=1)
    os.rename(partial, out_dir)
    print(f"done {out_dir}")


if __name__ == "__main__":
    main()
