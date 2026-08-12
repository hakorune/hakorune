#!/usr/bin/env python3
"""Focused rejection tests for the normalized TextScan contract."""

from __future__ import annotations

import importlib.util
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
GENERATOR_PATH = ROOT / "tools/provider_slot_contract_manifest_codegen.py"
SPEC = importlib.util.spec_from_file_location("provider_slot_contract_codegen", GENERATOR_PATH)
assert SPEC is not None and SPEC.loader is not None
CODEGEN = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CODEGEN)


def role(name: str, **overrides: str) -> dict[str, str]:
    base = dict(CODEGEN.EXPECTED_ROLES[name])
    base["role"] = name
    base.update(overrides)
    return base


class ProviderSlotContractCodegenTests(unittest.TestCase):
    def setUp(self) -> None:
        self.metadata = {
            "contract_id": "hako.text.scan@1",
            "profile": "utf8-codepoint-clamped-v1",
            "receiver": "Text",
            "suspension": "non_suspending",
        }

    def test_exact_two_roles_are_required(self) -> None:
        with self.assertRaisesRegex(ValueError, "role count"):
            CODEGEN.validate_contract(self.metadata, [role("TextSliceRange")])

    def test_duplicate_role_is_rejected(self) -> None:
        with self.assertRaisesRegex(ValueError, "duplicate TextScan role"):
            CODEGEN.validate_contract(
                self.metadata,
                [role("TextSliceRange"), role("TextSliceRange")],
            )

    def test_result_and_effect_fields_are_not_part_of_role_schema(self) -> None:
        self.assertNotIn("result_kind", CODEGEN.ROLE_FIELDS)
        self.assertNotIn("effect", CODEGEN.ROLE_FIELDS)

    def test_wrong_operation_or_lifecycle_is_rejected(self) -> None:
        with self.assertRaisesRegex(ValueError, "unexpected TextScan role"):
            CODEGEN.validate_contract(
                self.metadata,
                [
                    role("TextSliceRange", core_op="StringIndexOf"),
                    role("TextFindNeedle"),
                ],
            )


if __name__ == "__main__":
    unittest.main()
