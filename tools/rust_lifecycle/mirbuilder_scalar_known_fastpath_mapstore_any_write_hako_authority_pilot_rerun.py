#!/usr/bin/env python3
"""Rerun after the scoped MapStoreAny Write `.hako` authority pilot."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-mapstore-any-write-hako-authority-pilot-rerun-v0.json"

TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPSTORE-ANY-WRITE-HAKO-AUTHORITY-PILOT-RERUN-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-NEXT-WRITE-HAKO-AUTHORITY-SURFACE-DESIGN-STOP-003"

PILOT = FIXTURES / "mirbuilder-scalar-known-fastpath-mapstore-any-write-hako-route-decision-authority-pilot-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def build_fixture() -> dict:
    pilot = json.loads(PILOT.read_text(encoding="utf-8"))
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathMapStoreAnyWriteHakoAuthorityPilotRerunV1",
        "token": TOKEN,
        "input_state": {
            "pilot_fixture": rel(PILOT),
            "pilot_fixture_hash": sha256_file(PILOT),
            "pilot_selected_next_card": (pilot.get("decision") or {}).get("selected_next_card"),
        },
        "rerun": {
            "write_set_mapstore_i64_hako_route_decision_authority_pilot": True,
            "push_hako_route_decision_authority_pilot": True,
            "mapstore_any_hako_route_decision_authority_pilot": True,
            "mapstore_any_any_boundary_metadata_only": True,
            "remaining_delete_surface_requires_design": True,
        },
        "decision": {
            "kind": "KeepStoppedForNextWriteAuthoritySurfaceDesign",
            "reason_token": "MapStoreAnyScopedAuthorityPilotCompleteDeleteSurfaceStillConsultationGated",
            "selected_next_card": NEXT_CARD,
        },
        "summary": {
            "mapstore_any_write_hako_authority_pilot_rerun": 1,
            "mapstore_any_hako_route_decision_authority_pilot": 1,
            "mapstore_any_any_boundary_metadata_only": 1,
            "next_write_authority_surface_design_required": 1,
            "runtime_mutation_authority": 0,
            "source_selfhost_claim": 0,
        },
        "claims": {
            "mapstore_any_write_hako_authority_pilot_rerun": 1,
            "mapstore_any_hako_route_decision_authority_pilot": 1,
            "mapstore_any_any_boundary_metadata_only": 1,
            "next_write_authority_surface_design_required": 1,
            "write_surface_authority_closeout": 0,
            "mapdeleteany_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
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
        print("mirbuilder-scalar-known-fastpath-mapstore-any-write-hako-authority-pilot-rerun unchanged")
        return 0
    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
