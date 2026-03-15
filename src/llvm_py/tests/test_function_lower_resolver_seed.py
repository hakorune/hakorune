#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

import builders.function_lower as function_lower
from context import FunctionLowerContext


class _ResolverStub:
    def __init__(self):
        self.value_types = None
        self.non_negative_ids = None
        self.integerish_ids = None
        self.array_ids = None
        self.string_ids = None


class _BuilderStub:
    def __init__(self):
        self.resolver = _ResolverStub()


class TestFunctionLowerResolverSeed(unittest.TestCase):
    def test_load_value_types_metadata_converts_int_keys(self):
        builder = _BuilderStub()
        func_data = {"metadata": {"value_types": {"10": {"kind": "int"}, "bad": {"kind": "skip"}}}}

        function_lower._load_value_types_metadata(builder, func_data)

        self.assertEqual(builder.resolver.value_types, {10: {"kind": "int"}})

    def test_seed_resolver_fact_sets_syncs_context_and_resolver(self):
        builder = _BuilderStub()
        context = FunctionLowerContext("Demo/0")
        blocks = [{"id": 1, "instructions": []}]

        prev_nonneg = function_lower.collect_non_negative_value_ids
        prev_integerish = function_lower.collect_integerish_value_ids
        prev_arrayish = function_lower.collect_arrayish_value_ids
        prev_stringish = function_lower.collect_stringish_value_ids

        function_lower.collect_non_negative_value_ids = lambda _blocks: {1, 2}
        function_lower.collect_integerish_value_ids = lambda _blocks: {3}
        function_lower.collect_arrayish_value_ids = lambda _blocks: {4}
        function_lower.collect_stringish_value_ids = lambda _blocks: {5, 6}

        try:
            function_lower._seed_resolver_fact_sets(builder, context, blocks)
        finally:
            function_lower.collect_non_negative_value_ids = prev_nonneg
            function_lower.collect_integerish_value_ids = prev_integerish
            function_lower.collect_arrayish_value_ids = prev_arrayish
            function_lower.collect_stringish_value_ids = prev_stringish

        self.assertEqual(context.non_negative_value_ids, {1, 2})
        self.assertEqual(builder.resolver.non_negative_ids, {1, 2})
        self.assertEqual(context.integerish_value_ids, {3})
        self.assertEqual(builder.resolver.integerish_ids, {3})
        self.assertEqual(context.resolver_array_ids, {4})
        self.assertEqual(builder.resolver.array_ids, {4})
        self.assertEqual(context.resolver_string_ids, {5, 6})
        self.assertIs(builder.resolver.string_ids, context.resolver_string_ids)


if __name__ == "__main__":
    unittest.main()
