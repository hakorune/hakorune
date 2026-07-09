#!/usr/bin/env python3
"""Rerun ScalarKnown fastpath-connected closeout inventory."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-v0.json"

TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-SET-MAPSTORE-ANY-BASIS-001"

BASIS = FIXTURES / "mirbuilder-scalar-known-fastpath-connected-closeout-basis-v0.json"
MAPSTORE_ANY_HAKO = ROOT / "lang/src/compiler/lib/write_set_mapstore_any_policy_classifier.hako"
MAPSTORE_I64_ARTIFACT = (
    ROOT / "src/mir/generic_method_route_plan/generated/write_set_mapstore_i64_hako_policy.rs"
)
WRITE_ROUTES = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    basis_rows = basis.get("basis") or {}
    connected = basis_rows.get("connected_surface_rows") or []
    unconnected = basis_rows.get("known_unconnected_surface_rows") or []

    selected_candidate = {
        "surface_id": "WriteScalarI64Routes",
        "subsurface_id": "SetSurfacePolicy/MapStoreAny",
        "route_kind": "MapStoreAny",
        "selection_kind": "SameSetSurfacePolicyGeneratedTypedArtifactShadowConsume",
        "reason": "MapStoreAny shares SetSurfacePolicy with the already connected MapStoreI64 handoff and has .hako parity/adoption/scoped closeout evidence.",
    }

    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathConnectedCloseoutInventoryRerunV1",
        "token": TOKEN,
        "input_state": {
            "fastpath_connected_closeout_basis": rel(BASIS),
            "fastpath_connected_closeout_basis_hash": sha256_file(BASIS),
            "basis_decision": (basis.get("decision") or {}).get("kind"),
            "basis_selected_next_card": (basis.get("decision") or {}).get("selected_next_card"),
        },
        "provenance": {
            "mapstore_any_hako_source": rel(MAPSTORE_ANY_HAKO),
            "mapstore_any_hako_source_hash": sha256_file(MAPSTORE_ANY_HAKO),
            "mapstore_i64_generated_artifact": rel(MAPSTORE_I64_ARTIFACT),
            "mapstore_i64_generated_artifact_hash": sha256_file(MAPSTORE_I64_ARTIFACT),
            "write_routes_hash": sha256_file(WRITE_ROUTES),
        },
        "inventory": {
            "connected_surface_rows": connected,
            "known_unconnected_surface_rows": unconnected,
            "connected_surface_row_count": len(connected),
            "known_unconnected_surface_row_count": len(unconnected),
            "selection_eligible_candidate_count": 1,
            "selected_candidate": selected_candidate,
            "selection_rule": {
                "name": "PriorGeneratedTypedArtifactSameSetSurfacePolicyMinimalDeltaV1",
                "route_count_as_proof": False,
                "manual_surface_selection": False,
                "hako_runtime_authority_switch": False,
            },
        },
        "summary": {
            "fastpath_connected_closeout_inventory_rerun": 1,
            "connected_surface_row_count": len(connected),
            "known_unconnected_surface_row_count": len(unconnected),
            "selection_eligible_candidate_count": 1,
            "selected_surface": "WriteScalarI64Routes/SetSurfacePolicy/MapStoreAny",
            "fastpath_connected_closeout": 0,
            "hako_runtime_route_authority": 0,
            "rust_fastpath_rewired": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectMapStoreAnyGeneratedTypedArtifactShadowConsumeBasis",
            "reason_token": "PriorMapStoreI64GeneratedTypedArtifactSameSetSurfacePolicyMinimalDelta",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "fastpath_connected_closeout_inventory_rerun": 1,
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
        print("mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
