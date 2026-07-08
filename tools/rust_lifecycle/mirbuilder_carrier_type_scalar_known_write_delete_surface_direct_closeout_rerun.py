#!/usr/bin/env python3
"""Rerun the Write Delete surface typed direct closeout contract."""

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
    / "mirbuilder-carrier-type-scalar-known-write-delete-surface-direct-closeout-rerun-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-"
    "DIRECT-CLOSEOUT-RERUN-001"
)
NEXT_CARD = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-REMAINING-SUBSURFACE-"
    "POST-DELETE-CLOSEOUT-RERUN-001"
)

DELETE_BASIS = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-delete-surface-typed-direct-closeout-contract-basis-v0.json"
)
PUSH_CLOSEOUT = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-push-surface-direct-closeout-rerun-v0.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    basis = read_json(DELETE_BASIS)
    push_closeout = read_json(PUSH_CLOSEOUT)
    prior_closeouts = push_closeout.get("accepted_scoped_closeouts") or []
    contract = basis.get("contract") or {}

    delete_contract = {
        "contract_id": contract.get("contract_id"),
        "surface_id": contract.get("surface_id"),
        "subsurface_id": contract.get("subsurface_id"),
        "routes": contract.get("route_kind_set"),
        "proof_or_policy_source": contract.get("proof_or_policy_source"),
        "core_method_op": contract.get("core_method_op"),
        "core_method_lowering_tier": contract.get("core_method_lowering_tier"),
        "result_class": contract.get("result_class"),
        "return_shape": contract.get("return_shape"),
        "value_demand": contract.get("value_demand"),
        "publication_policy": contract.get("publication_policy"),
        "effect_class": contract.get("effect_class"),
        "mutation_class": contract.get("mutation_class"),
        "runtime_mutation_authority": False,
        "publication_execution": False,
    }
    accepted_closeouts = prior_closeouts + [delete_contract]

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownWriteDeleteSurfaceDirectCloseoutRerunV1",
        "token": TOKEN,
        "input_state": {
            "write_delete_surface_contract_basis": rel(DELETE_BASIS),
            "write_delete_surface_contract_basis_hash": sha256_file(DELETE_BASIS),
            "prior_push_closeout_rerun": rel(PUSH_CLOSEOUT),
            "prior_push_closeout_rerun_hash": sha256_file(PUSH_CLOSEOUT),
            "basis_decision": basis.get("decision", {}).get("kind"),
            "basis_selected_next_card": basis.get("decision", {}).get("selected_next_card"),
        },
        "accepted_scoped_closeouts": accepted_closeouts,
        "materialized_contract": delete_contract,
        "remaining_write_subsurfaces": ["SetSurfacePolicy"],
        "remaining_write_subsurface_blockers": {
            "SetSurfacePolicy": "NoHakoAdoptedWriteSubsurfacePilot",
        },
        "summary": {
            "write_delete_surface_direct_closeout_materialized": 1,
            "accepted_scoped_closeout_count": len(accepted_closeouts),
            "remaining_write_subsurface_count": 1,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectWriteRemainingSubsurfacePostDeleteCloseoutRerun",
            "reason_token": "DeleteScopedCloseoutMaterializedSetRemains",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "write_delete_surface_direct_closeout_materialized": 1,
            "accepted_scoped_closeout_count": len(accepted_closeouts),
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
        "basis_summary": basis.get("summary"),
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-carrier-type-scalar-known-write-delete-surface-direct-closeout-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
