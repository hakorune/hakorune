#!/usr/bin/env python3
"""Rerun ScalarKnown fastpath-connected closeout inventory after MapStoreAny."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = (
    FIXTURES / "mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-002-v0.json"
)

TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-002"
NEXT_CARD = (
    "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-"
    "WRITE-PUSH-BASIS-001"
)

MAPSTORE_I64 = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-set-mapstore-i64-v0.json"
)
MAPSTORE_ANY = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-set-mapstore-any-v0.json"
)
PUSH_HAKO = ROOT / "lang/src/compiler/lib/write_push_surface_policy_classifier.hako"
WRITE_ROUTES = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    mapstore_i64 = read_json(MAPSTORE_I64)
    mapstore_any = read_json(MAPSTORE_ANY)
    connected = [
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
    ]
    unconnected = [
        {"surface_id": "MapLoadScalarI64Routes", "connected": False},
        {"surface_id": "StringScalarI64Routes", "connected": False},
        {"surface_id": "CollectionScalarI64Routes", "connected": False},
        {
            "surface_id": "WriteScalarI64Routes",
            "subsurface_id": "PushSurfacePolicy",
            "route_kind": "ArrayAppendAny",
            "connected": False,
        },
    ]

    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathConnectedCloseoutInventoryRerun002V1",
        "token": TOKEN,
        "input_state": {
            "mapstore_i64_shadow_consume": rel(MAPSTORE_I64),
            "mapstore_i64_shadow_consume_hash": sha256_file(MAPSTORE_I64),
            "mapstore_i64_decision": (mapstore_i64.get("decision") or {}).get("kind"),
            "mapstore_any_shadow_consume": rel(MAPSTORE_ANY),
            "mapstore_any_shadow_consume_hash": sha256_file(MAPSTORE_ANY),
            "mapstore_any_decision": (mapstore_any.get("decision") or {}).get("kind"),
        },
        "provenance": {
            "push_hako_source": rel(PUSH_HAKO),
            "push_hako_source_hash": sha256_file(PUSH_HAKO),
            "write_routes_hash": sha256_file(WRITE_ROUTES),
        },
        "inventory": {
            "connected_surface_rows": connected,
            "known_unconnected_surface_rows": unconnected,
            "connected_surface_row_count": len(connected),
            "known_unconnected_surface_row_count": len(unconnected),
            "selection_eligible_candidate_count": 1,
            "selected_candidate": {
                "surface_id": "WriteScalarI64Routes",
                "subsurface_id": "PushSurfacePolicy",
                "route_kind": "ArrayAppendAny",
                "selection_kind": "PriorWriteRouteGeneratedTypedArtifactContinuation",
                "reason": "Push is the remaining WriteScalarI64Routes subsurface with an existing .hako policy mirror and live Rust write_routes fast-path owner.",
            },
            "selection_rule": {
                "name": "PriorWriteRouteGeneratedTypedArtifactContinuationV1",
                "route_count_as_proof": False,
                "manual_surface_selection": False,
                "hako_runtime_authority_switch": False,
                "read_surface_selection": False,
            },
        },
        "summary": {
            "fastpath_connected_closeout_inventory_rerun_002": 1,
            "connected_surface_row_count": len(connected),
            "known_unconnected_surface_row_count": len(unconnected),
            "selection_eligible_candidate_count": 1,
            "selected_surface": "WriteScalarI64Routes/PushSurfacePolicy",
            "fastpath_connected_closeout": 0,
            "hako_runtime_route_authority": 0,
            "rust_fastpath_rewired": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectWritePushGeneratedTypedArtifactShadowConsumeBasis",
            "reason_token": "PriorWriteRouteGeneratedTypedArtifactContinuation",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "fastpath_connected_closeout_inventory_rerun_002": 1,
            "selection_eligible_candidate_count": 1,
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
        print("mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-002 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
