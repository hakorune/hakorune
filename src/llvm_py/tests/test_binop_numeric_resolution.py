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

    def resolve_i64(self, value_id, current_block, preds, block_end_values, vmap, bb_map):
        self.resolve_calls += 1
        return ir.Constant(ir.IntType(64), int(value_id))


class TestBinopNumericResolution(unittest.TestCase):
    def _make_builder(self):
        mod = ir.Module(name="binop_numeric_resolution")
        i64 = ir.IntType(64)
        fn = ir.Function(mod, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("bb1")
        return mod, ir.IRBuilder(bb), bb, i64

    def test_alias_add_normalizes_to_integer_add(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        vmap = {1: ir.Constant(i64, 4), 2: ir.Constant(i64, 5)}

        lower_binop(
            builder,
            resolver,
            "add",
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

        self.assertIn("add_", getattr(vmap[3], "name", ""))
        self.assertIn(" add i64 ", str(mod))

    def test_fast_int_prefers_local_vmap_without_resolver_roundtrip(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        vmap = {1: ir.Constant(i64, 7), 2: ir.Constant(i64, 9)}
        prev_fast_int = os.environ.get("NYASH_LLVM_FAST_INT")
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
            if prev_fast_int is None:
                os.environ.pop("NYASH_LLVM_FAST_INT", None)
            else:
                os.environ["NYASH_LLVM_FAST_INT"] = prev_fast_int

        self.assertEqual(resolver.resolve_calls, 0)
        self.assertIn(" add i64 ", str(mod))


if __name__ == "__main__":
    unittest.main()
