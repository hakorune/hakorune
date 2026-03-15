#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

from llvmlite import ir

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from builders.function_lower import _build_predecessor_map, _create_basic_blocks, _index_blocks_by_id


class _BuilderStub:
    def __init__(self):
        self.bb_map = {}


class TestFunctionLowerCfgScaffold(unittest.TestCase):
    def test_build_predecessor_map_tracks_jump_and_branch_edges(self):
        blocks = [
            {"id": 1, "instructions": [{"op": "branch", "cond": 7, "then": 2, "else": 3}]},
            {"id": 2, "instructions": [{"op": "jump", "target": 4}]},
            {"id": 3, "instructions": [{"op": "ret", "value": 0}]},
            {"id": 4, "instructions": [{"op": "ret", "value": 1}]},
        ]

        self.assertEqual(_build_predecessor_map(blocks), {1: [], 2: [1], 3: [1], 4: [2]})

    def test_create_basic_blocks_populates_bb_map(self):
        builder = _BuilderStub()
        module = ir.Module(name="test_mod")
        i64 = ir.IntType(64)
        func = ir.Function(module, ir.FunctionType(i64, []), name="Demo/0")

        _create_basic_blocks(builder, func, [{"id": 5}, {"id": 8}])

        self.assertEqual(sorted(builder.bb_map.keys()), [5, 8])
        self.assertEqual(str(builder.bb_map[5].name), "bb5")
        self.assertEqual(str(builder.bb_map[8].name), "bb8")

    def test_index_blocks_by_id_builds_direct_lookup(self):
        blocks = [{"id": 2, "tag": "a"}, {"id": 7, "tag": "b"}]

        block_by_id = _index_blocks_by_id(blocks)

        self.assertEqual(block_by_id[2]["tag"], "a")
        self.assertEqual(block_by_id[7]["tag"], "b")


if __name__ == "__main__":
    unittest.main()
