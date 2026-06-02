#!/usr/bin/env python3
import os
import sys
import unittest
from pathlib import Path

import llvmlite.ir as ir

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from instructions.mir_call.constructor_call import lower_constructor_call
from instructions.newbox import lower_newbox


class _ResolverStub:
    pass


class _OwnerStub:
    pass


class TestDirectArrayI64ConstructorLowering(unittest.TestCase):
    def _make_builder(self, name="main"):
        mod = ir.Module(name="direct_array_i64_constructor_lowering")
        i64 = ir.IntType(64)
        fn = ir.Function(mod, ir.FunctionType(i64, []), name=name)
        bb = fn.append_basic_block("entry")
        return mod, ir.IRBuilder(bb), i64

    def _with_env(self, value, fn):
        old_value = os.environ.get("HAKO_ARRAY_SLOT_STORE")
        if value is None:
            os.environ.pop("HAKO_ARRAY_SLOT_STORE", None)
        else:
            os.environ["HAKO_ARRAY_SLOT_STORE"] = value
        try:
            return fn()
        finally:
            if old_value is None:
                os.environ.pop("HAKO_ARRAY_SLOT_STORE", None)
            else:
                os.environ["HAKO_ARRAY_SLOT_STORE"] = old_value

    def test_newbox_arraybox_default_uses_public_array_birth(self):
        def run():
            mod, builder, _ = self._make_builder()
            resolver = _ResolverStub()
            vmap = {}
            lower_newbox(builder, mod, "ArrayBox", [], 1, vmap, resolver)
            ir_txt = str(mod)
            self.assertIn("nyash.array.birth_h", ir_txt)
            self.assertNotIn("nyash.array.direct_i64.birth_h", ir_txt)
            self.assertFalse(hasattr(resolver, "direct_array_i64_ids"))
            self.assertFalse(hasattr(resolver, "arrayrepr_facts"))

        self._with_env(None, run)

    def test_newbox_arraybox_direct_lane_uses_direct_array_birth(self):
        def run():
            mod, builder, _ = self._make_builder()
            resolver = _ResolverStub()
            vmap = {}
            lower_newbox(builder, mod, "ArrayBox", [], 7, vmap, resolver)
            ir_txt = str(mod)
            self.assertIn("nyash.array.direct_i64.birth_h", ir_txt)
            self.assertNotIn("declare i64 @\"nyash.array.birth_h\"()", ir_txt)
            self.assertEqual(resolver.direct_array_i64_ids, {7})
            self.assertEqual(resolver.arrayrepr_facts, {7: "ArrayRepr::DirectI64"})

        self._with_env("direct_array_i64_exact", run)

    def test_newbox_explicit_direct_array_i64_uses_direct_array_birth_without_env(self):
        def run():
            mod, builder, _ = self._make_builder()
            resolver = _ResolverStub()
            vmap = {}
            lower_newbox(builder, mod, "DirectArrayI64", [], 11, vmap, resolver)
            ir_txt = str(mod)
            self.assertIn("nyash.array.direct_i64.birth_h", ir_txt)
            self.assertNotIn("nyash.array.birth_h", ir_txt)
            self.assertEqual(resolver.direct_array_i64_ids, {11})
            self.assertEqual(resolver.arrayrepr_facts, {11: "ArrayRepr::DirectI64"})

        self._with_env(None, run)

    def test_constructor_arraybox_direct_lane_uses_direct_array_birth(self):
        def run():
            mod, builder, _ = self._make_builder()
            resolver = _ResolverStub()
            vmap = {}
            lower_constructor_call(
                builder,
                mod,
                "ArrayBox",
                [],
                9,
                vmap,
                resolver,
                _OwnerStub(),
            )
            ir_txt = str(mod)
            self.assertIn("nyash.array.direct_i64.birth_h", ir_txt)
            self.assertNotIn("declare i64 @\"nyash.array.birth_h\"()", ir_txt)
            self.assertEqual(resolver.direct_array_i64_ids, {9})
            self.assertEqual(resolver.arrayrepr_facts, {9: "ArrayRepr::DirectI64"})

        self._with_env("direct_array_i64_exact", run)

    def test_constructor_explicit_direct_array_i64_uses_direct_array_birth_without_env(self):
        def run():
            mod, builder, _ = self._make_builder()
            resolver = _ResolverStub()
            vmap = {}
            lower_constructor_call(
                builder,
                mod,
                "DirectArrayI64",
                [],
                13,
                vmap,
                resolver,
                _OwnerStub(),
            )
            ir_txt = str(mod)
            self.assertIn("nyash.array.direct_i64.birth_h", ir_txt)
            self.assertNotIn("nyash.array.birth_h", ir_txt)
            self.assertEqual(resolver.direct_array_i64_ids, {13})
            self.assertEqual(resolver.arrayrepr_facts, {13: "ArrayRepr::DirectI64"})

        self._with_env(None, run)


if __name__ == "__main__":
    unittest.main()
