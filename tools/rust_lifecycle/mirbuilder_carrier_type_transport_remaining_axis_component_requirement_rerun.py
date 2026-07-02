#!/usr/bin/env python3
"""Rerun carrier/type component requirement priority after inventory."""

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
    / "mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun-v0.json"
)

TOKEN = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-RERUN-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

INVENTORY = (
    FIXTURES
    / "mirbuilder-carrier-type-transport-remaining-axis-component-requirement-inventory-v0.json"
)
BASIS = (
    FIXTURES
    / "mirbuilder-carrier-type-transport-remaining-axis-component-requirement-basis-v0.json"
)
PRIORITY_RERUN = (
    FIXTURES / "mirbuilder-carrier-type-transport-remaining-axis-priority-rerun-v0.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def requirement_rerun_row(row: dict[str, Any]) -> dict[str, Any]:
    root = row.get("root_authority") or {}
    return {
        "requirement_id": row.get("requirement_id"),
        "candidate_axes": row.get("candidate_axes") or [],
        "requirement_kind": row.get("requirement_kind"),
        "inventory_state": row.get("inventory_state"),
        "component_scope_eligible": row.get("component_scope_eligible") is True,
        "guard_clean_authority": row.get("guard_clean_authority"),
        "evidence_inventory_completeness": row.get("evidence_inventory_completeness"),
        "root_authority": {
            "status": root.get("status"),
            "reason_token": root.get("reason_token"),
            "proof_sources": root.get("proof_sources") or [],
        },
        "proof_tuple_complete": False,
        "selection_eligible": False,
        "selected_next_card_if_root": row.get("selected_next_card_if_root"),
        "blocked_by": row.get("blocked_by") or [],
    }


def build_fixture() -> dict[str, Any]:
    inventory = read_json(INVENTORY)
    basis = read_json(BASIS)
    priority_rerun = read_json(PRIORITY_RERUN)

    requirement_rows = [
        requirement_rerun_row(row)
        for row in inventory.get("component_evidence_inventory_rows") or []
    ]
    candidate_axes = []
    by_axis: dict[str, list[dict[str, Any]]] = {}
    for row in requirement_rows:
        for axis in row["candidate_axes"]:
            by_axis.setdefault(axis, []).append(row)

    for row in inventory.get("candidate_axes") or []:
        axis = row.get("axis")
        requirements = by_axis.get(axis, [])
        candidate_axes.append(
            {
                "axis": axis,
                "component_requirement_ids": [
                    item["requirement_id"] for item in requirements
                ],
                "ready_component_requirement_count": 0,
                "root_component_requirement_count": 0,
                "component_requirement_complete": False,
                "axis_selection_eligible": False,
                "blocked_by": sorted(
                    {
                        reason
                        for item in requirements
                        for reason in (item.get("blocked_by") or [])
                    }
                ),
            }
        )

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeTransportRemainingAxisComponentRequirementRerunV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "remaining_axis_component_requirement_inventory": rel(INVENTORY),
            "remaining_axis_component_requirement_basis": rel(BASIS),
            "remaining_axis_priority_rerun_001": rel(PRIORITY_RERUN),
        },
        "provenance": {
            "remaining_axis_component_requirement_inventory_hash": sha256_file(
                INVENTORY
            ),
            "remaining_axis_component_requirement_basis_hash": sha256_file(BASIS),
            "remaining_axis_priority_rerun_001_hash": sha256_file(PRIORITY_RERUN),
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
            "ready_component_requirement_count": inventory.get("summary", {}).get(
                "ready_component_requirement_count"
            ),
            "root_component_requirement_count": inventory.get("summary", {}).get(
                "root_component_requirement_count"
            ),
            "basis_component_requirement_count": basis.get("summary", {}).get(
                "component_requirement_count"
            ),
            "priority_rerun_reason_token": priority_rerun.get("decision", {}).get(
                "reason_token"
            ),
        },
        "selector_rule": {
            "name": "CarrierTypeRemainingAxisComponentRequirementSelectorV1",
            "concrete_carrier_type_axis_selection": False,
            "component_specific_card_selection_allowed_if_exactly_one_root_requirement": True,
            "selection_requires_exactly_one_root_component_requirement": True,
            "tie_breaking_forbidden": True,
            "if_zero_root_requirements_keep_stopped": True,
            "if_multiple_root_requirements_keep_stopped": True,
            "forbidden_priority_sources": [
                "row_count",
                "source_path",
                "owner_name",
                "route_membership_alone",
                "return_type_string_mapping",
                "lexical_order",
                "coverage_percentage",
                "apparent_simplicity",
                "observed_subaxis_set",
                "hardcoded_carrier_axis_priority",
                "manual_axis_selection",
            ],
        },
        "component_requirement_rows": requirement_rows,
        "candidate_axes": candidate_axes,
        "summary": {
            "candidate_axis_count": len(candidate_axes),
            "component_requirement_count": len(requirement_rows),
            "accepted_component_evidence_source_count": 0,
            "ready_component_requirement_count": 0,
            "root_component_requirement_count": 0,
            "component_specific_card_selection_eligible_count": 0,
            "selection_eligible_component_requirement_count": 0,
            "concrete_carrier_type_axis_selection": 0,
        },
        "decision": {
            "kind": "KeepStopped",
            "reason_token": "NoCarrierTypeRemainingAxisRootComponentRequirement",
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
        print("mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
