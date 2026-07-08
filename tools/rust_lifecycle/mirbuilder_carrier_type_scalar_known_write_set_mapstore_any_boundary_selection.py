#!/usr/bin/env python3
"""Select the MapStoreAny Any-write boundary basis after MapStoreI64 closeout."""

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
    / "mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-boundary-selection-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-"
    "BOUNDARY-SELECTION-001"
)
NEXT_CARD = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-"
    "WRITE-BOUNDARY-BASIS-001"
)
PREVIOUS = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-direct-closeout-rerun-v0.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    previous = read_json(PREVIOUS)

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownWriteSetMapStoreAnyBoundarySelectionV1",
        "token": TOKEN,
        "input_state": {
            "previous_card_fixture": rel(PREVIOUS),
            "previous_card_hash": sha256_file(PREVIOUS),
            "previous_selected_next_card": previous.get("decision", {}).get("selected_next_card"),
            "remaining_write_scoped_surfaces": previous.get("remaining_write_scoped_surfaces"),
            "remaining_write_surface_blockers": previous.get("remaining_write_surface_blockers"),
            "accepted_scoped_closeout_count": previous.get("summary", {}).get(
                "accepted_scoped_closeout_count"
            ),
        },
        "consultation_result": {
            "selected_option": "B",
            "selected_basis": "AnyWriteBoundaryBasis",
            "reason_token": "MapStoreAnyRequiresAnyWriteBoundaryBeforeHakoPilot",
            "mapstore_any_within_scalar_known_closeout_chain": True,
            "immediate_hako_pilot_allowed": False,
            "scalar_known_lane_escape_selected": False,
            "partial_closeout_escape_selected": False,
        },
        "selection_rule": {
            "name": "MapStoreAnyBoundarySelectionConsultationV1",
            "basis_first": True,
            "hako_pilot_requires_boundary_basis": True,
            "any_write_boundary_declared_at_next_basis": True,
            "any_write_boundary_opened_at_selection": False,
            "route_count_as_proof": False,
            "apparent_simplicity_as_proof": False,
            "manual_subsurface_selection": False,
            "source_path_as_authority": False,
            "owner_name_as_proof": False,
            "route_membership_alone_as_proof": False,
        },
        "summary": {
            "mapstore_any_boundary_selection": 1,
            "selected_option_b": 1,
            "selected_next_is_boundary_basis": 1,
            "mapstore_any_remaining": 1,
            "mapstore_i64_already_scoped_closeout": 1,
            "any_write_boundary_declared": 0,
            "any_write_boundary_opened": 0,
            "mapstore_any_hako_pilot_selected": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectMapStoreAnyWriteBoundaryBasis",
            "reason_token": "ConsultationSelectedBasisFirstForAnyWriteBoundary",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "mapstore_any_boundary_selection": 1,
            "selected_option_b": 1,
            "selected_next_is_boundary_basis": 1,
            "any_write_boundary_declared": 0,
            "any_write_boundary_opened": 0,
            "mapstore_any_hako_pilot_selected": 0,
            "mapstore_any_direct_closeout_materialized": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "source_selfhost_claim": 0,
            "new_route_authority": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "runtime_fallback": 0,
            "behavior_change": 0,
            "hako_generation": 0,
            "native_seed_materialization": 0,
            "route_count_as_proof": 0,
            "apparent_simplicity_as_proof": 0,
            "manual_subsurface_selection": 0,
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
        print("mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-boundary-selection unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
