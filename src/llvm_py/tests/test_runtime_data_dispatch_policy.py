#!/usr/bin/env python3
import os
import unittest

from src.llvm_py.instructions.mir_call.runtime_data_dispatch import (
    _prefer_array_mono_route_default,
    _reset_runtime_data_array_route_policy_cache_for_tests,
    select_runtime_data_call_spec,
)


class _DummyResolver:
    def __init__(self, value_types=None, integerish_ids=None):
        self.value_types = value_types or {}
        self.integerish_ids = set(integerish_ids or [])

    def is_arrayish(self, value_id: int) -> bool:
        value = self.value_types.get(int(value_id))
        return isinstance(value, dict) and value.get("box_type") == "ArrayBox"


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
        self.assertEqual(spec[0], "nyash.array.set_hii")

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


if __name__ == "__main__":
    unittest.main()
