#!/usr/bin/env python3
"""Inventory the ConstructorLifecycleMismatch decision."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from context_fact_extraction import require


ROOT = Path(__file__).resolve().parents[2]
TASK_ORDER = ROOT / "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CONSTRUCTOR_BIRTH = ROOT / "docs/development/current/main/design/constructor-birth-new-lifecycle-ssot.md"
ORDERED_MAP_BOUNDARY = ROOT / "docs/development/current/main/design/ordered-map-box-boundary-ssot.md"
BOX_COMPILATION_CONTEXT_PLAN = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-plan-v0.json"
BOX_COMPILATION_CONTEXT_BEHAVIOR_RECIPE = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-behavior-recipe-v0.json"
BOX_COMPILATION_CONTEXT_VERIFIER_RESULT = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-derived-artifact-verifier-result-v0.json"
REFERENCE = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/constructor-lifecycle-mismatch-v0.json"


def inventory_constructor_lifecycle_mismatch() -> dict[str, Any]:
    task_order = TASK_ORDER.read_text()
    constructor_birth = CONSTRUCTOR_BIRTH.read_text()
    ordered_map_boundary = ORDERED_MAP_BOUNDARY.read_text()
    box_compilation_context_plan = BOX_COMPILATION_CONTEXT_PLAN.read_text()
    box_compilation_context_behavior_recipe = BOX_COMPILATION_CONTEXT_BEHAVIOR_RECIPE.read_text()
    box_compilation_context_verifier_result = json.loads(BOX_COMPILATION_CONTEXT_VERIFIER_RESULT.read_text())

    require("39. `ConstructorLifecycleMismatch`" in task_order, "ConstructorLifecycleMismatch row missing")
    require("Status: landed." in task_order, "ConstructorLifecycleMismatch row is not marked as landed")
    require("For each `new Counter(...)`, field initializers run before `birth(...)`." in constructor_birth, "constructor lifecycle doc missing field-initializer ordering")
    require("they belong to declaration-site stored field initializers, not to `birth`." in ordered_map_boundary, "ordered-map boundary missing declaration-site initializer rule")
    require("BoxCompilationContext::new" in box_compilation_context_plan, "BoxCompilationContext plan missing constructor entry")
    require("DefaultConstruct" in box_compilation_context_plan, "BoxCompilationContext plan missing DefaultConstruct")
    require("BoxCompilationContext.birth initializes three ordered maps" in box_compilation_context_behavior_recipe, "behavior recipe missing birth initialization note")
    require(box_compilation_context_verifier_result.get("transport_notes", {}).get("box_birth") == "three ordered maps", "verifier result missing box_birth transport note")

    return {
        "schema_version": 0,
        "kind": "MirBuilderConstructorLifecycleMismatchInventory",
        "subject": "Constructor lifecycle mismatch around BoxCompilationContext and OrderedMapBox",
        "source": {
            "task_order": "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md",
            "constructor_birth_new_lifecycle": "docs/development/current/main/design/constructor-birth-new-lifecycle-ssot.md",
            "ordered_map_boundary": "docs/development/current/main/design/ordered-map-box-boundary-ssot.md",
            "box_compilation_context_plan": "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-plan-v0.json",
            "box_compilation_context_behavior_recipe": "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-behavior-recipe-v0.json",
            "box_compilation_context_verifier_result": "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-derived-artifact-verifier-result-v0.json",
        },
        "current_contract": "inventory_only",
        "decision": [
            "keep ConstructorLifecycleMismatch parked until a dedicated field-initializer-vs-birth contract is named",
            "keep declaration-site stored field initializers separate from birth-time constructor logic",
            "do not select route or nightly rustc adapter",
        ],
        "supporting_evidence": [
            "field initializers run before birth",
            "OrderedMapBox internal arrays belong to declaration-site stored field initializers, not to `birth`.",
            "BoxCompilationContext.birth initializes three ordered maps.",
            "box_birth is recorded as three ordered maps.",
        ],
        "open_questions": [
            "Should BoxCompilationContext keep its current birth-time initialization as compatibility residue, or move the simple defaults into stored field initializers?",
            "Should constructor lifecycle remain separate from route selection until a dedicated contract lands?",
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

    report = inventory_constructor_lifecycle_mismatch()
    if args.check_reference:
        expected = json.loads(REFERENCE.read_text())
        require(report == expected, "constructor lifecycle mismatch inventory differs from reference fixture")
    if args.emit_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0

    print("output_contract=rust-mirbuilder-constructor-lifecycle-mismatch-v0")
    print("constructor_lifecycle_mismatch_recorded=1")
    print("subject=Constructor lifecycle mismatch around BoxCompilationContext and OrderedMapBox")
    print("route_selection=0")
    print("nightly_rustc_adapter=0")
    print("decision=inventory_only")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
