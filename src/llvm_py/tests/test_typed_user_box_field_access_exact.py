#!/usr/bin/env python3
import os
import sys
import unittest
from pathlib import Path

import llvmlite.ir as ir

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from instructions.field_access import lower_field_get, lower_field_set
from instructions.newbox import lower_newbox
from builders.entry import ensure_ny_main
from instructions.field_access_helpers_typed import (
    _selected_typed_object_exact_slot_native_direct_route_decision,
    _selected_typed_object_exact_slot_route_decision,
)
from type_facts import make_box_handle_fact


class _ResolverStub:
    def __init__(self):
        self.value_types = {}
        self.integerish_ids = set()
        self.def_blocks = {}
        self.thin_entry_selection_by_value = {}
        self.thin_entry_selection_by_subject = {}
        self.thin_entry_selections = []
        self.route_decisions_by_site = {}
        self.route_decisions_metadata_present = False
        self.current_block_id = 0
        self.current_instruction_index = 0

    def resolve_i64(self, value_id, current_block, preds, block_end_values, vmap, bb_map):
        if value_id in vmap:
            return vmap[value_id]
        return ir.Constant(ir.IntType(64), int(value_id))


class TestTypedUserBoxFieldAccessExact(unittest.TestCase):
    def _make_builder(self, name="main"):
        mod = ir.Module(name="typed_user_box_field_access")
        i64 = ir.IntType(64)
        fn = ir.Function(mod, ir.FunctionType(i64, []), name=name)
        bb = fn.append_basic_block("bb1")
        return mod, ir.IRBuilder(bb), bb, i64

    def _exact_page_plan(self):
        return [
            {
                "box_name": "Page",
                "type_id": 294019300,
                "layout_kind": "typed_object_v0",
                "field_count": 2,
                "fields": [
                    {
                        "name": "capacity",
                        "slot": 0,
                        "declared_type": "usize",
                        "storage": "usize",
                        "weak": False,
                    },
                    {
                        "name": "used",
                        "slot": 1,
                        "declared_type": "i64",
                        "storage": "i64",
                        "weak": False,
                    },
                ],
            }
        ]

    def _exact_page_model_plan(self):
        return [
            {
                "box_name": "HakoAllocPageModel",
                "type_id": 294019301,
                "layout_kind": "typed_object_v0",
                "field_count": 4,
                "fields": [
                    {
                        "name": "free_top",
                        "slot": 0,
                        "declared_type": "usize",
                        "storage": "usize",
                        "weak": False,
                    },
                    {
                        "name": "used",
                        "slot": 1,
                        "declared_type": "u64",
                        "storage": "u64",
                        "weak": False,
                    },
                    {
                        "name": "next",
                        "slot": 2,
                        "declared_type": "handle",
                        "storage": "handle",
                        "weak": False,
                    },
                    {
                        "name": "signed_count",
                        "slot": 3,
                        "declared_type": "i64",
                        "storage": "i64",
                        "weak": False,
                    },
                ],
            }
        ]

    def _with_env(self, key, value, fn):
        old_value = os.environ.get(key)
        if value is None:
            os.environ.pop(key, None)
        else:
            os.environ[key] = value
        try:
            return fn()
        finally:
            if old_value is None:
                os.environ.pop(key, None)
            else:
                os.environ[key] = old_value

    def _seed_typed_object_exact_slot_route_decision(
        self,
        resolver,
        *,
        semantic_op,
        selected_route,
        selected_bridge_symbol,
        selected_slot,
        selected_storage,
        field_id,
        receiver_box_name="Page",
        block_id=0,
        instruction_index=0,
    ):
        resolver.route_decisions_metadata_present = True
        resolver.current_block_id = block_id
        resolver.current_instruction_index = instruction_index
        resolver.route_decisions_by_site[
            (block_id, instruction_index)
        ] = [
            {
                "route_id": "route.decision",
                "site_id": f"b{block_id}.i{instruction_index}",
                "block": block_id,
                "instruction_index": instruction_index,
                "semantic_op": semantic_op,
                "access_kind": f"typed_object_exact_slot_{semantic_op.lower()}",
                "preferred_route": selected_route,
                "selected_route": selected_route,
                "fallback_route": "compat_field_get_i64",
                "fallback_policy": "fail_fast",
                "proof_ids": [
                    "typed_object_plan",
                    "field_decl_authority",
                    "receiver_exact_type_id",
                    "slot_in_bounds",
                    "storage_exact_slot",
                    "non_weak_field",
                    "materialization_boundary_known",
                    "exact_slot_bridge_available",
                ],
                "miss_reason": None,
                "source_plan_kind": "TypedObjectExactSlotRoute",
                "selected_i64_const": None,
                "selected_bool_const": None,
                "selected_lowering_form": "exact_helper_bridge",
                "selected_bridge_symbol": selected_bridge_symbol,
                "selected_slot": selected_slot,
                "selected_storage": selected_storage,
                "receiver_box_name": receiver_box_name,
                "field_id": field_id,
            }
        ]

    def _seed_typed_object_exact_slot_native_direct_route_decision(
        self,
        resolver,
        *,
        semantic_op,
        selected_route,
        selected_slot,
        selected_storage,
        field_id,
        receiver_box_name="Page",
        block_id=0,
        instruction_index=0,
    ):
        resolver.route_decisions_metadata_present = True
        resolver.current_block_id = block_id
        resolver.current_instruction_index = instruction_index
        resolver.route_decisions_by_site[
            (block_id, instruction_index)
        ] = [
            {
                "route_id": "route.decision",
                "site_id": f"b{block_id}.i{instruction_index}",
                "block": block_id,
                "instruction_index": instruction_index,
                "semantic_op": semantic_op,
                "access_kind": f"typed_object_exact_slot_{semantic_op.lower()}",
                "preferred_route": selected_route,
                "selected_route": selected_route,
                "fallback_route": "compat_field_get_i64",
                "fallback_policy": "fail_fast",
                "proof_ids": [
                    "typed_object_plan",
                    "field_decl_authority",
                    "receiver_exact_type_id",
                    "slot_in_bounds",
                    "storage_exact_slot",
                    "non_weak_field",
                    "materialization_boundary_known",
                    "exact_slot_bridge_available",
                ],
                "miss_reason": None,
                "source_plan_kind": "TypedObjectExactSlotRoute",
                "selected_i64_const": None,
                "selected_bool_const": None,
                "selected_lowering_form": "native_direct",
                "selected_bridge_symbol": None,
                "selected_slot": selected_slot,
                "selected_storage": selected_storage,
                "receiver_box_name": receiver_box_name,
                "field_id": field_id,
                "selected_backend": "typed_object_exact_slot_nativedirect",
            }
        ]

    def test_newbox_uses_exact_typed_object_helper_for_exact_storage_plan(self):
        mod, builder, _bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.typed_object_plans = self._exact_page_plan()
        vmap = {}

        lower_newbox(builder, mod, "Page", [], 1, vmap, resolver)

        ir_txt = str(mod)
        self.assertIn('call i64 @"nyash.object.new_typed_hi"', ir_txt, msg=ir_txt)
        self.assertNotIn('@"nyash.env.box.new_i64x"', ir_txt, msg=ir_txt)
        self.assertEqual(resolver.value_types[1], make_box_handle_fact("Page"))
        self.assertIn(1, vmap)

    def test_field_get_uses_exact_unsigned_slot_helper_for_usize_plan(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("Page")
        resolver.typed_object_plans = self._exact_page_plan()
        vmap = {1: ir.Constant(i64, 101)}

        lower_field_get(
            builder,
            mod,
            1,
            "capacity",
            2,
            "usize",
            [],
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn('call i64 @"hako.object.exact_slot_get_u64_hii"', ir_txt, msg=ir_txt)
        self.assertIn("i64 0", ir_txt, msg=ir_txt)
        self.assertNotIn("nyash.instance.get_i64_field_h", ir_txt, msg=ir_txt)
        self.assertEqual(resolver.value_types[2], "i64")

    def test_field_get_uses_exact_signed_slot_helper_for_i64_field_in_exact_plan(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("Page")
        resolver.typed_object_plans = self._exact_page_plan()
        vmap = {1: ir.Constant(i64, 101)}

        lower_field_get(
            builder,
            mod,
            1,
            "used",
            2,
            "i64",
            [],
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn('call i64 @"hako.object.exact_slot_get_i64_hii"', ir_txt, msg=ir_txt)
        self.assertNotIn("nyash.instance.get_i64_field_h", ir_txt, msg=ir_txt)
        self.assertEqual(resolver.value_types[2], "i64")

    def test_field_set_uses_exact_unsigned_slot_helper_and_traps_on_failed_status(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("Page")
        resolver.value_types[2] = "i64"
        resolver.typed_object_plans = self._exact_page_plan()
        vmap = {
            1: ir.Constant(i64, 101),
            2: ir.Constant(i64, 64),
        }

        lower_field_set(
            builder,
            mod,
            1,
            "capacity",
            2,
            "usize",
            [],
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn('call i64 @"hako.object.exact_slot_set_u64_hiu"', ir_txt, msg=ir_txt)
        self.assertIn("unreachable", ir_txt, msg=ir_txt)
        self.assertNotIn("nyash.instance.set_i64_field_h", ir_txt, msg=ir_txt)

    def test_field_set_uses_exact_signed_slot_helper_for_i64_field_in_exact_plan(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("Page")
        resolver.value_types[2] = "i64"
        resolver.typed_object_plans = self._exact_page_plan()
        vmap = {
            1: ir.Constant(i64, 101),
            2: ir.Constant(i64, 7),
        }

        lower_field_set(
            builder,
            mod,
            1,
            "used",
            2,
            "i64",
            [],
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn('call i64 @"hako.object.exact_slot_set_i64_hii"', ir_txt, msg=ir_txt)
        self.assertIn("unreachable", ir_txt, msg=ir_txt)
        self.assertNotIn("nyash.instance.set_i64_field_h", ir_txt, msg=ir_txt)

    def test_selected_exact_slot_route_decision_is_resolved_for_field_get(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("Page")
        resolver.typed_object_plans = self._exact_page_plan()
        vmap = {1: ir.Constant(i64, 101)}

        self._seed_typed_object_exact_slot_route_decision(
            resolver,
            semantic_op="FieldGet",
            selected_route="hako.typed_object.slot_load_u64",
            selected_bridge_symbol="hako.object.exact_slot_get_u64_hii",
            selected_slot=0,
            selected_storage="u64",
            field_id="capacity",
        )

        decision = _selected_typed_object_exact_slot_route_decision(
            resolver=resolver,
            box_vid=1,
            field_name="capacity",
            semantic_op="FieldGet",
        )

        self.assertIsNotNone(decision)
        self.assertEqual(
            decision["selected_route"],
            "hako.typed_object.slot_load_u64",
        )
        self.assertEqual(
            decision["selected_bridge_symbol"],
            "hako.object.exact_slot_get_u64_hii",
        )

        lower_field_get(
            builder,
            mod,
            1,
            "capacity",
            2,
            "usize",
            [],
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn('call i64 @"hako.object.exact_slot_get_u64_hii"', ir_txt, msg=ir_txt)
        self.assertNotIn("nyash.object.field_get_u64_hii", ir_txt, msg=ir_txt)
        self.assertEqual(resolver.value_types[2], "i64")

    def test_native_direct_exact_slot_route_decision_loads_payload(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("Page")
        resolver.typed_object_plans = self._exact_page_plan()
        vmap = {1: ir.Constant(i64, 101)}

        self._seed_typed_object_exact_slot_native_direct_route_decision(
            resolver,
            semantic_op="FieldGet",
            selected_route="hako.typed_object.slot_load_i64",
            selected_slot=1,
            selected_storage="i64",
            field_id="used",
        )

        lower_field_get(
            builder,
            mod,
            1,
            "used",
            2,
            "i64",
            [],
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn("typed_object_native_direct_payload_load", ir_txt, msg=ir_txt)
        self.assertNotIn("hako.object.exact_slot_get_i64_hii", ir_txt, msg=ir_txt)
        self.assertNotIn("nyash.instance.get_i64_field_h", ir_txt, msg=ir_txt)
        self.assertEqual(resolver.value_types[2], "i64")

    def test_native_direct_exact_slot_route_decision_stores_payload(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("Page")
        resolver.value_types[2] = "i64"
        resolver.typed_object_plans = self._exact_page_plan()
        vmap = {
            1: ir.Constant(i64, 101),
            2: ir.Constant(i64, 7),
        }

        self._seed_typed_object_exact_slot_native_direct_route_decision(
            resolver,
            semantic_op="FieldSet",
            selected_route="hako.typed_object.slot_store_i64",
            selected_slot=1,
            selected_storage="i64",
            field_id="used",
        )

        lower_field_set(
            builder,
            mod,
            1,
            "used",
            2,
            "i64",
            [],
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn("direct_slot_payload_ptr", ir_txt, msg=ir_txt)
        self.assertIn("store i64 7", ir_txt, msg=ir_txt)
        self.assertNotIn("hako.object.exact_slot_set_i64_hii", ir_txt, msg=ir_txt)
        self.assertNotIn("nyash.instance.set_i64_field_h", ir_txt, msg=ir_txt)
        self.assertNotIn("unreachable", ir_txt, msg=ir_txt)

    def test_native_direct_exact_slot_route_decision_helper_selects_native_direct(self):
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("Page")
        resolver.typed_object_plans = self._exact_page_plan()

        self._seed_typed_object_exact_slot_native_direct_route_decision(
            resolver,
            semantic_op="FieldGet",
            selected_route="hako.typed_object.slot_load_i64",
            selected_slot=1,
            selected_storage="i64",
            field_id="used",
        )

        decision = _selected_typed_object_exact_slot_native_direct_route_decision(
            resolver=resolver,
            box_vid=1,
            field_name="used",
            semantic_op="FieldGet",
        )

        self.assertIsNotNone(decision)
        self.assertEqual(decision["selected_lowering_form"], "native_direct")
        self.assertEqual(decision["selected_route"], "hako.typed_object.slot_load_i64")
        self.assertEqual(decision["selected_backend"], "typed_object_exact_slot_nativedirect")

    def test_direct_slot_nativedirect_selected_method_get_loads_payload(self):
        def run():
            mod, builder, bb, i64 = self._make_builder(
                name="HakoAllocPageModel.acquire_usize/1"
            )
            resolver = _ResolverStub()
            resolver.value_types[1] = make_box_handle_fact("HakoAllocPageModel")
            resolver.typed_object_plans = self._exact_page_model_plan()
            vmap = {1: ir.Constant(i64, 101)}

            lower_field_get(
                builder,
                mod,
                1,
                "free_top",
                2,
                "usize",
                [],
                vmap,
                resolver,
                preds={},
                block_end_values={},
                bb_map={1: bb},
            )

            ir_txt = str(mod)
            self.assertIn("direct_slot_object_base", ir_txt, msg=ir_txt)
            self.assertIn("direct_slot_payload_addr", ir_txt, msg=ir_txt)
            self.assertIn("direct_slot_payload_ptr", ir_txt, msg=ir_txt)
            self.assertIn("load i64", ir_txt, msg=ir_txt)
            self.assertNotIn("nyash.object.field_get", ir_txt, msg=ir_txt)
            self.assertNotIn("nyash.object.exact_slot_get", ir_txt, msg=ir_txt)
            self.assertEqual(resolver.value_types[2], "i64")

        self._with_env("HAKO_TYPED_OBJECT_STORE", "direct_slot_exact", run)

    def test_direct_slot_nativedirect_selected_method_set_stores_payload(self):
        def run():
            mod, builder, bb, i64 = self._make_builder(
                name="HakoAllocPageModel.acquire_usize/1"
            )
            resolver = _ResolverStub()
            resolver.value_types[1] = make_box_handle_fact("HakoAllocPageModel")
            resolver.value_types[2] = "i64"
            resolver.typed_object_plans = self._exact_page_model_plan()
            vmap = {
                1: ir.Constant(i64, 101),
                2: ir.Constant(i64, 64),
            }

            lower_field_set(
                builder,
                mod,
                1,
                "free_top",
                2,
                "usize",
                [],
                vmap,
                resolver,
                preds={},
                block_end_values={},
                bb_map={1: bb},
            )

            ir_txt = str(mod)
            self.assertIn("direct_slot_object_base", ir_txt, msg=ir_txt)
            self.assertIn("direct_slot_payload_addr", ir_txt, msg=ir_txt)
            self.assertIn("store i64 64", ir_txt, msg=ir_txt)
            self.assertNotIn("nyash.object.field_set", ir_txt, msg=ir_txt)
            self.assertNotIn("nyash.object.exact_slot_set", ir_txt, msg=ir_txt)
            self.assertNotIn("unreachable", ir_txt, msg=ir_txt)

        self._with_env("HAKO_TYPED_OBJECT_STORE", "direct_slot_exact", run)

    def test_direct_slot_nativedirect_keeps_non_selected_method_on_helper_path(self):
        def run():
            mod, builder, bb, i64 = self._make_builder(name="main")
            resolver = _ResolverStub()
            resolver.value_types[1] = make_box_handle_fact("HakoAllocPageModel")
            resolver.typed_object_plans = self._exact_page_model_plan()
            vmap = {1: ir.Constant(i64, 101)}

            lower_field_get(
                builder,
                mod,
                1,
                "free_top",
                2,
                "usize",
                [],
                vmap,
                resolver,
                preds={},
                block_end_values={},
                bb_map={1: bb},
            )

            ir_txt = str(mod)
            self.assertIn('call i64 @"hako.object.exact_slot_get_u64_hii"', ir_txt, msg=ir_txt)
            self.assertNotIn("direct_slot_payload_ptr", ir_txt, msg=ir_txt)

        self._with_env("HAKO_TYPED_OBJECT_STORE", "direct_slot_exact", run)

    def test_ny_main_registers_exact_typed_object_layout_before_main_call(self):
        mod = ir.Module(name="typed_object_exact_entry")
        i64 = ir.IntType(64)
        ir.Function(mod, ir.FunctionType(i64, []), name="main")

        class Builder:
            pass

        builder = Builder()
        builder.module = mod
        builder.i64 = i64
        builder.i32 = ir.IntType(32)
        builder.i8 = ir.IntType(8)
        builder.i8p = builder.i8.as_pointer()
        builder.user_box_decls = []
        builder.typed_object_plans = self._exact_page_plan()

        ensure_ny_main(builder)

        ir_txt = str(mod)
        self.assertIn('call i64 @"nyash.object.register_typed_layout_hi"', ir_txt, msg=ir_txt)
        self.assertIn('call i64 @"nyash.object.register_typed_layout_slot_iii"', ir_txt, msg=ir_txt)
        self.assertIn("i64 4", ir_txt, msg=ir_txt)
