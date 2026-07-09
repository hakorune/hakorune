#!/usr/bin/env python3
"""Rerun ScalarKnown transport closeout after WriteScalarI64Routes closeout."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = (
    FIXTURES / "mirbuilder-carrier-type-scalar-known-transport-closeout-rerun-002-v0.json"
)

TOKEN = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-RERUN-002"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-BASIS-001"

MAPLOAD_BASIS = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-map-load-i64-typed-direct-closeout-contract-basis-v0.json"
)
STRING_RERUN = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-string-search-scalar-i64-typed-direct-closeout-contract-rerun-v0.json"
)
COLLECTION_RERUN = (
    FIXTURES / "mirbuilder-carrier-type-scalar-known-collection-len-scalar-i64-contract-rerun-v0.json"
)
WRITE_RERUN = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-scalar-i64-routes-closeout-rerun-v0.json"
)
RUST_BOUNDARY = ROOT / "src/mir/generic_method_route_plan/scalar_known_typed_direct_closeout_contract.rs"
SHADOW = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    mapload = read_json(MAPLOAD_BASIS)
    string = read_json(STRING_RERUN)
    collection = read_json(COLLECTION_RERUN)
    write = read_json(WRITE_RERUN)

    accepted_surfaces = [
        {
            "surface_id": "MapLoadScalarI64Routes",
            "contract_id": mapload.get("contract", {}).get("contract_id"),
            "source": rel(MAPLOAD_BASIS),
        },
        {
            "surface_id": "StringScalarI64Routes",
            "contract_id": "StringSearchScalarI64TypedDirectCloseoutContract",
            "source": rel(STRING_RERUN),
        },
        {
            "surface_id": "CollectionScalarI64Routes",
            "contract_id": (collection.get("materialized_contract") or {}).get("contract_id"),
            "source": rel(COLLECTION_RERUN),
        },
        {
            "surface_id": "WriteScalarI64Routes",
            "contract_id": "WriteScalarI64RoutesScopedCloseout",
            "source": rel(WRITE_RERUN),
        },
    ]

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownTransportCloseoutRerun002V1",
        "token": TOKEN,
        "input_state": {
            "mapload_basis": rel(MAPLOAD_BASIS),
            "mapload_basis_hash": sha256_file(MAPLOAD_BASIS),
            "string_search_rerun": rel(STRING_RERUN),
            "string_search_rerun_hash": sha256_file(STRING_RERUN),
            "collection_len_rerun": rel(COLLECTION_RERUN),
            "collection_len_rerun_hash": sha256_file(COLLECTION_RERUN),
            "write_scalar_i64_routes_closeout_rerun": rel(WRITE_RERUN),
            "write_scalar_i64_routes_closeout_rerun_hash": sha256_file(WRITE_RERUN),
            "write_selected_next_card": (write.get("decision") or {}).get("selected_next_card"),
        },
        "provenance": {
            "rust_boundary_hash": sha256_file(RUST_BOUNDARY),
            "scalar_known_hako_shadow_hash": sha256_file(SHADOW),
        },
        "accepted_scalar_known_surfaces": accepted_surfaces,
        "rust_boundary_expectation": {
            "accepted_status": "AcceptedScopedCloseout",
            "accepted_surface_count": 4,
            "candidate_surface_count": 0,
            "write_contract_id": "WriteScalarI64RoutesScopedCloseout",
        },
        "summary": {
            "scalar_known_transport_axis_closeout": 1,
            "accepted_scalar_known_surface_count": len(accepted_surfaces),
            "uncovered_scalar_known_surface_count": 0,
            "write_scalar_i64_routes_closeout": 1,
            "fastpath_connected_closeout": 0,
            "hako_runtime_route_authority": 0,
            "rust_fastpath_rewired": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectFastpathConnectedCloseoutBasis",
            "reason_token": "ScalarKnownTransportAxisScopedCloseoutMaterializedButFastpathConnectedCloseoutStillOpen",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "scalar_known_transport_axis_closeout": 1,
            "accepted_scalar_known_surface_count": len(accepted_surfaces),
            "uncovered_scalar_known_surface_count": 0,
            "write_scalar_i64_routes_closeout": 1,
            "rust_boundary_status_refreshed": 1,
            "fastpath_connected_closeout": 0,
            "hako_runtime_route_authority": 0,
            "rust_fastpath_rewired": 0,
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
        print("mirbuilder-carrier-type-scalar-known-transport-closeout-rerun-002 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
