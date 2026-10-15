#!/usr/bin/env python3
"""Verify a full-canvas golden directory: completion marker, path containment,
blob sizes/hashes, entry bounds/shapes/dtypes, finite floats.

    python full_canvas/verify.py ../fixtures/numerical-full-canvas
"""
import hashlib
import json
import os
import sys

import numpy as np

ALLOWED = {"<f4": 4, "<i4": 4}


def main():
    root = os.path.realpath(sys.argv[1])
    manifest = json.load(open(os.path.join(root, "manifest.json")))
    assert manifest.get("complete") is True, "manifest not marked complete"
    for name, info in manifest["files"].items():
        path = os.path.realpath(os.path.join(root, name))
        assert os.path.dirname(path) == root and not os.path.isabs(name) and ".." not in name, f"bad path {name}"
        data = open(path, "rb").read()
        assert len(data) == info["bytes"], name
        assert hashlib.sha256(data).hexdigest() == info["sha256"], name
    listed = set(manifest["files"]) | {"manifest.json"}
    extra = set(os.listdir(root)) - listed
    assert not extra, f"unlisted files: {sorted(extra)}"
    for e in manifest["entries"]:
        size = ALLOWED[e["dtype"]]
        count = int(np.prod(e["shape"]))
        assert e["file"] in manifest["files"] and e["offset"] % size == 0, e["name"]
        assert e["offset"] + count * size <= manifest["files"][e["file"]]["bytes"], e["name"]
        a = np.fromfile(os.path.join(root, e["file"]), dtype=e["dtype"], count=count, offset=e["offset"])
        if e["dtype"] == "<f4":
            assert np.all(np.isfinite(a)), e["name"]
        else:
            assert a.min() >= 0, e["name"]
    assert sum(f["bytes"] for f in manifest["files"].values()) == manifest["total_bytes"]
    print(json.dumps({"ok": True, "entries": len(manifest["entries"]), "files": len(manifest["files"]),
                      "total_bytes": manifest["total_bytes"]}))


if __name__ == "__main__":
    main()
