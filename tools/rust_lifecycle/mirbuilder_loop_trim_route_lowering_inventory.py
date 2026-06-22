#!/usr/bin/env python3
"""Inventory the Loop / trim route lowering decision."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from context_fact_extraction import require


ROOT = Path(__file__).resolve().parents[2]
TASK_ORDER = ROOT / "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
TRIM_INVENTORY = ROOT / "docs/development/current/main/phases/phase-296x/296x-1436-TRIM-ROUTE-LOWERING-INVENTORY-001.md"
READINESS_GATE = ROOT / "docs/development/current/main/phases/phase-296x/296x-1458-TRIM-ROUTE-LOWERING-READINESS-GATE-001.md"
READINESS_INVENTORY = ROOT / "docs/development/current/main/phases/phase-296x/296x-1460-TRIM-ROUTE-LOWERING-READINESS-INTEGRATION-INVENTORY-001.md"
ROUTE_BOUNDARY_PROBE = ROOT / "docs/development/current/main/phases/phase-296x/296x-1462-ROUTE-BOUNDARY-TRIM-READINESS-INTEGRATION-PROBE-001.md"
ROUTE_BOUNDARY_OWNER = ROOT / "docs/development/current/main/phases/phase-296x/296x-1463-POST-ROUTE-BOUNDARY-TRIM-READINESS-PROBE-OWNER-SELECTION-001.md"
REFERENCE = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/loop-trim-route-lowering-v0.json"


def inventory_loop_trim_route_lowering() -> dict[str, Any]:
    task_order = TASK_ORDER.read_text()
    trim_inventory = TRIM_INVENTORY.read_text()
    readiness_gate = READINESS_GATE.read_text()
    readiness_inventory = READINESS_INVENTORY.read_text()
    route_boundary_probe = ROUTE_BOUNDARY_PROBE.read_text()
    route_boundary_owner = ROUTE_BOUNDARY_OWNER.read_text()

    require("34. `Loop / trim route lowering`" in task_order, "Loop / trim route lowering row missing")
    require("Status: landed." in task_order, "Loop / trim route lowering row is not marked as landed")
    require("trim_route_lowering_boundary_documented=1" in trim_inventory, "trim route lowering inventory missing boundary documentation")
    require("readiness_gate_exists=1" in readiness_gate, "trim readiness gate missing")
    require("readiness_integration_inventory=1" in readiness_inventory, "trim readiness integration inventory missing")
    require("route_boundary_readiness_probe_exists=1" in route_boundary_probe, "route boundary readiness probe missing")
    require("Status: parked" in route_boundary_owner, "route boundary owner selection is not parked")

    return {
        "schema_version": 0,
        "kind": "MirBuilderLoopTrimRouteLoweringInventory",
        "subject": "Loop / trim route lowering",
        "source": {
            "trim_route_lowering_inventory": "docs/development/current/main/phases/phase-296x/296x-1436-TRIM-ROUTE-LOWERING-INVENTORY-001.md",
            "trim_route_lowering_readiness_gate": "docs/development/current/main/phases/phase-296x/296x-1458-TRIM-ROUTE-LOWERING-READINESS-GATE-001.md",
            "trim_route_lowering_readiness_integration_inventory": "docs/development/current/main/phases/phase-296x/296x-1460-TRIM-ROUTE-LOWERING-READINESS-INTEGRATION-INVENTORY-001.md",
            "route_boundary_probe": "docs/development/current/main/phases/phase-296x/296x-1462-ROUTE-BOUNDARY-TRIM-READINESS-INTEGRATION-PROBE-001.md",
            "route_boundary_owner_selection": "docs/development/current/main/phases/phase-296x/296x-1463-POST-ROUTE-BOUNDARY-TRIM-READINESS-PROBE-OWNER-SELECTION-001.md",
            "task_order": "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md",
        },
        "current_contract": "inventory_only",
        "decision": [
            "keep trim route lowering parked until a concrete trim fixture is selected",
            "keep route-boundary readiness and boundary probes separate from executable lowering",
            "do not select route or nightly rustc adapter",
        ],
        "supporting_evidence": [
            "trim route lowering inventory documents the boundary and no executable lowering",
            "readiness gate exists and denies missing trim helper, invalid trim metadata, and missing condition binding identity",
            "route-boundary probe consumes CarrierInfo and condition_bindings without backend lowering",
            "owner selection is parked pending a concrete trim fixture",
        ],
        "open_questions": [
            "Which concrete trim fixture should be selected before executable lowering?",
            "Should loop/trim lowering remain separate from promoted carrier identity and join_id?",
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

    report = inventory_loop_trim_route_lowering()
    if args.check_reference:
        expected = json.loads(REFERENCE.read_text())
        require(report == expected, "loop trim route lowering inventory differs from reference fixture")
    if args.emit_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0

    print("output_contract=rust-mirbuilder-loop-trim-route-lowering-v0")
    print("loop_trim_route_lowering_recorded=1")
    print("subject=Loop / trim route lowering")
    print("route_selection=0")
    print("nightly_rustc_adapter=0")
    print("decision=inventory_only")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
