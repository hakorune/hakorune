#!/usr/bin/env python3
import unittest
import os
from functools import partial

import llvmlite.ir as ir

from src.llvm_py.instructions.llvm_decl import declare_function
from src.llvm_py.instructions.mir_call.collection_method_call import (
    lower_collection_method_call,
)


class _DummyResolver:
    def __init__(self, value_types=None, integerish_ids=None):
        self.value_types = value_types or {}
        self.integerish_ids = set(integerish_ids or [])
        self.direct_array_access_plans_by_site = {}
        self.map_lookup_fusion_routes_by_site = {}
        self.route_decisions_by_site = {}
        self.route_decisions_metadata_present = False
        self.current_block_id = 0
        self.current_instruction_index = 0


def _new_builder():
    return _new_builder_named("main")


def _new_builder_named(name):
    i64 = ir.IntType(64)
    module = ir.Module(name="test_collection_method_call")
    fn = ir.Function(module, ir.FunctionType(i64, []), name=name)
    bb = fn.append_basic_block("entry")
    builder = ir.IRBuilder(bb)
    return i64, module, builder


def _seed_direct_array_plan(
    resolver,
    *,
    op,
    receiver_value=1,
    index_value=2,
    value_value=None,
    result_value=9,
    bounds_policy="checked",
    cfg_shape="checked_branching",
    fallback_policy="allow_checked",
):
    plan = {
        "op": op,
        "receiver_value": receiver_value,
        "index_value": index_value,
        "value_value": value_value,
        "result_value": result_value,
        "array_kind": "DirectArrayI64",
        "element_type": "i64",
        "route": f"direct_array_i64_{'load' if op == 'load' else 'store'}",
        "bounds_policy": bounds_policy,
        "proof_kind": "range_index" if bounds_policy == "proved_unchecked" else "exact_front_contract",
        "proof_ids": ["range_index"] if bounds_policy == "proved_unchecked" else ["exact_front_contract"],
        "fallback_policy": fallback_policy,
        "cfg_shape": cfg_shape,
        "store_semantics": "append_or_overwrite" if op == "store" else "not_store",
    }
    resolver.direct_array_access_plans_by_site[(resolver.current_block_id, resolver.current_instruction_index)] = [plan]


def _seed_route_decision(resolver, *, selected_route):
    resolver.route_decisions_by_site[
        (resolver.current_block_id, resolver.current_instruction_index)
    ] = [
        {
            "selected_route": selected_route,
            "fallback_policy": "opportunistic",
            "source_plan_kind": "DirectArrayAccessPlan",
        }
    ]


def _seed_map_lookup_fusion_route(
    resolver,
    *,
    receiver_value=1,
    key_value=2,
    key_const=-1,
    stored_value_const=None,
    stored_value_proof="unknown_scalar",
    get_instruction_index=3,
    has_instruction_index=4,
):
    route = {
        "route_id": "map_lookup.same_key",
        "block": int(resolver.current_block_id),
        "get_instruction_index": int(get_instruction_index),
        "has_instruction_index": int(has_instruction_index),
        "fusion_op": "MapLookupSameKey",
        "receiver_origin_box": "MapBox",
        "receiver_value": int(receiver_value),
        "key_value": int(key_value),
        "key_const": int(key_const),
        "get_result_value": 30,
        "has_result_value": 31,
        "get_return_shape": "scalar_i64_or_missing_zero",
        "get_value_demand": "scalar_i64",
        "get_publication_policy": "no_publication",
        "has_result_shape": "presence_bool",
        "stored_value_proof": stored_value_proof,
        "stored_value_const": stored_value_const,
        "stored_value_known_nonzero": None
        if stored_value_const is None
        else bool(int(stored_value_const) != 0),
        "proof": "same_receiver_same_i64_key_scalar_get_has",
        "lowering_tier": "cold_fallback",
    }
    resolver.map_lookup_fusion_routes_by_site.setdefault(
        (int(resolver.current_block_id), int(get_instruction_index)),
        [],
    ).append(route)
    resolver.map_lookup_fusion_routes_by_site.setdefault(
        (int(resolver.current_block_id), int(has_instruction_index)),
        [],
    ).append(route)


