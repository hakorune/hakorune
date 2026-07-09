#!/usr/bin/env python3
"""Define the all-surfaces ScalarKnown fastpath-connected closeout basis."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-connected-closeout-all-surfaces-basis-v0.json"

TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-ALL-SURFACES-BASIS-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-RERUN-001"

RERUN_006 = (
    FIXTURES / "mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-006-v0.json"
)
BRIDGE_PLAN = (
    FIXTURES / "mirbuilder-fastpath-hako-shadow-artifact-to-caller-orientation-bridge-plan-v0.json"
)
SHADOW_SOURCE = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
WRITE_ROUTES = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"
STRING_ROUTES = ROOT / "src/mir/generic_method_route_plan/string_routes.rs"
COLLECTION_READ_ROUTES = ROOT / "src/mir/generic_method_route_plan/collection_read_routes.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    rerun = read_json(RERUN_006)
    bridge_plan = read_json(BRIDGE_PLAN)
    inventory = rerun.get("inventory") or {}
    rows = inventory.get("connected_surface_rows") or []
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathConnectedCloseoutAllSurfacesBasisV1",
        "token": TOKEN,
        "input_state": {
            "inventory_rerun_006": rel(RERUN_006),
            "inventory_rerun_006_hash": sha256_file(RERUN_006),
            "inventory_selected_next_card": (rerun.get("decision") or {}).get("selected_next_card"),
            "bridge_plan": rel(BRIDGE_PLAN),
            "bridge_plan_hash": sha256_file(BRIDGE_PLAN),
            "bridge_plan_selected_path": (bridge_plan.get("decision") or {}).get("selected_path"),
        },
        "provenance": {
            "shadow_consumer": rel(SHADOW_SOURCE),
            "shadow_consumer_hash": sha256_file(SHADOW_SOURCE),
            "write_routes": rel(WRITE_ROUTES),
            "write_routes_hash": sha256_file(WRITE_ROUTES),
            "string_routes": rel(STRING_ROUTES),
            "string_routes_hash": sha256_file(STRING_ROUTES),
            "collection_read_routes": rel(COLLECTION_READ_ROUTES),
            "collection_read_routes_hash": sha256_file(COLLECTION_READ_ROUTES),
        },
        "basis": {
            "name": "ScalarKnownFastpathConnectedCloseoutAllSurfacesBasisV1",
            "basis_only": True,
            "scope": "AllKnownScalarKnownFastpathShadowConnections",
            "required_connection_kind": "CheckedInGeneratedTypedHakoArtifactShadowConsumedAtRustFastpathDecisionPoint",
            "required_connected_surface_row_count": 6,
            "required_known_unconnected_surface_row_count": 0,
            "connected_surface_rows": rows,
            "known_unconnected_surface_rows": [],
            "rust_authority_retained": True,
            "hako_runtime_route_authority": False,
            "runtime_source_text_parsing_allowed": False,
            "closeout_rerun_required": True,
            "closeout_acceptance_rule": {
                "all_known_surface_rows_shadow_consumed": True,
                "write_surface_connection_complete": True,
                "read_surface_connection_complete": True,
                "generated_typed_artifact_check_guards_required": True,
                "runtime_authority_switch_allowed": False,
                "row_count_alone_as_proof": False,
                "route_count_as_proof": False,
            },
        },
        "summary": {
            "fastpath_connected_closeout_all_surfaces_basis": 1,
            "basis_only": 1,
            "connected_surface_row_count": inventory.get("connected_surface_row_count"),
            "known_unconnected_surface_row_count": inventory.get("known_unconnected_surface_row_count"),
            "write_surface_connection_complete": int(bool(inventory.get("write_surface_connection_complete"))),
            "read_surface_connection_complete": int(bool(inventory.get("read_surface_connection_complete"))),
            "all_known_scalar_known_surfaces_shadow_consumed": int(
                bool(inventory.get("all_known_scalar_known_surfaces_shadow_consumed"))
            ),
            "fastpath_connected_closeout": 0,
            "hako_runtime_route_authority": 0,
            "rust_fastpath_rewired": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectFastpathConnectedCloseoutRerun",
            "reason_token": "AllKnownScalarKnownFastpathConnectedCloseoutBasisDefined",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "fastpath_connected_closeout_all_surfaces_basis": 1,
            "basis_only": 1,
            "write_surface_connection_complete": 1,
            "read_surface_connection_complete": 1,
            "all_known_scalar_known_surfaces_shadow_consumed": 1,
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
        print("mirbuilder-scalar-known-fastpath-connected-closeout-all-surfaces-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
