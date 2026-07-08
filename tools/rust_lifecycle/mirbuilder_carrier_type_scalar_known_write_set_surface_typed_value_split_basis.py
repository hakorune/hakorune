#!/usr/bin/env python3
"""Define the SetSurfacePolicy typed value split basis."""

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
    / "mirbuilder-carrier-type-scalar-known-write-set-surface-typed-value-split-basis-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-SURFACE-"
    "TYPED-VALUE-SPLIT-BASIS-001"
)
NEXT_CARD = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-"
    "RUST-ORACLE-PARITY-FIXTURE-001"
)

POST_DELETE_CLOSEOUT = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-remaining-subsurface-post-delete-closeout-rerun-v0.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def set_surface(post_delete: dict[str, Any]) -> dict[str, Any]:
    rows = post_delete.get("remaining_subsurfaces") or []
    for row in rows:
        if row.get("subsurface_id") == "SetSurfacePolicy":
            return row
    raise SystemExit("SetSurfacePolicy remaining row not found")


def build_fixture() -> dict[str, Any]:
    post_delete = read_json(POST_DELETE_CLOSEOUT)
    set_row = set_surface(post_delete)

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownWriteSetSurfaceTypedValueSplitBasisV1",
        "token": TOKEN,
        "input_state": {
            "post_delete_closeout_rerun": rel(POST_DELETE_CLOSEOUT),
            "post_delete_closeout_rerun_hash": sha256_file(POST_DELETE_CLOSEOUT),
            "post_delete_decision": post_delete.get("decision", {}).get("kind"),
            "recommended_consultation_topic": post_delete.get("decision", {}).get(
                "recommended_consultation_topic"
            ),
            "remaining_subsurface": set_row.get("subsurface_id"),
            "remaining_routes": set_row.get("routes"),
        },
        "proof_axis": {
            "name": "PriorHakoAdoptedWriteSurfaceMetadataCoverageAndTypedScalarWriteBeforeAnyWrite",
            "prior_hako_adopted_write_surface_metadata_coverage": True,
            "set_surface_typed_value_boundary_split_proof_axis": True,
            "typed_scalar_write_before_any_write": True,
            "already_covered_by_push_delete": [
                "MutatesReceiverOrContainerMetadata",
                "NonePublicationMetadata",
            ],
            "new_for_set": [
                "NoneResultMetadata",
                "TypedVsAnyWriteValueBoundary",
            ],
            "forbidden_proof_sources": [
                "route_count",
                "apparent_simplicity",
                "manual_subsurface_selection",
                "accepted_read_contract_similarity",
                "owner_name",
                "source_path",
                "route_membership_alone",
            ],
        },
        "split_plan": {
            "surface": "SetSurfacePolicy",
            "whole_set_hako_pilot_allowed": False,
            "mapstore_i64": {
                "route": "MapStoreI64",
                "first_candidate": True,
                "typed_scalar_write": True,
                "write_value_boundary": "ScalarI64",
                "scalar_known_lane_local": True,
            },
            "mapstore_any": {
                "route": "MapStoreAny",
                "deferred": True,
                "typed_scalar_write": False,
                "write_value_boundary": "Any",
                "requires_any_write_boundary": True,
            },
        },
        "selection_rule": {
            "name": "SetSurfaceTypedValueSplitBasisOnlyV1",
            "basis_only": True,
            "rerun_or_fixture_required_before_hako_pilot": True,
            "set_hako_pilot_selection_allowed": False,
            "mapstore_i64_hako_pilot_selection_allowed": False,
            "mapstore_any_hako_pilot_selection_allowed": False,
            "route_count_as_proof": False,
            "apparent_simplicity_as_proof": False,
            "manual_subsurface_selection": False,
            "accepted_read_contract_similarity_as_proof": False,
            "owner_name_as_proof": False,
            "source_path_as_authority": False,
            "route_membership_alone_as_proof": False,
        },
        "summary": {
            "set_surface_typed_value_split_basis": 1,
            "set_surface_policy_remaining": 1,
            "mapstore_i64_first_candidate": 1,
            "mapstore_any_deferred": 1,
            "typed_scalar_write_before_any_write": 1,
            "prior_hako_adopted_write_surface_metadata_coverage": 1,
            "basis_only": 1,
            "rerun_or_fixture_required_before_hako_pilot": 1,
            "set_hako_pilot_selected": 0,
            "mapstore_i64_hako_pilot_selected": 0,
            "mapstore_any_hako_pilot_selected": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectSetMapStoreI64RustOracleParityFixture",
            "reason_token": "SetSurfaceTypedValueSplitBasisDefined",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "set_surface_typed_value_split_basis": 1,
            "set_surface_policy_remaining": 1,
            "mapstore_i64_first_candidate": 1,
            "mapstore_any_deferred": 1,
            "typed_scalar_write_before_any_write": 1,
            "prior_hako_adopted_write_surface_metadata_coverage": 1,
            "basis_only": 1,
            "rerun_or_fixture_required_before_hako_pilot": 1,
            "set_hako_pilot_selected": 0,
            "mapstore_i64_hako_pilot_selected": 0,
            "mapstore_any_hako_pilot_selected": 0,
            "set_split_unnecessary": 0,
            "write_direct_closeout_materialized": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "new_route_authority": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "behavior_change": 0,
            "runtime_fallback": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "new_python_semantic_projector": 0,
            "route_count_as_proof": 0,
            "apparent_simplicity_as_proof": 0,
            "manual_subsurface_selection": 0,
            "accepted_read_contract_similarity_as_proof": 0,
            "owner_name_as_proof": 0,
            "source_path_as_authority": 0,
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
        print("mirbuilder-carrier-type-scalar-known-write-set-surface-typed-value-split-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
