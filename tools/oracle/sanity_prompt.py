#!/usr/bin/env python3
"""Semantic sanity check of the independent reference (diagnostic, not a golden).

Encodes a chat prompt, then runs a few greedy denoising passes over a short
canvas with self-conditioning (processed logits = logits / 0.8) and prints the
argmax tokens. A working reference should answer the question. Token IDs were
produced by transformers apply_chat_template with the pinned tokenizer:
"What is the capital of France? Answer with one word." (' Paris' = 9079,
'Paris' = 50429).

    python sanity_prompt.py <model.gguf> [canvas_length] [steps]
"""
import json
import sys
import time

import numpy as np

import ggufref
import reference

PROMPT = [2, 105, 2364, 107, 3689, 563, 506, 5279, 529, 7001, 236881, 25685, 607, 886, 3658, 236761, 106, 107,
          105, 4368, 107]


def main():
    canvas_length = int(sys.argv[2]) if len(sys.argv) > 2 else 4
    steps = int(sys.argv[3]) if len(sys.argv) > 3 else 3
    t0 = time.time()
    g = ggufref.GGUF(sys.argv[1])
    m = reference.Model(g, "f64")
    tokens = g.metadata["tokenizer.ggml.tokens"]
    _, caches = m.encode(PROMPT)
    canvas = np.random.default_rng(0).integers(0, 262144, canvas_length).tolist()
    processed = None
    history = []
    for step in range(steps):
        logits = m.logits(m.decode(canvas, caches, processed))
        canvas = logits.argmax(-1).tolist()
        processed = logits / 0.8
        top = [[(tokens[int(i)], round(float(row[i]), 3)) for i in np.argsort(-row)[:3]] for row in logits]
        history.append({"step": step, "canvas": canvas, "text": "".join(tokens[i] for i in canvas), "top3": top})
        print(json.dumps(history[-1], ensure_ascii=False), flush=True)
    print(json.dumps({"seconds": time.time() - t0}))


if __name__ == "__main__":
    main()
