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
        self.thin_entry_selection_by_subject = {}
        self.user_box_local_aggregate_layouts = {}


class _BuilderStub:
    def __init__(self):
        self.resolver = _ResolverStub()
        self.user_box_decls = []


class TestFunctionLowerResolverSeed(unittest.TestCase):
    def test_load_value_types_metadata_converts_int_keys(self):
        builder = _BuilderStub()
        func_data = {"metadata": {"value_types": {"10": {"kind": "int"}, "bad": {"kind": "skip"}}}}

        function_lower._load_value_types_metadata(builder, func_data)

        self.assertEqual(builder.resolver.value_types, {10: {"kind": "int"}})

    def test_load_sum_placement_metadata_extracts_local_sum_maps(self):
        builder = _BuilderStub()
        func_data = {
            "metadata": {
                "sum_placement_selections": [
                    {
                        "surface": "variant_make",
                        "value": 12,
                        "selected_path": "local_aggregate",
                    },
                    {
                        "surface": "variant_make",
                        "value": 13,
                        "selected_path": "compat_runtime_box",
                    },
                    {
                        "surface": "variant_project",
                        "value": 21,
                        "source_sum": 12,
                        "selected_path": "local_aggregate",
                    },
                ],
                "sum_placement_layouts": [
                    {
                        "surface": "variant_make",
                        "value": 12,
                        "layout": "tag_i64_payload",
                    },
                    {
                        "surface": "variant_project",
                        "value": 21,
                        "source_sum": 12,
                        "layout": "tag_i64_payload",
                    },
                ],
            }
        }

        function_lower._load_sum_placement_metadata(builder, func_data)

        self.assertEqual(builder.resolver.sum_local_aggregate_paths, {12: "local_aggregate"})
        self.assertEqual(builder.resolver.sum_local_aggregate_layouts, {12: "tag_i64_payload"})

    def test_load_sum_placement_metadata_reads_generic_placement_effect_routes_first(self):
        builder = _BuilderStub()
        func_data = {
            "metadata": {
                "placement_effect_routes": [
                    {
                        "source": "sum_placement",
                        "value": 12,
                        "decision": "local_aggregate",
                        "detail": "variant_make.local_aggregate",
                    },
                    {
                        "source": "agg_local_scalarization",
                        "value": 12,
                        "decision": "local_aggregate",
                        "detail": "sum_local_layout(tag_i64_payload)",
                    },
                    {
                        "source": "agg_local_scalarization",
                        "value": 99,
                        "decision": "local_aggregate",
                        "detail": "user_box_local_body(inline_i64)",
                    },
                ]
            }
        }

        function_lower._load_sum_placement_metadata(builder, func_data)

        self.assertEqual(builder.resolver.sum_local_aggregate_paths, {12: "local_aggregate"})
        self.assertEqual(builder.resolver.sum_local_aggregate_layouts, {12: "tag_i64_payload"})

    def test_load_thin_entry_selection_metadata_indexes_rows_by_value(self):
        builder = _BuilderStub()
        func_data = {
            "metadata": {
                "thin_entry_selections": [
                    {
                        "surface": "variant_make",
                        "value": 12,
                        "subject": "Option::Some",
                        "manifest_row": "variant_make.aggregate_local",
                        "selected_entry": "thin_internal_entry",
                        "state": "candidate",
                    },
                    {
                        "surface": "user_box_method",
                        "value": 18,
                        "subject": "Point.length",
                        "manifest_row": "user_box_method.known_receiver",
                        "selected_entry": "thin_internal_entry",
                        "state": "candidate",
                    },
                    {
                        "surface": "user_box_field_get",
                        "value": None,
                        "subject": "Point.x",
                        "manifest_row": "user_box_field_get.inline_scalar",
                        "selected_entry": "thin_internal_entry",
                        "state": "already_satisfied",
                    },
                ]
            }
        }

        function_lower._load_thin_entry_selection_metadata(builder, func_data)

        self.assertEqual(len(builder.resolver.thin_entry_selections), 3)
        self.assertEqual(builder.resolver.thin_entry_selection_by_value[12][0]["subject"], "Option::Some")
        self.assertEqual(builder.resolver.thin_entry_selection_by_value[18][0]["manifest_row"], "user_box_method.known_receiver")
        self.assertEqual(
            builder.resolver.thin_entry_selection_by_subject[("user_box_method", "Point.length")][0]["manifest_row"],
            "user_box_method.known_receiver",
        )
        self.assertEqual(
            builder.resolver.thin_entry_selection_by_subject[("user_box_field_get", "Point.x")][0]["manifest_row"],
            "user_box_field_get.inline_scalar",
        )
        self.assertNotIn(None, builder.resolver.thin_entry_selection_by_value)

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

    def test_load_user_box_local_aggregate_metadata_selects_fully_initialized_primitive_box(self):
        builder = _BuilderStub()
        builder.user_box_decls = [
            {
                "name": "Point",
                "field_decls": [
                    {"name": "x", "declared_type": "IntegerBox", "is_weak": False},
                    {"name": "y", "declared_type": "IntegerBox", "is_weak": False},
                ],
            }
        ]
        builder.resolver.thin_entry_selection_by_subject = {
            ("user_box_field_set", "Point.x"): [
                {
                    "manifest_row": "user_box_field_set.inline_scalar",
                    "selected_entry": "thin_internal_entry",
                }
            ],
            ("user_box_field_set", "Point.y"): [
                {
                    "manifest_row": "user_box_field_set.inline_scalar",
                    "selected_entry": "thin_internal_entry",
                }
            ],
            ("user_box_field_get", "Point.x"): [
                {
                    "manifest_row": "user_box_field_get.inline_scalar",
                    "selected_entry": "thin_internal_entry",
                }
            ],
            ("user_box_field_get", "Point.y"): [
                {
                    "manifest_row": "user_box_field_get.inline_scalar",
                    "selected_entry": "thin_internal_entry",
                }
            ],
        }
        func_data = {
            "blocks": [
                {
                    "id": 0,
                    "instructions": [
                        {"op": "newbox", "dst": 10, "type": "Point", "args": []},
                        {"op": "field_set", "box": 10, "field": "x", "value": 1},
                        {"op": "field_set", "box": 10, "field": "y", "value": 2},
                    ],
                },
                {
                    "id": 1,
                    "instructions": [
                        {"op": "field_get", "box": 10, "field": "x", "dst": 11},
                        {"op": "field_get", "box": 10, "field": "y", "dst": 12},
                        {"op": "ret", "value": 10},
                    ],
                },
            ]
        }

        function_lower._load_user_box_local_aggregate_metadata(builder, func_data)

        self.assertEqual(
            builder.resolver.user_box_local_aggregate_layouts,
            {
                10: {
                    "box_name": "Point",
                    "field_order": ["x", "y"],
                    "field_layouts": {
                        "x": "inline_i64",
                        "y": "inline_i64",
                    },
                    "reason": "primitive user-box fields initialize in the birth block and only leave the local route through explicit call/ret boundaries",
                }
            },
        )

    def test_load_user_box_local_aggregate_metadata_rejects_read_before_init(self):
        builder = _BuilderStub()
        builder.user_box_decls = [
            {
                "name": "Flag",
                "field_decls": [
                    {"name": "enabled", "declared_type": "BoolBox", "is_weak": False},
                ],
            }
        ]
        builder.resolver.thin_entry_selection_by_subject = {
            ("user_box_field_set", "Flag.enabled"): [
                {
                    "manifest_row": "user_box_field_set.inline_scalar",
                    "selected_entry": "thin_internal_entry",
                }
            ],
            ("user_box_field_get", "Flag.enabled"): [
                {
                    "manifest_row": "user_box_field_get.inline_scalar",
                    "selected_entry": "thin_internal_entry",
                }
            ],
        }
        func_data = {
            "blocks": [
                {
                    "id": 0,
                    "instructions": [
                        {"op": "newbox", "dst": 10, "type": "Flag", "args": []},
                        {"op": "field_get", "box": 10, "field": "enabled", "dst": 11},
                        {"op": "field_set", "box": 10, "field": "enabled", "value": 1},
                    ],
                }
            ]
        }

        function_lower._load_user_box_local_aggregate_metadata(builder, func_data)

        self.assertEqual(builder.resolver.user_box_local_aggregate_layouts, {})


if __name__ == "__main__":
    unittest.main()
