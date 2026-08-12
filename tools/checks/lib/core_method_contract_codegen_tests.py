#!/usr/bin/env python3
"""Focused rejection tests for CoreMethodContract artifact generation."""

from __future__ import annotations

import importlib.util
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
GENERATOR_PATH = ROOT / "tools/core_method_contract_manifest_codegen.py"
SPEC = importlib.util.spec_from_file_location("core_method_contract_codegen", GENERATOR_PATH)
assert SPEC is not None and SPEC.loader is not None
CODEGEN = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CODEGEN)


def row(
    receiver: str,
    canonical: str,
    arity: str,
    *,
    aliases: list[str] | None = None,
    core_op: str = "ArrayGet",
    result_kind: str = "Dynamic",
) -> dict[str, object]:
    return {
        "box": receiver,
        "canonical": canonical,
        "aliases": aliases or [],
        "arity": arity,
        "effect": "pure_read",
        "core_op": core_op,
        "result_kind": result_kind,
        "lowering_tier": "warm_direct_abi",
        "cold_lowering": "test.helper",
        "runtime_owner": "test owner",
        "id": f"{receiver}.{canonical}/{arity}",
    }


class CoreMethodContractCodegenTests(unittest.TestCase):
    def test_same_spelling_on_different_receivers_is_allowed(self) -> None:
        CODEGEN.validate_rows(
            [row("ArrayBox", "length", "0"), row("StringBox", "length", "0")]
        )

    def test_same_receiver_disjoint_arities_are_allowed(self) -> None:
        CODEGEN.validate_rows(
            [row("StringBox", "find", "1"), row("StringBox", "find", "2")]
        )

    def test_canonical_alias_collision_is_rejected(self) -> None:
        with self.assertRaisesRegex(ValueError, "spelling collision"):
            CODEGEN.validate_rows(
                [
                    row("StringBox", "length", "0", aliases=["len"]),
                    row("StringBox", "len", "0", core_op="StringLength"),
                ]
            )

    def test_alias_alias_collision_is_rejected(self) -> None:
        with self.assertRaisesRegex(ValueError, "spelling collision"):
            CODEGEN.validate_rows(
                [
                    row("StringBox", "indexOf", "1", aliases=["find"]),
                    row(
                        "StringBox",
                        "search",
                        "1",
                        aliases=["find"],
                        core_op="StringSearch",
                    ),
                ]
            )

    def test_expanded_arity_overlap_is_rejected(self) -> None:
        with self.assertRaisesRegex(ValueError, "spelling collision"):
            CODEGEN.validate_rows(
                [
                    row("StringBox", "substring", "1|2"),
                    row("StringBox", "substring", "2", core_op="StringSubstring"),
                ]
            )

    def test_same_receiver_operation_and_arity_is_rejected(self) -> None:
        with self.assertRaisesRegex(ValueError, "operation collision"):
            CODEGEN.validate_rows(
                [
                    row("StringBox", "find", "1", core_op="StringIndexOf"),
                    row("StringBox", "search", "1", core_op="StringIndexOf"),
                ]
            )

    def test_duplicate_alias_inside_one_row_is_rejected(self) -> None:
        with self.assertRaisesRegex(ValueError, "spelling collision"):
            CODEGEN.validate_rows(
                [row("StringBox", "length", "0", aliases=["len", "len"])]
            )

    def test_unknown_result_kind_is_rejected(self) -> None:
        with self.assertRaisesRegex(ValueError, "unknown result_kind"):
            CODEGEN.validate_rows(
                [row("StringBox", "length", "0", result_kind="ScalarMaybe")]
            )

    def test_unknown_effect_is_rejected(self) -> None:
        malformed = row("StringBox", "length", "0")
        malformed["effect"] = "observable_maybe"
        with self.assertRaisesRegex(ValueError, "unknown effect"):
            CODEGEN.validate_rows([malformed])

    def test_missing_result_kind_is_rejected(self) -> None:
        malformed = row("StringBox", "length", "0")
        del malformed["result_kind"]
        with self.assertRaisesRegex(ValueError, "missing fields"):
            CODEGEN.validate_rows([malformed])

    def test_malformed_arity_is_rejected(self) -> None:
        for arity in ("", "2|1", "1|1", "-1", "1|"):
            with self.subTest(arity=arity), self.assertRaises(ValueError):
                CODEGEN.validate_rows([row("StringBox", "substring", arity)])


if __name__ == "__main__":
    unittest.main()
