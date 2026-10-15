#!/usr/bin/env python3
"""Compare data-only independent golden traces with observed engine traces.

Stdlib only; never generates/updates expected values. Tolerances are explicit,
NOT increased automatically using a fixture's f32_storage_deviation field.
"""
import argparse
import array
import heapq
import json
import math
from pathlib import Path
import sys


def load(root):
    manifest = json.loads((root / "manifest.json").read_text())
    entries = {}
    for entry in manifest["entries"]:
        if entry["name"] in entries:
            raise ValueError("duplicate trace name: " + entry["name"])
        entries[entry["name"]] = entry
    return manifest, entries


def values(root, entry):
    if entry["dtype"] != "<f4":
        raise ValueError("only little-endian F32 traces are accepted")
    shape = entry["shape"]
    if not shape or any(type(n) is not int or n <= 0 for n in shape):
        raise ValueError("invalid trace shape")
    cells = math.prod(shape)
    if cells * 4 > 128 * 1024 * 1024:
        raise ValueError("entry exceeds diagnostic 128-MiB read limit")
    offset = entry.get("offset", 0)
    if type(offset) is not int or offset < 0 or offset % 4:
        raise ValueError("invalid trace offset")
    path = (root / entry["file"]).resolve()
    try:
        path.relative_to(root.resolve())
    except ValueError:
        raise ValueError("trace path escapes fixture directory") from None
    with path.open("rb") as stream:
        stream.seek(offset)
        data = stream.read(cells * 4)
    if len(data) != cells * 4:
        raise ValueError("truncated trace: " + entry["name"])
    result = array.array("f")
    if result.itemsize != 4:
        raise ValueError("host float array is not 32-bit")
    result.frombytes(data)
    if sys.byteorder != "little":
        result.byteswap()
    if not all(math.isfinite(x) for x in result):
        raise ValueError("nonfinite trace: " + entry["name"])
    return result


def compare(expected, observed, atol, rtol):
    max_abs = max_rel = square = 0.0
    failures = 0
    for reference, actual in zip(expected, observed):
        delta = abs(actual - reference)
        max_abs = max(max_abs, delta)
        max_rel = max(max_rel, delta / max(abs(reference), 1e-30))
        square += delta * delta
        failures += delta > atol + rtol * abs(reference)
    return max_abs, max_rel, math.sqrt(square / len(expected)), failures


def top(values, start, width):
    return heapq.nlargest(5, range(start, start + width), key=lambda i: (values[i], -i))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("golden", type=Path)
    parser.add_argument("observed", type=Path)
    parser.add_argument("--prefix", action="append", help="repeatable; default encoder. and decoder_a.")
    parser.add_argument("--atol", type=float, default=1e-4)
    parser.add_argument("--rtol", type=float, default=1e-4)
    args = parser.parse_args()
    if any(not math.isfinite(v) or v < 0 for v in (args.atol, args.rtol)):
        parser.error("tolerances must be finite and nonnegative")
    gm, golden = load(args.golden)
    om, observed = load(args.observed)
    if om.get("kind") != "engine-observation-NOT-independent-golden" or om.get("complete") is not True:
        raise ValueError("observed manifest is not a completed engine export")
    if gm.get("kind") == om.get("kind"):
        raise ValueError("engine observations cannot substitute for independent expected values")
    if gm["model"]["bytes"] != om["model"]["file_size"]:
        raise ValueError("model file sizes differ")
    prefixes = args.prefix or ["encoder.", "decoder_a."]
    if any("decoder_b.".startswith(p) or p.startswith("decoder_b.") for p in prefixes):
        if om["decoder_b"].get("raw_input_path") is None:
            raise ValueError("decoder_b comparison requires trace-forward --self-condition-from golden/logits_a.f32")
    full = gm.get("full", {})
    for case in ("encoder", "decoder_a", "decoder_b", "decoder_c"):
        if any((case + ".").startswith(p) or p.startswith(case + ".") for p in prefixes):
            if case in full and full[case]["ids"] != om[case]["ids"]:
                raise ValueError("input token IDs differ for " + case)
    selected = [name for name in golden if any(name.startswith(p) for p in prefixes)]
    if not selected:
        raise ValueError("no golden entries matched requested prefixes")
    failed = checked = 0
    print(f"Explicit tolerance: abs={args.atol:g} + rel={args.rtol:g} * abs(reference)")
    for name in selected:
        if name not in observed:
            print("MISSING", name)
            failed += 1
            continue
        if golden[name]["shape"] != observed[name]["shape"]:
            print("SHAPE MISMATCH", name)
            failed += 1
            continue
        expected = values(args.golden, golden[name])
        actual = values(args.observed, observed[name])
        maximum, relative, rms, count = compare(expected, actual, args.atol, args.rtol)
        print(f"{name}: max_abs={maximum:.7g} max_rel={relative:.7g} rms={rms:.7g} bad_cells={count}")
        if "f32_storage_deviation" in golden[name]:
            print("  independently recorded storage deviation:", golden[name]["f32_storage_deviation"])
        bad_top = False
        if name.endswith(".logits"):
            width = golden[name]["shape"][-1]
            for start in range(0, len(expected), width):
                e, a = top(expected, start, width), top(actual, start, width)
                print("  golden top:", [(i-start, expected[i]) for i in e])
                print("  engine top:", [(i-start, actual[i]) for i in a])
                bad_top |= e[0] != a[0]
        failed += bool(count or bad_top)
        checked += 1
    print(f"Compared {checked} entries; {failed} missing/mismatched/failed entries.")
    print("Selected diagnostic cases only; NOT full-canvas/quality/throughput validation.")
    return 1 if failed else 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ValueError, KeyError, OSError) as error:
        print("comparison error:", error, file=sys.stderr)
        sys.exit(2)
