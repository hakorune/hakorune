#!/usr/bin/env python3
"""Define the ScalarKnown WriteResultPolicy basis."""

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
    / "mirbuilder-carrier-type-scalar-known-write-result-policy-basis-v0.json"
)

TOKEN = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-RESULT-POLICY-BASIS-001"
NEXT_CARD = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-RESULT-POLICY-RERUN-001"

COLLECTION_RERUN = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-collection-len-scalar-i64-contract-rerun-v0.json"
)
BOUNDARY_RERUN = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-remaining-surface-boundary-inventory-rerun-v0.json"
)
RUST_BOUNDARY = (
    ROOT / "src/mir/generic_method_route_plan/scalar_known_typed_direct_closeout_contract.rs"
)
WRITE_SOURCE = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"
WRITE_TEST = ROOT / "src/mir/generic_method_route_plan/tests/map_set_routes/collection_routes.rs"
DESCRIPTORS = ROOT / "src/mir/generated/generic_method_route_descriptors.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    collection_rerun = read_json(COLLECTION_RERUN)
    boundary_rerun = read_json(BOUNDARY_RERUN)

    sub_surfaces = [
        {
            "subsurface_id": "PushSurfacePolicy",
            "routes": ["ArrayAppendAny"],
            "proof_or_policy_source": "PushSurfacePolicy",
            "normalized_result_class": "ScalarI64Result",
            "publication_class": "NoPublication",
            "mutation_class": "MutatesReceiverOrContainer",
            "future_direct_contract_split_allowed": True,
        },
        {
            "subsurface_id": "DeleteSurfacePolicy",
            "routes": ["MapDeleteAny"],
            "proof_or_policy_source": "DeleteSurfacePolicy",
            "normalized_result_class": "ScalarI64Result",
            "publication_class": "NonePublication",
            "mutation_class": "MutatesReceiverOrContainer",
            "future_direct_contract_split_allowed": True,
        },
        {
            "subsurface_id": "SetSurfacePolicy",
            "routes": ["MapStoreI64", "MapStoreAny"],
            "proof_or_policy_source": "SetSurfacePolicy",
            "normalized_result_class": "NoneResult",
            "publication_class": "NonePublication",
            "mutation_class": "MutatesReceiverOrContainer",
            "subcases": [
                {
                    "route_kind": "MapStoreI64",
                    "typed_scalar_write": 1,
                    "write_value_kind": "ScalarI64",
                },
                {
                    "route_kind": "MapStoreAny",
                    "typed_scalar_write": 0,
                    "write_value_kind": "Any",
                },
            ],
            "future_direct_contract_split_allowed": True,
        },
    ]

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownWriteResultPolicyBasisV1",
        "token": TOKEN,
        "input_state": {
            "collection_len_contract_rerun": rel(COLLECTION_RERUN),
            "remaining_surface_boundary_rerun": rel(BOUNDARY_RERUN),
            "accepted_scoped_closeout_count_before_basis": collection_rerun.get("summary", {}).get(
                "accepted_scoped_closeout_count"
            ),
            "remaining_candidate_surface_id": collection_rerun.get("summary", {}).get(
                "remaining_candidate_surface_id"
            ),
            "write_blocker": collection_rerun.get("write_blocker"),
            "boundary_write_row": next(
                (
                    row
                    for row in boundary_rerun.get("evaluated_surfaces", [])
                    if row.get("surface_id") == "WriteScalarI64Routes"
                ),
                None,
            ),
        },
        "provenance": {
            "collection_len_contract_rerun_hash": sha256_file(COLLECTION_RERUN),
            "remaining_surface_boundary_rerun_hash": sha256_file(BOUNDARY_RERUN),
            "rust_boundary_hash": sha256_file(RUST_BOUNDARY),
            "write_source_hash": sha256_file(WRITE_SOURCE),
            "write_test_hash": sha256_file(WRITE_TEST),
            "generated_descriptor_hash": sha256_file(DESCRIPTORS),
        },
        "policy": {
            "policy_id": "WriteResultPolicyV1",
            "target_surface_id": "WriteScalarI64Routes",
            "basis_only": True,
            "route_kind_set": [
                "ArrayAppendAny",
                "MapDeleteAny",
                "MapStoreI64",
                "MapStoreAny",
            ],
            "sub_surfaces": sub_surfaces,
            "mixed_return_publication_decomposition": {
                "observed_return_shape": "ScalarI64OrNoneMixed",
                "observed_publication_policy": "MixedNoPublicationAndNone",
                "normalized_result_classes": [
                    "ScalarI64Result",
                    "NoneResult",
                    "MixedResult",
                ],
                "publication_classes": [
                    "NoPublication",
                    "NonePublication",
                    "MixedNoPublicationAndNone",
                ],
                "mixed_state_is_not_direct_closeout_contract": True,
            },
            "effect_boundary": {
                "effect_class": "mutate",
                "mutation_class": "MutatesReceiverOrContainer",
                "direct_closeout_requires_rerun": True,
            },
        },
        "selection_rule": {
            "name": "WriteResultPolicyBasisOnlyV1",
            "basis_only": True,
            "direct_closeout_materialization_allowed": False,
            "rerun_required_before_direct_closeout": True,
            "subsurface_classification_allowed_in_basis": True,
            "write_surface_direct_closeout_forbidden_at_basis": True,
            "axis_closeout_forbidden_at_basis": True,
            "source_path_as_authority": False,
            "owner_name_as_proof": False,
            "row_count_as_proof": False,
            "route_membership_alone_as_proof": False,
        },
        "summary": {
            "write_result_policy_basis": 1,
            "write_surface_policy_boundary_defined": 1,
            "mutate_effect_boundary_declared": 1,
            "write_subsurface_classification_defined": 1,
            "push_surface_policy_defined": 1,
            "delete_surface_policy_defined": 1,
            "set_surface_policy_defined": 1,
            "mixed_return_publication_policy_declared": 1,
            "basis_only": 1,
            "rerun_required_before_direct_closeout": 1,
            "write_direct_closeout_materialized": 0,
            "write_result_policy_ready": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectWriteResultPolicyRerun",
            "reason_token": "WriteResultPolicyBasisDefined",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "write_result_policy_basis": 1,
            "write_surface_policy_boundary_defined": 1,
            "mutate_effect_boundary_declared": 1,
            "write_subsurface_classification_defined": 1,
            "push_surface_policy_defined": 1,
            "delete_surface_policy_defined": 1,
            "set_surface_policy_defined": 1,
            "mixed_return_publication_policy_declared": 1,
            "basis_only": 1,
            "rerun_required_before_direct_closeout": 1,
            "write_direct_closeout_materialized": 0,
            "write_result_policy_ready": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "component_specific_card_selection": 0,
            "concrete_carrier_type_axis_selection": 0,
            "hako_adoption": 0,
            "source_selfhost_claim": 0,
            "new_route_authority": 0,
            "behavior_change": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
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
        print("mirbuilder-carrier-type-scalar-known-write-result-policy-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
