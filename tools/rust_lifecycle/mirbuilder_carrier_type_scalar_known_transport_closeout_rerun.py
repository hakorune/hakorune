#!/usr/bin/env python3
"""Rerun ScalarKnown transport closeout after the closeout basis."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = (
    FIXTURES / "mirbuilder-carrier-type-scalar-known-transport-closeout-rerun-v0.json"
)

TOKEN = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-RERUN-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

BASIS = FIXTURES / "mirbuilder-carrier-type-scalar-known-transport-closeout-basis-v0.json"
SOURCE_FILES = [
    ROOT / "src/mir/generic_method_route_plan/collection_read_routes.rs",
    ROOT / "src/mir/generic_method_route_plan/string_routes.rs",
    ROOT / "src/mir/generic_method_route_plan/write_routes.rs",
]

UNCOVERED_SCALAR_SURFACES = [
    {
        "surface_id": "StringScalarI64Routes",
        "source": "src/mir/generic_method_route_plan/string_routes.rs",
        "reason": "ScalarI64NoPublicationRouteOutsideMapLoadScalarI64Contract",
    },
    {
        "surface_id": "CollectionScalarI64Routes",
        "source": "src/mir/generic_method_route_plan/collection_read_routes.rs",
        "reason": "ScalarI64NoPublicationRouteOutsideMapLoadScalarI64Contract",
    },
    {
        "surface_id": "WriteScalarI64Routes",
        "source": "src/mir/generic_method_route_plan/write_routes.rs",
        "reason": "ScalarI64NoPublicationRouteOutsideMapLoadScalarI64Contract",
    },
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    accepted_contracts = basis.get("closeout_basis", {}).get("accepted_contracts") or []

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownTransportCloseoutRerunV1",
        "token": TOKEN,
        "input_state": {
            "closeout_basis": rel(BASIS),
            "current_target_axis": "ScalarKnownTransportAxis",
            "current_target_requirement": "ScalarKnownCloseoutAuthority",
        },
        "provenance": {
            "closeout_basis_hash": sha256_file(BASIS),
            "source_file_hashes": [
                {"path": rel(path), "sha256": sha256_file(path)}
                for path in SOURCE_FILES
            ],
        },
        "accepted_scoped_closeouts": accepted_contracts,
        "uncovered_scalar_known_surfaces": UNCOVERED_SCALAR_SURFACES,
        "summary": {
            "accepted_scoped_closeout_count": len(accepted_contracts),
            "uncovered_scalar_known_surface_count": len(UNCOVERED_SCALAR_SURFACES),
            "scalar_known_transport_axis_closeout": 0,
            "scoped_map_load_scalar_i64_closeout": 1,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "KeepScopedCloseout",
            "reason_token": "ScalarKnownTransportAxisHasUncoveredScalarSurfaces",
            "selected_carrier_type_axis": None,
            "selected_component_requirement": "ScalarKnownCloseoutAuthority",
            "selected_next_card": DESIGN_STOP,
            "consultation_required": True,
        },
        "claims": {
            "scoped_map_load_scalar_i64_closeout": 1,
            "accepted_scoped_closeout_count": len(accepted_contracts),
            "scalar_known_transport_axis_closeout": 0,
            "concrete_carrier_type_axis_selection": 0,
            "source_selfhost_claim": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
            "row_count_as_proof": 0,
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
        print("mirbuilder-carrier-type-scalar-known-transport-closeout-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
