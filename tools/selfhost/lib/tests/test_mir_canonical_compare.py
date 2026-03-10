#!/usr/bin/env python3
from __future__ import annotations

import json
import pathlib
import sys
import unittest

THIS_DIR = pathlib.Path(__file__).resolve().parent
LIB_DIR = THIS_DIR.parent
if str(LIB_DIR) not in sys.path:
    sys.path.insert(0, str(LIB_DIR))

from mir_canonical_compare import canonicalize_module  # noqa: E402


def _dump(payload: dict) -> str:
    return json.dumps(canonicalize_module(payload), sort_keys=True, separators=(",", ":"))


def _phi_bundle_module(copy_sources: list[int]) -> dict:
    bundle = [
        {"dst": 10 + idx, "op": "copy", "src": src}
        for idx, src in enumerate(copy_sources)
    ]
    return {
        "functions": [
            {
                "name": "PhiBundle.test/1",
                "params": [0],
                "blocks": [
                    {
                        "id": 0,
                        "instructions": [
                            {"dst": 1, "op": "const", "value": {"type": "i64", "value": 0}},
                            {"dst": 2, "op": "copy", "src": 0},
                            {"dst": 3, "op": "copy", "src": 1},
                            {"op": "jump", "target": 1},
                        ],
                    },
                    {
                        "id": 1,
                        "instructions": [
                            {"dst": 4, "incoming": [[2, 0], [5, 2]], "op": "phi"},
                            *bundle,
                            {"op": "jump", "target": 3},
                        ],
                    },
                    {
                        "id": 2,
                        "instructions": [
                            {"dst": 5, "op": "const", "value": {"type": "i64", "value": 1}},
                            {"op": "jump", "target": 1},
                        ],
                    },
                    {"id": 3, "instructions": [{"op": "ret", "value": 4}]},
                ],
                "metadata": {"value_types": {}},
            }
        ],
        "user_box_decls": [],
    }


def _boxcall_bundle_module(copy_sources: list[int]) -> dict:
    bundle = [
        {"dst": 20 + idx, "op": "copy", "src": src}
        for idx, src in enumerate(copy_sources)
    ]
    return {
        "functions": [
            {
                "name": "BoxcallBundle.test/1",
                "params": [0],
                "blocks": [
                    {
                        "id": 0,
                        "instructions": [
                            {"dst": 1, "op": "const", "value": {"type": "i64", "value": 7}},
                            {
                                "args": [],
                                "box": 0,
                                "dst": 2,
                                "dst_type": "i64",
                                "method": "length",
                                "op": "boxcall",
                            },
                            *bundle,
                            {"op": "ret", "value": 2},
                        ],
                    }
                ],
                "metadata": {"value_types": {}},
            }
        ],
        "user_box_decls": [],
    }


class MirCanonicalCompareTests(unittest.TestCase):
    def test_phi_local_def_bundle_reorder_is_canonicalized(self) -> None:
        lhs = _phi_bundle_module([2, 4, 3, 0, 1])
        rhs = _phi_bundle_module([0, 1, 2, 3, 4])
        self.assertEqual(_dump(lhs), _dump(rhs))

    def test_boxcall_local_def_bundle_reorder_is_canonicalized(self) -> None:
        lhs = _boxcall_bundle_module([2, 0, 1])
        rhs = _boxcall_bundle_module([0, 1, 2])
        self.assertEqual(_dump(lhs), _dump(rhs))

    def test_copy_bundle_semantic_difference_still_fails(self) -> None:
        lhs = _phi_bundle_module([2, 4, 3, 0, 1])
        rhs = _phi_bundle_module([2, 4, 3, 0, 0])
        self.assertNotEqual(_dump(lhs), _dump(rhs))


if __name__ == "__main__":
    unittest.main()
