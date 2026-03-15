#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

import llvmlite.ir as ir

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from instructions.binop import _binop_plus_prefers_string_path, _binop_plus_string_tags


class _ResolverStub:
    def __init__(self):
        self.string_literals = {}
        self.value_types = {}
        self._stringish = set()

    def is_stringish(self, value_id: int) -> bool:
        return int(value_id) in self._stringish


class TestBinopRoutePolicy(unittest.TestCase):
    def test_operand_string_fact_overrides_integer_hint(self):
        resolver = _ResolverStub()
        resolver.string_literals[10] = "hello"

        is_str, explicit_integer, explicit_string, operand_is_string = _binop_plus_prefers_string_path(
            resolver,
            10,
            11,
            None,
            None,
            "i64",
        )

        self.assertTrue(is_str)
        self.assertTrue(explicit_integer)
        self.assertFalse(explicit_string)
        self.assertTrue(operand_is_string)

    def test_explicit_integer_without_string_operands_stays_numeric(self):
        resolver = _ResolverStub()

        is_str, explicit_integer, explicit_string, operand_is_string = _binop_plus_prefers_string_path(
            resolver,
            10,
            11,
            None,
            None,
            "i64",
        )

        self.assertFalse(is_str)
        self.assertTrue(explicit_integer)
        self.assertFalse(explicit_string)
        self.assertFalse(operand_is_string)

    def test_pointer_side_forces_string_path(self):
        resolver = _ResolverStub()
        i8p = ir.IntType(8).as_pointer()

        is_str, _explicit_integer, _explicit_string, operand_is_string = _binop_plus_prefers_string_path(
            resolver,
            10,
            11,
            ir.Constant(i8p, None),
            None,
            None,
        )

        self.assertTrue(is_str)
        self.assertTrue(operand_is_string)

    def test_string_tags_consult_stringish_and_value_types(self):
        resolver = _ResolverStub()
        resolver._stringish.add(10)
        resolver.value_types[11] = {"kind": "handle", "box_type": "StringBox"}

        lhs_tag, rhs_tag = _binop_plus_string_tags(resolver, 10, 11)

        self.assertTrue(lhs_tag)
        self.assertTrue(rhs_tag)


if __name__ == "__main__":
    unittest.main()
