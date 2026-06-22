#!/usr/bin/env python3
"""Inventory the PHI and join_id lifecycle decision."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from context_fact_extraction import require


ROOT = Path(__file__).resolve().parents[2]
TASK_ORDER = ROOT / "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
JOIN_ID_INVENTORY = ROOT / "docs/development/current/main/phases/phase-296x/296x-1410-PHI-CARRIER-JOIN-ID-LIFECYCLE-PRODUCER-INVENTORY-001.md"
JOIN_ID_SELECTION = ROOT / "docs/development/current/main/phases/phase-296x/296x-1411-POST-JOIN-ID-PRODUCER-INVENTORY-OWNER-SELECTION-001.md"
PHI_CARRIER = ROOT / "docs/development/current/main/phases/phase-296x/296x-1408-PHI-CARRIER-LIFECYCLE-CONSUMER-INVENTORY-001.md"
TRIM_HELPER = ROOT / "docs/development/current/main/phases/phase-296x/296x-1424-TRIM-HELPER-CARRIER-LIFECYCLE-INVENTORY-001.md"
PROMOTED_BODY_LOCALS = ROOT / "docs/development/current/main/phases/phase-296x/296x-1428-PROMOTED-BODY-LOCALS-LIFECYCLE-INVENTORY-001.md"
REFERENCE = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/phi-and-join-id-lifecycle-v0.json"


def inventory_phi_and_join_id_lifecycle() -> dict[str, Any]:
    task_order = TASK_ORDER.read_text()
    join_id_inventory = JOIN_ID_INVENTORY.read_text()
    join_id_selection = JOIN_ID_SELECTION.read_text()
    phi_carrier = PHI_CARRIER.read_text()
    trim_helper = TRIM_HELPER.read_text()
    promoted_body_locals = PROMOTED_BODY_LOCALS.read_text()

    require("33. `PHI and join_id lifecycle`" in task_order, "PHI and join_id lifecycle row missing")
    require("Status: landed." in task_order, "PHI and join_id lifecycle row is not marked as landed")
    require("production_join_id_initializers=None_only" in join_id_inventory, "join_id inventory missing None-only production finding")
    require("selected_owner=B-lite" in join_id_selection, "join_id selection missing merge_from selection")
    require("CarrierVar.join_id producer/consumer boundary" in phi_carrier, "PHI carrier inventory missing join_id consumer boundary")
    require("trim_helper_lifecycle_owner_selected=0" in trim_helper, "trim_helper inventory missing parked ownership")
    require("promoted_body_locals_lifecycle_owner_selected=0" in promoted_body_locals, "promoted_body_locals inventory missing parked ownership")

    return {
        "schema_version": 0,
        "kind": "MirBuilderPhiAndJoinIdLifecycleInventory",
        "subject": "CarrierVar.join_id and PHI carrier lifecycle",
        "source": {
            "join_id_inventory": "docs/development/current/main/phases/phase-296x/296x-1410-PHI-CARRIER-JOIN-ID-LIFECYCLE-PRODUCER-INVENTORY-001.md",
            "join_id_selection": "docs/development/current/main/phases/phase-296x/296x-1411-POST-JOIN-ID-PRODUCER-INVENTORY-OWNER-SELECTION-001.md",
            "carrier_inventory": "docs/development/current/main/phases/phase-296x/296x-1408-PHI-CARRIER-LIFECYCLE-CONSUMER-INVENTORY-001.md",
            "trim_helper_inventory": "docs/development/current/main/phases/phase-296x/296x-1424-TRIM-HELPER-CARRIER-LIFECYCLE-INVENTORY-001.md",
            "promoted_body_locals_inventory": "docs/development/current/main/phases/phase-296x/296x-1428-PROMOTED-BODY-LOCALS-LIFECYCLE-INVENTORY-001.md",
            "task_order": "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md",
        },
        "current_contract": "inventory_only",
        "decision": [
            "keep CarrierVar.join_id parked as test vocabulary until a production producer is named",
            "keep PHI and join_id lifecycle separate from trim_helper, promoted_body_locals, and merge_from ownership",
            "do not select route or nightly rustc adapter",
        ],
        "supporting_evidence": [
            "production constructors initialize join_id as None",
            "join_id has no production Some(ValueId) producer",
            "merge_from is a real production mutation boundary, but join_id remains parked",
            "trim_helper and promoted_body_locals are independently inventoried",
        ],
        "open_questions": [
            "Does join_id need a dedicated production producer or remain parked as test vocabulary?",
            "Should future PHI consumers be named before any trim_helper/promoted_body_locals route work?",
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

    report = inventory_phi_and_join_id_lifecycle()
    if args.check_reference:
        expected = json.loads(REFERENCE.read_text())
        require(report == expected, "phi and join id lifecycle inventory differs from reference fixture")
    if args.emit_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0

    print("output_contract=rust-mirbuilder-phi-and-join-id-lifecycle-v0")
    print("phi_and_join_id_lifecycle_recorded=1")
    print("subject=CarrierVar.join_id")
    print("route_selection=0")
    print("nightly_rustc_adapter=0")
    print("decision=inventory_only")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
