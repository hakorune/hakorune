#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

from llvmlite import ir

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from builders.function_lower import _build_function_type, _get_or_create_function


class _BuilderStub:
    def __init__(self):
        self.i64 = ir.IntType(64)
        self.module = ir.Module(name="test_mod")
        self.call_arities = {}


class TestFunctionLowerSignature(unittest.TestCase):
    def test_build_function_type_uses_ny_main_special_case(self):
        builder = _BuilderStub()

        func_ty = _build_function_type(builder, "ny_main", [1, 2])

        self.assertEqual(str(func_ty), "i64 ()")

    def test_build_function_type_prefers_suffix_arity(self):
        builder = _BuilderStub()

        func_ty = _build_function_type(builder, "Foo.run/3", [])

        self.assertEqual(str(func_ty), "i64 (i64, i64, i64)")

    def test_build_function_type_uses_call_arity_for_global_when_params_missing(self):
        builder = _BuilderStub()
        builder.call_arities["Box.method"] = 2

        func_ty = _build_function_type(builder, "Box.method", [])

        self.assertEqual(str(func_ty), "i64 (i64, i64)")

    def test_get_or_create_function_reuses_existing_function(self):
        builder = _BuilderStub()
        func_ty = ir.FunctionType(builder.i64, [builder.i64])
        existing = ir.Function(builder.module, func_ty, name="Demo/1")

        func = _get_or_create_function(builder, "Demo/1", func_ty)

        self.assertIs(func, existing)
        self.assertEqual(len(list(builder.module.functions)), 1)


if __name__ == "__main__":
    unittest.main()
