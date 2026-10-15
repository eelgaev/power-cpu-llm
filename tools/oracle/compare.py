#!/usr/bin/env python3
"""Compare an engine trace against the independent golden fixtures.

    python compare.py <golden_dir> <candidate_dir> [--factor 64]

Both directories hold `manifest.json` with an "entries" list of
{name, file, offset, shape, dtype: "<f4"} over little-endian float32 files
(the candidate needs only the entries it exports). Optional candidate
`manifest.json["routers"]`: {"<trace>.layer.<i>": [[top-8 ids per row]]}, compared as sets.

Per entry: finite/shape checks, max/RMS absolute error, max relative error
(to the entry's max |golden|), pass if max_abs_err <= factor * measured
F32-storage deviation (+ 4 float32 ulps of the entry max). Logit entries also
compare argmax and top-10 IDs. Exit status 1 on any failure. Never regenerate
the golden values from engine output, and do not raise `--factor` to hide a
layer/head/normalization/routing bug.
"""
import argparse
import json
import os
import sys

import numpy as np


def load(directory):
    with open(os.path.join(directory, "manifest.json")) as f:
        manifest = json.load(f)
    blobs = {}

    def array(entry):
        path = os.path.join(directory, entry["file"])
        if path not in blobs:
            blobs[path] = np.fromfile(path, dtype="<f4")
        count = int(np.prod(entry["shape"]))
        start = entry["offset"] // 4
        return blobs[path][start:start + count].reshape(entry["shape"]).astype(np.float64)

    return manifest, {e["name"]: (e, array) for e in manifest["entries"]}


def golden_routers(manifest):
    out = {}
    for case in ("encoder", "decoder_a", "decoder_b", "decoder_c"):
        for r in manifest.get("full", {}).get(case, {}).get("router", []):
            out[f"{case}.layer.{r['layer']}"] = r["top_ids"]
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("golden")
    ap.add_argument("candidate")
    ap.add_argument("--factor", type=float, default=64.0)
    args = ap.parse_args()
    gm, golden = load(args.golden)
    cm, candidate = load(args.candidate)
    failures, rows = [], []
    for name, (centry, carray) in candidate.items():
        if name not in golden:
            failures.append(f"{name}: not in golden fixtures")
            continue
        gentry, garray = golden[name]
        g, c = garray(gentry), carray(centry)
        if list(c.shape) != list(g.shape):
            failures.append(f"{name}: shape {list(c.shape)} != golden {list(g.shape)}")
            continue
        if not np.all(np.isfinite(c)):
            failures.append(f"{name}: non-finite values")
            continue
        err = np.abs(c - g)
        gmax = max(float(np.abs(g).max()), 1e-30)
        floor = gentry.get("f32_storage_deviation", {}).get("max_abs", 0.0)
        tol = args.factor * floor + 4 * np.spacing(np.float32(gmax))
        row = {"name": name, "max_abs": float(err.max()), "rms": float(np.sqrt(np.mean(err ** 2))),
               "max_rel": float(err.max() / gmax), "tolerance": float(tol), "ok": bool(err.max() <= tol)}
        if name.endswith(".logits"):
            for r, (gr, cr) in enumerate(zip(g.reshape(-1, g.shape[-1]), c.reshape(-1, c.shape[-1]))):
                gt, ct = np.argsort(-gr)[:10], np.argsort(-cr)[:10]
                row[f"argmax_{r}"] = [int(gt[0]), int(ct[0])]
                if gt[0] != ct[0] or set(gt) != set(ct):
                    failures.append(f"{name}[{r}]: top-10 IDs differ (golden argmax {gt[0]}, candidate {ct[0]})")
        if not row["ok"]:
            failures.append(f"{name}: max |err| {row['max_abs']:.3e} > tolerance {tol:.3e}")
        rows.append(row)
    routers = golden_routers(gm)
    for key, ids in cm.get("routers", {}).items():
        if key not in routers:
            failures.append(f"router {key}: not in golden fixtures")
        elif [sorted(map(int, r)) for r in ids] != [sorted(r) for r in routers[key]]:  # selection, any order
            failures.append(f"router {key}: top-8 IDs {ids} != golden {routers[key]}")
    for row in rows:
        print(json.dumps(row))
    print(json.dumps({"compared": len(rows), "routers": len(cm.get("routers", {})), "failures": failures}, indent=1))
    sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
