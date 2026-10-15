#!/usr/bin/env python3
"""Stdlib regression tests for the primary observation comparator."""
import importlib.util
import json
from pathlib import Path
import struct
import tempfile
import unittest

spec = importlib.util.spec_from_file_location("comparator",Path(__file__).with_name("compare-reference-traces.py"))
comparator = importlib.util.module_from_spec(spec)
spec.loader.exec_module(comparator)

class Checks(unittest.TestCase):
    def test_explicit_error_metrics(self):
        self.assertEqual(comparator.compare([1.0,-2.0],[1.0,-2.0],0,0),(0,0,0,0))
        absolute, relative, rms, bad = comparator.compare([1.0,2.0],[1.01,2.0],1e-4,1e-4)
        self.assertAlmostEqual(absolute,0.01)
        self.assertAlmostEqual(relative,0.01)
        self.assertGreater(rms,0)
        self.assertEqual(bad,1)
        self.assertEqual(comparator.top([3.0,3.0,1.0],0,3),[0,1,2])

    def test_f32_shapes_offsets_and_bounded_paths(self):
        with tempfile.TemporaryDirectory(prefix="power-llm-comparison-") as directory:
            root=Path(directory)
            (root/"x.f32").write_bytes(struct.pack("<fff",42.0,1.0,-2.5))
            entry={"name":"x","file":"x.f32","offset":4,"shape":[1,2],"dtype":"<f4"}
            self.assertEqual(list(comparator.values(root,entry)),[1.0,-2.5])
            for changes in [{"offset":1},{"offset":-4},{"shape":[1,4]},{"shape":[0]},
                            {"shape":[True]},{"shape":[1<<30]},{"dtype":">f4"},{"file":"../outside.f32"}]:
                with self.assertRaises(ValueError): comparator.values(root,dict(entry,**changes))
            (root/"nan.f32").write_bytes(struct.pack("<f",float("nan")))
            with self.assertRaises(ValueError): comparator.values(root,dict(entry,file="nan.f32",offset=0,shape=[1]))
            (root/"manifest.json").write_text(json.dumps({"entries":[entry,entry]}))
            with self.assertRaises(ValueError): comparator.load(root)

if __name__=="__main__": unittest.main()
