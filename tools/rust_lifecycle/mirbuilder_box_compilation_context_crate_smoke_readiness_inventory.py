#!/usr/bin/env python3
"""Inventory the remaining easy-tier smoke readiness before crate smoke."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from context_fact_extraction import require, require_current_task_pointer, require_not_bundle_mode


ROOT = Path(__file__).resolve().parents[2]
TASK_ORDER = ROOT / "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CURRENT_TASK = ROOT / "CURRENT_TASK.md"
CURRENT_STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
NOW = ROOT / "docs/development/current/main/10-Now.md"
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
GENERATED = ROOT / "lang/generated/rust_derived/hakorune_mir_builder"
SOURCE = ROOT / "crates/hakorune_mir_builder/src/context.rs"
REFERENCE = FIXTURES / "box-compilation-context-crate-smoke-readiness-v0.json"


def _require_present(paths: list[Path]) -> None:
    for path in paths:
        require(path.exists(), f"missing required path: {path.relative_to(ROOT)}")


def inventory_box_compilation_context_crate_smoke_readiness() -> dict[str, Any]:
    task_order = TASK_ORDER.read_text()
    current_task = CURRENT_TASK.read_text()
    current_state = CURRENT_STATE.read_text()
    now = NOW.read_text()
    source = SOURCE.read_text()

    require("pub struct BoxCompilationContext" in source, "missing BoxCompilationContext struct")
    require("pub fn new() -> Self" in source, "missing BoxCompilationContext::new")
    require("pub fn is_empty(&self) -> bool" in source, "missing BoxCompilationContext::is_empty")
    require("pub fn size_info(&self)" in source, "missing BoxCompilationContext::size_info")

    require_current_task_pointer(
        current_task,
        "Inventory remaining easy-tier smoke readiness before crate smoke",
        "current task pointer not updated to crate-smoke readiness inventory",
    )
    require_not_bundle_mode(
        "BoxCompilationContext crate-smoke readiness inventory is now landed" in now,
        "dashboard pointer not updated to crate-smoke readiness inventory",
    )
    require_not_bundle_mode(
        "inventory-first crate smoke probe" in task_order,
        "task-order SSOT does not mention inventory-first crate smoke probe",
    )
    require_not_bundle_mode(
        "crate-level probe" in current_state,
        "current state summary does not mention crate-level probe",
    )

    _require_present(
        [
            FIXTURES / "box-compilation-context-facts-v0.json",
            FIXTURES / "box-compilation-context-plan-v0.json",
            FIXTURES / "box-compilation-context-oracle-v0.json",
            FIXTURES / "box-compilation-context-behavior-recipe-v0.json",
            FIXTURES / "box-compilation-context-derived-artifact-verifier-result-v0.json",
            GENERATED / "box_compilation_context.hako",
            GENERATED / "box_compilation_context.artifact.json",
            ROOT / "tools/checks/rust_lifecycle_box_compilation_context_facts_guard.sh",
            ROOT / "tools/checks/rust_lifecycle_box_compilation_context_plan_oracle_guard.sh",
            ROOT / "tools/checks/rust_lifecycle_box_compilation_context_derived_artifact_guard.sh",
            ROOT / "tools/checks/rust_lifecycle_box_compilation_context_derived_route_selection_guard.sh",
        ]
    )

    return {
        "schema_version": 0,
        "kind": "MirBuilderEasyTierSmokeReadinessInventory",
        "subject": "hakorune_mir_builder::context::BoxCompilationContext",
        "source": {
            "crate": "hakorune_mir_builder",
            "module": "crate::context",
            "source_path": "crates/hakorune_mir_builder/src/context.rs",
        },
        "current_landed_slice": "BoxCompilationContext_ctor_is_empty_only",
        "remaining_easy_tier_consultation_candidates": [
            "CoreContext",
            "TypeContext",
            "MetadataContext",
        ],
        "present": {
            "facts_fixture": 1,
            "plan_fixture": 1,
            "oracle_fixture": 1,
            "recipe_fixture": 1,
            "verifier_fixture": 1,
            "generated_hako": 1,
            "generated_manifest": 1,
        },
        "crate_level_probe_opened": 0,
        "nightly_rustc_adapter_opened": 0,
        "route_selection_opened": 0,
        "runtime_fallback_opened": 0,
        "next_move": "inventory remaining easy-tier smoke readiness before crate smoke",
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

    report = inventory_box_compilation_context_crate_smoke_readiness()
    if args.check_reference:
        expected = json.loads(REFERENCE.read_text())
        require(report == expected, "crate smoke readiness inventory differs from reference fixture")
    if args.emit_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0

    print("output_contract=rust-mirbuilder-box-compilation-context-crate-smoke-readiness-v0")
    print("subject=BoxCompilationContext")
    print("current_landed_slice=BoxCompilationContext_ctor_is_empty_only")
    print("crate_level_probe_opened=0")
    print("nightly_rustc_adapter_opened=0")
    print("route_selection_opened=0")
    print("runtime_fallback_opened=0")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
