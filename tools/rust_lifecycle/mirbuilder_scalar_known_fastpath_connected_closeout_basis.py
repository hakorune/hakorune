#!/usr/bin/env python3
"""Define the ScalarKnown fastpath-connected closeout basis."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-connected-closeout-basis-v0.json"

TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-BASIS-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-001"

TRANSPORT_CLOSEOUT = (
    FIXTURES / "mirbuilder-carrier-type-scalar-known-transport-closeout-rerun-002-v0.json"
)
TYPED_SHADOW = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-set-mapstore-i64-v0.json"
)
BRIDGE_PLAN = (
    FIXTURES / "mirbuilder-fastpath-hako-shadow-artifact-to-caller-orientation-bridge-plan-v0.json"
)
WRITE_ROUTES = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"
SHADOW_SOURCE = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
GENERATED_POLICY = (
    ROOT / "src/mir/generic_method_route_plan/generated/write_set_mapstore_i64_hako_policy.rs"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    transport = read_json(TRANSPORT_CLOSEOUT)
    typed_shadow = read_json(TYPED_SHADOW)
    bridge_plan = read_json(BRIDGE_PLAN)

    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathConnectedCloseoutBasisV1",
        "token": TOKEN,
        "input_state": {
            "transport_closeout": rel(TRANSPORT_CLOSEOUT),
            "transport_closeout_hash": sha256_file(TRANSPORT_CLOSEOUT),
            "transport_selected_next_card": (transport.get("decision") or {}).get("selected_next_card"),
            "typed_shadow_consume": rel(TYPED_SHADOW),
            "typed_shadow_consume_hash": sha256_file(TYPED_SHADOW),
            "bridge_plan": rel(BRIDGE_PLAN),
            "bridge_plan_hash": sha256_file(BRIDGE_PLAN),
            "bridge_plan_selected_path": (bridge_plan.get("decision") or {}).get("selected_path"),
        },
        "provenance": {
            "write_routes_hash": sha256_file(WRITE_ROUTES),
            "scalar_known_hako_shadow_hash": sha256_file(SHADOW_SOURCE),
            "generated_mapstore_i64_policy_hash": sha256_file(GENERATED_POLICY),
        },
        "basis": {
            "name": "ScalarKnownFastpathConnectedCloseoutBasisV1",
            "basis_only": True,
            "scope": "ScalarKnownTransportAxisFastpathConnection",
            "required_connection_kind": "CheckedInGeneratedTypedHakoArtifactShadowConsumedAtRustFastpathDecisionPoint",
            "rust_authority_retained": True,
            "hako_runtime_route_authority": False,
            "runtime_source_text_parsing_allowed": False,
            "rerun_required_before_connected_closeout": True,
            "connected_surface_rows": [
                {
                    "surface_id": "WriteScalarI64Routes",
                    "subsurface_id": "SetSurfacePolicy/MapStoreI64",
                    "route_kind": "MapStoreI64",
                    "connection_kind": "GeneratedTypedHakoArtifactShadowConsumed",
                    "connected": True,
                }
            ],
            "known_unconnected_surface_rows": [
                {"surface_id": "MapLoadScalarI64Routes", "connected": False},
                {"surface_id": "StringScalarI64Routes", "connected": False},
                {"surface_id": "CollectionScalarI64Routes", "connected": False},
                {"surface_id": "WriteScalarI64Routes", "subsurface_id": "PushSurfacePolicy", "connected": False},
                {"surface_id": "WriteScalarI64Routes", "subsurface_id": "SetSurfacePolicy/MapStoreAny", "connected": False},
            ],
        },
        "summary": {
            "fastpath_connected_closeout_basis": 1,
            "scalar_known_transport_axis_closeout": transport.get("summary", {}).get(
                "scalar_known_transport_axis_closeout"
            ),
            "generated_typed_hako_artifact_shadow_consumed": typed_shadow.get("claims", {}).get(
                "generated_typed_hako_artifact_shadow_consumed"
            ),
            "connected_surface_row_count": 1,
            "known_unconnected_surface_row_count": 5,
            "fastpath_connected_closeout": 0,
            "hako_runtime_route_authority": 0,
            "rust_fastpath_rewired": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectFastpathConnectedCloseoutInventoryRerun",
            "reason_token": "FastpathConnectedCloseoutBasisDefinedMapStoreI64OnlyConnected",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "fastpath_connected_closeout_basis": 1,
            "basis_only": 1,
            "scalar_known_transport_axis_closeout": transport.get("summary", {}).get(
                "scalar_known_transport_axis_closeout"
            ),
            "generated_typed_hako_artifact_shadow_consumed": typed_shadow.get("claims", {}).get(
                "generated_typed_hako_artifact_shadow_consumed"
            ),
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
        print("mirbuilder-scalar-known-fastpath-connected-closeout-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
