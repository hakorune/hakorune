#!/usr/bin/env python3
"""Rerun remaining Write sub-surfaces after the Delete scoped closeout."""

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
    / "mirbuilder-carrier-type-scalar-known-write-remaining-subsurface-post-delete-closeout-rerun-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-REMAINING-SUBSURFACE-"
    "POST-DELETE-CLOSEOUT-RERUN-001"
)
NEXT_CARD = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

DELETE_CLOSEOUT = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-delete-surface-direct-closeout-rerun-v0.json"
)
WRITE_RESULT_POLICY = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-result-policy-rerun-v0.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def set_surface_row(policy: dict[str, Any]) -> dict[str, Any]:
    for row in policy.get("write_subsurfaces") or policy.get("subsurfaces") or []:
        if row.get("subsurface_id") == "SetSurfacePolicy":
            return row
    # The older policy fixture stores the normalized subsurfaces under policy_summary.
    for row in policy.get("policy_summary", {}).get("subsurfaces") or []:
        if row.get("subsurface_id") == "SetSurfacePolicy":
            return row
    return {
        "subsurface_id": "SetSurfacePolicy",
        "routes": ["MapStoreI64", "MapStoreAny"],
        "normalized_result_class": "NoneResult",
        "publication_class": "NonePublication",
        "mutation_class": "MutatesReceiverOrContainer",
    }


def build_fixture() -> dict[str, Any]:
    delete_closeout = read_json(DELETE_CLOSEOUT)
    policy = read_json(WRITE_RESULT_POLICY)
    set_row = set_surface_row(policy)

    remaining = {
        "subsurface_id": "SetSurfacePolicy",
        "routes": set_row.get("routes") or ["MapStoreI64", "MapStoreAny"],
        "normalized_result_class": set_row.get("normalized_result_class", "NoneResult"),
        "publication_class": set_row.get("publication_class", "NonePublication"),
        "mutation_class": set_row.get("mutation_class", "MutatesReceiverOrContainer"),
        "hako_adopted": False,
        "basis_selection_eligible": False,
        "blocked_by": [
            "NoHakoAdoptedWriteSubsurfacePilot",
            "NoConsultationApprovedSetSurfacePilotOrSplitProofAxis",
        ],
        "typed_non_typed_split_present": True,
        "candidate_routes_require_split_consultation": True,
    }

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownWriteRemainingSubsurfacePostDeleteCloseoutRerunV1",
        "token": TOKEN,
        "input_state": {
            "write_delete_surface_direct_closeout_rerun": rel(DELETE_CLOSEOUT),
            "write_delete_surface_direct_closeout_hash": sha256_file(DELETE_CLOSEOUT),
            "write_result_policy_rerun": rel(WRITE_RESULT_POLICY),
            "write_result_policy_rerun_hash": sha256_file(WRITE_RESULT_POLICY),
            "delete_closeout_decision": delete_closeout.get("decision", {}).get("kind"),
            "delete_closeout_selected_next_card": delete_closeout.get("decision", {}).get("selected_next_card"),
            "accepted_scoped_closeout_count": delete_closeout.get("summary", {}).get("accepted_scoped_closeout_count"),
        },
        "remaining_subsurfaces": [remaining],
        "selector_rule": {
            "name": "WriteRemainingSubsurfacePostDeleteCloseoutRerunSelectorV1",
            "if_no_hako_adopted_remaining_subsurface_keep_stopped": True,
            "next_pilot_requires_design_consultation": True,
            "set_surface_direct_pilot_selection_allowed": False,
            "set_surface_split_selection_allowed": False,
            "manual_subsurface_selection": False,
            "route_count_as_proof": False,
            "apparent_simplicity_as_proof": False,
            "accepted_read_contract_similarity_as_proof": False,
        },
        "summary": {
            "write_remaining_subsurface_post_delete_closeout_rerun": 1,
            "remaining_write_subsurface_count": 1,
            "remaining_subsurfaces": ["SetSurfacePolicy"],
            "hako_adopted_remaining_write_subsurface_count": 0,
            "basis_selection_eligible_subsurface_count": 0,
            "selected_write_subsurface_count": 0,
            "set_surface_policy_remaining": 1,
            "set_direct_hako_pilot_selected": 0,
            "set_split_consultation_required": 1,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "KeepStopped",
            "reason_token": "NoConsultationApprovedSetSurfacePilotOrSplitProofAxis",
            "recommended_consultation_topic": "WriteSetSurfacePolicyPilotOrSplitSelection",
            "selected_next_card": NEXT_CARD,
            "selected_subsurface": None,
        },
        "claims": {
            "write_remaining_subsurface_post_delete_closeout_rerun": 1,
            "remaining_write_subsurface_count": 1,
            "set_surface_policy_remaining": 1,
            "hako_adopted_remaining_write_subsurface_count": 0,
            "basis_selection_eligible_subsurface_count": 0,
            "selected_write_subsurface_count": 0,
            "set_direct_hako_pilot_selected": 0,
            "set_split_unnecessary": 0,
            "write_direct_closeout_materialized": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
            "hako_generation": 0,
            "new_route_authority": 0,
            "behavior_change": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "native_seed_materialization": 0,
            "new_python_semantic_projector": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
            "manual_subsurface_selection": 0,
            "row_count_as_proof": 0,
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
        print("mirbuilder-carrier-type-scalar-known-write-remaining-subsurface-post-delete-closeout-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
