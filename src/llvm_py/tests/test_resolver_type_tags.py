#!/usr/bin/env python3
import unittest

from src.llvm_py.resolver import Resolver


class TestResolverTypeTags(unittest.TestCase):
    def test_mark_string_populates_string_ids_and_value_types(self):
        resolver = Resolver({}, {})
        resolver.mark_string(7)

        self.assertIn(7, resolver.string_ids)
        self.assertEqual(
            resolver.value_types.get(7),
            {"kind": "handle", "box_type": "StringBox"},
        )

    def test_mark_string_normalizes_non_dict_value_types_storage(self):
        resolver = Resolver({}, {})
        resolver.value_types = None

        resolver.mark_string(8)

        self.assertIsInstance(resolver.value_types, dict)
        self.assertTrue(resolver.is_stringish(8))

    def test_is_stringish_accepts_legacy_fact_shapes(self):
        resolver = Resolver({}, {})
        resolver.value_types[9] = {"kind": "string"}
        resolver.value_types[10] = "StringBox"

        self.assertTrue(resolver.is_stringish(9))
        self.assertTrue(resolver.is_stringish(10))
        self.assertFalse(resolver.is_stringish(11))

    def test_is_arrayish_accepts_metadata_fallback(self):
        resolver = Resolver({}, {})
        resolver.value_types[12] = {"kind": "handle", "box_type": "ArrayBox"}

        self.assertTrue(resolver.is_arrayish(12))
        self.assertFalse(resolver.is_arrayish(13))
