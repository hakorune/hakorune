#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

import llvmlite.ir as ir

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from instructions.binop import lower_binop
from instructions.compare import lower_compare
from type_facts import make_box_handle_fact


class _ResolverStub:
    def __init__(self):
        self.value_types = {}
        self.def_blocks = {}

    def resolve_i64(self, value_id, current_block, preds, block_end_values, vmap, bb_map):
        return ir.Constant(ir.IntType(64), int(value_id))


class TestPrimitiveHandleNumericLowering(unittest.TestCase):
    def _make_builder(self):
        mod = ir.Module(name="primitive_handle_numeric")
        i64 = ir.IntType(64)
        fn = ir.Function(mod, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("bb1")
        return mod, ir.IRBuilder(bb), bb, i64

    def test_integerbox_add_unboxes_before_integer_add(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("IntegerBox")
        resolver.value_types[2] = make_box_handle_fact("IntegerBox")
        vmap = {
            1: ir.Constant(i64, 101),
            2: ir.Constant(i64, 202),
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
            dst_type="i64",
        )

        ir_txt = str(mod)
        self.assertIn('@"nyash.integer.get_h"', ir_txt, msg=ir_txt)
        self.assertEqual(ir_txt.count('call i64 @"nyash.integer.get_h"'), 2, msg=ir_txt)
        self.assertIn(" add i64 ", ir_txt, msg=ir_txt)

    def test_integerbox_plus_float_unboxes_before_fadd(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("IntegerBox")
        resolver.value_types[2] = "Float"
        vmap = {
            1: ir.Constant(i64, 303),
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
        self.assertIn('call i64 @"nyash.integer.get_h"', ir_txt, msg=ir_txt)
        self.assertIn("fadd double", ir_txt, msg=ir_txt)

    def test_boolbox_compare_unboxes_before_icmp(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("BoolBox")
        resolver.value_types[2] = make_box_handle_fact("BoolBox")
        vmap = {
            1: ir.Constant(i64, 1),
            2: ir.Constant(i64, 0),
        }

        lower_compare(
            builder,
            "==",
            1,
            2,
            3,
            vmap,
            resolver=resolver,
            current_block=bb,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertEqual(ir_txt.count('call i64 @"nyash.bool.get_h"'), 2, msg=ir_txt)
        self.assertIn("icmp eq i64", ir_txt, msg=ir_txt)


if __name__ == "__main__":
    unittest.main()
