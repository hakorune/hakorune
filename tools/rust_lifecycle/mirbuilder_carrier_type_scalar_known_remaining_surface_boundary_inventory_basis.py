#!/usr/bin/env python3
"""Define the ScalarKnown remaining-surface boundary inventory basis."""

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
    / "mirbuilder-carrier-type-scalar-known-remaining-surface-boundary-inventory-basis-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-REMAINING-SURFACE-"
    "BOUNDARY-INVENTORY-BASIS-001"
)
NEXT_CARD = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-REMAINING-SURFACE-"
    "BOUNDARY-INVENTORY-RERUN-001"
)
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

PREVIOUS_RERUN = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-string-search-scalar-i64-typed-direct-closeout-contract-rerun-v0.json"
)
CLASSIFICATION_BASIS = (
    FIXTURES / "mirbuilder-carrier-type-scalar-known-uncovered-surface-classification-basis-v0.json"
)
CLASSIFICATION_RERUN = (
    FIXTURES / "mirbuilder-carrier-type-scalar-known-uncovered-surface-classification-rerun-v0.json"
)
COLLECTION_SOURCE = ROOT / "src/mir/generic_method_route_plan/collection_read_routes.rs"
WRITE_SOURCE = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"
MODEL_SOURCE = ROOT / "src/mir/generic_method_route_plan/model.rs"
ROUTE_REGISTRY = ROOT / "src/llvm_py/generated/generic_method_route_registry.py"


BOUNDARY_DIMENSIONS = [
    "surface_id",
    "candidate_contract_id",
    "route_kind_set",
    "return_shape",
    "value_demand",
    "publication_policy",
    "effect_class",
    "prior_scoped_closeout_overlap",
    "write_result_policy_required",
    "direct_closeout_ready",
]

SURFACE_BOUNDARIES = [
    {
        "surface_id": "CollectionScalarI64Routes",
        "candidate_contract_id": "CollectionLenScalarI64TypedDirectCloseoutContract",
        "route_kind_set": ["MapEntryCount", "ArraySlotLen", "StringLen", "AnyLength"],
        "proof_or_policy_source": ["LenSurfacePolicy"],
        "return_shape": "ScalarI64",
        "value_demand": "ScalarI64",
        "publication_policy": "NoPublication",
        "effect_class": "observe",
        "prior_scoped_closeout_overlap": "MapEntryCount shares MapBox surface family with MapLoadScalarI64",
        "boundary_question": "Can LenSurfacePolicy be accepted as a separate typed direct closeout without extending the prior MapLoadScalarI64 contract?",
        "write_result_policy_required": False,
        "direct_closeout_ready": False,
        "blocked_by": ["CollectionBoundarySeparationFromMapLoadRequired"],
    },
    {
        "surface_id": "WriteScalarI64Routes",
        "candidate_contract_id": "WriteResultScalarI64ClassificationOnly",
        "route_kind_set": ["ArrayAppendAny", "MapDeleteAny", "MapStoreI64", "MapStoreAny"],
        "proof_or_policy_source": [
            "PushSurfacePolicy",
            "DeleteSurfacePolicy",
            "SetSurfacePolicy",
        ],
        "return_shape": "ScalarI64OrNoneMixed",
        "value_demand": "WriteAny",
        "publication_policy": "MixedNoPublicationAndNone",
        "effect_class": "mutate",
        "prior_scoped_closeout_overlap": "none",
        "boundary_question": "Which write result/publication policy is required before a typed direct closeout can be accepted?",
        "write_result_policy_required": True,
        "direct_closeout_ready": False,
        "blocked_by": ["WriteResultPolicyRequiredBeforeDirectCloseout"],
    },
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    previous = read_json(PREVIOUS_RERUN)
    classification_rerun = read_json(CLASSIFICATION_RERUN)

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownRemainingSurfaceBoundaryInventoryBasisV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "previous_rerun": rel(PREVIOUS_RERUN),
            "previous_reason_token": previous.get("decision", {}).get("reason_token"),
            "remaining_uncovered_surface_ids": previous.get("remaining_uncovered_surface_ids"),
            "classification_rerun": rel(CLASSIFICATION_RERUN),
        },
        "provenance": {
            "previous_rerun_hash": sha256_file(PREVIOUS_RERUN),
            "classification_basis_hash": sha256_file(CLASSIFICATION_BASIS),
            "classification_rerun_hash": sha256_file(CLASSIFICATION_RERUN),
            "source_file_hashes": [
                {"path": rel(path), "sha256": sha256_file(path)}
                for path in [
                    COLLECTION_SOURCE,
                    WRITE_SOURCE,
                    MODEL_SOURCE,
                    ROUTE_REGISTRY,
                ]
            ],
        },
        "prior_accepted_scoped_closeouts": previous.get("accepted_scoped_closeouts"),
        "boundary_dimensions": BOUNDARY_DIMENSIONS,
        "surface_boundaries": SURFACE_BOUNDARIES,
        "classification_reference": {
            "selected_surface_id": classification_rerun.get("decision", {}).get(
                "selected_surface_id"
            ),
            "remaining_after_string_search": previous.get("remaining_uncovered_surface_ids"),
        },
        "selection_rule": {
            "name": "ScalarKnownRemainingSurfaceBoundaryInventoryBasisOnlyV1",
            "basis_only": True,
            "direct_contract_selection_allowed": False,
            "collection_direct_closeout_forbidden_at_basis": True,
            "write_direct_closeout_forbidden_at_basis": True,
            "boundary_inventory_rerun_required": True,
            "route_membership_alone_as_proof": False,
            "source_path_as_authority": False,
            "owner_name_as_proof": False,
            "row_count_as_proof": False,
        },
        "summary": {
            "remaining_surface_boundary_inventory_basis": 1,
            "remaining_surface_count": len(SURFACE_BOUNDARIES),
            "collection_surface_inventory": 1,
            "write_surface_inventory": 1,
            "direct_contract_selection": 0,
            "collection_direct_closeout_ready": 0,
            "write_direct_closeout_ready": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectRemainingSurfaceBoundaryInventoryRerun",
            "reason_token": "CollectionMixedWithPriorMapLoadAndWriteResultPolicyUnresolved",
            "selected_surface_id": None,
            "selected_contract_id": None,
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "remaining_surface_boundary_inventory_basis": 1,
            "collection_surface_inventory": 1,
            "write_surface_inventory": 1,
            "basis_only": 1,
            "direct_contract_selection": 0,
            "collection_direct_closeout_ready": 0,
            "write_direct_closeout_ready": 0,
            "scalar_known_transport_axis_closeout": 0,
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
        print("mirbuilder-carrier-type-scalar-known-remaining-surface-boundary-inventory-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
