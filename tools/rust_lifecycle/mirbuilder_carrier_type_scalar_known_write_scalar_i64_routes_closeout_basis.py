#!/usr/bin/env python3
"""Define the WriteScalarI64Routes closeout review basis."""

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
    / "mirbuilder-carrier-type-scalar-known-write-scalar-i64-routes-closeout-basis-v0.json"
)

TOKEN = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SCALAR-I64-ROUTES-CLOSEOUT-BASIS-001"
NEXT_CARD = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SCALAR-I64-ROUTES-CLOSEOUT-RERUN-001"

PUSH_CLOSEOUT = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-push-surface-direct-closeout-rerun-v0.json"
)
MAPSTORE_I64_CLOSEOUT = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-direct-closeout-rerun-v0.json"
)
MAPSTORE_ANY_CLOSEOUT = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-direct-closeout-rerun-v0.json"
)
DELETE_RETIRE_CARD = (
    ROOT
    / "docs/development/current/main/phases/phase-296x/"
    / "3353-MIRBUILDER-SCALAR-KNOWN-WRITE-DELETE-SURFACE-MIRROR-RETIRE-001.md"
)
DELETE_RETIRE_GUARD = (
    ROOT
    / "tools/checks/rust_lifecycle_mirbuilder_scalar_known_write_delete_surface_mirror_retire_guard.sh"
)
WRITE_SOURCE = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def materialized_contract(path: Path) -> dict[str, Any]:
    data = read_json(path)
    return data.get("materialized_contract") or {}


def build_fixture() -> dict[str, Any]:
    push = read_json(PUSH_CLOSEOUT)
    mapstore_i64 = read_json(MAPSTORE_I64_CLOSEOUT)
    mapstore_any = read_json(MAPSTORE_ANY_CLOSEOUT)

    scoped_contracts = [
        materialized_contract(PUSH_CLOSEOUT),
        materialized_contract(MAPSTORE_I64_CLOSEOUT),
        materialized_contract(MAPSTORE_ANY_CLOSEOUT),
    ]

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownWriteScalarI64RoutesCloseoutBasisV1",
        "token": TOKEN,
        "input_state": {
            "write_push_surface_closeout": rel(PUSH_CLOSEOUT),
            "write_push_surface_closeout_hash": sha256_file(PUSH_CLOSEOUT),
            "write_set_mapstore_i64_closeout": rel(MAPSTORE_I64_CLOSEOUT),
            "write_set_mapstore_i64_closeout_hash": sha256_file(MAPSTORE_I64_CLOSEOUT),
            "write_set_mapstore_any_closeout": rel(MAPSTORE_ANY_CLOSEOUT),
            "write_set_mapstore_any_closeout_hash": sha256_file(MAPSTORE_ANY_CLOSEOUT),
            "delete_surface_mirror_retire_card": rel(DELETE_RETIRE_CARD),
            "delete_surface_mirror_retire_card_hash": sha256_file(DELETE_RETIRE_CARD),
            "delete_surface_mirror_retire_guard": rel(DELETE_RETIRE_GUARD),
            "delete_surface_mirror_retire_guard_hash": sha256_file(DELETE_RETIRE_GUARD),
            "write_source": rel(WRITE_SOURCE),
            "write_source_hash": sha256_file(WRITE_SOURCE),
            "mapstore_any_selected_next": (mapstore_any.get("decision") or {}).get("selected_next_card"),
        },
        "write_surface_review": {
            "surface_id": "WriteScalarI64Routes",
            "scoped_direct_closeout_contract_count": len(scoped_contracts),
            "scoped_direct_closeout_contracts": scoped_contracts,
            "delete_surface_policy": {
                "subsurface_id": "DeleteSurfacePolicy/MapDeleteAny",
                "hako_mirror_retired": True,
                "lifecycle_artifacts_deleted": True,
                "rust_map_delete_route_preserved": True,
                "direct_closeout_materialized": False,
                "closeout_treatment": "RetiredUnconnectedMirrorLiveRustRoutePreserved",
            },
            "ready_for_rerun": True,
            "rerun_required_before_write_surface_closeout": True,
        },
        "selection_rule": {
            "name": "WriteScalarI64RoutesCloseoutBasisOnlyV1",
            "basis_only": True,
            "write_surface_closeout_requires_rerun": True,
            "delete_retire_treatment_must_remain_explicit": True,
            "delete_retire_does_not_count_as_hako_direct_closeout": True,
            "axis_closeout_forbidden_at_basis": True,
            "source_path_as_authority": False,
            "owner_name_as_proof": False,
            "row_count_as_proof": False,
            "route_count_as_proof": False,
            "route_membership_alone_as_proof": False,
        },
        "summary": {
            "write_scalar_i64_routes_closeout_basis": 1,
            "scoped_direct_closeout_contract_count": len(scoped_contracts),
            "push_surface_direct_closeout_materialized": int(
                push.get("summary", {}).get("write_push_surface_direct_closeout_materialized") == 1
            ),
            "set_mapstore_i64_direct_closeout_materialized": int(
                mapstore_i64.get("summary", {}).get("write_set_mapstore_i64_direct_closeout_materialized")
                == 1
            ),
            "set_mapstore_any_direct_closeout_materialized": int(
                mapstore_any.get("summary", {}).get("write_set_mapstore_any_direct_closeout_materialized")
                == 1
            ),
            "delete_surface_hako_mirror_retired": 1,
            "rust_map_delete_route_preserved": 1,
            "write_scalar_i64_routes_closeout_ready_for_rerun": 1,
            "delete_surface_direct_closeout_materialized": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectWriteScalarI64RoutesCloseoutRerun",
            "reason_token": "ScopedWriteContractsAndDeleteRetireTreatmentCollectedForCloseoutReview",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "write_scalar_i64_routes_closeout_basis": 1,
            "basis_only": 1,
            "scoped_direct_closeout_contract_count": len(scoped_contracts),
            "delete_surface_hako_mirror_retired": 1,
            "rust_map_delete_route_preserved": 1,
            "write_scalar_i64_routes_closeout_ready_for_rerun": 1,
            "delete_surface_direct_closeout_materialized": 0,
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
        print("mirbuilder-carrier-type-scalar-known-write-scalar-i64-routes-closeout-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
