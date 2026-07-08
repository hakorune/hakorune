#!/usr/bin/env python3
"""Rerun the ScalarKnown write sub-surface priority basis."""

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
    / "mirbuilder-carrier-type-scalar-known-write-subsurface-priority-rerun-v0.json"
)

TOKEN = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SUBSURFACE-PRIORITY-RERUN-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

PRIORITY_BASIS = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-subsurface-priority-basis-v0.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def rerun_row(row: dict[str, Any]) -> dict[str, Any]:
    return {
        "subsurface_id": row.get("subsurface_id"),
        "routes": row.get("routes") or [],
        "scope_eligible": row.get("scope_eligible") is True,
        "stable_result_publication_contract": {
            "status": "Unproven",
            "proof_sources": [],
        },
        "mutation_semantics_policy_ready": {
            "status": "Unproven",
            "proof_sources": [],
        },
        "direct_contract_shape_ready": {
            "status": "Unproven",
            "proof_sources": [],
        },
        "typed_value_boundary_ready": {
            "status": "Unproven",
            "proof_sources": [],
        },
        "proof_tuple_complete": False,
        "selection_eligible": False,
        "blocked_by": [
            "NoStableResultPublicationContractProof",
            "NoMutationSemanticsPolicyReadinessProof",
            "NoDirectContractShapeReadinessProof",
            "NoTypedValueBoundaryReadinessProof",
        ],
    }


def build_fixture() -> dict[str, Any]:
    basis = read_json(PRIORITY_BASIS)
    rows = [rerun_row(row) for row in basis.get("candidate_subsurfaces") or []]

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownWriteSubsurfacePriorityRerunV1",
        "token": TOKEN,
        "input_state": {
            "write_subsurface_priority_basis": rel(PRIORITY_BASIS),
            "basis_decision": basis.get("decision", {}).get("kind"),
            "basis_selected_next_card": basis.get("decision", {}).get("selected_next_card"),
            "basis_selection_eligible_subsurface_count": basis.get("summary", {}).get(
                "basis_selection_eligible_subsurface_count"
            ),
        },
        "provenance": {
            "write_subsurface_priority_basis_hash": sha256_file(PRIORITY_BASIS),
        },
        "selector_rule": basis.get("selector_rule"),
        "candidate_subsurfaces": rows,
        "summary": {
            "write_subsurface_priority_rerun": 1,
            "write_subsurface_priority_basis_consumed": 1,
            "candidate_write_subsurface_count": len(rows),
            "proof_tuple_complete_subsurface_count": 0,
            "selection_eligible_subsurface_count": 0,
            "selected_write_subsurface_count": 0,
            "write_direct_closeout_materialized": 0,
            "write_result_policy_ready": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "KeepStopped",
            "reason_token": "NoWriteSubsurfacePriorityProofTuple",
            "selected_subsurface": None,
            "selected_next_card": DESIGN_STOP,
            "recommended_consultation_topic": "WriteSubsurfacePriorityProofAxis",
        },
        "claims": {
            "write_subsurface_priority_rerun": 1,
            "write_subsurface_priority_basis_consumed": 1,
            "write_subsurface_selected": 0,
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
        print("mirbuilder-carrier-type-scalar-known-write-subsurface-priority-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
