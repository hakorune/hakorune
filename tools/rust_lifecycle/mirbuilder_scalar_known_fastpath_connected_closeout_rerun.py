#!/usr/bin/env python3
"""Close out all known ScalarKnown fast-path shadow connections."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-connected-closeout-rerun-v0.json"

TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-RERUN-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-CALLER-ORIENTATION-AUTHORITY-DESIGN-STOP-001"

BASIS = FIXTURES / "mirbuilder-scalar-known-fastpath-connected-closeout-all-surfaces-basis-v0.json"
BRIDGE_PLAN = (
    FIXTURES / "mirbuilder-fastpath-hako-shadow-artifact-to-caller-orientation-bridge-plan-v0.json"
)
SHADOW_SOURCE = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    bridge_plan = read_json(BRIDGE_PLAN)
    basis_summary = basis.get("summary") or {}
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathConnectedCloseoutRerunV1",
        "token": TOKEN,
        "input_state": {
            "all_surfaces_basis": rel(BASIS),
            "all_surfaces_basis_hash": sha256_file(BASIS),
            "basis_selected_next_card": (basis.get("decision") or {}).get("selected_next_card"),
            "bridge_plan": rel(BRIDGE_PLAN),
            "bridge_plan_hash": sha256_file(BRIDGE_PLAN),
            "bridge_plan_long_term": (bridge_plan.get("decision") or {}).get("long_term"),
        },
        "provenance": {
            "shadow_consumer": rel(SHADOW_SOURCE),
            "shadow_consumer_hash": sha256_file(SHADOW_SOURCE),
        },
        "closeout": {
            "fastpath_connected_closeout": True,
            "connected_surface_row_count": basis_summary.get("connected_surface_row_count"),
            "known_unconnected_surface_row_count": basis_summary.get("known_unconnected_surface_row_count"),
            "write_surface_connection_complete": bool(basis_summary.get("write_surface_connection_complete")),
            "read_surface_connection_complete": bool(basis_summary.get("read_surface_connection_complete")),
            "all_known_scalar_known_surfaces_shadow_consumed": bool(
                basis_summary.get("all_known_scalar_known_surfaces_shadow_consumed")
            ),
            "connection_authority_kind": "GeneratedTypedHakoArtifactShadowConsumedAtRustFastpathDecisionPoint",
            "rust_authority_retained": True,
            "hako_runtime_route_authority": False,
            "caller_orientation_requires_design_consultation": True,
        },
        "summary": {
            "fastpath_connected_closeout_rerun": 1,
            "fastpath_connected_closeout": 1,
            "connected_surface_row_count": basis_summary.get("connected_surface_row_count"),
            "known_unconnected_surface_row_count": basis_summary.get("known_unconnected_surface_row_count"),
            "write_surface_connection_complete": basis_summary.get("write_surface_connection_complete"),
            "read_surface_connection_complete": basis_summary.get("read_surface_connection_complete"),
            "all_known_scalar_known_surfaces_shadow_consumed": basis_summary.get(
                "all_known_scalar_known_surfaces_shadow_consumed"
            ),
            "hako_runtime_route_authority": 0,
            "rust_fastpath_rewired": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "KeepStoppedForCallerOrientationAuthorityDesign",
            "reason_token": "FastpathConnectedCloseoutCompleteAuthoritySwitchStillConsultationGated",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "fastpath_connected_closeout_rerun": 1,
            "fastpath_connected_closeout": 1,
            "write_surface_connection_complete": 1,
            "read_surface_connection_complete": 1,
            "all_known_scalar_known_surfaces_shadow_consumed": 1,
            "caller_orientation_design_consultation_required": 1,
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
        print("mirbuilder-scalar-known-fastpath-connected-closeout-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
