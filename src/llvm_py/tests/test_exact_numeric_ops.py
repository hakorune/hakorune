#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

import llvmlite.ir as ir

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from builders.function_metadata import _load_exact_numeric_route_metadata
from instructions.binop import lower_binop
from instructions.compare import lower_compare


class _ResolverStub:
    def __init__(self):
        self.value_types = {}
        self.def_blocks = {}
        self.exact_numeric_binary_op_routes_by_dst = {}
        self.exact_numeric_compare_routes_by_dst = {}
        self.exact_numeric_shift_routes_by_dst = {}

    def resolve_i64(self, value_id, current_block, preds, block_end_values, vmap, bb_map):
        return ir.Constant(ir.IntType(64), int(value_id))


class _BuilderStub:
    def __init__(self):
        self.resolver = _ResolverStub()


class TestExactNumericOps(unittest.TestCase):
    def _make_builder(self):
        mod = ir.Module(name="exact_numeric_ops")
        i64 = ir.IntType(64)
        fn = ir.Function(mod, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("bb1")
        return mod, ir.IRBuilder(bb), bb, i64

    def test_function_metadata_loads_exact_numeric_routes_by_dst(self):
        builder = _BuilderStub()
        _load_exact_numeric_route_metadata(
            builder,
            {
                "metadata": {
                    "exact_numeric_binary_op_routes": [
                        {
                            "dst": 3,
                            "operation": "+",
                            "lhs": 1,
                            "rhs": 2,
                            "declared_type": "usize",
                        }
                    ],
                    "exact_numeric_compare_routes": [
                        {
                            "dst": 4,
                            "operation": "<",
                            "lhs": 1,
                            "rhs": 2,
                            "declared_type": "usize",
                        }
                    ],
                    "exact_numeric_shift_routes": [
                        {
                            "dst": 5,
                            "operation": ">>",
                            "lhs": 1,
                            "rhs": 2,
                            "declared_type": "usize",
                        }
                    ],
                }
            },
        )

        self.assertEqual(
            builder.resolver.exact_numeric_binary_op_routes_by_dst[3]["declared_type"],
            "usize",
        )
        self.assertEqual(builder.resolver.exact_numeric_compare_routes_by_dst[4]["lhs"], 1)
        self.assertEqual(builder.resolver.exact_numeric_shift_routes_by_dst[5]["rhs"], 2)

    def test_exact_usize_add_uses_checked_unsigned_overflow_intrinsic(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.exact_numeric_binary_op_routes_by_dst[3] = {
            "dst": 3,
            "operation": "+",
            "lhs": 1,
            "rhs": 2,
            "declared_type": "usize",
        }
        vmap = {1: ir.Constant(i64, 4), 2: ir.Constant(i64, 5)}

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
        self.assertIn('@"llvm.uadd.with.overflow.i64"', ir_txt, msg=ir_txt)
        self.assertIn("extractvalue", ir_txt, msg=ir_txt)
        self.assertIn("unreachable", ir_txt, msg=ir_txt)
        self.assertIn(3, vmap)

    def test_exact_usize_compare_uses_unsigned_predicate(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.exact_numeric_compare_routes_by_dst[3] = {
            "dst": 3,
            "operation": "<",
            "lhs": 1,
            "rhs": 2,
            "declared_type": "usize",
        }
        vmap = {1: ir.Constant(i64, -1), 2: ir.Constant(i64, 1)}

        lower_compare(
            builder,
            "Lt",
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
        self.assertIn("icmp ult i64", ir_txt, msg=ir_txt)
        self.assertNotIn("icmp slt i64", ir_txt, msg=ir_txt)
        self.assertIn(3, vmap)

    def test_exact_usize_right_shift_uses_lshr_and_shift_count_trap(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.exact_numeric_shift_routes_by_dst[3] = {
            "dst": 3,
            "operation": ">>",
            "lhs": 1,
            "rhs": 2,
            "declared_type": "usize",
        }
        vmap = {1: ir.Constant(i64, 8), 2: ir.Constant(i64, 1)}

        lower_binop(
            builder,
            resolver,
            ">>",
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
        self.assertIn("icmp uge i64", ir_txt, msg=ir_txt)
        self.assertIn("lshr i64", ir_txt, msg=ir_txt)
        self.assertNotIn("ashr i64", ir_txt, msg=ir_txt)
        self.assertIn("unreachable", ir_txt, msg=ir_txt)


if __name__ == "__main__":
    unittest.main()
