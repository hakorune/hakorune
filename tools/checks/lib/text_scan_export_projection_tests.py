#!/usr/bin/env python3
"""Parity and negative checks for the neutral TextScan export projections."""

from __future__ import annotations

import importlib.util
import json
import re
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
MANIFEST = ROOT / "lang/src/runtime/meta/generated/provider_slot_contract_manifest.json"
HEADER = ROOT / "include/nyrt_dynamic_text_scan_v1.h"
PYTHON_PROJECTION = ROOT / "src/llvm_py/builders/dynamic_v2_text_scan_export_facts.py"


def load_projection():
    spec = importlib.util.spec_from_file_location("text_scan_export_facts", PYTHON_PROJECTION)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def macro(name: str) -> str:
    text = HEADER.read_text(encoding="utf-8")
    match = re.search(rf"^#define {re.escape(name)} (.+)$", text, re.MULTILINE)
    if match is None:
        raise AssertionError(f"missing C macro: {name}")
    return match.group(1).strip()


def macro_uint(name: str) -> int:
    value = macro(name)
    match = re.fullmatch(r"UINT32_C\((\d+)\)", value)
    if match is None:
        raise AssertionError(f"C macro is not a fixed uint32: {name}={value}")
    return int(match.group(1))


class TextScanExportProjectionTests(unittest.TestCase):
    def test_manifest_has_two_roles_without_result_or_effect_redefinition(self) -> None:
        manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
        self.assertEqual(manifest["role_count"], 2)
        self.assertEqual(
            [(row["core_op"], row["arity"]) for row in manifest["roles"]],
            [("StringSubstring", "2"), ("StringIndexOf", "1")],
        )
        for row in manifest["roles"]:
            self.assertNotIn("result_kind", row)
            self.assertNotIn("effect", row)

    def test_c_header_matches_python_projection(self) -> None:
        projection = load_projection()
        self.assertEqual(macro("HAKO_TEXT_SCAN_CONTRACT_ID"), '"hako.text.scan@1"')
        self.assertEqual(macro_uint("HAKO_TEXT_SCAN_ABI_REVISION"), projection.ABI_REVISION)
        self.assertEqual(
            macro_uint("HAKO_TEXT_SCAN_PROFILE_CODEPOINT_CLAMPED"),
            projection.PROFILE_CODEPOINT_CLAMPED,
        )
        self.assertEqual(macro_uint("HAKO_TEXT_SCAN_ENTRY_SUBSTRING"), projection.ENTRY_SUBSTRING)
        self.assertEqual(macro_uint("HAKO_TEXT_SCAN_ENTRY_INDEX_OF"), projection.ENTRY_INDEX_OF)
        self.assertEqual(macro_uint("HAKO_TEXT_SCAN_ENTRY_COUNT"), len(projection.EXPORT_FACTS))
        self.assertEqual(
            macro_uint("HAKO_TEXT_SCAN_SUBSTRING_ARITY"),
            projection.EXPORT_FACTS[0]["arity"],
        )
        self.assertEqual(
            macro_uint("HAKO_TEXT_SCAN_INDEX_OF_ARITY"),
            projection.EXPORT_FACTS[1]["arity"],
        )
        self.assertEqual(
            macro_uint("HAKO_TEXT_SCAN_SUBSTRING_RECEIVER_LANE"),
            projection.EXPORT_FACTS[0]["receiver_lane"],
        )
        self.assertEqual(
            macro_uint("HAKO_TEXT_SCAN_INDEX_OF_RECEIVER_LANE"),
            projection.EXPORT_FACTS[1]["receiver_lane"],
        )

        self.assertEqual(
            [fact["symbol"] for fact in projection.EXPORT_FACTS],
            ["hako.text.scan.substring.v1", "hako.text.scan.index_of.v1"],
        )
        self.assertEqual(
            macro("HAKO_TEXT_SCAN_SYMBOL_SUBSTRING"),
            '"hako.text.scan.substring.v1"',
        )
        self.assertEqual(
            macro("HAKO_TEXT_SCAN_SYMBOL_INDEX_OF"),
            '"hako.text.scan.index_of.v1"',
        )
        self.assertEqual(
            macro_uint("HAKO_TEXT_SCAN_VALUE_HOST_HANDLE"),
            projection.VALUE_HOST_HANDLE,
        )
        self.assertEqual(
            macro_uint("HAKO_TEXT_SCAN_VALUE_IMMEDIATE_I64"),
            projection.VALUE_IMMEDIATE_I64,
        )
        self.assertEqual(
            macro_uint("HAKO_TEXT_SCAN_LEASE_NONE"),
            projection.LEASE_NONE,
        )
        self.assertEqual(
            macro_uint("HAKO_TEXT_SCAN_LEASE_END_AUTHORIZED"),
            projection.LEASE_END_AUTHORIZED,
        )

    def test_strict_lanes_and_leases_are_fixed(self) -> None:
        projection = load_projection()
        substring, index_of = projection.EXPORT_FACTS
        self.assertEqual(substring["receiver_lane"], projection.VALUE_HOST_HANDLE)
        self.assertEqual(index_of["receiver_lane"], projection.VALUE_HOST_HANDLE)
        self.assertEqual(substring["argument_lanes"], (projection.VALUE_IMMEDIATE_I64,) * 2)
        self.assertEqual(substring["result_lane"], projection.VALUE_HOST_HANDLE)
        self.assertEqual(substring["lease"], projection.LEASE_END_AUTHORIZED)
        self.assertEqual(index_of["argument_lanes"], (projection.VALUE_HOST_HANDLE,))
        self.assertEqual(index_of["result_lane"], projection.VALUE_IMMEDIATE_I64)
        self.assertEqual(index_of["lease"], projection.LEASE_NONE)


if __name__ == "__main__":
    unittest.main()
