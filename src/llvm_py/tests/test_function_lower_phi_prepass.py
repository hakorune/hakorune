#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from builders.function_lower import (
    _collect_block_defs,
    _collect_block_uses,
    _dedup_non_self_preds,
    _seed_multi_pred_block_phi_incomings,
)


class _ResolverStub:
    def __init__(self):
        self.block_phi_incomings = None


class _BuilderStub:
    def __init__(self):
        self.block_phi_incomings = {}
        self.resolver = _ResolverStub()


class TestFunctionLowerPhiPrepass(unittest.TestCase):
    def test_dedup_non_self_preds_filters_duplicates_and_self_edges(self):
        preds_map = {3: [1, 3, 1, 2, 2]}
        self.assertEqual(_dedup_non_self_preds(preds_map, 3), [1, 2])

    def test_collect_block_defs_and_uses(self):
        block = {
            "instructions": [
                {"op": "copy", "dst": 10, "src": 1},
                {"op": "binop", "dst": 11, "lhs": 2, "rhs": 3},
                {"op": "branch", "cond": 4},
                {"op": "newbox", "dst": 12, "box_val": 5},
            ]
        }

        self.assertEqual(_collect_block_defs(block), {10, 11, 12})
        self.assertEqual(_collect_block_uses(block), {2, 3, 4, 5})

    def test_seed_multi_pred_block_phi_incomings_only_for_needed_values(self):
        builder = _BuilderStub()
        block_by_id = {
            0: {
                "id": 0,
                "instructions": [
                    {"op": "const", "dst": 7, "value": {"type": "int", "value": 1}},
                    {"op": "branch", "cond": 7, "then": 2, "else": 3},
                ],
            },
            1: {
                "id": 1,
                "instructions": [
                    {"op": "const", "dst": 8, "value": {"type": "int", "value": 2}},
                    {"op": "jump", "target": 2},
                ],
            },
            2: {
                "id": 2,
                "instructions": [
                    {"op": "binop", "dst": 9, "lhs": 7, "rhs": 8, "operation": "+"},
                    {"op": "copy", "dst": 10, "src": 9},
                ],
            },
            3: {
                "id": 3,
                "instructions": [
                    {"op": "copy", "dst": 11, "src": 7},
                ],
            },
        }

        _seed_multi_pred_block_phi_incomings(builder, block_by_id)

        self.assertEqual(builder.block_phi_incomings, {2: {7: [(0, 7), (1, 7)], 8: [(0, 8), (1, 8)]}})
        self.assertIs(builder.resolver.block_phi_incomings, builder.block_phi_incomings)


if __name__ == "__main__":
    unittest.main()
