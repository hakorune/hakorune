#!/usr/bin/env python3
"""Define the representative crate-smoke probe result contract."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from context_fact_extraction import require, require_current_task_pointer


ROOT = Path(__file__).resolve().parents[2]
CURRENT_TASK = ROOT / "CURRENT_TASK.md"
COMMAND_CONTRACT = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-crate-smoke-harness-command-contract-v0.json"
PROBE_INVENTORY = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-crate-smoke-probe-inventory-v0.json"
PROBE_OUTPUT_CONTRACT = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-crate-smoke-probe-output-contract-v0.json"
READINESS = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-crate-smoke-readiness-v0.json"
SELECTION = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-crate-smoke-selection-v0.json"
OWNER_SELECTION = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-crate-smoke-harness-owner-selection-v0.json"
HARNESS_DESIGN = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-crate-smoke-harness-design-v0.json"
REFERENCE = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-crate-smoke-probe-result-contract-v0.json"


def define_box_compilation_context_crate_smoke_probe_result_contract() -> dict[str, Any]:
    current_task = CURRENT_TASK.read_text()
    command_contract = json.loads(COMMAND_CONTRACT.read_text())
    probe_inventory = json.loads(PROBE_INVENTORY.read_text())
    probe_output_contract = json.loads(PROBE_OUTPUT_CONTRACT.read_text())
    readiness = json.loads(READINESS.read_text())
    selection = json.loads(SELECTION.read_text())
    owner = json.loads(OWNER_SELECTION.read_text())
    harness = json.loads(HARNESS_DESIGN.read_text())

    require_current_task_pointer(current_task, "Define representative BoxCompilationContext crate smoke probe result contract", "current task pointer not set to probe result contract")
    require(command_contract.get("current_contract") == "command_contract_only", "command contract fixture does not describe the thin wrapper")
    require(probe_inventory.get("current_contract") == "probe_inventory_only", "probe inventory fixture does not describe the representative surface")
    require(probe_output_contract.get("current_contract") == "probe_output_contract_only", "probe output contract fixture does not describe the representative output contract")
    require(readiness.get("current_landed_slice") == "BoxCompilationContext_ctor_is_empty_only", "readiness fixture does not describe the landed BoxCompilationContext slice")
    require(selection.get("crate_level_probe_candidate") == "BoxCompilationContext", "crate-smoke selection does not name BoxCompilationContext")
    require(owner.get("selected_next_owner") == "minimal crate smoke harness design", "owner selection does not name the minimal harness design")
    require(harness.get("current_contract") == "selection_only", "harness design fixture does not describe the landed thin wrapper")

    return {
        "schema_version": 0,
        "kind": "MirBuilderEasyTierCrateSmokeProbeResultContract",
        "subject": "hakorune_mir_builder::context::BoxCompilationContext",
        "current_landed_slice": "BoxCompilationContext_ctor_is_empty_only",
        "crate_level_probe_candidate": "BoxCompilationContext",
        "selected_next_owner": "representative crate smoke probe result contract",
        "current_contract": "probe_result_contract_only",
        "source": {
            "readiness_inventory": "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-crate-smoke-readiness-v0.json",
            "probe_selection": "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-crate-smoke-selection-v0.json",
            "owner_selection": "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-crate-smoke-harness-owner-selection-v0.json",
            "harness_design": "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-crate-smoke-harness-design-v0.json",
            "command_contract": "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-crate-smoke-harness-command-contract-v0.json",
            "probe_inventory": "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-crate-smoke-probe-inventory-v0.json",
            "probe_output_contract": "docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-crate-smoke-probe-output-contract-v0.json",
        },
        "probe_result": [
            "summary=ok",
            "crate_level_probe_opened=0",
            "nightly_rustc_adapter_opened=0",
            "route_selection_opened=0",
            "runtime_fallback_opened=0",
        ],
        "decision": [
            "define the representative probe result contract after the probe output contract is fixed",
            "keep the probe result bounded to the landed ctor + is_empty slice",
            "do not open route selection or the nightly rustc adapter",
        ],
        "harness_boundary": [
            "readiness inventory guard",
            "probe selection guard",
            "harness owner selection guard",
            "harness design guard",
            "command contract guard",
            "probe inventory guard",
            "probe output contract guard",
            "probe result contract guard",
        ],
        "supporting_evidence": [
            "the probe candidate is already fixed",
            "the minimal harness owner is already fixed",
            "the thin harness wrapper is already fixed",
            "the command contract is already fixed",
            "the probe surface is already fixed",
            "the probe output contract is already fixed",
            "crate-level probe remains unopened",
        ],
        "crate_level_probe_opened": 0,
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

    report = define_box_compilation_context_crate_smoke_probe_result_contract()
    if args.check_reference:
        expected = json.loads(REFERENCE.read_text())
        require(report == expected, "crate smoke probe result contract differs from reference fixture")
    if args.emit_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0

    print("output_contract=rust-mirbuilder-box-compilation-context-crate-smoke-probe-result-contract-v0")
    print("subject=BoxCompilationContext")
    print("crate_level_probe_candidate=BoxCompilationContext")
    print("selected_next_owner=representative crate smoke probe result contract")
    print("crate_level_probe_opened=0")
    print("nightly_rustc_adapter_opened=0")
    print("decision=probe_result_contract_only")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
