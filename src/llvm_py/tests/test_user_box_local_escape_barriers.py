#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

import llvmlite.ir as ir

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from instructions.boxcall import lower_boxcall
from instructions.call import lower_call
from instructions.ret import lower_return
from type_facts import make_box_handle_fact


class _ResolverStub:
    def __init__(self):
        self.value_types = {}
        self.integerish_ids = set()
        self.array_ids = set()
        self.string_literals = {}
        self.string_ptrs = {}
        self.def_blocks = {}
        self.user_box_local_aggregate_layouts = {}
        self.global_vmap = None

    def resolve_i64(self, value_id, current_block, preds, block_end_values, vmap, bb_map):
        return ir.Constant(ir.IntType(64), int(value_id))

    def resolve_ptr(self, value_id, current_block, preds, block_end_values, vmap):
        return None

    def is_arrayish(self, value_id: int) -> bool:
        value = self.value_types.get(int(value_id))
        return isinstance(value, dict) and value.get("box_type") == "ArrayBox"


class TestUserBoxLocalEscapeBarriers(unittest.TestCase):
    def _make_builder(self, name: str = "user_box_escape"):
        mod = ir.Module(name=name)
        i64 = ir.IntType(64)
        fn = ir.Function(mod, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("bb1")
        return mod, ir.IRBuilder(bb), bb, i64

    def _seed_local_point(self, i64):
        resolver = _ResolverStub()
        resolver.value_types[2] = make_box_handle_fact("Point")
        resolver.value_types[100] = {"kind": "handle", "box_type": "ArrayBox"}
        resolver.user_box_local_aggregate_layouts[2] = {
            "box_name": "Point",
            "field_order": ["x", "y"],
            "field_layouts": {"x": "inline_i64", "y": "inline_i64"},
            "reason": "test",
        }
        vmap = {
            2: {
                "kind": "local_user_box_aggregate",
                "box_name": "Point",
                "field_order": ["x", "y"],
                "field_layouts": {"x": "inline_i64", "y": "inline_i64"},
                "fields": {
                    "x": ir.Constant(i64, 1),
                    "y": ir.Constant(i64, 2),
                },
            },
            100: ir.Constant(i64, 7),
        }
        resolver.global_vmap = vmap
        return resolver, vmap

    def test_call_materializes_local_user_box_argument(self):
        mod, builder, bb, i64 = self._make_builder("user_box_escape_call")
        resolver, vmap = self._seed_local_point(i64)
        ir.Function(mod, ir.FunctionType(i64, [i64]), name="take_point")

        lower_call(
            builder,
            mod,
            "take_point",
            [2],
            3,
            vmap,
            resolver=resolver,
            preds={1: []},
            block_end_values={},
            bb_map={1: bb},
        )
        builder.ret(vmap[3])

        ir_txt = str(mod)
        self.assertIn('call i64 @"nyash.env.box.new_i64x"', ir_txt, msg=ir_txt)
        self.assertIn('call i64 @"nyash.instance.set_i64_field_h"', ir_txt, msg=ir_txt)
        self.assertIn('call i64 @"take_point"', ir_txt, msg=ir_txt)

    def test_boxcall_materializes_local_user_box_argument(self):
        mod, builder, bb, i64 = self._make_builder("user_box_escape_boxcall")
        resolver, vmap = self._seed_local_point(i64)

        lower_boxcall(
            builder=builder,
            module=mod,
            box_vid=100,
            method_name="push",
            args=[2],
            dst_vid=3,
            vmap=vmap,
            resolver=resolver,
            preds={1: []},
            block_end_values={},
            bb_map={1: bb},
        )
        builder.ret(vmap[3])

        ir_txt = str(mod)
        self.assertIn('call i64 @"nyash.env.box.new_i64x"', ir_txt, msg=ir_txt)
        self.assertIn('call i64 @"nyash.instance.set_i64_field_h"', ir_txt, msg=ir_txt)
        self.assertIn("nyash.array.slot_append_hh", ir_txt, msg=ir_txt)

    def test_return_materializes_local_user_box_value(self):
        mod, builder, bb, i64 = self._make_builder("user_box_escape_return")
        resolver, vmap = self._seed_local_point(i64)

        lower_return(
            builder,
            2,
            vmap,
            i64,
            resolver=resolver,
            preds={1: []},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn('call i64 @"nyash.env.box.new_i64x"', ir_txt, msg=ir_txt)
        self.assertIn('call i64 @"nyash.instance.set_i64_field_h"', ir_txt, msg=ir_txt)
        self.assertIn("ret i64", ir_txt, msg=ir_txt)


if __name__ == "__main__":
    unittest.main()
