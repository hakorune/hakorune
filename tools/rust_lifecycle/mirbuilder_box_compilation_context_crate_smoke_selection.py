#!/usr/bin/env python3
"""Select the first crate-level probe candidate after BoxCompilationContext readiness inventory."""

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
READINESS = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-crate-smoke-readiness-v0.json"
REFERENCE = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-crate-smoke-selection-v0.json"


def select_box_compilation_context_crate_smoke() -> dict[str, Any]:
    task_order = TASK_ORDER.read_text()
    current_task = CURRENT_TASK.read_text()
    current_state = CURRENT_STATE.read_text()
    now = NOW.read_text()
    readiness = json.loads(READINESS.read_text())

    require_not_bundle_mode("Inventory remaining easy-tier smoke readiness before crate smoke" in task_order, "crate-smoke readiness row missing")
    require_current_task_pointer(current_task, "Select BoxCompilationContext crate smoke probe candidate", "current task pointer not set to crate-smoke selection")
    require_not_bundle_mode("BoxCompilationContext crate-smoke readiness inventory is now landed" in now, "dashboard pointer not set to crate-smoke readiness")
    require(readiness.get("current_landed_slice") == "BoxCompilationContext_ctor_is_empty_only", "readiness fixture does not describe the landed BoxCompilationContext slice")
    require(readiness.get("crate_level_probe_opened") == 0, "crate-level probe already opened in readiness fixture")

    return {
        "schema_version": 0,
        "kind": "MirBuilderEasyTierCrateSmokeSelection",
        "subject": "hakorune_mir_builder::context::BoxCompilationContext",
        "source": {
            "task_order": "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md",
            "readiness_inventory": "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-crate-smoke-readiness-v0.json",
        },
        "current_contract": "selection_only",
        "crate_level_probe_candidate": "BoxCompilationContext",
        "selected_next_probe": "BoxCompilationContext",
        "probe_scope": "BoxCompilationContext_ctor_is_empty_only",
        "decision": [
            "select BoxCompilationContext as the first crate-level probe candidate",
            "keep the probe bounded to the landed ctor + is_empty slice",
            "do not open route selection or the nightly rustc adapter",
        ],
        "supporting_evidence": [
            "the readiness inventory fixes BoxCompilationContext as the current crate-smoke boundary",
            "remaining easy-tier consultation candidates are explicit",
            "crate-level probe remains unopened",
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

    report = select_box_compilation_context_crate_smoke()
    if args.check_reference:
        expected = json.loads(REFERENCE.read_text())
        require(report == expected, "crate smoke selection differs from reference fixture")
    if args.emit_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0

    print("output_contract=rust-mirbuilder-box-compilation-context-crate-smoke-selection-v0")
    print("subject=BoxCompilationContext")
    print("selected_next_probe=BoxCompilationContext")
    print("probe_scope=BoxCompilationContext_ctor_is_empty_only")
    print("route_selection=0")
    print("nightly_rustc_adapter=0")
    print("decision=selection_only")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
