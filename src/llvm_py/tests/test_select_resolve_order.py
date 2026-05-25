#!/usr/bin/env python3
import unittest
import sys
from pathlib import Path

import llvmlite.ir as ir

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from resolver import Resolver
from instructions.select import lower_select


class TestSelectResolveOrder(unittest.TestCase):
    def setUp(self):
        self.i64 = ir.IntType(64)
        self.mod = ir.Module(name="select_resolve_order")
        fn = ir.Function(self.mod, ir.FunctionType(self.i64, []), name="main")
        self.bb = fn.append_basic_block("bb4")
        self.bb_map = {4: self.bb}
        self.resolver = Resolver({}, self.bb_map)

    def test_prefers_same_block_vmap_before_snapshot(self):
        vmap = {10: ir.Constant(self.i64, 7)}
        block_end_values = {4: {10: ir.Constant(self.i64, 99)}}
        got = self.resolver.resolve_same_block_then_snapshot_i64(
            10, self.bb, {}, block_end_values, vmap, self.bb_map
        )
        self.assertEqual(int(getattr(got, "constant", -1)), 7)

    def test_uses_snapshot_when_same_block_value_missing(self):
        vmap = {}
        block_end_values = {4: {20: ir.Constant(self.i64, 5)}}
        got = self.resolver.resolve_same_block_then_snapshot_i64(
            20, self.bb, {}, block_end_values, vmap, self.bb_map
        )
        self.assertEqual(int(getattr(got, "constant", -1)), 5)

    def test_same_block_i1_is_canonicalized_to_i64(self):
        vmap = {40: ir.Constant(ir.IntType(1), 1)}
        got = self.resolver.resolve_same_block_then_snapshot_i64(
            40, self.bb, {}, {}, vmap, self.bb_map
        )
        self.assertEqual(got.type, self.i64)

    def test_falls_back_to_zero_when_missing_everywhere(self):
        got = self.resolver.resolve_same_block_then_snapshot_i64(
            30, self.bb, {}, {}, {}, self.bb_map
        )
        self.assertEqual(int(getattr(got, "constant", -1)), 0)

    def test_lower_select_canonicalizes_i1_branch_values_to_i64(self):
        vmap = {
            1: ir.Constant(ir.IntType(1), 1),
            2: ir.Constant(ir.IntType(1), 0),
            3: ir.Constant(ir.IntType(1), 1),
        }

        lower_select(
            ir.IRBuilder(self.bb),
            self.resolver,
            1,
            2,
            3,
            4,
            vmap,
            {},
            {},
            self.bb_map,
        )

        self.assertEqual(vmap[4].type, self.i64)


if __name__ == "__main__":
    unittest.main()
