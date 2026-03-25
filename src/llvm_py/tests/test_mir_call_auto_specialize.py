#!/usr/bin/env python3
import unittest

from src.llvm_py.instructions.mir_call.auto_specialize import (
    prefer_array_len_h_route,
    prefer_map_len_h_route,
    prefer_runtime_data_array_i64_key_i64_value_route,
    prefer_runtime_data_array_i64_key_route,
    prefer_runtime_data_array_route,
    prefer_string_len_h_route,
    receiver_is_arrayish,
    receiver_is_mapish,
    receiver_is_stringish,
)


class _DummyResolver:
    def __init__(self, string_ids=None, array_ids=None, value_types=None, integerish_ids=None):
        self._string_ids = set(string_ids or [])
        self._array_ids = set(array_ids or [])
        self.value_types = value_types or {}
        self.integerish_ids = set(integerish_ids or [])

    def is_stringish(self, value_id: int) -> bool:
        return int(value_id) in self._string_ids

    def is_arrayish(self, value_id: int) -> bool:
        return int(value_id) in self._array_ids

    def is_mapish(self, value_id: int) -> bool:
        value_type = self.value_types.get(int(value_id))
        return isinstance(value_type, dict) and value_type.get("box_type") == "MapBox"


class TestMirCallAutoSpecialize(unittest.TestCase):
    def test_receiver_is_stringish_via_tag(self):
        resolver = _DummyResolver(string_ids={10})
        self.assertTrue(receiver_is_stringish(resolver, 10))
        self.assertFalse(receiver_is_stringish(resolver, 11))

    def test_receiver_is_stringish_via_value_types(self):
        resolver = _DummyResolver(value_types={7: {"kind": "handle", "box_type": "StringBox"}})
        self.assertTrue(receiver_is_stringish(resolver, 7))
        self.assertFalse(receiver_is_stringish(resolver, 8))

    def test_prefer_string_len_h_route(self):
        resolver = _DummyResolver(string_ids={4})
        self.assertTrue(prefer_string_len_h_route("length", 0, resolver, 4))
        self.assertTrue(prefer_string_len_h_route("size", 0, resolver, 4))
        self.assertFalse(prefer_string_len_h_route("indexOf", 0, resolver, 4))
        self.assertFalse(prefer_string_len_h_route("length", 1, resolver, 4))
        self.assertFalse(prefer_string_len_h_route("length", 0, resolver, 9))

    def test_prefer_array_len_h_route(self):
        resolver = _DummyResolver(array_ids={6})
        self.assertTrue(prefer_array_len_h_route("length", 0, resolver, 6))
        self.assertTrue(prefer_array_len_h_route("size", 0, resolver, 6))
        self.assertFalse(prefer_array_len_h_route("indexOf", 0, resolver, 6))
        self.assertFalse(prefer_array_len_h_route("length", 1, resolver, 6))
        self.assertFalse(prefer_array_len_h_route("length", 0, resolver, 9))

    def test_receiver_is_arrayish_via_value_types(self):
        resolver = _DummyResolver(value_types={11: {"kind": "handle", "box_type": "ArrayBox"}})
        self.assertTrue(receiver_is_arrayish(resolver, 11))
        self.assertFalse(receiver_is_arrayish(resolver, 12))

    def test_receiver_is_arrayish_via_tag(self):
        resolver = _DummyResolver(array_ids={21})
        self.assertTrue(receiver_is_arrayish(resolver, 21))
        self.assertFalse(receiver_is_arrayish(resolver, 22))

    def test_receiver_is_mapish_via_value_types(self):
        resolver = _DummyResolver(value_types={31: {"kind": "handle", "box_type": "MapBox"}})
        self.assertTrue(receiver_is_mapish(resolver, 31))
        self.assertFalse(receiver_is_mapish(resolver, 32))

    def test_prefer_map_len_h_route(self):
        resolver = _DummyResolver(value_types={41: {"kind": "handle", "box_type": "MapBox"}})
        self.assertTrue(prefer_map_len_h_route("size", 0, resolver, 41))
        self.assertTrue(prefer_map_len_h_route("length", 0, resolver, 41))
        self.assertFalse(prefer_map_len_h_route("indexOf", 0, resolver, 41))
        self.assertFalse(prefer_map_len_h_route("size", 1, resolver, 41))

    def test_prefer_runtime_data_array_route_push(self):
        resolver = _DummyResolver(value_types={1: {"kind": "handle", "box_type": "ArrayBox"}})
        self.assertTrue(prefer_runtime_data_array_route("push", "RuntimeDataBox", resolver, 1, [2]))

    def test_prefer_runtime_data_array_route_get_allows_any_key_value(self):
        resolver = _DummyResolver(
            value_types={
                1: {"kind": "handle", "box_type": "ArrayBox"},
                2: {"kind": "handle", "box_type": "StringBox"},
                3: "i64",
            }
        )
        self.assertTrue(prefer_runtime_data_array_route("get", "RuntimeDataBox", resolver, 1, [2]))
        self.assertTrue(prefer_runtime_data_array_route("get", "RuntimeDataBox", resolver, 1, [3]))

    def test_prefer_runtime_data_array_route_non_array_receiver_stays_off(self):
        resolver = _DummyResolver(value_types={1: {"kind": "handle", "box_type": "MapBox"}})
        self.assertFalse(prefer_runtime_data_array_route("get", "RuntimeDataBox", resolver, 1, [2]))

    def test_prefer_runtime_data_array_i64_key_route_integerish_hint(self):
        resolver = _DummyResolver(integerish_ids={2})
        self.assertTrue(prefer_runtime_data_array_i64_key_route("get", resolver, [2]))
        self.assertTrue(prefer_runtime_data_array_i64_key_route("set", resolver, [2, 9]))
        self.assertFalse(prefer_runtime_data_array_i64_key_route("push", resolver, [2]))

    def test_prefer_runtime_data_array_i64_key_route_value_type_hint(self):
        resolver = _DummyResolver(value_types={2: "i64", 3: {"kind": "i64"}})
        self.assertTrue(prefer_runtime_data_array_i64_key_route("has", resolver, [2]))
        self.assertTrue(prefer_runtime_data_array_i64_key_route("set", resolver, [3, 9]))
        self.assertFalse(prefer_runtime_data_array_i64_key_route("get", resolver, [9]))

    def test_prefer_runtime_data_array_i64_key_i64_value_route(self):
        resolver = _DummyResolver(integerish_ids={2, 3})
        self.assertTrue(
            prefer_runtime_data_array_i64_key_i64_value_route("set", resolver, [2, 3])
        )
        self.assertFalse(
            prefer_runtime_data_array_i64_key_i64_value_route("set", resolver, [2, 9])
        )
        self.assertFalse(
            prefer_runtime_data_array_i64_key_i64_value_route("get", resolver, [2])
        )


if __name__ == "__main__":
    unittest.main()
