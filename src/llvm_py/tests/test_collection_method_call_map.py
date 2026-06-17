#!/usr/bin/env python3
import unittest
from functools import partial

import llvmlite.ir as ir

from src.llvm_py.instructions.llvm_decl import declare_function
from src.llvm_py.instructions.mir_call.collection_method_call import (
    lower_collection_method_call,
)
from src.llvm_py.tests.collection_method_call_fixtures import (
    _DummyResolver,
    _new_builder,
    _new_builder_named,
    _seed_local_fastpath_known_receiver_direct_call_fact,
    _seed_local_i64_map_shadow_get_plan,
    _seed_local_i64_map_storage_realization_plan,
    _seed_map_lookup_fusion_route,
    _seed_map_lookup_route_decision,
    _seed_map_missing_empty_route_decision,
    _seed_non_map_route_decision,
)


class TestCollectionMethodCallMap(unittest.TestCase):
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

    def test_mapbox_local_i64_shadow_get_falls_back_after_consumer_retire(self):
        i64, module, builder = _new_builder_named("MapLocalI64.get/0")
        resolver = _DummyResolver(value_types={2: "i64"}, integerish_ids={2})
        _seed_local_i64_map_shadow_get_plan(
            resolver,
            receiver_value=1,
            key_value=2,
            instruction_index=0,
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
        self.assertNotIn("local_i64_map_get_hi", ir_text)

    def test_mapbox_local_fastpath_fact_get_uses_known_receiver_direct_call_helper(self):
        i64, module, builder = _new_builder_named("MapLocalFastPath.get/0")
        resolver = _DummyResolver(value_types={2: "i64"}, integerish_ids={2})
        _seed_local_fastpath_known_receiver_direct_call_fact(
            resolver,
            receiver_value=1,
            key_value=2,
            instruction_index=0,
        )
        _seed_local_i64_map_storage_realization_plan(resolver, receiver_value=1)

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
        self.assertIn("nyash.map.local_i64_get_hi", ir_text)
        self.assertIn("local_fastpath_map_get_hi", ir_text)
        self.assertNotIn("nyash.map.slot_load_hh", ir_text)

    def test_mapbox_local_fastpath_fact_get_requires_storage_plan(self):
        i64, module, builder = _new_builder_named("MapLocalFastPathNoStorage.get/0")
        resolver = _DummyResolver(value_types={2: "i64"}, integerish_ids={2})
        _seed_local_fastpath_known_receiver_direct_call_fact(
            resolver,
            receiver_value=1,
            key_value=2,
            instruction_index=0,
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
        self.assertNotIn("local_fastpath_map_get_hi", ir_text)

    def test_mapbox_local_fastpath_fact_get_ignores_fallback_reason(self):
        i64, module, builder = _new_builder_named("MapLocalFastPathFallback.get/0")
        resolver = _DummyResolver(value_types={2: "i64"}, integerish_ids={2})
        _seed_local_fastpath_known_receiver_direct_call_fact(
            resolver,
            receiver_value=1,
            key_value=2,
            instruction_index=0,
            fallback_reason="MaybePublishedBeforeSite",
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
        self.assertNotIn("nyash.map.local_i64_get_hi", ir_text)

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

    def test_mapbox_missing_empty_route_decision_folds_to_zero(self):
        i64, module, builder = _new_builder_named("MapMissing.get/0")
        resolver = _DummyResolver(value_types={2: "i64"}, integerish_ids={2})
        _seed_map_missing_empty_route_decision(resolver, selected_i64_const=0)

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
        self.assertIn("ret i64 0", ir_text)
        self.assertNotIn("nyash.map.slot_load_hh", ir_text)

    def test_runtime_data_map_missing_empty_route_decision_folds_before_dispatch(self):
        i64, module, builder = _new_builder_named("MapMissing.runtimeDataGet/0")
        resolver = _DummyResolver(value_types={2: "i64"}, integerish_ids={2})
        _seed_map_missing_empty_route_decision(resolver, selected_i64_const=0)

        result = lower_collection_method_call(
            builder=builder,
            declare=partial(declare_function, module),
            box_name="RuntimeDataBox",
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
        self.assertIn("ret i64 0", ir_text)
        self.assertNotIn("nyash.runtime_data.get_hh", ir_text)
        self.assertNotIn("nyash.map.slot_load_hh", ir_text)

    def test_mapbox_missing_empty_ignores_non_map_source_plan_kind(self):
        i64, module, builder = _new_builder_named("MapMissing.get/0")
        resolver = _DummyResolver(value_types={2: "i64"}, integerish_ids={2})
        _seed_non_map_route_decision(
            resolver, selected_route="map_get_missing_empty_const_zero"
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
        self.assertNotIn("ret i64 0", ir_text)

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
