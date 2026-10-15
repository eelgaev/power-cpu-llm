"""Full-canvas extensions of the independent reference (no edits to reference.py).

`BatchedModel` only replaces the per-token expert loop with one product per
expert over the tokens that selected it. Equations, operand values and the
ascending-expert accumulation order per token are unchanged; only BLAS
reduction grouping differs (float64 either way). `encode_extend` continues a
causal encoder from an existing cache using the reference `layer()`.
"""
import numpy as np

import reference


class BatchedModel(reference.Model):
    def experts(self, layer, x, ids, weights):
        out = np.zeros_like(x)
        for e in sorted(set(int(i) for i in ids.reshape(-1))):  # ascending expert IDs, as reference
            tokens, slots = np.nonzero(ids == e)  # each token selects an expert at most once
            gu, down = self.expert(layer, e)
            g = self.st(x[tokens] @ gu.T)
            hidden = self.st(reference.gelu_tanh(g[:, :reference.MOE_FF]) * g[:, reference.MOE_FF:])
            out[tokens] += self.st(hidden @ down.T) * weights[tokens, slots][:, None]
        return self.st(out)

    def encode_extend(self, ids, caches, layer_traces=None):
        """Causal encoder continuation at positions len(cache)..; returns final-normed
        hidden rows and NEW caches (old caches are not modified)."""
        start = len(caches[0]["pos"])
        positions = np.arange(start, start + len(ids))
        x = self.embed(ids)
        new_caches = []
        for layer in range(30):
            info = {} if layer_traces is not None else None
            x, cache = self.layer(layer, x, positions, caches[layer], encoder=True, trace=info)
            new_caches.append(cache)
            if layer_traces is not None:
                layer_traces.append(dict(info, output=x))
        return self.rmsnorm(x, self.vec("output_norm.weight")), new_caches
