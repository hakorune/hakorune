#!/usr/bin/env python3
import sys
import unittest
from pathlib import Path

import llvmlite.ir as ir

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from instructions.field_access import lower_field_get, lower_field_set
from type_facts import make_box_handle_fact


class _ResolverStub:
    def __init__(self):
        self.value_types = {}
        self.integerish_ids = set()
        self.def_blocks = {}
        self.thin_entry_selection_by_value = {}
        self.thin_entry_selection_by_subject = {}
        self.thin_entry_selections = []

    def resolve_i64(self, value_id, current_block, preds, block_end_values, vmap, bb_map):
        return ir.Constant(ir.IntType(64), int(value_id))


class TestTypedUserBoxFieldAccess(unittest.TestCase):
    def _make_builder(self):
        mod = ir.Module(name="typed_user_box_field_access")
        i64 = ir.IntType(64)
        fn = ir.Function(mod, ir.FunctionType(i64, []), name="main")
        bb = fn.append_basic_block("bb1")
        return mod, ir.IRBuilder(bb), bb, i64

    def test_field_get_uses_typed_integer_helper_for_typed_user_box_field(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("Point")
        vmap = {1: ir.Constant(i64, 101)}
        user_box_decls = [
            {
                "name": "Point",
                "fields": ["x"],
                "field_decls": [
                    {"name": "x", "declared_type": "IntegerBox", "is_weak": False}
                ],
            }
        ]

        lower_field_get(
            builder,
            mod,
            1,
            "x",
            2,
            {"kind": "handle", "box_type": "IntegerBox"},
            user_box_decls,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn('call i64 @"nyash.instance.get_i64_field_h"', ir_txt, msg=ir_txt)
        self.assertNotIn("RuntimeDataBox", ir_txt, msg=ir_txt)
        self.assertEqual(resolver.value_types[2], "i64")
        self.assertIn(2, resolver.integerish_ids)

    def test_field_set_uses_typed_integer_helper_and_unboxes_integer_handles(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("Point")
        resolver.value_types[2] = make_box_handle_fact("IntegerBox")
        vmap = {
            1: ir.Constant(i64, 101),
            2: ir.Constant(i64, 202),
        }
        user_box_decls = [
            {
                "name": "Point",
                "fields": ["x"],
                "field_decls": [
                    {"name": "x", "declared_type": "IntegerBox", "is_weak": False}
                ],
            }
        ]

        lower_field_set(
            builder,
            mod,
            1,
            "x",
            2,
            {"kind": "handle", "box_type": "IntegerBox"},
            user_box_decls,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn('call i64 @"nyash.integer.get_h"', ir_txt, msg=ir_txt)
        self.assertIn('call i64 @"nyash.instance.set_i64_field_h"', ir_txt, msg=ir_txt)
        self.assertNotIn("RuntimeDataBox", ir_txt, msg=ir_txt)

    def test_field_get_uses_thin_entry_selector_without_decl_rediscovery(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("Point")
        resolver.thin_entry_selection_by_subject[("user_box_field_get", "Point.x")] = [
            {
                "surface": "user_box_field_get",
                "subject": "Point.x",
                "manifest_row": "user_box_field_get.inline_scalar",
                "selected_entry": "thin_internal_entry",
                "state": "already_satisfied",
            }
        ]
        vmap = {1: ir.Constant(i64, 101)}

        lower_field_get(
            builder,
            mod,
            1,
            "x",
            2,
            {"kind": "handle", "box_type": "IntegerBox"},
            [],
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn('call i64 @"nyash.instance.get_i64_field_h"', ir_txt, msg=ir_txt)
        self.assertNotIn("RuntimeDataBox", ir_txt, msg=ir_txt)

    def test_field_set_uses_thin_entry_selector_without_decl_rediscovery(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("Point")
        resolver.value_types[2] = make_box_handle_fact("IntegerBox")
        resolver.thin_entry_selection_by_subject[("user_box_field_set", "Point.x")] = [
            {
                "surface": "user_box_field_set",
                "subject": "Point.x",
                "manifest_row": "user_box_field_set.inline_scalar",
                "selected_entry": "thin_internal_entry",
                "state": "already_satisfied",
            }
        ]
        vmap = {
            1: ir.Constant(i64, 101),
            2: ir.Constant(i64, 202),
        }

        lower_field_set(
            builder,
            mod,
            1,
            "x",
            2,
            {"kind": "handle", "box_type": "IntegerBox"},
            [],
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn('call i64 @"nyash.integer.get_h"', ir_txt, msg=ir_txt)
        self.assertIn('call i64 @"nyash.instance.set_i64_field_h"', ir_txt, msg=ir_txt)
        self.assertNotIn("RuntimeDataBox", ir_txt, msg=ir_txt)

    def test_field_get_keeps_public_entry_when_selector_says_public_default(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("Point")
        resolver.thin_entry_selection_by_subject[("user_box_field_get", "Point.x")] = [
            {
                "surface": "user_box_field_get",
                "subject": "Point.x",
                "manifest_row": "user_box_field_get.public_default",
                "selected_entry": "public_entry",
                "state": "already_satisfied",
            }
        ]
        vmap = {1: ir.Constant(i64, 101)}
        user_box_decls = [
            {
                "name": "Point",
                "fields": ["x"],
                "field_decls": [
                    {"name": "x", "declared_type": "IntegerBox", "is_weak": False}
                ],
            }
        ]

        lower_field_get(
            builder,
            mod,
            1,
            "x",
            2,
            {"kind": "handle", "box_type": "IntegerBox"},
            user_box_decls,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertNotIn('@"nyash.instance.get_i64_field_h"', ir_txt, msg=ir_txt)
        self.assertIn('call i64 @"nyash.map.slot_load_hh"', ir_txt, msg=ir_txt)

    def test_field_get_uses_typed_bool_helper_for_typed_user_box_field(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("Flag")
        vmap = {1: ir.Constant(i64, 101)}
        user_box_decls = [
            {
                "name": "Flag",
                "fields": ["enabled"],
                "field_decls": [
                    {"name": "enabled", "declared_type": "BoolBox", "is_weak": False}
                ],
            }
        ]

        lower_field_get(
            builder,
            mod,
            1,
            "enabled",
            2,
            {"kind": "handle", "box_type": "BoolBox"},
            user_box_decls,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn('call i64 @"nyash.instance.get_bool_field_h"', ir_txt, msg=ir_txt)
        self.assertNotIn("RuntimeDataBox", ir_txt, msg=ir_txt)
        self.assertEqual(resolver.value_types[2], "i1")

    def test_field_set_uses_typed_bool_helper_and_unboxes_bool_handles(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("Flag")
        resolver.value_types[2] = make_box_handle_fact("BoolBox")
        vmap = {
            1: ir.Constant(i64, 101),
            2: ir.Constant(i64, 202),
        }
        user_box_decls = [
            {
                "name": "Flag",
                "fields": ["enabled"],
                "field_decls": [
                    {"name": "enabled", "declared_type": "BoolBox", "is_weak": False}
                ],
            }
        ]

        lower_field_set(
            builder,
            mod,
            1,
            "enabled",
            2,
            {"kind": "handle", "box_type": "BoolBox"},
            user_box_decls,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn('call i64 @"nyash.bool.get_h"', ir_txt, msg=ir_txt)
        self.assertIn('call i64 @"nyash.instance.set_bool_field_h"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.map.slot_store_hhh"', ir_txt, msg=ir_txt)

    def test_field_get_uses_typed_float_helper_for_typed_user_box_field(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("PointF")
        vmap = {1: ir.Constant(i64, 101)}
        user_box_decls = [
            {
                "name": "PointF",
                "fields": ["x"],
                "field_decls": [
                    {"name": "x", "declared_type": "FloatBox", "is_weak": False}
                ],
            }
        ]

        lower_field_get(
            builder,
            mod,
            1,
            "x",
            2,
            {"kind": "handle", "box_type": "FloatBox"},
            user_box_decls,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn('call double @"nyash.instance.get_float_field_h"', ir_txt, msg=ir_txt)
        self.assertNotIn("RuntimeDataBox", ir_txt, msg=ir_txt)
        self.assertEqual(resolver.value_types[2], "Float")
        self.assertIsInstance(vmap[2].type, ir.DoubleType)

    def test_field_set_uses_typed_float_helper_and_unboxes_float_handles(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("PointF")
        resolver.value_types[2] = make_box_handle_fact("FloatBox")
        vmap = {
            1: ir.Constant(i64, 101),
            2: ir.Constant(i64, 202),
        }
        user_box_decls = [
            {
                "name": "PointF",
                "fields": ["x"],
                "field_decls": [
                    {"name": "x", "declared_type": "FloatBox", "is_weak": False}
                ],
            }
        ]

        lower_field_set(
            builder,
            mod,
            1,
            "x",
            2,
            {"kind": "handle", "box_type": "FloatBox"},
            user_box_decls,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn('call double @"nyash.float.unbox_to_f64"', ir_txt, msg=ir_txt)
        self.assertIn('call i64 @"nyash.instance.set_float_field_h"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.map.slot_store_hhh"', ir_txt, msg=ir_txt)

    def test_field_set_uses_typed_float_helper_for_float_immediates(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("PointF")
        resolver.value_types[2] = "Float"
        vmap = {
            1: ir.Constant(i64, 101),
            2: ir.Constant(ir.DoubleType(), 1.5),
        }
        user_box_decls = [
            {
                "name": "PointF",
                "fields": ["x"],
                "field_decls": [
                    {"name": "x", "declared_type": "FloatBox", "is_weak": False}
                ],
            }
        ]

        lower_field_set(
            builder,
            mod,
            1,
            "x",
            2,
            {"kind": "handle", "box_type": "FloatBox"},
            user_box_decls,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn('call i64 @"nyash.instance.set_float_field_h"', ir_txt, msg=ir_txt)
        self.assertNotIn('@"nyash.float.unbox_to_f64"', ir_txt, msg=ir_txt)

    def test_field_set_keeps_generic_fallback_for_float_field_when_source_is_not_floatish(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("PointF")
        resolver.value_types[2] = make_box_handle_fact("IntegerBox")
        vmap = {
            1: ir.Constant(i64, 101),
            2: ir.Constant(i64, 202),
        }
        user_box_decls = [
            {
                "name": "PointF",
                "fields": ["x"],
                "field_decls": [
                    {"name": "x", "declared_type": "FloatBox", "is_weak": False}
                ],
            }
        ]

        lower_field_set(
            builder,
            mod,
            1,
            "x",
            2,
            {"kind": "handle", "box_type": "FloatBox"},
            user_box_decls,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertNotIn('call i64 @"nyash.instance.set_float_field_h"', ir_txt, msg=ir_txt)
        self.assertIn('call i64 @"nyash.map.slot_store_hhh"', ir_txt, msg=ir_txt)

    def test_field_set_uses_typed_bool_helper_for_bool_immediates(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("Flag")
        resolver.value_types[2] = "i1"
        vmap = {
            1: ir.Constant(i64, 101),
            2: ir.Constant(i64, 1),
        }
        user_box_decls = [
            {
                "name": "Flag",
                "fields": ["enabled"],
                "field_decls": [
                    {"name": "enabled", "declared_type": "BoolBox", "is_weak": False}
                ],
            }
        ]

        lower_field_set(
            builder,
            mod,
            1,
            "enabled",
            2,
            {"kind": "handle", "box_type": "BoolBox"},
            user_box_decls,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertIn('call i64 @"nyash.instance.set_bool_field_h"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.bool.get_h"', ir_txt, msg=ir_txt)

    def test_field_set_keeps_generic_fallback_for_bool_field_when_source_is_not_boolish(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("Flag")
        resolver.value_types[2] = make_box_handle_fact("IntegerBox")
        vmap = {
            1: ir.Constant(i64, 101),
            2: ir.Constant(i64, 202),
        }
        user_box_decls = [
            {
                "name": "Flag",
                "fields": ["enabled"],
                "field_decls": [
                    {"name": "enabled", "declared_type": "BoolBox", "is_weak": False}
                ],
            }
        ]

        lower_field_set(
            builder,
            mod,
            1,
            "enabled",
            2,
            {"kind": "handle", "box_type": "BoolBox"},
            user_box_decls,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertNotIn('call i64 @"nyash.instance.set_bool_field_h"', ir_txt, msg=ir_txt)
        self.assertIn('call i64 @"nyash.map.slot_store_hhh"', ir_txt, msg=ir_txt)

    def test_field_get_keeps_generic_fallback_for_weak_typed_fields(self):
        mod, builder, bb, i64 = self._make_builder()
        resolver = _ResolverStub()
        resolver.value_types[1] = make_box_handle_fact("Point")
        vmap = {1: ir.Constant(i64, 101)}
        user_box_decls = [
            {
                "name": "Point",
                "fields": ["x"],
                "field_decls": [
                    {"name": "x", "declared_type": "IntegerBox", "is_weak": True}
                ],
            }
        ]

        lower_field_get(
            builder,
            mod,
            1,
            "x",
            2,
            {"kind": "handle", "box_type": "IntegerBox"},
            user_box_decls,
            vmap,
            resolver,
            preds={},
            block_end_values={},
            bb_map={1: bb},
        )

        ir_txt = str(mod)
        self.assertNotIn('@"nyash.instance.get_i64_field_h"', ir_txt, msg=ir_txt)
        self.assertIn('call i64 @"nyash.map.slot_load_hh"', ir_txt, msg=ir_txt)
        self.assertEqual(resolver.value_types[2], {"kind": "handle"})


if __name__ == "__main__":
    unittest.main()
