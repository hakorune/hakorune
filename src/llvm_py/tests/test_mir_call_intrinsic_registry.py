#!/usr/bin/env python3
import unittest

from src.llvm_py.instructions.mir_call.intrinsic_registry import (
    TAG_INTRINSIC_CANDIDATE,
    IntrinsicSpec,
    get_registry_consistency_errors,
    is_length_like_method,
    iter_intrinsic_specs,
    lookup_intrinsic_spec,
    produces_string_result,
    requires_string_receiver_tag,
    validate_intrinsic_specs,
)


class TestMirCallIntrinsicRegistry(unittest.TestCase):
    def test_registry_consistency_has_no_errors(self):
        self.assertEqual(get_registry_consistency_errors(), ())

    def test_length_like_aliases(self):
        self.assertTrue(is_length_like_method("length"))
        self.assertTrue(is_length_like_method("len"))
        self.assertTrue(is_length_like_method("size"))
        self.assertFalse(is_length_like_method("substring"))

    def test_receiver_string_tag_methods(self):
        self.assertTrue(requires_string_receiver_tag("substring"))
        self.assertTrue(requires_string_receiver_tag("indexOf"))
        self.assertTrue(requires_string_receiver_tag("lastIndexOf"))
        self.assertFalse(requires_string_receiver_tag("length"))

    def test_string_result_methods(self):
        self.assertTrue(produces_string_result("substring"))
        self.assertTrue(produces_string_result("toJson"))
        self.assertTrue(produces_string_result("resolve_for_source"))
        self.assertTrue(produces_string_result("emit_program_json_v0"))
        self.assertTrue(produces_string_result("emit_from_source_v0"))
        self.assertFalse(produces_string_result("indexOf"))
        self.assertFalse(produces_string_result(None))

    def test_lookup_intrinsic_spec_by_method_and_arity(self):
        spec = lookup_intrinsic_spec("substring", 2)
        self.assertIsNotNone(spec)
        self.assertEqual(spec.symbol, "nyash.string.substring_hii")
        self.assertIn(TAG_INTRINSIC_CANDIDATE, spec.tags)
        self.assertIsNone(lookup_intrinsic_spec("substring", 1))

    def test_intrinsic_candidate_entries_require_symbol_and_arity(self):
        for spec in iter_intrinsic_specs():
            if TAG_INTRINSIC_CANDIDATE not in spec.tags:
                continue
            self.assertIsNotNone(spec.symbol, f"symbol required for candidate: {spec.method}")
            self.assertIsNotNone(spec.arity, f"arity required for candidate: {spec.method}")

    def test_validate_intrinsic_specs_detects_duplicate_method_arity(self):
        specs = (
            IntrinsicSpec(
                method="demo",
                arity=1,
                symbol="nyash.demo.one_h",
                tags=frozenset((TAG_INTRINSIC_CANDIDATE,)),
            ),
            IntrinsicSpec(
                method="demo",
                arity=1,
                symbol="nyash.demo.one_h_v2",
                tags=frozenset((TAG_INTRINSIC_CANDIDATE,)),
            ),
        )
        errors = validate_intrinsic_specs(specs)
        self.assertTrue(any("duplicate method/arity entry" in err for err in errors), errors)

    def test_validate_intrinsic_specs_detects_candidate_missing_symbol_or_arity(self):
        specs = (
            IntrinsicSpec(
                method="missing_symbol",
                arity=0,
                symbol=None,
                tags=frozenset((TAG_INTRINSIC_CANDIDATE,)),
            ),
            IntrinsicSpec(
                method="missing_arity",
                arity=None,
                symbol="nyash.missing.arity_h",
                tags=frozenset((TAG_INTRINSIC_CANDIDATE,)),
            ),
        )
        errors = validate_intrinsic_specs(specs)
        self.assertTrue(any("requires symbol" in err for err in errors), errors)
        self.assertTrue(any("requires explicit arity" in err for err in errors), errors)


if __name__ == "__main__":
    unittest.main()
