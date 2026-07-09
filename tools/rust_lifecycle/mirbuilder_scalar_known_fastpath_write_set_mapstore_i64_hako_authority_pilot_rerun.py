#!/usr/bin/env python3
"""Rerun after the scoped MapStoreI64 Write `.hako` authority pilot."""

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
    / "mirbuilder-scalar-known-fastpath-write-set-mapstore-i64-hako-authority-pilot-rerun-v0.json"
)

TOKEN = (
    "MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-"
    "HAKO-AUTHORITY-PILOT-RERUN-001"
)
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-NEXT-WRITE-HAKO-AUTHORITY-SURFACE-DESIGN-STOP-001"

PILOT = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-write-set-mapstore-i64-hako-route-decision-authority-pilot-v0.json"
)
SHADOW_SOURCE = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
WRITE_ROUTES = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def file_entry(path: Path) -> dict[str, str]:
    return {"path": rel(path), "sha256": sha256_file(path)}


def build_fixture() -> dict[str, Any]:
    pilot = read_json(PILOT)
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathWriteSetMapStoreI64HakoAuthorityPilotRerunV1",
        "token": TOKEN,
        "input_state": {
            "pilot_fixture": rel(PILOT),
            "pilot_fixture_hash": sha256_file(PILOT),
            "pilot_selected_next_card": (pilot.get("decision") or {}).get("selected_next_card"),
            "write_set_mapstore_i64_hako_route_decision_authority_pilot": (
                pilot.get("summary") or {}
            ).get("write_set_mapstore_i64_hako_route_decision_authority_pilot"),
        },
        "provenance": {
            "shadow_consumer": file_entry(SHADOW_SOURCE),
            "write_routes": file_entry(WRITE_ROUTES),
        },
        "rerun": {
            "read_surface_authority_closeout": True,
            "write_set_mapstore_i64_hako_route_decision_authority_pilot": True,
            "write_set_mapstore_i64_rust_oracle_compat_checker": True,
            "write_set_mapstore_i64_mismatch_fail_fast": True,
            "remaining_write_authority_surfaces_require_design": True,
            "scalar_known_hako_runtime_route_authority": False,
        },
        "decision": {
            "kind": "KeepStoppedForNextWriteAuthoritySurfaceDesign",
            "reason_token": "FirstWriteScopedAuthorityPilotCompleteRemainingWriteSurfaceSelectionConsultationGated",
            "selected_next_card": NEXT_CARD,
        },
        "summary": {
            "write_set_mapstore_i64_hako_authority_pilot_rerun": 1,
            "read_surface_authority_closeout": 1,
            "write_set_mapstore_i64_hako_route_decision_authority_pilot": 1,
            "write_set_mapstore_i64_rust_oracle_compat_checker": 1,
            "write_set_mapstore_i64_mismatch_fail_fast": 1,
            "next_write_authority_surface_design_required": 1,
            "write_surface_authority_pilot": 0,
            "scalar_known_hako_runtime_route_authority": 0,
            "source_selfhost_claim": 0,
        },
        "claims": {
            "write_set_mapstore_i64_hako_authority_pilot_rerun": 1,
            "read_surface_authority_closeout": 1,
            "write_set_mapstore_i64_hako_route_decision_authority_pilot": 1,
            "write_set_mapstore_i64_rust_oracle_compat_checker": 1,
            "write_set_mapstore_i64_mismatch_fail_fast": 1,
            "next_write_authority_surface_design_required": 1,
            "write_surface_authority_pilot": 0,
            "mapstore_authority": 0,
            "mapdelete_authority": 0,
            "arrayappend_authority": 0,
            "write_mutation_authority": 0,
            "write_publication_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "scalar_known_hako_runtime_route_authority": 0,
            "scalar_known_transport_axis_authority_switch": 0,
            "rust_fastpath_rewired": 0,
            "route_selection_authority_switch": 0,
            "backend_lowering_authority": 0,
            "caller_orientation_runtime_path": 0,
            "build_rs_hako_compiler_invocation": 0,
            "live_hako_authority": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
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
        print("mirbuilder-scalar-known-fastpath-write-set-mapstore-i64-hako-authority-pilot-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
