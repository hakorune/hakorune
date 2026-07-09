#!/usr/bin/env python3
"""Rerun after the scoped Collection `.hako` authority pilot."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-collection-hako-authority-pilot-rerun-v0.json"

TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-HAKO-AUTHORITY-PILOT-RERUN-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-SURFACE-AUTHORITY-CLOSEOUT-DESIGN-STOP-001"

PILOT = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-collection-hako-route-decision-authority-pilot-v0.json"
)
SHADOW_SOURCE = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
COLLECTION_ROUTES = ROOT / "src/mir/generic_method_route_plan/collection_read_routes.rs"


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
        "kind": "MirBuilderScalarKnownFastpathCollectionHakoAuthorityPilotRerunV1",
        "token": TOKEN,
        "input_state": {
            "pilot_fixture": rel(PILOT),
            "pilot_fixture_hash": sha256_file(PILOT),
            "pilot_selected_next_card": (pilot.get("decision") or {}).get("selected_next_card"),
            "collection_hako_route_decision_authority_pilot": (pilot.get("summary") or {}).get(
                "collection_hako_route_decision_authority_pilot"
            ),
        },
        "provenance": {
            "shadow_consumer": file_entry(SHADOW_SOURCE),
            "collection_routes": file_entry(COLLECTION_ROUTES),
        },
        "rerun": {
            "mapload_hako_route_decision_authority_pilot": True,
            "string_hako_route_decision_authority_pilot": True,
            "collection_hako_route_decision_authority_pilot": True,
            "collection_mixed_receiver_domain_guarded": True,
            "collection_anylength_box_domain_guarded": True,
            "scalar_known_hako_runtime_route_authority": False,
            "next_authority_step_requires_design_consultation": True,
        },
        "decision": {
            "kind": "KeepStoppedForReadSurfaceAuthorityCloseoutDesign",
            "reason_token": "AllReadSurfacesScopedAuthorityPilotsCompleteCloseoutStillConsultationGated",
            "selected_next_card": NEXT_CARD,
        },
        "summary": {
            "collection_hako_authority_pilot_rerun": 1,
            "mapload_hako_route_decision_authority_pilot": 1,
            "string_hako_route_decision_authority_pilot": 1,
            "collection_hako_route_decision_authority_pilot": 1,
            "collection_mixed_receiver_domain_guarded": 1,
            "collection_anylength_box_domain_guarded": 1,
            "read_surface_authority_closeout_design_required": 1,
            "scalar_known_hako_runtime_route_authority": 0,
            "source_selfhost_claim": 0,
        },
        "claims": {
            "collection_hako_authority_pilot_rerun": 1,
            "mapload_hako_route_decision_authority_pilot": 1,
            "string_hako_route_decision_authority_pilot": 1,
            "collection_hako_route_decision_authority_pilot": 1,
            "collection_mixed_receiver_domain_guarded": 1,
            "collection_anylength_box_domain_guarded": 1,
            "read_surface_authority_closeout_design_required": 1,
            "read_surface_authority_closeout": 0,
            "scalar_known_hako_runtime_route_authority": 0,
            "scalar_known_transport_axis_authority_switch": 0,
            "rust_fastpath_rewired": 0,
            "route_selection_authority_switch": 0,
            "backend_lowering_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
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
        print("mirbuilder-scalar-known-fastpath-collection-hako-authority-pilot-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
