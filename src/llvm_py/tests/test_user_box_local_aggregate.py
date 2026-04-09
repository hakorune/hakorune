#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

import llvmlite.ir as ir

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from instructions.copy import lower_copy
from instructions.field_access import lower_field_get, lower_field_set
from instructions.newbox import lower_newbox
from type_facts import make_box_handle_fact


class _ResolverStub:
    def __init__(self):
        self.value_types = {}
        self.integerish_ids = set()
        self.def_blocks = {}
        self.thin_entry_selection_by_value = {}
        self.thin_entry_selection_by_subject = {}
        self.thin_entry_selections = []
        self.user_box_local_aggregate_layouts = {}
        self.global_vmap = None

    def resolve_i64(self, value_id, current_block, preds, block_end_values, vmap, bb_map):
        return ir.Constant(ir.IntType(64), int(value_id))


class TestUserBoxLocalAggregate(unittest.TestCase):
    def _make_builder(self):
        mod = ir.Module(name="user_box_local_aggregate")
        i64 = ir.IntType(64)
        fn = ir.Function(mod, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("bb1")
        return mod, ir.IRBuilder(bb), bb, i64

    def _point_layout(self):
        return {
            "box_name": "Point",
            "field_order": ["x", "y"],
            "field_layouts": {"x": "inline_i64", "y": "inline_i64"},
            "reason": "test",
        }

    def test_newbox_uses_local_user_box_aggregate_when_layout_is_selected(self):
        mod, builder, _bb, _i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.user_box_local_aggregate_layouts[1] = self._point_layout()
        vmap = {}

        lower_newbox(builder, mod, "Point", [], 1, vmap, resolver)

        self.assertEqual(vmap[1]["kind"], "local_user_box_aggregate")
        self.assertEqual(vmap[1]["box_name"], "Point")
        self.assertNotIn("nyash.env.box.new_i64x", str(mod), msg=str(mod))

    def test_field_get_reads_local_user_box_aggregate_without_runtime_helper(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("Point")
        resolver.user_box_local_aggregate_layouts[1] = self._point_layout()
        vmap = {
            1: {
                "kind": "local_user_box_aggregate",
                "box_name": "Point",
                "field_order": ["x", "y"],
                "field_layouts": {"x": "inline_i64", "y": "inline_i64"},
                "fields": {
                    "x": ir.Constant(i64, 41),
                    "y": ir.Constant(i64, 2),
                },
            }
        }
        resolver.global_vmap = vmap

        lower_field_get(
            builder,
            mod,
            1,
            "x",
            2,
            {"kind": "handle", "box_type": "IntegerBox"},
            [],
            vmap,
            resolver,
            preds={1: []},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertNotIn('@"nyash.instance.get_i64_field_h"', ir_txt, msg=ir_txt)
        self.assertNotIn("RuntimeDataBox", ir_txt, msg=ir_txt)
        self.assertEqual(str(vmap[2]), "i64 41")

    def test_field_set_updates_local_user_box_aggregate_without_runtime_setter(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("Point")
        resolver.value_types[2] = "i64"
        resolver.user_box_local_aggregate_layouts[1] = self._point_layout()
        vmap = {
            1: {
                "kind": "local_user_box_aggregate",
                "box_name": "Point",
                "field_order": ["x", "y"],
                "field_layouts": {"x": "inline_i64", "y": "inline_i64"},
                "fields": {
                    "x": ir.Constant(i64, 1),
                    "y": ir.Constant(i64, 2),
                },
            },
            2: ir.Constant(i64, 99),
        }
        resolver.global_vmap = vmap

        lower_field_set(
            builder,
            mod,
            1,
            "x",
            2,
            {"kind": "handle", "box_type": "IntegerBox"},
            [],
            vmap,
            resolver,
            preds={1: []},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertNotIn('@"nyash.instance.set_i64_field_h"', ir_txt, msg=ir_txt)
        self.assertEqual(str(vmap[1]["fields"]["x"]), "i64 99")

    def test_copy_keeps_local_user_box_alias_route(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("Point")
        resolver.user_box_local_aggregate_layouts[1] = self._point_layout()
        vmap = {
            1: {
                "kind": "local_user_box_aggregate",
                "box_name": "Point",
                "field_order": ["x", "y"],
                "field_layouts": {"x": "inline_i64", "y": "inline_i64"},
                "fields": {
                    "x": ir.Constant(i64, 7),
                    "y": ir.Constant(i64, 9),
                },
            }
        }
        resolver.global_vmap = vmap

        lower_copy(
            builder,
            3,
            1,
            vmap,
            resolver=resolver,
            current_block=bb,
            preds={1: []},
            block_end_values={},
            bb_map={1: bb},
        )
        lower_field_get(
            builder,
            mod,
            3,
            "y",
            4,
            {"kind": "handle", "box_type": "IntegerBox"},
            [],
            vmap,
            resolver,
            preds={1: []},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertNotIn("RuntimeDataBox", ir_txt, msg=ir_txt)
        self.assertEqual(str(vmap[4]), "i64 9")


if __name__ == "__main__":
    unittest.main()
