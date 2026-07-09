#!/usr/bin/env python3
"""Rerun and materialize WriteScalarI64Routes scoped closeout."""

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
    / "mirbuilder-carrier-type-scalar-known-write-scalar-i64-routes-closeout-rerun-v0.json"
)

TOKEN = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SCALAR-I64-ROUTES-CLOSEOUT-RERUN-001"
NEXT_CARD = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-RERUN-002"

BASIS = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-scalar-i64-routes-closeout-basis-v0.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    review = basis.get("write_surface_review") or {}
    summary = basis.get("summary") or {}

    scoped_contracts = review.get("scoped_direct_closeout_contracts") or []
    delete_treatment = review.get("delete_surface_policy") or {}

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownWriteScalarI64RoutesCloseoutRerunV1",
        "token": TOKEN,
        "input_state": {
            "write_scalar_i64_routes_closeout_basis": rel(BASIS),
            "write_scalar_i64_routes_closeout_basis_hash": sha256_file(BASIS),
            "basis_decision": (basis.get("decision") or {}).get("kind"),
            "basis_selected_next_card": (basis.get("decision") or {}).get("selected_next_card"),
            "basis_ready_for_rerun": summary.get("write_scalar_i64_routes_closeout_ready_for_rerun"),
        },
        "materialized_closeout": {
            "surface_id": "WriteScalarI64Routes",
            "closeout_kind": "ScopedWriteSurfaceCloseout",
            "scoped_direct_closeout_contract_count": len(scoped_contracts),
            "scoped_direct_closeout_contracts": scoped_contracts,
            "delete_surface_treatment": delete_treatment,
            "delete_surface_counts_as_hako_direct_closeout": False,
            "delete_surface_live_rust_route_preserved": True,
            "runtime_mutation_authority": False,
            "publication_execution": False,
            "route_authority_switch": False,
        },
        "summary": {
            "write_scalar_i64_routes_closeout": 1,
            "write_scalar_i64_routes_scoped_closeout_materialized": 1,
            "scoped_direct_closeout_contract_count": len(scoped_contracts),
            "delete_surface_hako_mirror_retired": 1,
            "delete_surface_direct_closeout_materialized": 0,
            "delete_surface_live_rust_route_preserved": 1,
            "scalar_known_transport_axis_closeout": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectScalarKnownTransportCloseoutRerunAfterWriteCloseout",
            "reason_token": "WriteScalarI64RoutesScopedCloseoutMaterialized",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "write_scalar_i64_routes_closeout": 1,
            "write_scalar_i64_routes_scoped_closeout_materialized": 1,
            "scoped_direct_closeout_contract_count": len(scoped_contracts),
            "delete_surface_hako_mirror_retired": 1,
            "delete_surface_live_rust_route_preserved": 1,
            "delete_surface_direct_closeout_materialized": 0,
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
        print("mirbuilder-carrier-type-scalar-known-write-scalar-i64-routes-closeout-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
