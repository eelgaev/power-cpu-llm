#!/usr/bin/env python3
"""Cross-check ggufref.py against gguf-py's NumPy dequantizers (a second,
separately written implementation) on sampled rows of every tensor.

    python crosscheck_dequant.py <model.gguf> [rows_per_tensor]
"""
import collections
import json
import sys

import gguf
import numpy as np

import ggufref

QTYPE = {"F32": gguf.GGMLQuantizationType.F32, "Q8_0": gguf.GGMLQuantizationType.Q8_0,
         "Q5_0": gguf.GGMLQuantizationType.Q5_0, "Q4_K": gguf.GGMLQuantizationType.Q4_K,
         "Q6_K": gguf.GGMLQuantizationType.Q6_K}


def main():
    path = sys.argv[1]
    per_tensor = int(sys.argv[2]) if len(sys.argv) > 2 else 3
    ours = ggufref.GGUF(path)
    theirs = {t.name: t for t in gguf.GGUFReader(path).tensors}
    assert set(theirs) == set(ours.tensors), "tensor sets differ"
    stats = collections.defaultdict(lambda: {"tensors": 0, "rows": 0, "values": 0, "max_ulp": 0, "max_abs": 0.0})
    for name, t in ours.tensors.items():
        other = theirs[name]
        assert list(other.shape) == t.dims and other.tensor_type == QTYPE[t.kind], name
        assert int(other.data_offset) == ours.data_start + t.offset, name
        picks = sorted({0, t.rows // 2, t.rows - 1} | {int(r) for r in np.linspace(0, t.rows - 1, per_tensor)})
        raw_all = np.asarray(other.data).reshape(t.rows, -1) if t.kind != "F32" else None
        for r in picks:
            a = ours.rows(name, r, 1)[0]
            if t.kind == "F32":
                b = np.asarray(other.data).reshape(-1)[r * t.row_width:(r + 1) * t.row_width].astype(np.float32)
            else:
                b = gguf.quants.dequantize(raw_all[r:r + 1], QTYPE[t.kind]).reshape(-1).astype(np.float32)
            assert np.all(np.isfinite(a)), (name, r)
            ulp = np.abs(a.view(np.int32).astype(np.int64) - b.view(np.int32).astype(np.int64))
            s = stats[t.kind]
            s["rows"] += 1
            s["values"] += a.size
            s["max_ulp"] = max(s["max_ulp"], int(ulp.max()))
            s["max_abs"] = max(s["max_abs"], float(np.abs(a - b).max()))
        stats[t.kind]["tensors"] += 1
    print(json.dumps({"gguf_py": gguf.__version__ if hasattr(gguf, "__version__") else "see pip", "by_type": stats},
                     indent=1))


if __name__ == "__main__":
    main()
