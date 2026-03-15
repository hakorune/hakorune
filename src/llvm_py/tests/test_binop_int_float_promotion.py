#!/usr/bin/env python3
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
        self.value_types = {}
        self.def_blocks = {}

    def resolve_i64(self, value_id, current_block, preds, block_end_values, vmap, bb_map):
        return ir.Constant(ir.IntType(64), int(value_id))


class TestBinopIntFloatPromotion(unittest.TestCase):
    def _make_builder(self):
        mod = ir.Module(name="binop_int_float")
        i64 = ir.IntType(64)
        fn = ir.Function(mod, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("bb1")
        return mod, ir.IRBuilder(bb), bb, i64

    def test_int_plus_double_constant_uses_fadd_without_unbox(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = "Integer"
        resolver.value_types[2] = "Float"
        vmap = {
            1: ir.Constant(i64, 3),
            2: ir.Constant(ir.DoubleType(), 1.5),
        }

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
        )

        ir_txt = str(mod)
        self.assertIn("fadd double", ir_txt, msg=ir_txt)
        self.assertNotIn('@"nyash.float.unbox_to_f64"', ir_txt, msg=ir_txt)
        self.assertTrue(hasattr(vmap[3], "type"))
        self.assertIsInstance(vmap[3].type, ir.DoubleType)

    def test_float_handle_plus_int_unboxes_before_fadd(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = "Float"
        resolver.value_types[2] = "Integer"
        vmap = {
            1: ir.Constant(i64, 100),
            2: ir.Constant(i64, 3),
        }

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
        )

        ir_txt = str(mod)
        self.assertIn('@"nyash.float.unbox_to_f64"', ir_txt, msg=ir_txt)
        self.assertIn("fadd double", ir_txt, msg=ir_txt)
        self.assertTrue(hasattr(vmap[3], "type"))
        self.assertIsInstance(vmap[3].type, ir.DoubleType)


if __name__ == "__main__":
    unittest.main()
