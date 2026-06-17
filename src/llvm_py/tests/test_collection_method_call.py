#!/usr/bin/env python3
import unittest
import os
from functools import partial

import llvmlite.ir as ir

from src.llvm_py.instructions.llvm_decl import declare_function
from src.llvm_py.instructions.mir_call.collection_method_call import (
    _current_local_i64_map_direct_storage_shadow_candidate,
    _current_local_i64_map_entry_value_tracking_shadow_candidate,
    lower_collection_method_call,
)
from src.llvm_py.tests.collection_method_call_fixtures import (
    _DummyResolver,
    _new_builder,
    _new_builder_named,
    _seed_direct_array_plan,
    _seed_local_fastpath_known_receiver_direct_call_fact,
    _seed_local_i64_map_direct_storage_plan,
    _seed_local_i64_map_entry_value_tracking_plan,
    _seed_route_decision,
)


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

    def test_local_i64_map_direct_storage_shadow_requires_fact_and_plan(self):
        resolver = _DummyResolver()
        _seed_local_fastpath_known_receiver_direct_call_fact(resolver)
        _seed_local_i64_map_direct_storage_plan(resolver)

        candidate = _current_local_i64_map_direct_storage_shadow_candidate(
            resolver=resolver,
            box_name="MapBox",
            method_name="get",
            receiver_vid=1,
            arg_ids=[2],
        )

        self.assertIsNotNone(candidate)
        self.assertEqual(candidate["fact"]["route_plan"], "map_repr.generic_hash_runtime")
        self.assertEqual(
            candidate["plan"]["representation"], "closed_world_i64_key_value_table"
        )

    def test_local_i64_map_direct_storage_shadow_rejects_plan_without_fact(self):
        resolver = _DummyResolver()
        _seed_local_i64_map_direct_storage_plan(resolver)

        candidate = _current_local_i64_map_direct_storage_shadow_candidate(
            resolver=resolver,
            box_name="MapBox",
            method_name="get",
            receiver_vid=1,
            arg_ids=[2],
        )

        self.assertIsNone(candidate)

    def test_local_i64_map_entry_value_tracking_shadow_requires_entry_rows(self):
        resolver = _DummyResolver()
        _seed_local_fastpath_known_receiver_direct_call_fact(resolver)
        _seed_local_i64_map_direct_storage_plan(resolver)
        _seed_local_i64_map_entry_value_tracking_plan(resolver)

        candidate = _current_local_i64_map_entry_value_tracking_shadow_candidate(
            resolver=resolver,
            box_name="MapBox",
            method_name="get",
            receiver_vid=1,
            arg_ids=[2],
        )

        self.assertIsNotNone(candidate)
        self.assertEqual(candidate["fact"]["route_plan"], "map_repr.generic_hash_runtime")
        self.assertEqual(
            candidate["plan"]["representation"], "closed_world_i64_key_value_table"
        )
        self.assertEqual(len(candidate["entry_value_tracking"]), 1)
        self.assertEqual(candidate["entry_value_tracking"][0]["value_const_if_known"], 1)

    def test_local_i64_map_entry_value_tracking_shadow_rejects_missing_rows(self):
        resolver = _DummyResolver()
        _seed_local_fastpath_known_receiver_direct_call_fact(resolver)
        _seed_local_i64_map_direct_storage_plan(resolver)

        candidate = _current_local_i64_map_entry_value_tracking_shadow_candidate(
            resolver=resolver,
            box_name="MapBox",
            method_name="get",
            receiver_vid=1,
            arg_ids=[2],
        )

        self.assertIsNone(candidate)

    def test_local_i64_map_entry_table_dispatch_uses_const_tracking_rows(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver()
        _seed_local_fastpath_known_receiver_direct_call_fact(resolver)
        _seed_local_i64_map_direct_storage_plan(resolver)
        _seed_local_i64_map_entry_value_tracking_plan(
            resolver,
            key_const_if_known=0,
            value_const_if_known=1,
        )
        _seed_local_i64_map_entry_value_tracking_plan(
            resolver,
            set_instruction_index=5,
            key_value=4,
            value_value=5,
            key_const_if_known=7,
            value_const_if_known=9,
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
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertIn("local_i64_map_entry_dispatch_result", ir_text)
        self.assertIn("local_i64_map_entry_dispatch_hit0", ir_text)
        self.assertIn("local_i64_map_entry_dispatch_hit1", ir_text)
        self.assertIn("nyash.map.slot_load_hh", ir_text)
        self.assertNotIn("nyash.map.local_i64_get_hi", ir_text)

    def test_local_i64_map_entry_table_dispatch_rejects_non_const_value(self):
        i64, module, builder = _new_builder()
        resolver = _DummyResolver()
        _seed_local_fastpath_known_receiver_direct_call_fact(resolver)
        _seed_local_i64_map_direct_storage_plan(resolver)
        _seed_local_i64_map_entry_value_tracking_plan(
            resolver,
            value_const_if_known=None,
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
        )
        builder.ret(result)

        ir_text = str(module)
        self.assertNotIn("local_i64_map_entry_dispatch_result", ir_text)
        self.assertIn("nyash.map.slot_load_hh", ir_text)

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

if __name__ == "__main__":
    unittest.main()
