#!/usr/bin/env python3
"""Freeze the Set MapStoreI64 Rust oracle fixture."""

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
    / "mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-rust-oracle-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-"
    "RUST-ORACLE-PARITY-FIXTURE-001"
)
NEXT_CARD = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-"
    "HAKO-PARITY-PILOT-001"
)

SPLIT_BASIS = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-set-surface-typed-value-split-basis-v0.json"
)
WRITE_SOURCE = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"
DESCRIPTORS = ROOT / "src/mir/generated/generic_method_route_descriptors.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    basis = read_json(SPLIT_BASIS)

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownWriteSetMapStoreI64RustOracleParityFixtureV1",
        "token": TOKEN,
        "input_state": {
            "set_surface_typed_value_split_basis": rel(SPLIT_BASIS),
            "set_surface_typed_value_split_basis_hash": sha256_file(SPLIT_BASIS),
            "basis_decision": basis.get("decision", {}).get("kind"),
            "basis_selected_next_card": basis.get("decision", {}).get("selected_next_card"),
            "proof_axis": basis.get("proof_axis", {}).get("name"),
        },
        "provenance": {
            "write_source_hash": sha256_file(WRITE_SOURCE),
            "generated_descriptor_hash": sha256_file(DESCRIPTORS),
        },
        "proof_axis": {
            "name": "TypedScalarWriteBeforeAnyWrite",
            "set_surface": "SetSurfacePolicy",
            "selected_route": "MapStoreI64",
            "deferred_route": "MapStoreAny",
            "typed_scalar_write": True,
            "write_value_boundary": "ScalarI64",
            "any_write_boundary_opened": False,
            "route_count_as_proof": False,
            "apparent_simplicity_as_proof": False,
            "accepted_read_contract_similarity_as_proof": False,
            "manual_subsurface_selection": False,
        },
        "oracle_fixture": {
            "fixture_id": "WriteSetMapStoreI64RustOracleV0",
            "row_count": 1,
            "rows": [
                {
                    "case_id": "map_store_i64_set_surface",
                    "subsurface_id": "SetSurfacePolicy",
                    "route_kind": "MapStoreI64",
                    "proof_or_policy_source": "SetSurfacePolicy",
                    "core_method_op": "MapSet",
                    "core_method_lowering_tier": "ColdFallback",
                    "result_class": "NoneResult",
                    "return_shape": "None",
                    "value_demand": "WriteAny",
                    "write_value_boundary": "ScalarI64",
                    "publication_policy": "NonePublication",
                    "effect_class": "mutate",
                    "mutation_class": "MutatesReceiverOrContainer",
                    "hako_role": "classifier_policy_mirror_only",
                }
            ],
        },
        "metadata_boundary": {
            "none_result_metadata_declared": True,
            "none_publication_metadata_reused": True,
            "publication_execution": False,
            "mutate_effect_boundary_reused": True,
            "runtime_mutation_authority": False,
            "any_write_boundary_opened": False,
            "hako_implementation_mirrors_classifier_policy_decision": True,
        },
        "selection_rule": {
            "name": "WriteSetMapStoreI64HakoImplementationPilotFixtureOnlyV1",
            "fixture_only": True,
            "mapstore_any_deferred": True,
            "direct_closeout_materialization_allowed": False,
            "hako_adoption_allowed": False,
            "next_hako_parity_pilot_selected": True,
            "route_count_as_proof": False,
            "apparent_simplicity_as_proof": False,
            "accepted_read_contract_similarity_as_proof": False,
            "manual_subsurface_selection": False,
        },
        "summary": {
            "write_set_mapstore_i64_hako_implementation_candidate": 1,
            "set_surface_policy_scope": 1,
            "mapstore_i64_scope": 1,
            "typed_scalar_write_before_any_write": 1,
            "mapstore_any_deferred": 1,
            "none_result_metadata_declared": 1,
            "none_publication_metadata_reused": 1,
            "mutate_effect_metadata_boundary_reused": 1,
            "rust_oracle_fixture_defined": 1,
            "next_hako_parity_pilot_selected": 1,
            "any_write_boundary_opened": 0,
            "write_direct_closeout_materialized": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectWriteSetMapStoreI64HakoParityPilot",
            "reason_token": "TypedScalarWriteBeforeAnyWrite",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "write_set_mapstore_i64_hako_implementation_candidate": 1,
            "set_surface_policy_scope": 1,
            "mapstore_i64_scope": 1,
            "typed_scalar_write_before_any_write": 1,
            "mapstore_any_deferred": 1,
            "none_result_metadata_declared": 1,
            "none_publication_metadata_reused": 1,
            "mutate_effect_metadata_boundary_reused": 1,
            "rust_oracle_fixture_defined": 1,
            "next_hako_parity_pilot_selected": 1,
            "any_write_boundary_opened": 0,
            "mapstore_any_hako_pilot_selected": 0,
            "write_subsurface_selected_for_closeout": 0,
            "write_direct_closeout_materialized": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "hako_adoption": 0,
            "source_selfhost_claim": 0,
            "new_route_authority": 0,
            "behavior_change": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "manual_subsurface_selection": 0,
            "route_count_as_proof": 0,
            "apparent_simplicity_as_proof": 0,
            "accepted_read_contract_similarity_as_proof": 0,
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
        print("mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-rust-oracle unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
