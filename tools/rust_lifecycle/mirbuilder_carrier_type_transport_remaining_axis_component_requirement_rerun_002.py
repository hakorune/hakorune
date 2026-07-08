#!/usr/bin/env python3
"""Rerun carrier/type component requirement priority after source discovery."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = (
    FIXTURES
    / "mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun-002-v0.json"
)

TOKEN = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-RERUN-002"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

DISCOVERY_INVENTORY = (
    FIXTURES
    / "mirbuilder-carrier-type-transport-component-evidence-source-discovery-inventory-v0.json"
)
PREVIOUS_RERUN = (
    FIXTURES
    / "mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun-v0.json"
)
BASIS = (
    FIXTURES
    / "mirbuilder-carrier-type-transport-component-evidence-source-discovery-basis-v0.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def rerun_row(row: dict[str, Any]) -> dict[str, Any]:
    return {
        "requirement_id": row.get("requirement_id"),
        "candidate_axis": row.get("candidate_axis"),
        "accepted_source_kinds": row.get("accepted_source_kinds") or [],
        "accepted_sources": row.get("accepted_sources") or [],
        "root_authority_status": "Unproven",
        "root_authority_reason_token": row.get("reason_token"),
        "proof_tuple_complete": False,
        "selection_eligible": False,
        "blocked_until": row.get("blocked_until") or [],
    }


def build_fixture() -> dict[str, Any]:
    inventory = read_json(DISCOVERY_INVENTORY)
    previous_rerun = read_json(PREVIOUS_RERUN)
    basis = read_json(BASIS)

    requirement_rows = [
        rerun_row(row)
        for row in inventory.get("component_requirement_source_rows") or []
    ]

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeTransportRemainingAxisComponentRequirementRerunV2",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "component_evidence_source_discovery_inventory": rel(
                DISCOVERY_INVENTORY
            ),
            "component_requirement_rerun_001": rel(PREVIOUS_RERUN),
            "component_evidence_source_discovery_basis": rel(BASIS),
        },
        "provenance": {
            "component_evidence_source_discovery_inventory_hash": sha256_file(
                DISCOVERY_INVENTORY
            ),
            "component_requirement_rerun_001_hash": sha256_file(PREVIOUS_RERUN),
            "component_evidence_source_discovery_basis_hash": sha256_file(BASIS),
        },
        "previous_state": {
            "inventory_decision": inventory.get("decision", {}).get("kind"),
            "inventory_reason_token": inventory.get("decision", {}).get(
                "reason_token"
            ),
            "inventory_selected_next_card": inventory.get("decision", {}).get(
                "selected_next_card"
            ),
            "accepted_component_evidence_source_count": inventory.get(
                "summary", {}
            ).get("accepted_component_evidence_source_count"),
            "component_authority_source_count": inventory.get("summary", {}).get(
                "component_authority_source_count"
            ),
            "previous_rerun_reason_token": previous_rerun.get("decision", {}).get(
                "reason_token"
            ),
            "basis_allowed_source_kind_count": len(
                basis.get("allowed_evidence_source_kinds") or []
            ),
        },
        "selector_rule": {
            "name": "CarrierTypeRemainingAxisComponentRequirementSelectorV2",
            "selection_requires_exactly_one_root_component_requirement": True,
            "if_zero_root_requirements_keep_stopped": True,
            "if_multiple_root_requirements_keep_stopped": True,
            "component_specific_card_selection_allowed_if_exactly_one_root_requirement": True,
            "concrete_carrier_type_axis_selection": False,
            "tie_breaking_forbidden": True,
            "zero_root_does_not_open_parent_policy_lane": True,
            "zero_root_returns_to_design_consultation": True,
            "forbidden_priority_sources": [
                "row_count",
                "source_path",
                "owner_name",
                "route_membership_alone",
                "return_type_string_mapping",
                "lexical_order",
                "coverage_percentage",
                "observed_subaxis_set",
                "hardcoded_carrier_axis_priority",
                "manual_axis_selection",
            ],
        },
        "component_requirement_rows": requirement_rows,
        "summary": {
            "component_requirement_count": len(requirement_rows),
            "accepted_component_evidence_source_count": 0,
            "component_authority_source_count": 0,
            "root_component_requirement_count": 0,
            "selection_eligible_component_requirement_count": 0,
            "component_specific_card_selection_eligible_count": 0,
            "concrete_carrier_type_axis_selection": 0,
        },
        "decision": {
            "kind": "KeepStopped",
            "reason_token": "NoCarrierTypeComponentEvidenceSourceAuthority",
            "selected_carrier_type_axis": None,
            "selected_component_requirement": None,
            "selected_next_card": DESIGN_STOP,
        },
        "claims": {
            "source_selfhost_claim": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "accepted_typed_dependency_edge_materialized": 0,
            "component_specific_card_selection": 0,
            "concrete_carrier_type_axis_selection": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
            "hardcoded_carrier_axis_priority": 0,
            "row_count_as_proof": 0,
            "source_path_as_authority": 0,
            "owner_name_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "return_type_string_mapping_as_proof": 0,
            "observed_subaxis_set_as_proof": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun-002 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
