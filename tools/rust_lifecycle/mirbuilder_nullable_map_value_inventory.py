#!/usr/bin/env python3
"""Inventory the NullableMapValue decision."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from context_fact_extraction import require


ROOT = Path(__file__).resolve().parents[2]
TASK_ORDER = ROOT / "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
OPTION_POLICY = ROOT / "docs/development/current/main/design/hako-option-null-no-match-policy-ssot.md"
ENUM_SURFACE = ROOT / "docs/development/current/main/design/enum-sum-and-generic-surface-ssot.md"
NEGATIVE_CORPUS = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-negative-converter-fixtures-v0.json"
REFERENCE = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/nullable-map-value-v0.json"


def inventory_nullable_map_value() -> dict[str, Any]:
    task_order = TASK_ORDER.read_text()
    option_policy = OPTION_POLICY.read_text()
    enum_surface = ENUM_SURFACE.read_text()
    negative_corpus = json.loads(NEGATIVE_CORPUS.read_text())

    require("37. `NullableMapValue`" in task_order, "NullableMapValue row missing")
    require("Status: design stop." in task_order, "NullableMapValue row is not marked as a design stop")
    require("`null` is a language value, `Option<T>` is a public null-free optional value" in option_policy, "option policy missing null-free contract")
    require("`Option::None` is not `null`" in option_policy, "option policy missing none-not-null rule")
    require("`Option::Some(null)` is forbidden" in option_policy, "option policy missing some-null prohibition")
    require("`Option::Some(null)` / `Option::Some(void)` are forbidden" in enum_surface, "enum surface missing null-free option rule")
    require(any(case.get("id") == "todo_null_placeholder_emission" and case.get("status") == "green" for case in negative_corpus.get("cases", [])), "negative corpus missing todo_null_placeholder_emission green case")

    return {
        "schema_version": 0,
        "kind": "MirBuilderNullableMapValueInventory",
        "subject": "MirBuilder nullable map value boundary",
        "source": {
            "task_order": "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md",
            "option_policy": "docs/development/current/main/design/hako-option-null-no-match-policy-ssot.md",
            "enum_surface": "docs/development/current/main/design/enum-sum-and-generic-surface-ssot.md",
            "negative_converter_fixtures": "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-negative-converter-fixtures-v0.json",
        },
        "current_contract": "inventory_only",
        "decision": [
            "keep NullableMapValue parked until an explicit missing-vs-null carrier contract is named",
            "keep null-free Option and nullable map payload disambiguation separate",
            "do not select route or nightly rustc adapter",
        ],
        "supporting_evidence": [
            "null is a language value, Option<T> is a public null-free optional value",
            "Option::None is not null",
            "Option::Some(null) is forbidden",
            "TODO/null placeholder emission remains a separate negative fixture case",
        ],
        "open_questions": [
            "Should nullable map payloads use explicit Option<T> or a dedicated missing-value carrier?",
            "Should missing and explicit null remain distinct at the map boundary?",
        ],
        "stop_line": [
            "do_not_select_route=1",
            "do_not_open_nightly_rustc_adapter=1",
            "do_not_claim_mirbuilder_wide_conversion=1",
            "do_not_add_runtime_fallback=1",
        ],
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--emit-json", action="store_true")
    parser.add_argument("--check-reference", action="store_true")
    args = parser.parse_args()

    report = inventory_nullable_map_value()
    if args.check_reference:
        expected = json.loads(REFERENCE.read_text())
        require(report == expected, "nullable map value inventory differs from reference fixture")
    if args.emit_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0

    print("output_contract=rust-mirbuilder-nullable-map-value-v0")
    print("nullable_map_value_recorded=1")
    print("subject=MirBuilder nullable map value boundary")
    print("route_selection=0")
    print("nightly_rustc_adapter=0")
    print("decision=inventory_only")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
