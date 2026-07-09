#!/usr/bin/env python3
"""Rerun ScalarKnown fastpath-connected closeout inventory after Collection."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = (
    FIXTURES / "mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-006-v0.json"
)

TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-006"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-ALL-SURFACES-BASIS-001"

MAPLOAD = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-mapload-scalar-i64-v0.json"
)
STRING = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-string-scalar-i64-v0.json"
)
COLLECTION = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-collection-scalar-i64-v0.json"
)
MAPSTORE_I64 = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-set-mapstore-i64-v0.json"
)
MAPSTORE_ANY = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-set-mapstore-any-v0.json"
)
WRITE_PUSH = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-write-push-v0.json"
)

SCALAR_CONTRACT = ROOT / "src/mir/generic_method_route_plan/scalar_known_typed_direct_closeout_contract.rs"
COLLECTION_READ_ROUTES = ROOT / "src/mir/generic_method_route_plan/collection_read_routes.rs"
STRING_ROUTES = ROOT / "src/mir/generic_method_route_plan/string_routes.rs"
WRITE_ROUTES = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"
SHADOW_CONSUMER = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def fixture_input(path: Path) -> dict[str, Any]:
    data = read_json(path)
    return {
        "fixture": rel(path),
        "fixture_hash": sha256_file(path),
        "decision": (data.get("decision") or {}).get("kind"),
        "claims": data.get("claims") or {},
    }


def connected_rows() -> list[dict[str, Any]]:
    return [
        {
            "surface_id": "WriteScalarI64Routes",
            "subsurface_id": "SetSurfacePolicy/MapStoreI64",
            "route_kind": "MapStoreI64",
            "connection_kind": "GeneratedTypedHakoArtifactShadowConsumed",
            "connected": True,
        },
        {
            "surface_id": "WriteScalarI64Routes",
            "subsurface_id": "SetSurfacePolicy/MapStoreAny",
            "route_kind": "MapStoreAny",
            "connection_kind": "GeneratedTypedHakoArtifactShadowConsumed",
            "connected": True,
        },
        {
            "surface_id": "WriteScalarI64Routes",
            "subsurface_id": "PushSurfacePolicy",
            "route_kind": "ArrayAppendAny",
            "connection_kind": "GeneratedTypedHakoArtifactShadowConsumed",
            "connected": True,
        },
        {
            "surface_id": "MapLoadScalarI64Routes",
            "route_kind": "MapLoadScalarI64",
            "connection_kind": "GeneratedTypedHakoArtifactShadowConsumed",
            "connected": True,
        },
        {
            "surface_id": "StringScalarI64Routes",
            "route_kind_family": [
                "StringIndexOf",
                "StringLastIndexOf",
                "StringContains",
            ],
            "connection_kind": "GeneratedTypedHakoArtifactShadowConsumed",
            "connected": True,
        },
        {
            "surface_id": "CollectionScalarI64Routes",
            "route_kind_family": [
                "MapEntryCount",
                "ArraySlotLen",
                "StringLen",
                "AnyLength",
            ],
            "connection_kind": "GeneratedTypedHakoArtifactShadowConsumed",
            "connected": True,
        },
    ]


def build_fixture() -> dict[str, Any]:
    rows = connected_rows()
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathConnectedCloseoutInventoryRerun006V1",
        "token": TOKEN,
        "input_state": {
            "mapload": fixture_input(MAPLOAD),
            "string": fixture_input(STRING),
            "collection": fixture_input(COLLECTION),
            "mapstore_i64": fixture_input(MAPSTORE_I64),
            "mapstore_any": fixture_input(MAPSTORE_ANY),
            "write_push": fixture_input(WRITE_PUSH),
        },
        "provenance": {
            "scalar_known_contract": rel(SCALAR_CONTRACT),
            "scalar_known_contract_hash": sha256_file(SCALAR_CONTRACT),
            "collection_read_routes": rel(COLLECTION_READ_ROUTES),
            "collection_read_routes_hash": sha256_file(COLLECTION_READ_ROUTES),
            "string_routes": rel(STRING_ROUTES),
            "string_routes_hash": sha256_file(STRING_ROUTES),
            "write_routes": rel(WRITE_ROUTES),
            "write_routes_hash": sha256_file(WRITE_ROUTES),
            "shadow_consumer": rel(SHADOW_CONSUMER),
            "shadow_consumer_hash": sha256_file(SHADOW_CONSUMER),
        },
        "inventory": {
            "connected_surface_rows": rows,
            "known_unconnected_surface_rows": [],
            "connected_surface_row_count": len(rows),
            "known_unconnected_surface_row_count": 0,
            "write_surface_connection_complete": True,
            "read_surface_connection_complete": True,
            "all_known_scalar_known_surfaces_shadow_consumed": True,
            "selection_eligible_candidate_count": 1,
            "selected_candidate": {
                "selection_kind": "AllKnownScalarKnownFastpathSurfacesConnectedCloseoutBasis",
                "selected_next_card": NEXT_CARD,
                "reason": "All known ScalarKnown Write and read surfaces now have generated typed .hako artifacts shadow-consumed from the live Rust fast path.",
            },
            "selection_rule": {
                "name": "AllKnownScalarKnownFastpathSurfacesConnectedV1",
                "requires_zero_unconnected_surfaces": True,
                "requires_write_surface_connection_complete": True,
                "requires_read_surface_connection_complete": True,
                "row_count_as_proof": False,
                "route_count_as_proof": False,
                "manual_surface_selection": False,
                "hako_runtime_authority_switch": False,
                "owner_name_as_proof": False,
                "source_path_as_authority": False,
            },
        },
        "summary": {
            "fastpath_connected_closeout_inventory_rerun_006": 1,
            "connected_surface_row_count": len(rows),
            "known_unconnected_surface_row_count": 0,
            "write_surface_connection_complete": 1,
            "read_surface_connection_complete": 1,
            "all_known_scalar_known_surfaces_shadow_consumed": 1,
            "selection_eligible_candidate_count": 1,
            "selected_next_card": NEXT_CARD,
            "fastpath_connected_closeout": 0,
            "hako_runtime_route_authority": 0,
            "rust_fastpath_rewired": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectAllKnownScalarKnownFastpathConnectedCloseoutBasis",
            "reason_token": "AllKnownScalarKnownFastpathSurfacesShadowConsumed",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "fastpath_connected_closeout_inventory_rerun_006": 1,
            "write_surface_connection_complete": 1,
            "read_surface_connection_complete": 1,
            "all_known_scalar_known_surfaces_shadow_consumed": 1,
            "selection_eligible_candidate_count": 1,
            "fastpath_connected_closeout": 0,
            "hako_runtime_route_authority": 0,
            "rust_fastpath_rewired": 0,
            "route_selection_authority_switch": 0,
            "backend_lowering_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "build_rs_hako_compiler_invocation": 0,
            "live_hako_authority": 0,
            "caller_orientation_runtime_path": 0,
            "source_selfhost_claim": 0,
            "hako_generation": 0,
            "new_route_authority": 0,
            "behavior_change": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "native_seed_materialization": 0,
            "new_python_semantic_projector": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
            "manual_subsurface_selection": 0,
            "manual_surface_selection": 0,
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
        print("mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-006 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
