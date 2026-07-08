#!/usr/bin/env python3
"""Define the ScalarKnown write sub-surface priority basis."""

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
    / "mirbuilder-carrier-type-scalar-known-write-subsurface-priority-basis-v0.json"
)

TOKEN = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SUBSURFACE-PRIORITY-BASIS-001"
NEXT_CARD = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SUBSURFACE-PRIORITY-RERUN-001"

WRITE_POLICY_RERUN = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-result-policy-rerun-v0.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def candidate_basis_row(row: dict[str, Any]) -> dict[str, Any]:
    return {
        "subsurface_id": row.get("subsurface_id"),
        "routes": row.get("routes") or [],
        "route_count": row.get("route_count"),
        "route_count_as_proof": False,
        "normalized_result_class": row.get("normalized_result_class"),
        "publication_class": row.get("publication_class"),
        "mutation_class": row.get("mutation_class"),
        "future_direct_contract_split_allowed": row.get(
            "future_direct_contract_split_allowed"
        )
        is True,
        "scope_eligible": True,
        "stable_result_publication_contract": {
            "status": "NotEvaluatedAtBasis",
            "proof_sources": [],
        },
        "mutation_semantics_policy_ready": {
            "status": "NotEvaluatedAtBasis",
            "proof_sources": [],
        },
        "direct_contract_shape_ready": {
            "status": "NotEvaluatedAtBasis",
            "proof_sources": [],
        },
        "typed_value_boundary_ready": {
            "status": "NotEvaluatedAtBasis",
            "proof_sources": [],
        },
        "proof_tuple_complete": False,
        "selection_eligible": False,
    }


def build_fixture() -> dict[str, Any]:
    rerun = read_json(WRITE_POLICY_RERUN)
    candidates = [
        candidate_basis_row(row)
        for row in rerun.get("priority_candidates") or []
    ]

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownWriteSubsurfacePriorityBasisV1",
        "token": TOKEN,
        "input_state": {
            "write_result_policy_rerun": rel(WRITE_POLICY_RERUN),
            "write_result_policy_rerun_decision": rerun.get("decision", {}).get("kind"),
            "write_result_policy_rerun_reason_token": rerun.get("decision", {}).get(
                "reason_token"
            ),
            "write_result_policy_rerun_selected_next_card": rerun.get("decision", {}).get(
                "selected_next_card"
            ),
            "write_subsurface_candidate_count": rerun.get("summary", {}).get(
                "write_subsurface_candidate_count"
            ),
            "whole_direct_contract_candidate_count": rerun.get("summary", {}).get(
                "whole_direct_contract_candidate_count"
            ),
        },
        "provenance": {
            "write_result_policy_rerun_hash": sha256_file(WRITE_POLICY_RERUN),
        },
        "selector_rule": {
            "name": "WriteSubsurfacePriorityMechanicalSelectorV1",
            "basis_selects_write_subsurface": False,
            "rerun_may_select_subsurface_only_if_exactly_one_proof_tuple_complete": True,
            "if_zero_subsurface_proof_tuples_keep_stopped": True,
            "if_multiple_subsurface_proof_tuples_keep_stopped": True,
            "proof_tuple_complete_requires": [
                "scope_eligible",
                "stable_result_publication_contract",
                "mutation_semantics_policy_ready",
                "direct_contract_shape_ready",
                "typed_value_boundary_ready_or_not_required",
            ],
            "forbidden_priority_sources": [
                "route_count",
                "owner_name",
                "source_path",
                "route_membership_alone",
                "lexical_order",
                "coverage_percentage",
                "apparent_simplicity",
                "accepted_read_contract_similarity",
                "manual_subsurface_selection",
            ],
        },
        "candidate_subsurfaces": candidates,
        "allowed_proof_axes": {
            "stable_result_publication_contract": (
                "selection proof only if the sub-surface has a stable result and publication contract"
            ),
            "mutation_semantics_policy_ready": (
                "selection proof only if mutation semantics are scoped for that sub-surface"
            ),
            "direct_contract_shape_ready": (
                "selection proof only if the sub-surface can become a typed direct closeout contract"
            ),
            "typed_value_boundary_ready": (
                "selection proof only if typed value boundaries are ready or explicitly not required"
            ),
        },
        "summary": {
            "write_subsurface_priority_basis": 1,
            "candidate_write_subsurface_count": len(candidates),
            "basis_selection_eligible_subsurface_count": 0,
            "basis_selects_write_subsurface": 0,
            "write_direct_closeout_materialized": 0,
            "write_result_policy_ready": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectWriteSubsurfacePriorityRerun",
            "reason_token": "WriteSubsurfacePriorityBasisDefined",
            "selected_subsurface": None,
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "write_subsurface_priority_basis": 1,
            "basis_selects_write_subsurface": 0,
            "write_direct_closeout_materialized": 0,
            "write_result_policy_ready": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "component_specific_direct_contract_materialized": 0,
            "hako_adoption": 0,
            "source_selfhost_claim": 0,
            "new_route_authority": 0,
            "behavior_change": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "new_python_semantic_projector": 0,
            "manual_subsurface_selection": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
            "route_count_as_proof": 0,
            "source_path_as_authority": 0,
            "owner_name_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "apparent_simplicity_as_proof": 0,
            "accepted_read_contract_similarity_as_proof": 0,
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
        print("mirbuilder-carrier-type-scalar-known-write-subsurface-priority-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
