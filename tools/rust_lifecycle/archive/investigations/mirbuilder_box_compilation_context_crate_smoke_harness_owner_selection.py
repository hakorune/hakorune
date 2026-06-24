#!/usr/bin/env python3
"""Select the minimal crate-smoke harness owner after BoxCompilationContext probe selection."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from context_fact_extraction import require, require_current_task_pointer


ROOT = Path(__file__).resolve().parents[2]
CURRENT_TASK = ROOT / "CURRENT_TASK.md"
READINESS = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-crate-smoke-readiness-v0.json"
SELECTION = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-crate-smoke-selection-v0.json"
REFERENCE = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-crate-smoke-harness-owner-selection-v0.json"


def select_box_compilation_context_crate_smoke_harness_owner() -> dict[str, Any]:
    current_task = CURRENT_TASK.read_text()
    readiness = json.loads(READINESS.read_text())
    selection = json.loads(SELECTION.read_text())

    require_current_task_pointer(current_task, "Select minimal BoxCompilationContext crate smoke harness owner", "current task pointer not set to crate-smoke harness owner selection")
    require(readiness.get("current_landed_slice") == "BoxCompilationContext_ctor_is_empty_only", "readiness fixture does not describe the landed BoxCompilationContext slice")
    require(selection.get("selected_next_probe") == "BoxCompilationContext", "crate-smoke selection does not name BoxCompilationContext")
    require(selection.get("crate_level_probe_opened") == 0 if "crate_level_probe_opened" in selection else True, "crate-level probe already opened in selection fixture")

    return {
        "schema_version": 0,
        "kind": "MirBuilderEasyTierCrateSmokeHarnessOwnerSelection",
        "subject": "hakorune_mir_builder::context::BoxCompilationContext",
        "current_landed_slice": "BoxCompilationContext_ctor_is_empty_only",
        "source": {
            "crate": "hakorune_mir_builder",
            "module": "crate::context",
            "source_path": "crates/hakorune_mir_builder/src/context.rs",
        },
        "current_contract": "selection_only",
        "crate_level_probe_candidate": "BoxCompilationContext",
        "crate_level_probe_opened": 0,
        "selected_next_owner": "minimal crate smoke harness design",
        "owner_scope": "BoxCompilationContext_ctor_is_empty_only",
        "decision": [
            "select the minimal crate smoke harness design as the next owner",
            "keep the harness bounded to the landed BoxCompilationContext slice",
            "do not open route selection or the nightly rustc adapter",
        ],
        "supporting_evidence": [
            "the crate-level probe candidate is already fixed",
            "the landed slice remains bounded to constructor plus is_empty",
            "crate-level probe remains unopened",
        ],
        "nightly_rustc_adapter_opened": 0,
        "route_selection_opened": 0,
        "runtime_fallback_opened": 0,
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

    report = select_box_compilation_context_crate_smoke_harness_owner()
    if args.check_reference:
        expected = json.loads(REFERENCE.read_text())
        require(report == expected, "crate smoke harness owner selection differs from reference fixture")
    if args.emit_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0

    print("output_contract=rust-mirbuilder-box-compilation-context-crate-smoke-harness-owner-selection-v0")
    print("subject=BoxCompilationContext")
    print("selected_next_owner=minimal crate smoke harness design")
    print("owner_scope=BoxCompilationContext_ctor_is_empty_only")
    print("route_selection=0")
    print("nightly_rustc_adapter=0")
    print("decision=selection_only")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
