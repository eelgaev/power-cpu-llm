#!/usr/bin/env python3
"""Compare engine full-canvas observations with the independent oracle fixtures.

    python3 tools/compare-full-canvas.py <oracle-fixture-dir> <engine-observation-dir> [--report out.json]

Contract (docs/validation.md, unchanged):
  * values: |engine - expected| <= 1e-4 + 1e-4 * |expected| per cell (hidden rows, inputs, caches,
    logit scores where IDs agree, logsumexp, full logit rows);
  * routing: exact top-8 expert SET per (layer, position); every mismatch reported with the expected
    rank-8/rank-9 probability gap;
  * logits: argmax and top-16 ID set per position; mismatches reported with the expected top-1/top-2 margin.

Divergence policy: per case, the FIRST routing divergence (lowest layer) is located. Value failures at
layers before it, and in that layer's own pre-routing inputs, are UNEXPLAINED. Failures in later layers
and in final/logit outputs of a case whose routing diverged are reported as DOWNSTREAM of that divergence
(never hidden, never counted as passing). Every routing-set mismatch is itself a failure (listed in
`routing_divergences` with both rank-8/9 margins for review; a near-tie is an explanation, not a pass).
Exit status 1 if there is any routing divergence or unexplained failure. Tolerances are never widened
and expected data is never modified.
"""
import argparse
import json
import os
import sys

import numpy as np

ATOL = RTOL = 1e-4
# Cache entries produced by an encoder case (attributed to it for divergence).
CACHE_OWNER = {"extended_cache": "case3_encode", "long_cache": "long_encode"}


def load(directory):
    manifest = json.load(open(os.path.join(directory, "manifest.json")))
    if manifest.get("complete") is not True:
        sys.exit(f"{directory}: manifest not complete")
    entries = {e["name"]: e for e in manifest["entries"]}

    def array(name):
        e = entries[name]
        count = int(np.prod(e["shape"]))
        data = np.fromfile(os.path.join(directory, e["file"]), dtype=e["dtype"], count=count, offset=e["offset"])
        return data.reshape(e["shape"])

    return manifest, entries, array


