#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

import llvmlite.ir as ir

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from instructions.copy import lower_copy
from instructions.sum_ops import lower_variant_make, lower_variant_project, lower_variant_tag
from type_facts import make_box_handle_fact


class _ResolverStub:
    def __init__(self):
        self.value_types = {}
        self.integerish_ids = set()
        self.def_blocks = {}
        self.sum_payload_facts = {}
        self.sum_local_aggregate_paths = {}
        self.sum_local_aggregate_layouts = {}
        self.global_vmap = None

    def resolve_i64(self, value_id, current_block, preds, block_end_values, vmap, bb_map):
        return ir.Constant(ir.IntType(64), int(value_id))

    def mark_string(self, value_id):
        self.value_types[int(value_id)] = make_box_handle_fact("StringBox")


class TestSumOps(unittest.TestCase):
    def _make_builder(self):
        mod = ir.Module(name="sum_ops")
        i64 = ir.IntType(64)
        fn = ir.Function(mod, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("bb1")
        return mod, ir.IRBuilder(bb), bb, i64

    def test_variant_make_uses_typed_integer_payload_lane_for_concrete_payload(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        vmap = {1: ir.Constant(i64, 7)}
        resolver.value_types[1] = "i64"

        lower_variant_make(
            builder,
            mod,
            2,
            "OptionInt",
            "Some",
            1,
            1,
            "Integer",
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn("__NyVariant_OptionInt", ir_txt, msg=ir_txt)
        self.assertIn('call i64 @"nyash.instance.set_i64_field_h"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.instance.set_field_h"', ir_txt, msg=ir_txt)
        self.assertEqual(vmap[2].type.width, 64)

    def test_variant_make_generic_integer_payload_uses_actual_integer_fact_for_typed_storage(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        vmap = {1: ir.Constant(i64, 7)}
        resolver.value_types[1] = "i64"

        lower_variant_make(
            builder,
            mod,
            2,
            "Option",
            "Some",
            1,
            1,
            None,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn('call i64 @"nyash.instance.set_i64_field_h"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.box.from_i64"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.instance.set_field_h"', ir_txt, msg=ir_txt)
        self.assertEqual(resolver.sum_payload_facts[2], "i64")

    def test_variant_make_generic_handle_payload_keeps_handle_storage(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        vmap = {1: ir.Constant(i64, 777)}
        resolver.value_types[1] = make_box_handle_fact("Point")

        lower_variant_make(
            builder,
            mod,
            2,
            "Option",
            "Some",
            1,
            1,
            None,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn('call i64 @"nyash.instance.set_field_h"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.box.from_i64"', ir_txt, msg=ir_txt)
        self.assertEqual(resolver.sum_payload_facts[2], make_box_handle_fact("Point"))

    def test_variant_make_selected_local_aggregate_skips_outer_runtime_box(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        vmap = {1: ir.Constant(i64, 7)}
        resolver.value_types[1] = "i64"
        resolver.sum_local_aggregate_paths[2] = "local_aggregate"
        resolver.sum_local_aggregate_layouts[2] = "tag_i64_payload"

        lower_variant_make(
            builder,
            mod,
            2,
            "Option",
            "Some",
            1,
            1,
            "Integer",
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertNotIn("__NyVariant_Option", ir_txt, msg=ir_txt)
        self.assertNotIn('nyash.instance.set_i64_field_h', ir_txt, msg=ir_txt)
        self.assertIsInstance(vmap[2], dict)
        self.assertEqual(vmap[2]["kind"], "local_sum_aggregate")
        self.assertEqual(vmap[2]["layout"], "tag_i64_payload")
        self.assertEqual(resolver.sum_payload_facts[2], "i64")

    def test_variant_project_uses_typed_integer_getter_and_fail_fast_trap(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        vmap = {1: ir.Constant(i64, 101)}
        resolver.value_types[1] = make_box_handle_fact("__NyVariant_OptionInt")

        lower_variant_project(
            builder,
            mod,
            2,
            1,
            "OptionInt",
            "Some",
            1,
            "Integer",
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn('call i64 @"nyash.instance.get_i64_field_h"', ir_txt, msg=ir_txt)
        self.assertIn("unreachable", ir_txt, msg=ir_txt)
        self.assertEqual(resolver.value_types[2], "i64")
        self.assertIn(2, resolver.integerish_ids)

    def test_variant_project_generic_integer_uses_recorded_fact_for_typed_getter(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        vmap = {1: ir.Constant(i64, 7)}
        resolver.value_types[1] = "i64"

        lower_variant_make(
            builder,
            mod,
            2,
            "Option",
            "Some",
            1,
            1,
            None,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )
        lower_variant_project(
            builder,
            mod,
            3,
            2,
            "Option",
            "Some",
            1,
            None,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn('call i64 @"nyash.instance.get_i64_field_h"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.instance.get_field_h"', ir_txt, msg=ir_txt)
        self.assertEqual(resolver.value_types[3], "i64")

    def test_variant_project_generic_bool_uses_recorded_fact_for_typed_getter(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        bool_val = ir.Constant(ir.IntType(1), 1)
        vmap = {1: bool_val}
        resolver.value_types[1] = "i1"

        lower_variant_make(
            builder,
            mod,
            2,
            "Option",
            "Some",
            1,
            1,
            None,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )
        lower_variant_project(
            builder,
            mod,
            3,
            2,
            "Option",
            "Some",
            1,
            None,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn('call i64 @"nyash.instance.set_bool_field_h"', ir_txt, msg=ir_txt)
        self.assertIn('call i64 @"nyash.instance.get_bool_field_h"', ir_txt, msg=ir_txt)
        self.assertEqual(resolver.value_types[3], "i1")

    def test_variant_project_generic_float_uses_recorded_fact_for_typed_getter(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        vmap = {1: ir.Constant(ir.DoubleType(), 1.25)}
        resolver.value_types[1] = "Float"

        lower_variant_make(
            builder,
            mod,
            2,
            "Option",
            "Some",
            1,
            1,
            None,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )
        lower_variant_project(
            builder,
            mod,
            3,
            2,
            "Option",
            "Some",
            1,
            None,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn('call i64 @"nyash.instance.set_float_field_h"', ir_txt, msg=ir_txt)
        self.assertIn('call double @"nyash.instance.get_float_field_h"', ir_txt, msg=ir_txt)
        self.assertEqual(resolver.value_types[3], "Float")

    def test_variant_tag_reads_selected_local_aggregate_without_runtime_getter(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        vmap = {1: ir.Constant(i64, 7)}
        resolver.value_types[1] = "i64"
        resolver.sum_local_aggregate_paths[2] = "local_aggregate"
        resolver.sum_local_aggregate_layouts[2] = "tag_i64_payload"

        lower_variant_make(
            builder,
            mod,
            2,
            "Option",
            "Some",
            1,
            1,
            "Integer",
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )
        lower_variant_tag(
            builder,
            mod,
            3,
            2,
            "Option",
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertNotIn('call i64 @"nyash.instance.get_i64_field_h"', ir_txt, msg=ir_txt)
        self.assertNotIn("__NyVariant_Option", ir_txt, msg=ir_txt)
        self.assertEqual(vmap[3].type.width, 64)

    def test_variant_project_reads_selected_local_aggregate_without_runtime_getter(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        vmap = {1: ir.Constant(i64, 7)}
        resolver.value_types[1] = "i64"
        resolver.sum_local_aggregate_paths[2] = "local_aggregate"
        resolver.sum_local_aggregate_layouts[2] = "tag_i64_payload"

        lower_variant_make(
            builder,
            mod,
            2,
            "Option",
            "Some",
            1,
            1,
            None,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )
        lower_variant_project(
            builder,
            mod,
            3,
            2,
            "Option",
            "Some",
            1,
            None,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertNotIn('call i64 @"nyash.instance.get_i64_field_h"', ir_txt, msg=ir_txt)
        self.assertNotIn("__NyVariant_Option", ir_txt, msg=ir_txt)
        self.assertIn("unreachable", ir_txt, msg=ir_txt)
        self.assertEqual(resolver.value_types[3], "i64")

    def test_copy_preserves_selected_local_sum_aggregate_alias(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        vmap = {1: ir.Constant(i64, 7)}
        resolver.value_types[1] = "i64"
        resolver.sum_local_aggregate_paths[2] = "local_aggregate"
        resolver.sum_local_aggregate_layouts[2] = "tag_i64_payload"

        lower_variant_make(
            builder,
            mod,
            2,
            "Option",
            "Some",
            1,
            1,
            None,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )
        lower_copy(
            builder,
            4,
            2,
            vmap,
            resolver=resolver,
            current_block=bb,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )
        lower_variant_project(
            builder,
            mod,
            5,
            4,
            "Option",
            "Some",
            1,
            None,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertNotIn('call i64 @"nyash.instance.get_i64_field_h"', ir_txt, msg=ir_txt)
        self.assertNotIn("__NyVariant_Option", ir_txt, msg=ir_txt)
        self.assertEqual(resolver.value_types[5], "i64")

    def test_copy_preserves_selected_local_sum_float_alias(self):
        mod, builder, bb, _i64 = self._make_builder()
        resolver = _ResolverStub()
        vmap = {1: ir.Constant(ir.DoubleType(), 1.5)}
        resolver.value_types[1] = "Float"
        resolver.sum_local_aggregate_paths[2] = "local_aggregate"
        resolver.sum_local_aggregate_layouts[2] = "tag_f64_payload"

        lower_variant_make(
            builder,
            mod,
            2,
            "Option",
            "Some",
            1,
            1,
            None,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )
        lower_copy(
            builder,
            4,
            2,
            vmap,
            resolver=resolver,
            current_block=bb,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )
        lower_variant_project(
            builder,
            mod,
            5,
            4,
            "Option",
            "Some",
            1,
            None,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertNotIn('call double @"nyash.instance.get_float_field_h"', ir_txt, msg=ir_txt)
        self.assertNotIn("__NyVariant_Option", ir_txt, msg=ir_txt)
        self.assertEqual(resolver.value_types[5], "Float")
        self.assertEqual(vmap[5].type, ir.DoubleType())

    def test_copy_preserves_selected_local_sum_handle_alias(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        vmap = {1: ir.Constant(i64, 777)}
        resolver.value_types[1] = make_box_handle_fact("StringBox")
        resolver.sum_local_aggregate_paths[2] = "local_aggregate"
        resolver.sum_local_aggregate_layouts[2] = "tag_handle_payload"

        lower_variant_make(
            builder,
            mod,
            2,
            "Option",
            "Some",
            1,
            1,
            None,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )
        lower_copy(
            builder,
            4,
            2,
            vmap,
            resolver=resolver,
            current_block=bb,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )
        lower_variant_project(
            builder,
            mod,
            5,
            4,
            "Option",
            "Some",
            1,
            None,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertNotIn('call i64 @"nyash.instance.get_field_h"', ir_txt, msg=ir_txt)
        self.assertNotIn("__NyVariant_Option", ir_txt, msg=ir_txt)
        self.assertEqual(resolver.value_types[5], make_box_handle_fact("StringBox"))
        self.assertEqual(vmap[5].type.width, 64)


if __name__ == "__main__":
    unittest.main()
