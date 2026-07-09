#!/usr/bin/env python3
"""Re-enter the Hako caller-orientation authority design stop from current evidence."""

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
    / "mirbuilder-scalar-known-fastpath-hako-caller-orientation-authority-design-stop-v0.json"
)

TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-CALLER-ORIENTATION-AUTHORITY-DESIGN-STOP-001"
POST_NON_DELETE_STOP = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-post-non-delete-write-authority-island-closeout-design-stop-v0.json"
)
CONNECTED_CLOSEOUT = (
    FIXTURES / "mirbuilder-scalar-known-fastpath-connected-closeout-rerun-v0.json"
)
BRIDGE_PLAN = (
    FIXTURES / "mirbuilder-fastpath-hako-shadow-artifact-to-caller-orientation-bridge-plan-v0.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    post_stop = read_json(POST_NON_DELETE_STOP)
    connected = read_json(CONNECTED_CLOSEOUT)
    bridge = read_json(BRIDGE_PLAN)
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathHakoCallerOrientationAuthorityDesignStopV1",
        "token": TOKEN,
        "input_state": {
            "post_non_delete_write_design_stop": rel(POST_NON_DELETE_STOP),
            "post_non_delete_write_design_stop_hash": sha256_file(POST_NON_DELETE_STOP),
            "connected_closeout_rerun": rel(CONNECTED_CLOSEOUT),
            "connected_closeout_rerun_hash": sha256_file(CONNECTED_CLOSEOUT),
            "shadow_artifact_to_caller_orientation_bridge_plan": rel(BRIDGE_PLAN),
            "shadow_artifact_to_caller_orientation_bridge_plan_hash": sha256_file(BRIDGE_PLAN),
            "post_stop_consultation_required": (post_stop.get("decision") or {}).get(
                "consultation_required"
            ),
            "connected_closeout_selected_next_card": (connected.get("decision") or {}).get(
                "selected_next_card"
            ),
            "bridge_plan_selected_path": (bridge.get("decision") or {}).get("selected_path"),
        },
        "evidence": {
            "all_known_scalar_known_surfaces_shadow_consumed": (
                (connected.get("claims") or {}).get("all_known_scalar_known_surfaces_shadow_consumed")
            ),
            "fastpath_connected_closeout": (connected.get("claims") or {}).get(
                "fastpath_connected_closeout"
            ),
            "read_surface_connection_complete": (connected.get("claims") or {}).get(
                "read_surface_connection_complete"
            ),
            "write_surface_connection_complete": (connected.get("claims") or {}).get(
                "write_surface_connection_complete"
            ),
            "non_delete_write_hako_route_decision_authority_island_closeout": (
                (post_stop.get("claims") or {}).get(
                    "non_delete_write_hako_route_decision_authority_island_closeout"
                )
            ),
            "delete_surface_retired_special_case_parked": (post_stop.get("claims") or {}).get(
                "delete_surface_retired_special_case_parked"
            ),
            "selected_long_term_hako_caller_orientation": (bridge.get("claims") or {}).get(
                "selected_long_term_hako_caller_orientation"
            ),
            "rust_authority_retained": (bridge.get("claims") or {}).get("rust_authority_retained"),
        },
        "decision": {
            "kind": "KeepStoppedForHakoCallerOrientationAuthorityDesign",
            "reason_token": "AllKnownSurfacesShadowConsumedNonDeleteWriteIslandClosedCallerOrientationStillConsultationGated",
            "selected_next_card": None,
            "consultation_required": True,
            "consultation_topic": "HakoCallerOrientationAuthorityDesign",
        },
        "claims": {
            "hako_caller_orientation_authority_design_stop": 1,
            "all_known_scalar_known_surfaces_shadow_consumed": 1,
            "fastpath_connected_closeout": 1,
            "non_delete_write_hako_route_decision_authority_island_closeout": 1,
            "delete_surface_retired_special_case_parked": 1,
            "selected_long_term_hako_caller_orientation": 1,
            "caller_orientation_requires_design_consultation": 1,
            "rust_oracle_compat_checker_retained": 1,
            "hako_runtime_route_authority": 0,
            "scalar_known_hako_runtime_route_authority": 0,
            "caller_orientation_runtime_path": 0,
            "rust_fastpath_rewired": 0,
            "route_selection_authority_switch": 0,
            "backend_lowering_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "write_surface_authority_closeout": 0,
            "write_wide_authority": 0,
            "delete_hako_route_decision_authority_pilot": 0,
            "mapdeleteany_authority": 0,
            "build_rs_hako_compiler_invocation": 0,
            "live_hako_authority": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "route_count_as_proof": 0,
            "row_count_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "owner_name_as_proof": 0,
            "source_path_as_authority": 0,
            "route_membership_alone_as_proof": 0,
            "manual_surface_selection": 0,
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
        print("mirbuilder-scalar-known-fastpath-hako-caller-orientation-authority-design-stop unchanged")
        return 0
    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
