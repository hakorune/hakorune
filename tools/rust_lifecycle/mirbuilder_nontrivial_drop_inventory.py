#!/usr/bin/env python3
"""Inventory the NonTrivialDrop decision."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from context_fact_extraction import require


ROOT = Path(__file__).resolve().parents[2]
TASK_ORDER = ROOT / "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
RUST_DROP_BOUNDARY = ROOT / "docs/development/current/main/design/rustc-semir-internal-adapter-boundary.md"
HAKO_PLAN = ROOT / "docs/development/current/main/design/hako-lifecycle-plan-vocab-v0.md"
BINDING_CONTEXT = ROOT / "docs/development/current/main/phases/phase-296x/296x-1387-MIRBUILDER-BINDING-CONTEXT-LIFECYCLE-PILOT-001.md"
VARIABLE_CONTEXT = ROOT / "docs/development/current/main/phases/phase-296x/296x-1391-VARIABLE-CONTEXT-LIFECYCLE-SIMPLE-MAP-PILOT-001.md"
REFERENCE = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/nontrivial-drop-v0.json"


def inventory_nontrivial_drop() -> dict[str, Any]:
    task_order = TASK_ORDER.read_text()
    rust_drop_boundary = RUST_DROP_BOUNDARY.read_text()
    hako_plan = HAKO_PLAN.read_text()
    binding_context = BINDING_CONTEXT.read_text()
    variable_context = VARIABLE_CONTEXT.read_text()

    require("35. `NonTrivialDrop`" in task_order, "NonTrivialDrop row missing")
    require("Status: landed." in task_order, "NonTrivialDrop row is not marked as landed")
    require("Drop may be erased only from a positive `TrivialMemory` fact." in rust_drop_boundary, "drop boundary missing positive TrivialMemory rule")
    require("do not erase Drop without TrivialMemory or verifier-approved cleanup" in hako_plan, "hako plan missing drop cleanup stop line")
    require("memory_drop_erased_only_with_TrivialMemory=green" in binding_context, "binding context pilot missing TrivialMemory drop result")
    require("memory_drop_erased_only_with_TrivialMemory=green" in variable_context, "variable context pilot missing TrivialMemory drop result")

    return {
        "schema_version": 0,
        "kind": "MirBuilderNonTrivialDropInventory",
        "subject": "MirBuilder Drop boundary",
        "source": {
            "task_order": "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md",
            "rust_drop_boundary": "docs/development/current/main/design/rustc-semir-internal-adapter-boundary.md",
            "hako_plan_vocab": "docs/development/current/main/design/hako-lifecycle-plan-vocab-v0.md",
            "binding_context_pilot": "docs/development/current/main/phases/phase-296x/296x-1387-MIRBUILDER-BINDING-CONTEXT-LIFECYCLE-PILOT-001.md",
            "variable_context_simple_map_pilot": "docs/development/current/main/phases/phase-296x/296x-1391-VARIABLE-CONTEXT-LIFECYCLE-SIMPLE-MAP-PILOT-001.md",
        },
        "current_contract": "inventory_only",
        "decision": [
            "keep NonTrivialDrop parked until a positive TrivialMemory or verifier-approved cleanup contract is named",
            "keep nontrivial Drop separate from the simple-map and snapshot/restore pilots that already require TrivialMemory",
            "do not select route or nightly rustc adapter",
        ],
        "supporting_evidence": [
            "Drop may be erased only from a positive TrivialMemory fact",
            "Hako lifecycle plan forbids erasing Drop without TrivialMemory or verifier-approved cleanup",
            "BindingContext and VariableContext simple-map pilots already close the memory-only Drop erase path under TrivialMemory",
            "No concrete nontrivial Drop owner is selected in the current task-order",
        ],
        "open_questions": [
            "Which future families need StructuralOwned, CustomSemanticDrop, HostResource, or Conditional Drop handling?",
            "Should nontrivial Drop remain parked until a later hard tier after ownership and borrow policy are named?",
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

    report = inventory_nontrivial_drop()
    if args.check_reference:
        expected = json.loads(REFERENCE.read_text())
        require(report == expected, "nontrivial drop inventory differs from reference fixture")
    if args.emit_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0

    print("output_contract=rust-mirbuilder-nontrivial-drop-v0")
    print("nontrivial_drop_recorded=1")
    print("subject=MirBuilder Drop boundary")
    print("route_selection=0")
    print("nightly_rustc_adapter=0")
    print("decision=inventory_only")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
