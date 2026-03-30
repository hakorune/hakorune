#!/usr/bin/env python3
import os
import unittest

import llvmlite.ir as ir

from src.llvm_py.instructions.mir_call.runtime_data_dispatch import (
    _prefer_array_mono_route_default,
    _reset_runtime_data_array_route_policy_cache_for_tests,
    lower_runtime_data_field_call,
    select_runtime_data_call_spec,
)


class _DummyResolver:
    def __init__(self, value_types=None, integerish_ids=None):
        self.value_types = value_types or {}
        self.integerish_ids = set(integerish_ids or [])

    def is_arrayish(self, value_id: int) -> bool:
        value = self.value_types.get(int(value_id))
        return isinstance(value, dict) and value.get("box_type") == "ArrayBox"


def _declare(module, name, ret, args):
    for f in module.functions:
        if f.name == name:
            return f
    fnty = ir.FunctionType(ret, args)
    return ir.Function(module, fnty, name=name)


class TestRuntimeDataDispatchPolicy(unittest.TestCase):
    def tearDown(self):
        os.environ.pop("NYASH_RUNTIME_DATA_ARRAY_ROUTE_POLICY", None)
        _reset_runtime_data_array_route_policy_cache_for_tests()

    def test_default_prefers_array_i64_route_when_hints_match(self):
        resolver = _DummyResolver(
            value_types={
                1: {"kind": "handle", "box_type": "ArrayBox"},
                2: "i64",
                3: "i64",
            },
            integerish_ids={2, 3},
        )
        spec = select_runtime_data_call_spec(
            method="set",
            box_name="RuntimeDataBox",
            resolver=resolver,
            receiver_vid=1,
            arg_vids=[2, 3],
            prefer_array_mono_route=True,
        )
        self.assertEqual(spec[0], "nyash.array.slot_store_hii")

    def test_default_prefers_array_i64_get_raw_route_when_hints_match(self):
        resolver = _DummyResolver(
            value_types={
                1: {"kind": "handle", "box_type": "ArrayBox"},
                2: "i64",
            },
            integerish_ids={2},
        )
        spec = select_runtime_data_call_spec(
            method="get",
            box_name="RuntimeDataBox",
            resolver=resolver,
            receiver_vid=1,
            arg_vids=[2],
            prefer_array_mono_route=True,
        )
        self.assertEqual(spec[0], "nyash.array.slot_load_hi")

    def test_default_prefers_array_push_slot_append_when_hints_match(self):
        resolver = _DummyResolver(
            value_types={
                1: {"kind": "handle", "box_type": "ArrayBox"},
                2: {"kind": "handle", "box_type": "StringBox"},
            },
            integerish_ids=set(),
        )
        spec = select_runtime_data_call_spec(
            method="push",
            box_name="RuntimeDataBox",
            resolver=resolver,
            receiver_vid=1,
            arg_vids=[2],
            prefer_array_mono_route=True,
        )
        self.assertEqual(spec[0], "nyash.array.slot_append_hh")

    def test_default_keeps_non_integer_array_get_on_runtime_data_facade(self):
        resolver = _DummyResolver(
            value_types={
                1: {"kind": "handle", "box_type": "ArrayBox"},
                2: {"kind": "handle", "box_type": "StringBox"},
            },
            integerish_ids=set(),
        )
        spec = select_runtime_data_call_spec(
            method="get",
            box_name="RuntimeDataBox",
            resolver=resolver,
            receiver_vid=1,
            arg_vids=[2],
            prefer_array_mono_route=True,
        )
        self.assertEqual(spec[0], "nyash.runtime_data.get_hh")

    def test_default_keeps_non_integer_array_set_on_runtime_data_facade(self):
        resolver = _DummyResolver(
            value_types={
                1: {"kind": "handle", "box_type": "ArrayBox"},
                2: {"kind": "handle", "box_type": "StringBox"},
                3: {"kind": "handle", "box_type": "StringBox"},
            },
            integerish_ids=set(),
        )
        spec = select_runtime_data_call_spec(
            method="set",
            box_name="RuntimeDataBox",
            resolver=resolver,
            receiver_vid=1,
            arg_vids=[2, 3],
            prefer_array_mono_route=True,
        )
        self.assertEqual(spec[0], "nyash.runtime_data.set_hhh")

    def test_default_keeps_array_has_on_runtime_data_facade(self):
        resolver = _DummyResolver(
            value_types={
                1: {"kind": "handle", "box_type": "ArrayBox"},
                2: "i64",
            },
            integerish_ids={2},
        )
        spec = select_runtime_data_call_spec(
            method="has",
            box_name="RuntimeDataBox",
            resolver=resolver,
            receiver_vid=1,
            arg_vids=[2],
            prefer_array_mono_route=True,
        )
        self.assertEqual(spec[0], "nyash.runtime_data.has_hh")

    def test_map_receiver_keeps_set_on_runtime_data_facade(self):
        resolver = _DummyResolver(
            value_types={
                1: {"kind": "handle", "box_type": "MapBox"},
                2: "i64",
                3: "i64",
            },
            integerish_ids={2, 3},
        )
        spec = select_runtime_data_call_spec(
            method="set",
            box_name="RuntimeDataBox",
            resolver=resolver,
            receiver_vid=1,
            arg_vids=[2, 3],
            prefer_array_mono_route=True,
        )
        self.assertEqual(spec[0], "nyash.runtime_data.set_hhh")

    def test_runtime_data_only_policy_disables_array_mono_route(self):
        resolver = _DummyResolver(
            value_types={
                1: {"kind": "handle", "box_type": "ArrayBox"},
                2: "i64",
                3: "i64",
            },
            integerish_ids={2, 3},
        )
        spec = select_runtime_data_call_spec(
            method="set",
            box_name="RuntimeDataBox",
            resolver=resolver,
            receiver_vid=1,
            arg_vids=[2, 3],
            prefer_array_mono_route=False,
        )
        self.assertEqual(spec[0], "nyash.runtime_data.set_hhh")

    def test_non_runtime_data_box_returns_none(self):
        spec = select_runtime_data_call_spec(
            method="get",
            box_name="MapBox",
            resolver=None,
            receiver_vid=None,
            arg_vids=[1],
        )
        self.assertIsNone(spec)

    def test_policy_default_is_array_mono(self):
        os.environ.pop("NYASH_RUNTIME_DATA_ARRAY_ROUTE_POLICY", None)
        _reset_runtime_data_array_route_policy_cache_for_tests()
        self.assertTrue(_prefer_array_mono_route_default())

    def test_policy_runtime_data_only_switch(self):
        os.environ["NYASH_RUNTIME_DATA_ARRAY_ROUTE_POLICY"] = "runtime_data_only"
        _reset_runtime_data_array_route_policy_cache_for_tests()
        self.assertFalse(_prefer_array_mono_route_default())

    def test_policy_invalid_value_fails_fast(self):
        os.environ["NYASH_RUNTIME_DATA_ARRAY_ROUTE_POLICY"] = "unexpected"
        _reset_runtime_data_array_route_policy_cache_for_tests()
        with self.assertRaises(RuntimeError):
            _prefer_array_mono_route_default()

    def test_runtime_data_get_field_uses_map_kernel(self):
        i64 = ir.IntType(64)
        module = ir.Module(name="test_runtime_data_get_field")
        fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("entry")
        builder = ir.IRBuilder(bb)

        result = lower_runtime_data_field_call(
            builder=builder,
            declare=lambda name, ret, args: _declare(module, name, ret, args),
            box_name="RuntimeDataBox",
            method="getField",
            recv_h=ir.Constant(i64, 1),
            args=[2],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
        )
        builder.ret(result)

        self.assertIn("nyash.map.get_hh", str(module))

    def test_runtime_data_set_field_uses_map_kernel(self):
        i64 = ir.IntType(64)
        module = ir.Module(name="test_runtime_data_set_field")
        fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("entry")
        builder = ir.IRBuilder(bb)

        result = lower_runtime_data_field_call(
            builder=builder,
            declare=lambda name, ret, args: _declare(module, name, ret, args),
            box_name="RuntimeDataBox",
            method="setField",
            recv_h=ir.Constant(i64, 1),
            args=[2, 3],
            resolve_arg=lambda vid: ir.Constant(i64, vid),
        )
        builder.ret(result)

        self.assertIn("nyash.map.set_hh", str(module))


if __name__ == "__main__":
    unittest.main()