def value_stats(expected, observed):
    e, o = expected.astype(np.float64), observed.astype(np.float64)
    err = np.abs(o - e)
    bad = err > ATOL + RTOL * np.abs(e)
    return {"max_abs": float(err.max()), "rms": float(np.sqrt(np.mean(err ** 2))),
            "max_rel_to_entry_max": float(err.max() / max(np.abs(e).max(), 1e-30)),
            "bad_cells": int(bad.sum()), "cells": int(e.size)}, bad


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("expected")
    ap.add_argument("observed")
    ap.add_argument("--report")
    args = ap.parse_args()
    em, ee, earr = load(args.expected)
    om, oe, oarr = load(args.observed)
    # Cases are discovered from the expected manifest (full-canvas, long-context
    # and prompt fixtures share the entry naming).
    CASES = sorted({n.split(".routing.")[0] for n in ee if n.endswith(".routing.top8_ids")})
    LOGIT_CASES = sorted({n.split(".logits.")[0] for n in ee if n.endswith(".logits.top_ids")})
    report = {"expected": args.expected, "observed": args.observed, "observed_backend": om.get("matrix_backend"),
              "tolerance": {"atol": ATOL, "rtol": RTOL}, "cases": {}, "values": {}, "missing": []}
    unexplained = []
    routing_divergences = []

    # ---- routing ---------------------------------------------------------------------------------
    first_divergence = {}
    for case in CASES:
        name = f"{case}.routing.top8_ids"
        if name not in oe:
            report["missing"].append(name)
            unexplained.append(f"missing {name}")
            continue
        e_ids, o_ids = earr(name), oarr(name)
        e_probs = earr(f"{case}.routing.top10_probs").astype(np.float64)
        o_probs = oarr(f"{case}.routing.top10_probs").astype(np.float64)
        if e_ids.shape != o_ids.shape:
            unexplained.append(f"{name}: shape {o_ids.shape} != {e_ids.shape}")
            continue
        mismatches = []
        weight_err = 0.0
        e_w, o_w = earr(f"{case}.routing.top8_weights"), oarr(f"{case}.routing.top8_weights")
        layers, positions, _ = e_ids.shape
        for layer in range(layers):
            for pos in range(positions):
                es, os_ = set(e_ids[layer, pos].tolist()), set(o_ids[layer, pos].tolist())
                if es != os_:
                    mismatches.append({
                        "layer": layer, "position": pos,
                        "only_expected": sorted(es - os_), "only_engine": sorted(os_ - es),
                        "expected_rank8_minus_rank9": float(e_probs[layer, pos, 7] - e_probs[layer, pos, 8]),
                        "engine_rank8_minus_rank9": float(o_probs[layer, pos, 7] - o_probs[layer, pos, 8])})
                else:
                    ew = dict(zip(e_ids[layer, pos].tolist(), e_w[layer, pos].astype(np.float64)))
                    ow = dict(zip(o_ids[layer, pos].tolist(), o_w[layer, pos].astype(np.float64)))
                    weight_err = max(weight_err, max(abs(ew[k] - ow[k]) for k in ew))
        prob_err = float(np.abs(e_probs - o_probs).max())
        first = min((m["layer"] for m in mismatches), default=None)
        first_divergence[case] = first
        routing_divergences += [dict(case=case, **m) for m in mismatches]
        report["cases"][case] = {
            "routing_sets_compared": int(layers * positions), "routing_set_mismatches": len(mismatches),
            "first_divergence_layer": first, "mismatches": mismatches[:50],
            "matched_set_max_weight_abs_error": weight_err, "top10_prob_max_abs_error": prob_err}

    # ---- values ----------------------------------------------------------------------------------
    def case_of(name):
        owner = CACHE_OWNER.get(name.split(".")[0])
        if owner:
            return owner
        for case in sorted(CASES, key=len, reverse=True):
            if name.startswith(case + "."):
                return case
        return None  # prompt_cache: precedes every routing decision compared here

    for name, e in ee.items():
        if e["dtype"] != "<f4" or ".routing." in name or ".logits." in name or ".processed_sc." in name:
            continue
        if name not in oe:
            report["missing"].append(name)
            unexplained.append(f"missing {name}")
            continue
        exp, obs = earr(name), oarr(name)
        if exp.shape != obs.shape or not np.all(np.isfinite(obs)):
            unexplained.append(f"{name}: shape/finite")
            continue
        stats, bad = value_stats(exp, obs)
        case = case_of(name)
        first = first_divergence.get(case)
        if name.endswith(".layers.selected"):
            per_layer = [int(bad[i].sum()) for i in range(bad.shape[0])]
            stats["bad_cells_per_layer"] = per_layer
            stats["max_abs_per_layer"] = [float(np.abs(obs[i].astype(np.float64) - exp[i]).max()) for i in range(bad.shape[0])]
            failing = [i for i, n in enumerate(per_layer) if n]
            early = [i for i in failing if first is None or i < first]
            stats["unexplained_layers"] = early
            stats["downstream_layers"] = [i for i in failing if first is not None and i >= first]
            if early:
                unexplained.append(f"{name}: layers {early} out of tolerance before any routing divergence")
        elif stats["bad_cells"]:
            downstream = first is not None and not name.endswith((".embedding.selected", ".soft_embedding.selected",
                                                                  ".self_conditioned.selected"))
            stats["classification"] = "downstream_of_routing_divergence" if downstream else "unexplained"
            if not downstream:
                unexplained.append(f"{name}: {stats['bad_cells']} cells out of tolerance")
        report["values"][name] = stats

    # ---- logits ------------------------------------------------------------------------------------
    for case in LOGIT_CASES:
        name = f"{case}.logits.top_ids"
        if name not in oe:
            report["missing"].append(name)
            unexplained.append(f"missing {name}")
            continue
        e_ids, o_ids = earr(name), oarr(name)
        e_sc = earr(f"{case}.logits.top_scores").astype(np.float64)
        o_sc = oarr(f"{case}.logits.top_scores").astype(np.float64)
        argmax_mismatch, set_mismatch, score_bad, score_err = [], 0, 0, 0.0
        for pos in range(e_ids.shape[0]):
            if e_ids[pos, 0] != o_ids[pos, 0]:
                argmax_mismatch.append({"position": pos, "expected": int(e_ids[pos, 0]), "engine": int(o_ids[pos, 0]),
                                        "expected_top1_minus_top2": float(e_sc[pos, 0] - e_sc[pos, 1])})
            if set(e_ids[pos].tolist()) != set(o_ids[pos].tolist()):
                set_mismatch += 1
            e_map = dict(zip(e_ids[pos].tolist(), e_sc[pos]))
            for i, tok in enumerate(o_ids[pos].tolist()):
                if tok in e_map:
                    err = abs(o_sc[pos, i] - e_map[tok])
                    score_err = max(score_err, err)
                    score_bad += int(err > ATOL + RTOL * abs(e_map[tok]))
        lse, lse_bad = value_stats(earr(f"{case}.logits.logsumexp"), oarr(f"{case}.logits.logsumexp"))
        entry = {"positions": int(e_ids.shape[0]), "argmax_mismatches": argmax_mismatch,
                 "top16_set_mismatches": set_mismatch, "matched_score_max_abs_error": score_err,
                 "matched_scores_out_of_tolerance": score_bad, "logsumexp": lse}
        for full in [n for n in ee if n.startswith(f"{case}.logits.full_row.")]:
            entry[full], _ = value_stats(earr(full), oarr(full)) if full in oe else ({"missing": True}, None)
        diverged = first_divergence.get(case) is not None
        failed = argmax_mismatch or set_mismatch or score_bad or lse["bad_cells"] or any(
            isinstance(v, dict) and v.get("bad_cells") for k, v in entry.items() if k.startswith(case))
        entry["classification"] = ("pass" if not failed else
                                   "downstream_of_routing_divergence" if diverged else "unexplained")
        if failed and not diverged:
            unexplained.append(f"{case} logits: argmax {len(argmax_mismatch)}, top16 sets {set_mismatch}, "
                               f"scores {score_bad}, logsumexp {lse['bad_cells']}")
        report["cases"].setdefault(case, {})["logits"] = entry

    report["unexplained_failures"] = unexplained
    report["routing_divergences"] = routing_divergences
    if args.report:
        with open(args.report, "w") as f:
            json.dump(report, f, indent=1)
    # concise console summary
    for case in CASES:
        c = report["cases"].get(case, {})
        line = f"{case}: routing mismatches {c.get('routing_set_mismatches')} / {c.get('routing_sets_compared')}" \
               f", first divergence layer {c.get('first_divergence_layer')}, top10 prob max err {c.get('top10_prob_max_abs_error', 0):.3e}"
        if "logits" in c:
            lg = c["logits"]
            line += (f"; logits argmax mismatches {len(lg['argmax_mismatches'])}, top16 set mismatches "
                     f"{lg['top16_set_mismatches']}, matched score max err {lg['matched_score_max_abs_error']:.3e} "
                     f"[{lg['classification']}]")
        print(line)
    worst = sorted(report["values"].items(), key=lambda kv: -kv[1]["max_abs"])[:6]
    for name, s in worst:
        print(f"  {name}: max_abs {s['max_abs']:.3e} rms {s['rms']:.3e} bad {s['bad_cells']}/{s['cells']}")
    print(json.dumps({"value_entries": len(report["values"]), "missing": report["missing"],
                      "routing_divergences": routing_divergences[:20],
                      "unexplained_failures": unexplained}, indent=1))
    sys.exit(1 if unexplained or routing_divergences else 0)


if __name__ == "__main__":
    main()
