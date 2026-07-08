#!/usr/bin/env python3
"""Rerun Write Set selection after the MapStoreI64 .hako adoption decision."""

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
    / "mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-post-adoption-rerun-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-"
    "POST-ADOPTION-RERUN-001"
)
NEXT_CARD = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-"
    "TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001"
)

MAPSTORE_I64_ADOPTION = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-hako-adoption-decision-v0.json"
)
SET_SPLIT_BASIS = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-set-surface-typed-value-split-basis-v0.json"
)
PUSH_CLOSEOUT = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-push-surface-direct-closeout-rerun-v0.json"
)
DELETE_CLOSEOUT = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-delete-surface-direct-closeout-rerun-v0.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    adoption = read_json(MAPSTORE_I64_ADOPTION)
    split = read_json(SET_SPLIT_BASIS)
    push_closeout = read_json(PUSH_CLOSEOUT)
    delete_closeout = read_json(DELETE_CLOSEOUT)

    adopted = adoption.get("adoption_decision") or {}
    mapstore_i64_adopted = adopted.get("hako_adopted") is True
    push_materialized = (
        push_closeout.get("claims", {}).get("write_push_surface_direct_closeout_materialized") == 1
    )
    delete_materialized = (
        delete_closeout.get("claims", {}).get("write_delete_surface_direct_closeout_materialized") == 1
    )

    candidate_rows = [
        {
            "candidate_id": "PushSurfacePolicy",
            "routes": ["ArrayAppendAny"],
            "hako_adopted": True,
            "direct_closeout_materialized": push_materialized,
            "basis_selection_eligible": False,
            "blocked_by": ["AlreadyScopedDirectCloseoutMaterialized"],
        },
        {
            "candidate_id": "DeleteSurfacePolicy",
            "routes": ["MapDeleteAny"],
            "hako_adopted": True,
            "direct_closeout_materialized": delete_materialized,
            "basis_selection_eligible": False,
            "blocked_by": ["AlreadyScopedDirectCloseoutMaterialized"],
        },
        {
            "candidate_id": "SetSurfacePolicy/MapStoreI64",
            "routes": ["MapStoreI64"],
            "hako_adopted": mapstore_i64_adopted,
            "direct_closeout_materialized": False,
            "typed_scalar_write_boundary": True,
            "any_write_boundary_opened": False,
            "basis_selection_eligible": mapstore_i64_adopted,
            "blocked_by": [] if mapstore_i64_adopted else ["NoHakoAdoptedWriteSubsurfacePilot"],
        },
        {
            "candidate_id": "SetSurfacePolicy/MapStoreAny",
            "routes": ["MapStoreAny"],
            "hako_adopted": False,
            "direct_closeout_materialized": False,
            "typed_scalar_write_boundary": False,
            "any_write_boundary_required": True,
            "basis_selection_eligible": False,
            "blocked_by": ["AnyWriteBoundaryRequired"],
        },
    ]

    eligible_rows = [row for row in candidate_rows if row["basis_selection_eligible"]]
    hako_adopted_rows = [row for row in candidate_rows if row["hako_adopted"]]

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownWriteSetMapStoreI64PostAdoptionRerunV1",
        "token": TOKEN,
        "input_state": {
            "write_set_mapstore_i64_adoption_decision": rel(MAPSTORE_I64_ADOPTION),
            "write_set_mapstore_i64_adoption_hash": sha256_file(MAPSTORE_I64_ADOPTION),
            "set_surface_typed_value_split_basis": rel(SET_SPLIT_BASIS),
            "set_surface_typed_value_split_basis_hash": sha256_file(SET_SPLIT_BASIS),
            "write_push_surface_direct_closeout_rerun": rel(PUSH_CLOSEOUT),
            "write_push_surface_direct_closeout_hash": sha256_file(PUSH_CLOSEOUT),
            "write_delete_surface_direct_closeout_rerun": rel(DELETE_CLOSEOUT),
            "write_delete_surface_direct_closeout_hash": sha256_file(DELETE_CLOSEOUT),
            "adoption_decision": adopted.get("decision"),
            "adopted_surface": adopted.get("adopted_surface"),
            "adopted_owner": adopted.get("adopted_owner"),
            "split_proof_axis": split.get("proof_axis"),
        },
        "candidate_surfaces": candidate_rows,
        "selector_rule": {
            "name": "WriteSetMapStoreI64PostAdoptionRerunSelectorV1",
            "basis_selection_allowed_after_exactly_one_hako_adopted_unmaterialized_scoped_surface": True,
            "already_materialized_scoped_closeouts_not_eligible": True,
            "any_write_boundary_not_eligible": True,
            "direct_closeout_materialization_allowed": False,
            "manual_subsurface_selection": False,
            "route_count_as_proof": False,
            "apparent_simplicity_as_proof": False,
            "accepted_read_contract_similarity_as_proof": False,
        },
        "summary": {
            "write_set_mapstore_i64_post_adoption_rerun": 1,
            "write_set_mapstore_i64_hako_adopted": 1,
            "hako_adopted_write_scoped_surface_count": len(hako_adopted_rows),
            "basis_selection_eligible_surface_count": len(eligible_rows),
            "selected_scoped_surface_count": 1 if len(eligible_rows) == 1 else 0,
            "selected_scoped_surface": eligible_rows[0]["candidate_id"] if len(eligible_rows) == 1 else None,
            "mapstore_any_deferred": 1,
            "any_write_boundary_opened": 0,
            "write_set_mapstore_i64_direct_closeout_materialized": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectWriteSetMapStoreI64TypedDirectCloseoutContractBasis"
            if len(eligible_rows) == 1
            else "KeepStopped",
            "reason_token": "ExactlyOneHakoAdoptedSetMapStoreI64PilotNeedsScopedCloseout"
            if len(eligible_rows) == 1
            else "NoExactlyOneHakoAdoptedSetMapStoreI64PilotNeedsScopedCloseout",
            "selected_scoped_surface": eligible_rows[0]["candidate_id"] if len(eligible_rows) == 1 else None,
            "selected_next_card": NEXT_CARD
            if len(eligible_rows) == 1
            else "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "claims": {
            "write_set_mapstore_i64_post_adoption_rerun": 1,
            "write_set_mapstore_i64_hako_adopted": 1,
            "hako_adopted_write_scoped_surface_count": len(hako_adopted_rows),
            "basis_selection_eligible_surface_count": len(eligible_rows),
            "write_scoped_surface_selected": 1 if len(eligible_rows) == 1 else 0,
            "mapstore_any_deferred": 1,
            "any_write_boundary_opened": 0,
            "write_set_mapstore_i64_direct_closeout_materialized": 0,
            "write_direct_closeout_materialized": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "component_specific_direct_contract_materialized": 0,
            "source_selfhost_claim": 0,
            "new_route_authority": 0,
            "behavior_change": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "manual_subsurface_selection": 0,
            "route_count_as_proof": 0,
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
        print("mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-post-adoption-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
