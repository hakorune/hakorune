#!/usr/bin/env python3
"""Inventory the CarrierSensitiveAlias proof decision."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from context_fact_extraction import require


ROOT = Path(__file__).resolve().parents[2]
TASK_ORDER = ROOT / "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
READINESS = (
    ROOT
    / "docs/development/current/main/phases/phase-296x/"
    / "296x-1527-VARIABLE-CONTEXT-CARRIER-DERIVED-ARTIFACT-READINESS-INVENTORY-001.md"
)
READ_VIEW_DECISION = (
    ROOT
    / "docs/development/current/main/phases/phase-296x/"
    / "296x-1551-RETURNED-READ-BORROW-READ-VIEW-DECISION-001.md"
)
REFERENCE = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "carrier-sensitive-alias-proof-v0.json"
)


def inventory_carrier_sensitive_alias_proof() -> dict[str, Any]:
    task_order = TASK_ORDER.read_text()
    readiness = READINESS.read_text()
    read_view = READ_VIEW_DECISION.read_text()

    require("32. `CarrierSensitiveAlias proof`" in task_order, "CarrierSensitiveAlias proof row missing")
    require("Status: landed." in task_order, "CarrierSensitiveAlias proof row is not marked as landed")
    require("carrier_sensitive_artifact_readiness=inventory_only" in readiness, "carrier readiness inventory missing inventory-only decision")
    require("read_borrow_ready=1" in readiness, "carrier readiness inventory missing read_borrow_ready")
    require("owned_transfer_ready=1" in readiness, "carrier readiness inventory missing owned_transfer_ready")
    require("NoReturnedAlias + OwnedReadSnapshotProjection" in read_view, "read-view decision missing current contract")

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierSensitiveAliasProof",
        "subject": "hakorune_mir_builder::variable_context::CarrierInfo carrier-sensitive consumers",
        "source": {
            "carrier_readiness_inventory": "docs/development/current/main/phases/phase-296x/296x-1527-VARIABLE-CONTEXT-CARRIER-DERIVED-ARTIFACT-READINESS-INVENTORY-001.md",
            "read_view_decision": "docs/development/current/main/phases/phase-296x/296x-1551-RETURNED-READ-BORROW-READ-VIEW-DECISION-001.md",
            "task_order": "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md",
        },
        "current_contract": "inventory_only",
        "decision": [
            "keep carrier-sensitive consumers parked until a separate hard-tier contract names the alias model",
            "keep read-only BorrowView and owned snapshot readiness separate",
            "do not select route or nightly rustc adapter",
        ],
        "supporting_evidence": [
            "carrier-sensitive consumers remain inventory-only in 296x-1527",
            "NoReturnedAlias + OwnedReadSnapshotProjection remains the current VariableContext read contract",
            "read_borrow_ready=1 and owned_transfer_ready=1 are already green",
        ],
        "open_questions": [
            "Does carrier extraction need a dedicated alias contract or remain owned-snapshot based?",
            "Where should join_id ownership be modeled relative to promoted_body_locals and trim_helper?",
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

    report = inventory_carrier_sensitive_alias_proof()
    if args.check_reference:
        expected = json.loads(REFERENCE.read_text())
        require(report == expected, "carrier-sensitive alias proof inventory differs from reference fixture")
    if args.emit_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0

    print("output_contract=rust-mirbuilder-carrier-sensitive-alias-proof-v0")
    print("carrier_sensitive_alias_proof_recorded=1")
    print("subject=CarrierInfo")
    print("route_selection=0")
    print("nightly_rustc_adapter=0")
    print("decision=inventory_only")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