def _seed_map_lookup_route_decision(
    resolver,
    *,
    selected_route="map_lookup_const_fold",
    selected_i64_const=None,
    selected_bool_const=None,
):
    resolver.route_decisions_by_site[
        (resolver.current_block_id, resolver.current_instruction_index)
    ] = [
        {
            "selected_route": selected_route,
            "fallback_policy": "opportunistic",
            "source_plan_kind": "MapLookupFusionRoute",
            "semantic_op": "MapHas"
            if int(resolver.current_instruction_index) == 1
            else "MapGet",
            "selected_i64_const": selected_i64_const,
            "selected_bool_const": selected_bool_const,
        }
    ]


def _seed_non_map_route_decision(resolver, *, selected_route="map_lookup_const_fold"):
    resolver.route_decisions_by_site[
        (resolver.current_block_id, resolver.current_instruction_index)
    ] = [
        {
            "selected_route": selected_route,
            "fallback_policy": "opportunistic",
            "source_plan_kind": "DirectArrayAccessPlan",
            "semantic_op": "MapGet",
            "selected_i64_const": 7,
            "selected_bool_const": None,
        }
    ]


class TestCollectionMethodCall(unittest.TestCase):
    def _with_array_backend(self, value, fn):
        old = os.environ.get("HAKO_ARRAY_SLOT_STORE")
        if value is None:
            os.environ.pop("HAKO_ARRAY_SLOT_STORE", None)
        else:
            os.environ["HAKO_ARRAY_SLOT_STORE"] = value
        try:
            fn()
        finally:
            if old is None:
                os.environ.pop("HAKO_ARRAY_SLOT_STORE", None)
            else:
                os.environ["HAKO_ARRAY_SLOT_STORE"] = old

    def test_non_runtime_data_get_falls_back_to_map_raw_kernel(self):
        i64, module, builder = _new_builder()

        result = lower_collection_method_call(
            builder=builder,
            declare=partial(declare_function, module),
            box_name="MapBox",
            method_name="get",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
        )
        builder.ret(result)

        self.assertIn("nyash.map.slot_load_hh", str(module))

    def test_runtime_data_push_uses_runtime_data_dispatch(self):
        i64, module, builder = _new_builder()

        result = lower_collection_method_call(
            builder=builder,
            declare=partial(declare_function, module),
            box_name="RuntimeDataBox",
            method_name="push",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            prefer_array_mono_route=False,
        )
        builder.ret(result)

        self.assertIn("nyash.runtime_data.push_hh", str(module))

    def test_arraybox_get_with_i64_key_uses_array_slot_load_hi(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver(value_types={2: "i64"}, integerish_ids={2})

        result = lower_collection_method_call(
            builder=builder,
            declare=partial(declare_function, module),
            box_name="ArrayBox",
            method_name="get",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            resolver=resolver,
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("nyash.array.slot_load_hi", ir_text)
        self.assertNotIn("nyash.map.slot_load_hh", ir_text)
        self.assertNotIn("nyash.runtime_data.get_hh", ir_text)

    def test_arraybox_get_with_non_i64_key_keeps_runtime_data_facade(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver(value_types={2: {"kind": "handle", "box_type": "StringBox"}})

        result = lower_collection_method_call(
            builder=builder,
            declare=partial(declare_function, module),
            box_name="ArrayBox",
            method_name="get",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            resolver=resolver,
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("nyash.runtime_data.get_hh", ir_text)
        self.assertNotIn("nyash.map.slot_load_hh", ir_text)

    def test_arraybox_set_with_i64_key_and_value_uses_array_slot_store_hii(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver(value_types={2: "i64", 3: "i64"}, integerish_ids={2, 3})

        result = lower_collection_method_call(
            builder=builder,
            declare=partial(declare_function, module),
            box_name="ArrayBox",
            method_name="set",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2, 3],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            resolver=resolver,
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("nyash.array.slot_store_hii", ir_text)
        self.assertNotIn("nyash.map.slot_store_hhh", ir_text)
        self.assertNotIn("nyash.runtime_data.set_hhh", ir_text)

    def test_direct_array_selected_method_get_lowers_to_direct_load(self):
        def run():
            i64, module, builder = _new_builder_named("SomeUserMethod/0")
            resolver = _DummyResolver(value_types={2: "i64"}, integerish_ids={2})
            resolver.arrayrepr_facts = {1: "ArrayRepr::DirectI64"}
            _seed_direct_array_plan(resolver, op="load")
            _seed_route_decision(resolver, selected_route="direct_array_i64_load")

            result = lower_collection_method_call(
                builder=builder,
                declare=partial(declare_function, module),
                box_name="ArrayBox",
                method_name="get",
                recv_h=ir.Constant(i64, 0x1003),
                arg_ids=[2],
                resolve_arg=lambda vid: ir.Constant(i64, vid),
                resolver=resolver,
                receiver_vid=1,
                dst_vid=9,
            )
            builder.ret(result)

            ir_text = str(module)
            self.assertIn("direct_array_i64_base", ir_text)
            self.assertIn("direct_array_i64_get_ptr", ir_text)
            self.assertIn("direct_array_i64_get_result", ir_text)
            self.assertNotIn("nyash.array.slot_load_hi", ir_text)
            self.assertNotIn("nyash.runtime_data.get_hh", ir_text)

        self._with_array_backend("direct_array_i64_exact", run)

    def test_direct_array_plan_with_env_off_keeps_helper_path(self):
        def run():
            i64, module, builder = _new_builder_named("SomeUserMethod/0")
            resolver = _DummyResolver(value_types={2: "i64"}, integerish_ids={2})
            resolver.arrayrepr_facts = {1: "ArrayRepr::DirectI64"}
            _seed_direct_array_plan(resolver, op="load")
            _seed_route_decision(resolver, selected_route="direct_array_i64_load")

            result = lower_collection_method_call(
                builder=builder,
                declare=partial(declare_function, module),
                box_name="ArrayBox",
                method_name="get",
                recv_h=ir.Constant(i64, 0x1003),
                arg_ids=[2],
                resolve_arg=lambda vid: ir.Constant(i64, vid),
                resolver=resolver,
                receiver_vid=1,
                dst_vid=9,
            )
            builder.ret(result)

            ir_text = str(module)
            self.assertIn("nyash.array.slot_load_hi", ir_text)
            self.assertNotIn("direct_array_i64_get_ptr", ir_text)

        self._with_array_backend(None, run)

    def test_direct_array_selected_method_set_lowers_to_direct_store(self):
        def run():
            i64, module, builder = _new_builder_named("SomeUserMethod/0")
            resolver = _DummyResolver(value_types={2: "i64", 3: "i64"}, integerish_ids={2, 3})
            resolver.arrayrepr_facts = {1: "ArrayRepr::DirectI64"}
            _seed_direct_array_plan(resolver, op="store", value_value=3)
            _seed_route_decision(resolver, selected_route="direct_array_i64_store")

            result = lower_collection_method_call(
                builder=builder,
                declare=partial(declare_function, module),
                box_name="ArrayBox",
                method_name="set",
                recv_h=ir.Constant(i64, 0x1003),
                arg_ids=[2, 3],
                resolve_arg=lambda vid: ir.Constant(i64, vid),
                resolver=resolver,
                receiver_vid=1,
                dst_vid=9,
            )
            builder.ret(result)

            ir_text = str(module)
            self.assertIn("direct_array_i64_base", ir_text)
            self.assertIn("direct_array_i64_set_ptr", ir_text)
            self.assertIn("direct_array_i64_next_len", ir_text)
            self.assertIn("direct_array_i64_set_result", ir_text)
            self.assertNotIn("nyash.array.slot_store_hii", ir_text)
            self.assertNotIn("nyash.runtime_data.set_hhh", ir_text)

        self._with_array_backend("direct_array_i64_exact", run)

    def test_direct_array_proved_unchecked_set_lowers_to_branchless_direct_store(self):
        def run():
            i64, module, builder = _new_builder_named("SomeUserMethod/0")
            resolver = _DummyResolver(value_types={2: "i64", 3: "i64"}, integerish_ids={2, 3})
            resolver.arrayrepr_facts = {1: "ArrayRepr::DirectI64"}
            _seed_direct_array_plan(
                resolver,
                op="store",
                value_value=3,
                bounds_policy="proved_unchecked",
                cfg_shape="branchless",
                fallback_policy="fail_fast",
            )
            _seed_route_decision(resolver, selected_route="direct_array_i64_store")

            result = lower_collection_method_call(
                builder=builder,
                declare=partial(declare_function, module),
                box_name="ArrayBox",
                method_name="set",
                recv_h=ir.Constant(i64, 0x1003),
                arg_ids=[2, 3],
                resolve_arg=lambda vid: ir.Constant(i64, vid),
                resolver=resolver,
                receiver_vid=1,
                dst_vid=9,
            )
            builder.ret(result)

            ir_text = str(module)
            self.assertIn("direct_array_i64_set_unchecked_ptr", ir_text)
            self.assertIn("direct_array_i64_set_unchecked_next_len", ir_text)
            self.assertNotIn("direct_array_i64_set_can_store", ir_text)
            self.assertNotIn("nyash.array.slot_store_hii", ir_text)

        self._with_array_backend("direct_array_i64_exact", run)

    def test_direct_array_route_decision_mismatch_keeps_helper_path(self):
        def run():
            i64, module, builder = _new_builder_named("SomeUserMethod/0")
            resolver = _DummyResolver(value_types={2: "i64"}, integerish_ids={2})
            resolver.arrayrepr_facts = {1: "ArrayRepr::DirectI64"}
            _seed_direct_array_plan(resolver, op="load")
            _seed_route_decision(resolver, selected_route="generic_array_get_helper")

            result = lower_collection_method_call(
                builder=builder,
                declare=partial(declare_function, module),
                box_name="ArrayBox",
                method_name="get",
                recv_h=ir.Constant(i64, 0x1003),
                arg_ids=[2],
                resolve_arg=lambda vid: ir.Constant(i64, vid),
                resolver=resolver,
                receiver_vid=1,
                dst_vid=9,
            )
            builder.ret(result)

            ir_text = str(module)
            self.assertIn("nyash.array.slot_load_hi", ir_text)
            self.assertNotIn("direct_array_i64_get_ptr", ir_text)

        self._with_array_backend("direct_array_i64_exact", run)

    def test_direct_array_plan_without_decision_in_modern_metadata_keeps_helper_path(self):
        def run():
            i64, module, builder = _new_builder_named("SomeUserMethod/0")
            resolver = _DummyResolver(value_types={2: "i64"}, integerish_ids={2})
            resolver.arrayrepr_facts = {1: "ArrayRepr::DirectI64"}
            resolver.route_decisions_metadata_present = True
            _seed_direct_array_plan(resolver, op="load")

            result = lower_collection_method_call(
                builder=builder,
                declare=partial(declare_function, module),
                box_name="ArrayBox",
                method_name="get",
                recv_h=ir.Constant(i64, 0x1003),
                arg_ids=[2],
                resolve_arg=lambda vid: ir.Constant(i64, vid),
                resolver=resolver,
                receiver_vid=1,
                dst_vid=9,
            )
            builder.ret(result)

            ir_text = str(module)
            self.assertIn("nyash.array.slot_load_hi", ir_text)
            self.assertNotIn("direct_array_i64_get_ptr", ir_text)

        self._with_array_backend("direct_array_i64_exact", run)

    def test_direct_array_non_origin_receiver_keeps_helper_path(self):
        def run():
            i64, module, builder = _new_builder_named("HakoAllocPageModel.acquireFreshSmall/1")
            resolver = _DummyResolver(value_types={2: "i64"}, integerish_ids={2})
            resolver.direct_array_i64_ids = {99}
            _seed_direct_array_plan(resolver, op="load")

            result = lower_collection_method_call(
                builder=builder,
                declare=partial(declare_function, module),
                box_name="ArrayBox",
                method_name="get",
                recv_h=ir.Constant(i64, 0x1003),
                arg_ids=[2],
                resolve_arg=lambda vid: ir.Constant(i64, vid),
                resolver=resolver,
                receiver_vid=1,
                dst_vid=9,
            )
            builder.ret(result)

            ir_text = str(module)
            self.assertIn("nyash.array.slot_load_hi", ir_text)
            self.assertNotIn("direct_array_i64_get_ptr", ir_text)

        self._with_array_backend("direct_array_i64_exact", run)

    def test_direct_array_method_name_without_plan_keeps_helper_path(self):
        def run():
            i64, module, builder = _new_builder_named("HakoAllocPageModel.acquireFreshSmall/1")
            resolver = _DummyResolver(value_types={2: "i64"}, integerish_ids={2})
            resolver.arrayrepr_facts = {1: "ArrayRepr::DirectI64"}

            result = lower_collection_method_call(
                builder=builder,
                declare=partial(declare_function, module),
                box_name="ArrayBox",
                method_name="get",
                recv_h=ir.Constant(i64, 0x1003),
                arg_ids=[2],
                resolve_arg=lambda vid: ir.Constant(i64, vid),
                resolver=resolver,
                receiver_vid=1,
                dst_vid=9,
            )
            builder.ret(result)

            ir_text = str(module)
            self.assertIn("nyash.array.slot_load_hi", ir_text)
            self.assertNotIn("direct_array_i64_get_ptr", ir_text)

        self._with_array_backend("direct_array_i64_exact", run)

    def test_direct_array_unsupported_method_keeps_runtime_data_facade(self):
        def run():
            i64, module, builder = _new_builder_named("SomeUserMethod/0")
            resolver = _DummyResolver(value_types={2: "i64"}, integerish_ids={2})
            resolver.arrayrepr_facts = {1: "ArrayRepr::DirectI64"}
            _seed_direct_array_plan(resolver, op="load")
            _seed_route_decision(resolver, selected_route="direct_array_i64_load")

            result = lower_collection_method_call(
                builder=builder,
                declare=partial(declare_function, module),
                box_name="ArrayBox",
                method_name="has",
                recv_h=ir.Constant(i64, 0x1003),
                arg_ids=[2],
                resolve_arg=lambda vid: ir.Constant(i64, vid),
                resolver=resolver,
                receiver_vid=1,
                dst_vid=9,
            )
            builder.ret(result)

            ir_text = str(module)
            self.assertIn("nyash.runtime_data.has_hh", ir_text)
            self.assertNotIn("direct_array_i64_get_ptr", ir_text)

        self._with_array_backend("direct_array_i64_exact", run)

    def test_arraybox_set_with_non_i64_key_keeps_runtime_data_facade(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver(
            value_types={
                2: {"kind": "handle", "box_type": "StringBox"},
                3: {"kind": "handle", "box_type": "StringBox"},
            }
        )

        result = lower_collection_method_call(
            builder=builder,
            declare=partial(declare_function, module),
            box_name="ArrayBox",
            method_name="set",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2, 3],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            resolver=resolver,
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("nyash.runtime_data.set_hhh", ir_text)
        self.assertNotIn("nyash.map.slot_store_hhh", ir_text)

    def test_arraybox_has_keeps_runtime_data_facade(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver(value_types={2: "i64"}, integerish_ids={2})

        result = lower_collection_method_call(
            builder=builder,
            declare=partial(declare_function, module),
            box_name="ArrayBox",
            method_name="has",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            resolver=resolver,
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("nyash.runtime_data.has_hh", ir_text)
        self.assertNotIn("nyash.map.probe_hh", ir_text)

    def test_mapbox_same_key_fusion_get_constant_folds_known_stored_value(self):
        i64, module, builder = _new_builder_named("MapFusion.get/0")
        resolver = _DummyResolver(value_types={2: "i64"}, integerish_ids={2})
        _seed_map_lookup_fusion_route(
            resolver,
            receiver_value=1,
            key_value=2,
            key_const=-1,
            stored_value_const=7,
            stored_value_proof="scalar_i64_const",
            get_instruction_index=0,
            has_instruction_index=1,
        )
        _seed_map_lookup_route_decision(resolver, selected_i64_const=7)

        result = lower_collection_method_call(
            builder=builder,
            declare=partial(declare_function, module),
            box_name="MapBox",
            method_name="get",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            resolver=resolver,
            receiver_vid=1,
            dst_vid=3,
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("ret i64 7", ir_text)
        self.assertNotIn("nyash.map.slot_load_hh", ir_text)

    def test_mapbox_const_fold_uses_route_decision_payload_without_fusion_metadata(self):
        i64, module, builder = _new_builder_named("MapFusion.get/0")
        resolver = _DummyResolver(value_types={2: "i64"}, integerish_ids={2})
        _seed_map_lookup_route_decision(resolver, selected_i64_const=7)

        result = lower_collection_method_call(
            builder=builder,
            declare=partial(declare_function, module),
            box_name="MapBox",
            method_name="get",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            resolver=resolver,
            receiver_vid=1,
            dst_vid=3,
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("ret i64 7", ir_text)
        self.assertNotIn("nyash.map.slot_load_hh", ir_text)

    def test_mapbox_same_key_fusion_get_without_route_decision_keeps_helper_path(self):
        i64, module, builder = _new_builder_named("MapFusion.get/0")
        resolver = _DummyResolver(value_types={2: "i64"}, integerish_ids={2})
        _seed_map_lookup_fusion_route(
            resolver,
            receiver_value=1,
            key_value=2,
            key_const=-1,
            stored_value_const=7,
            stored_value_proof="scalar_i64_const",
            get_instruction_index=0,
            has_instruction_index=1,
        )

        result = lower_collection_method_call(
            builder=builder,
            declare=partial(declare_function, module),
            box_name="MapBox",
            method_name="get",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            resolver=resolver,
            receiver_vid=1,
            dst_vid=3,
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("nyash.map.slot_load_hh", ir_text)
        self.assertNotIn("ret i64 7", ir_text)

    def test_mapbox_same_key_fusion_get_ignores_non_map_route_decision(self):
        i64, module, builder = _new_builder_named("MapFusion.get/0")
        resolver = _DummyResolver(value_types={2: "i64"}, integerish_ids={2})
        _seed_map_lookup_fusion_route(
            resolver,
            receiver_value=1,
            key_value=2,
            key_const=-1,
            stored_value_const=7,
            stored_value_proof="scalar_i64_const",
            get_instruction_index=0,
            has_instruction_index=1,
        )
        _seed_non_map_route_decision(resolver)

        result = lower_collection_method_call(
            builder=builder,
            declare=partial(declare_function, module),
            box_name="MapBox",
            method_name="get",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            resolver=resolver,
            receiver_vid=1,
            dst_vid=3,
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("nyash.map.slot_load_hh", ir_text)
        self.assertNotIn("ret i64 7", ir_text)

    def test_mapbox_same_key_fusion_has_constant_folds_known_presence(self):
        i64, module, builder = _new_builder_named("MapFusion.has/0")
        resolver = _DummyResolver(value_types={2: "i64"}, integerish_ids={2})
        _seed_map_lookup_fusion_route(
            resolver,
            receiver_value=1,
            key_value=2,
            key_const=-1,
            stored_value_const=7,
            stored_value_proof="scalar_i64_const",
            get_instruction_index=0,
            has_instruction_index=1,
        )
        resolver.current_instruction_index = 1
        _seed_map_lookup_route_decision(resolver, selected_bool_const=True)

        result = lower_collection_method_call(
            builder=builder,
            declare=partial(declare_function, module),
            box_name="MapBox",
            method_name="has",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
            resolver=resolver,
            receiver_vid=1,
            dst_vid=4,
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("ret i64 1", ir_text)
        self.assertNotIn("nyash.map.probe_hh", ir_text)

    def test_mapbox_clear_uses_clear_h(self):
        i64, module, builder = _new_builder()

        result = lower_collection_method_call(
            builder=builder,
            declare=partial(declare_function, module),
            box_name="MapBox",
            method_name="clear",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("nyash.map.clear_h", ir_text)
        self.assertNotIn("nyash.map.entry_count_i64", ir_text)

    def test_mapbox_delete_uses_delete_hh(self):
        i64, module, builder = _new_builder()

        result = lower_collection_method_call(
            builder=builder,
            declare=partial(declare_function, module),
            box_name="MapBox",
            method_name="delete",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("nyash.map.delete_hh", ir_text)
        self.assertNotIn("nyash.runtime_data.delete_hh", ir_text)

    def test_runtime_data_delete_stays_unrouted_on_facade(self):
        i64, module, builder = _new_builder()

        result = lower_collection_method_call(
            builder=builder,
            declare=partial(declare_function, module),
            box_name="RuntimeDataBox",
            method_name="delete",
            recv_h=ir.Constant(i64, 1),
            arg_ids=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
        )
        builder.ret(result if result is not None else ir.Constant(i64, 0))

        ir_text = str(module)
        self.assertIsNone(result)
        self.assertNotIn("nyash.runtime_data.delete_hh", ir_text)
        self.assertNotIn("nyash.map.delete_hh", ir_text)


if __name__ == "__main__":
    unittest.main()
