#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from instructions.sum_runtime import merge_user_box_decls


class TestSumRuntime(unittest.TestCase):
    def test_merge_user_box_decls_keeps_only_explicit_user_boxes(self):
        decls = merge_user_box_decls(
            [
                {"name": "Point", "fields": ["x", "y"]},
                {"name": "Point", "fields": ["x", "y"]},
                {"name": "Flag", "fields": ["enabled"]},
                {"name": ""},
                {"fields": ["missing_name"]},
                "skip-me",
            ]
        )

        self.assertEqual([decl["name"] for decl in decls], ["Point", "Flag"])
        self.assertEqual(decls[0]["fields"], ["x", "y"])
        self.assertEqual(decls[1]["fields"], ["enabled"])


if __name__ == "__main__":
    unittest.main()
