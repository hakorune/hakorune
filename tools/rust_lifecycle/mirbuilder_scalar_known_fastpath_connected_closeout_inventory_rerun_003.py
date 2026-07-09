#!/usr/bin/env python3
"""Rerun ScalarKnown fastpath-connected closeout inventory after Write Push."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = (
    FIXTURES / "mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-003-v0.json"
)

TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-003"
NEXT_CARD = (
    "MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-SURFACE-GENERATED-TYPED-ARTIFACT-"
    "SELECTION-DESIGN-CONSULTATION-001"
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

READ_POLICY_SOURCES = [
    ROOT / "lang/src/compiler/lib/map_load_scalar_i64_policy_classifier.hako",
    ROOT / "lang/src/compiler/lib/string_search_scalar_i64_policy_classifier.hako",
    ROOT / "lang/src/compiler/lib/collection_len_scalar_i64_policy_classifier.hako",
]
READ_GENERATED_ARTIFACTS = [
    ROOT / "src/mir/generic_method_route_plan/generated/map_load_scalar_i64_hako_policy.rs",
    ROOT / "src/mir/generic_method_route_plan/generated/string_search_scalar_i64_hako_policy.rs",
    ROOT / "src/mir/generic_method_route_plan/generated/collection_len_scalar_i64_hako_policy.rs",
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def absent(paths: list[Path]) -> list[str]:
    return [rel(path) for path in paths if not path.exists()]


def build_fixture() -> dict[str, Any]:
    mapstore_i64 = read_json(MAPSTORE_I64)
    mapstore_any = read_json(MAPSTORE_ANY)
    write_push = read_json(WRITE_PUSH)
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
        {
            "surface_id": "WriteScalarI64Routes",
            "subsurface_id": "PushSurfacePolicy",
            "route_kind": "ArrayAppendAny",
            "connection_kind": "GeneratedTypedHakoArtifactShadowConsumed",
            "connected": True,
        },
    ]
    unconnected = [
        {
            "surface_id": "MapLoadScalarI64Routes",
            "connected": False,
            "blocked_by": [
                "NoCheckedInGeneratedTypedHakoPolicyArtifact",
                "NoMechanicalReadSurfacePriorityAfterWriteContinuationConsumed",
            ],
        },
        {
            "surface_id": "StringScalarI64Routes",
            "connected": False,
            "blocked_by": [
                "NoCheckedInGeneratedTypedHakoPolicyArtifact",
                "NoMechanicalReadSurfacePriorityAfterWriteContinuationConsumed",
            ],
        },
        {
            "surface_id": "CollectionScalarI64Routes",
            "connected": False,
            "blocked_by": [
                "NoCheckedInGeneratedTypedHakoPolicyArtifact",
                "NoMechanicalReadSurfacePriorityAfterWriteContinuationConsumed",
            ],
        },
    ]

    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathConnectedCloseoutInventoryRerun003V1",
        "token": TOKEN,
        "input_state": {
            "mapstore_i64_shadow_consume": rel(MAPSTORE_I64),
            "mapstore_i64_shadow_consume_hash": sha256_file(MAPSTORE_I64),
            "mapstore_i64_decision": (mapstore_i64.get("decision") or {}).get("kind"),
            "mapstore_any_shadow_consume": rel(MAPSTORE_ANY),
            "mapstore_any_shadow_consume_hash": sha256_file(MAPSTORE_ANY),
            "mapstore_any_decision": (mapstore_any.get("decision") or {}).get("kind"),
            "write_push_shadow_consume": rel(WRITE_PUSH),
            "write_push_shadow_consume_hash": sha256_file(WRITE_PUSH),
            "write_push_decision": (write_push.get("decision") or {}).get("kind"),
        },
        "provenance": {
            "scalar_known_contract_hash": sha256_file(SCALAR_CONTRACT),
            "collection_read_routes_hash": sha256_file(COLLECTION_READ_ROUTES),
            "string_routes_hash": sha256_file(STRING_ROUTES),
            "write_routes_hash": sha256_file(WRITE_ROUTES),
            "absent_read_policy_sources": absent(READ_POLICY_SOURCES),
            "absent_read_generated_artifacts": absent(READ_GENERATED_ARTIFACTS),
        },
        "inventory": {
            "connected_surface_rows": connected,
            "known_unconnected_surface_rows": unconnected,
            "connected_surface_row_count": len(connected),
            "known_unconnected_surface_row_count": len(unconnected),
            "write_surface_connection_complete": True,
            "read_surface_connection_complete": False,
            "selection_eligible_candidate_count": 0,
            "selected_candidate": None,
            "selection_rule": {
                "name": "RequireConsultationApprovedReadSurfaceGeneratedTypedArtifactPriorityV1",
                "write_continuation_consumed": True,
                "route_count_as_proof": False,
                "manual_surface_selection": False,
                "hako_runtime_authority_switch": False,
                "read_surface_selection": False,
                "owner_name_as_proof": False,
                "source_path_as_authority": False,
            },
        },
        "summary": {
            "fastpath_connected_closeout_inventory_rerun_003": 1,
            "connected_surface_row_count": len(connected),
            "known_unconnected_surface_row_count": len(unconnected),
            "write_surface_connection_complete": 1,
            "read_surface_connection_complete": 0,
            "selection_eligible_candidate_count": 0,
            "fastpath_connected_closeout": 0,
            "hako_runtime_route_authority": 0,
            "rust_fastpath_rewired": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "KeepStoppedDesignConsultationRequired",
            "reason_token": "NoMechanicalReadSurfaceGeneratedTypedArtifactPriority",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "fastpath_connected_closeout_inventory_rerun_003": 1,
            "write_surface_connection_complete": 1,
            "read_surface_connection_complete": 0,
            "selection_eligible_candidate_count": 0,
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
        print("mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-003 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
