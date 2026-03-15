#!/usr/bin/env python3
import os
import sys
import unittest
from pathlib import Path

import llvmlite.ir as ir

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from instructions.binop import lower_binop


class _ResolverStub:
    def __init__(self):
        self.resolve_calls = 0
        self.binop_expr_cache = {}
        self.context = None

    def resolve_i64(self, value_id, current_block, preds, block_end_values, vmap, bb_map):
        self.resolve_calls += 1
        return ir.Constant(ir.IntType(64), int(value_id))


class TestBinopNumericTail(unittest.TestCase):
    def _make_builder(self):
        mod = ir.Module(name="binop_numeric_tail")
        i64 = ir.IntType(64)
        fn = ir.Function(mod, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("bb1")
        return mod, ir.IRBuilder(bb), bb, i64

    def test_expr_cache_hit_reuses_cached_value(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        cached = ir.Constant(i64, 33)
        resolver.binop_expr_cache[("i64", "+", ("c_i", 64, 4), ("c_i", 64, 5))] = (cached, 1)
        vmap = {1: ir.Constant(i64, 4), 2: ir.Constant(i64, 5)}
        prev_fast = os.environ.get("NYASH_LLVM_FAST")
        prev_fast_int = os.environ.get("NYASH_LLVM_FAST_INT")
        os.environ["NYASH_LLVM_FAST"] = "1"
        os.environ["NYASH_LLVM_FAST_INT"] = "1"
        try:
            lower_binop(
                builder,
                resolver,
                "+",
                1,
                2,
                3,
                vmap,
                bb,
                preds={},
                block_end_values={},
                bb_map={1: bb},
                dst_type="i64",
            )
        finally:
            if prev_fast is None:
                os.environ.pop("NYASH_LLVM_FAST", None)
            else:
                os.environ["NYASH_LLVM_FAST"] = prev_fast
            if prev_fast_int is None:
                os.environ.pop("NYASH_LLVM_FAST_INT", None)
            else:
                os.environ["NYASH_LLVM_FAST_INT"] = prev_fast_int

        self.assertIs(vmap[3], cached)
        self.assertNotIn(" add i64 ", str(mod))

    def test_shift_alias_dispatches_numeric_tail(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        vmap = {1: ir.Constant(i64, 4), 2: ir.Constant(i64, 1)}

        lower_binop(
            builder,
            resolver,
            "shl",
            1,
            2,
            3,
            vmap,
            bb,
            preds={},
            block_end_values={},
            bb_map={1: bb},
            dst_type="i64",
        )

        self.assertIn("shl_", getattr(vmap[3], "name", ""))
        self.assertIn(" shl i64 ", str(mod))


if __name__ == "__main__":
    unittest.main()
