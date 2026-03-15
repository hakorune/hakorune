#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from builders.function_lower import _compute_lower_order, _determine_entry_block_id


class TestFunctionLowerOrdering(unittest.TestCase):
    def test_determine_entry_block_id_prefers_pred_free_block(self):
        preds_map = {3: [1], 1: [], 2: [1]}
        blocks = [{"id": 9}, {"id": 1}]

        self.assertEqual(_determine_entry_block_id(preds_map, blocks), 1)

    def test_determine_entry_block_id_falls_back_to_first_block(self):
        preds_map = {3: [1], 1: [3]}
        blocks = [{"id": 7}, {"id": 1}]

        self.assertEqual(_determine_entry_block_id(preds_map, blocks), 7)

    def test_compute_lower_order_keeps_unreachable_deterministic(self):
        block_by_id = {
            1: {"id": 1, "instructions": [{"op": "branch", "cond": 10, "then": 2, "else": 3}]},
            2: {"id": 2, "instructions": [{"op": "jump", "target": 4}]},
            3: {"id": 3, "instructions": [{"op": "jump", "target": 4}]},
            4: {"id": 4, "instructions": [{"op": "ret", "value": 11}]},
            9: {"id": 9, "instructions": [{"op": "ret", "value": 99}]},
        }

        order, reachable, dominators = _compute_lower_order(block_by_id, 1)

        self.assertEqual(order, [9, 1, 3, 2, 4])
        self.assertEqual(reachable, {1, 2, 3, 4})
        self.assertEqual(dominators[1], {1})
        self.assertEqual(dominators[4], {1, 4})


if __name__ == "__main__":
    unittest.main()
