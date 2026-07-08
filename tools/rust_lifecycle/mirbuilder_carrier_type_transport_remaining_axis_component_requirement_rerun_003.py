#!/usr/bin/env python3
"""Rerun carrier/type component requirements after the scalar closeout basis."""

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
    / "mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun-003-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-"
    "REQUIREMENT-RERUN-003"
)
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
PREVIOUS_RERUN = (
    FIXTURES
    / "mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun-002-v0.json"
)
COMPONENT_BASIS = (
    FIXTURES
    / "mirbuilder-carrier-type-transport-remaining-axis-component-requirement-basis-v0.json"
)
SCALAR_CLOSEOUT_BASIS = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-map-load-i64-typed-direct-closeout-contract-basis-v0.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def selected_next_for(requirement_id: str, component_basis: dict[str, Any]) -> str:
    for row in component_basis.get("component_requirements") or []:
        if row.get("requirement_id") == requirement_id:
            return row.get("selected_next_card_if_root")
    raise SystemExit(f"missing component requirement: {requirement_id}")


def scalar_accepted_source(closeout: dict[str, Any]) -> dict[str, Any]:
    contract = closeout.get("contract") or {}
    target = closeout.get("target") or {}
    return {
        "source_kind": target.get("accepted_source_kind"),
        "closeout_contract_id": contract.get("contract_id"),
        "route_kind": contract.get("route_kind"),
        "return_shape": contract.get("return_shape"),
        "proof_function": contract.get("proof_function"),
        "value_demand": contract.get("value_demand"),
        "publication_policy": contract.get("publication_policy"),
        "all_rows_join_contract": contract.get("all_rows_join_contract"),
        "no_carrier_boundary_required_or_already_covered": contract.get(
            "no_carrier_boundary_required_or_already_covered"
        ),
        "proof_source_hash": sha256_file(SCALAR_CLOSEOUT_BASIS),
    }


def rerun_row(row: dict[str, Any], closeout: dict[str, Any]) -> dict[str, Any]:
    requirement_id = row.get("requirement_id")
    accepted_sources = []
    root_status = "Unproven"
    reason_token = row.get("root_authority_reason_token")
    proof_tuple_complete = False
    selection_eligible = False

    if requirement_id == "ScalarKnownCloseoutAuthority":
        accepted_sources = [scalar_accepted_source(closeout)]
        root_status = "Proven"
        reason_token = "MapLoadScalarI64TypedDirectCloseoutContractAccepted"
        proof_tuple_complete = True
        selection_eligible = True

    return {
        "requirement_id": requirement_id,
        "candidate_axis": row.get("candidate_axis"),
        "accepted_source_kinds": row.get("accepted_source_kinds") or [],
        "accepted_sources": accepted_sources,
        "root_authority_status": root_status,
        "root_authority_reason_token": reason_token,
        "proof_tuple_complete": proof_tuple_complete,
        "selection_eligible": selection_eligible,
        "blocked_until": row.get("blocked_until") or [],
    }


def build_fixture() -> dict[str, Any]:
    previous = read_json(PREVIOUS_RERUN)
    component_basis = read_json(COMPONENT_BASIS)
    closeout = read_json(SCALAR_CLOSEOUT_BASIS)

    rows = [
        rerun_row(row, closeout)
        for row in previous.get("component_requirement_rows") or []
    ]
    root_rows = [row for row in rows if row.get("root_authority_status") == "Proven"]
    eligible_rows = [row for row in rows if row.get("selection_eligible") is True]
    selected_requirement = root_rows[0]["requirement_id"] if len(root_rows) == 1 else None
    selected_next = (
        selected_next_for(selected_requirement, component_basis)
        if selected_requirement
        else DESIGN_STOP
    )

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeTransportRemainingAxisComponentRequirementRerunV3",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "previous_rerun": rel(PREVIOUS_RERUN),
            "component_requirement_basis": rel(COMPONENT_BASIS),
            "scalar_known_map_load_i64_typed_direct_closeout_contract_basis": rel(
                SCALAR_CLOSEOUT_BASIS
            ),
        },
        "provenance": {
            "previous_rerun_hash": sha256_file(PREVIOUS_RERUN),
            "component_requirement_basis_hash": sha256_file(COMPONENT_BASIS),
            "scalar_known_map_load_i64_typed_direct_closeout_contract_basis_hash": sha256_file(
                SCALAR_CLOSEOUT_BASIS
            ),
        },
        "selector_rule": {
            "name": "CarrierTypeRemainingAxisComponentRequirementSelectorV3",
            "selection_requires_exactly_one_root_component_requirement": True,
            "if_zero_root_requirements_keep_stopped": True,
            "if_multiple_root_requirements_keep_stopped": True,
            "component_specific_card_selection_allowed_if_exactly_one_root_requirement": True,
            "concrete_carrier_type_axis_selection": False,
            "tie_breaking_forbidden": True,
            "basis_source_materialization_required": True,
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
        "component_requirement_rows": rows,
        "summary": {
            "component_requirement_count": len(rows),
            "accepted_component_evidence_source_count": sum(
                len(row.get("accepted_sources") or []) for row in rows
            ),
            "component_authority_source_count": len(root_rows),
            "root_component_requirement_count": len(root_rows),
            "selection_eligible_component_requirement_count": len(eligible_rows),
            "component_specific_card_selection_eligible_count": 1
            if len(root_rows) == 1
            else 0,
            "concrete_carrier_type_axis_selection": 0,
        },
        "decision": {
            "kind": "SelectComponentSpecificCard"
            if len(root_rows) == 1
            else "KeepStopped",
            "reason_token": "ExactlyOneCarrierTypeComponentRequirementRoot"
            if len(root_rows) == 1
            else "NoCarrierTypeComponentEvidenceSourceAuthority",
            "selected_carrier_type_axis": None,
            "selected_component_requirement": selected_requirement,
            "selected_next_card": selected_next,
        },
        "claims": {
            "accepted_typed_direct_closeout_contract_materialized": 1,
            "scalar_known_closeout_authority_accepted_root": 1
            if selected_requirement == "ScalarKnownCloseoutAuthority"
            else 0,
            "component_specific_card_selection": 1 if selected_requirement else 0,
            "concrete_carrier_type_axis_selection": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
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
        print("mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun-003 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
