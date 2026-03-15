#!/usr/bin/env python3
"""
Contract tests for PHI incoming selection (Phase 97.2).

Goal:
- When multiple incoming candidates map to the same predecessor, do not let a
  failed candidate (synthesized 0) overwrite a successfully-resolved value.
"""

import unittest
import llvmlite.ir as ir

from src.llvm_py.phi_wiring.wiring import wire_incomings


class DummyContext:
    def get_block_snapshot(self, _block_id: int):
        return {}


class DummyResolver:
    def __init__(self, i64: ir.IntType):
        self.i64 = i64

    def resolve_incoming(self, _pred_bid: int, vs: int, context=None):
        # 100/200 are "missing" (simulate snapshot miss -> failure -> synthesized 0).
        if int(vs) == 101:
            return ir.Constant(self.i64, 7)
        if int(vs) == 201:
            return ir.Constant(self.i64, 9)
        if int(vs) == 301:
            return 11
        return None


class DummyBuilder:
    pass


class TestPhiWiringSelection(unittest.TestCase):
    def setUp(self):
        self.mod = ir.Module(name="phi_wiring_selection_mod")
        i64 = ir.IntType(64)
        fn = ir.Function(self.mod, ir.FunctionType(i64, []), name="f")
        bb1 = fn.append_basic_block(name="bb1")
        bb2 = fn.append_basic_block(name="bb2")

        b = DummyBuilder()
        b.module = self.mod
        b.i64 = i64
        b.bb_map = {1: bb1, 2: bb2}
        b.preds = {2: [1]}
        b.vmap = {}
        b.resolver = DummyResolver(i64)
        b.predeclared_ret_phis = {}
        self.builder = b
        self.ctx = DummyContext()

    def _phi_incoming_values(self, dst_vid: int):
        phi = self.builder.vmap[dst_vid]
        incoming = list(getattr(phi, "incomings", []))
        return [int(getattr(v, "constant", 0)) for (v, _blk) in incoming]

    def test_prefers_success_over_synthesized_zero_for_same_pred(self):
        dst_vid = 500
        # Two candidates map to the same predecessor:
        # - v100 fails -> becomes 0
        # - v101 succeeds -> becomes 7, and must replace the 0
        wire_incomings(self.builder, 2, dst_vid, [(1, 100), (1, 101)], context=self.ctx)
        self.assertIn(dst_vid, self.builder.vmap)
        self.assertEqual(self._phi_incoming_values(dst_vid), [7])

    def test_does_not_overwrite_success_with_failed_candidate(self):
        dst_vid = 501
        # Success comes first; later failure must not overwrite it.
        wire_incomings(self.builder, 2, dst_vid, [(1, 101), (1, 100)], context=self.ctx)
        self.assertIn(dst_vid, self.builder.vmap)
        self.assertEqual(self._phi_incoming_values(dst_vid), [7])

    def test_self_carry_reuses_first_non_self_source(self):
        dst_vid = 500
        wire_incomings(self.builder, 2, dst_vid, [(1, 500), (1, 101)], context=self.ctx)
        self.assertIn(dst_vid, self.builder.vmap)
        self.assertEqual(self._phi_incoming_values(dst_vid), [7])

    def test_coerces_plain_int_resolve_result_to_i64_constant(self):
        dst_vid = 502
        wire_incomings(self.builder, 2, dst_vid, [(1, 301)], context=self.ctx)
        self.assertIn(dst_vid, self.builder.vmap)
        self.assertEqual(self._phi_incoming_values(dst_vid), [11])


if __name__ == "__main__":
    unittest.main()
