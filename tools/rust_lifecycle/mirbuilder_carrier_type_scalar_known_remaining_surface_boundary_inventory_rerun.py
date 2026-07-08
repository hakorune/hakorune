#!/usr/bin/env python3
"""Rerun ScalarKnown remaining-surface boundary inventory after Rust boundary refactor."""

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
    / "mirbuilder-carrier-type-scalar-known-remaining-surface-boundary-inventory-rerun-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-REMAINING-SURFACE-"
    "BOUNDARY-INVENTORY-RERUN-001"
)
NEXT_CARD = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-COLLECTION-LEN-SCALAR-I64-"
    "CONTRACT-BASIS-001"
)

BOUNDARY_BASIS = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-remaining-surface-boundary-inventory-basis-v0.json"
)
RUST_REFACTOR = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-rust-typed-direct-closeout-contract-boundary-refactor-v0.json"
)
RUST_BOUNDARY_SOURCE = (
    ROOT / "src/mir/generic_method_route_plan/scalar_known_typed_direct_closeout_contract.rs"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    boundary_basis = read_json(BOUNDARY_BASIS)
    rust_refactor = read_json(RUST_REFACTOR)

    evaluated_surfaces = [
        {
            "surface_id": "CollectionScalarI64Routes",
            "candidate_contract_id": "CollectionLenScalarI64TypedDirectCloseoutContract",
            "rust_boundary_status": "CandidateNeedsPolicy",
            "route_kind_set": ["MapEntryCount", "ArraySlotLen", "StringLen", "AnyLength"],
            "return_shape": "ScalarI64",
            "value_demand": "ScalarI64",
            "publication_policy": "NoPublication",
            "effect_class": "observe",
            "collection_boundary_separated_from_map_load": True,
            "write_result_policy_required": False,
            "selection_eligible": True,
            "blocked_by": [],
            "selected_next_card_if_eligible": NEXT_CARD,
        },
        {
            "surface_id": "WriteScalarI64Routes",
            "candidate_contract_id": "WriteResultScalarI64ClassificationOnly",
            "rust_boundary_status": "CandidateNeedsPolicy",
            "route_kind_set": ["ArrayAppendAny", "MapDeleteAny", "MapStoreI64", "MapStoreAny"],
            "return_shape": "ScalarI64OrNoneMixed",
            "value_demand": "WriteAny",
            "publication_policy": "MixedNoPublicationAndNone",
            "effect_class": "mutate",
            "collection_boundary_separated_from_map_load": None,
            "write_result_policy_required": True,
            "selection_eligible": False,
            "blocked_by": ["WriteResultPolicyRequiredBeforeDirectCloseout"],
            "selected_next_card_if_eligible": None,
        },
    ]

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownRemainingSurfaceBoundaryInventoryRerunV1",
        "token": TOKEN,
        "input_state": {
            "boundary_inventory_basis": rel(BOUNDARY_BASIS),
            "rust_boundary_refactor": rel(RUST_REFACTOR),
            "accepted_contracts": rust_refactor.get("accepted_contracts"),
            "remaining_candidate_surfaces": rust_refactor.get("remaining_candidate_surfaces"),
        },
        "provenance": {
            "boundary_inventory_basis_hash": sha256_file(BOUNDARY_BASIS),
            "rust_boundary_refactor_hash": sha256_file(RUST_REFACTOR),
            "rust_boundary_source_hash": sha256_file(RUST_BOUNDARY_SOURCE),
        },
        "prior_boundary_summary": boundary_basis.get("summary"),
        "evaluated_surfaces": evaluated_surfaces,
        "selection_rule": {
            "name": "ScalarKnownRemainingSurfaceBoundaryInventoryRerunV1",
            "exactly_one_selection_eligible_surface_required": True,
            "collection_requires_boundary_separated_from_map_load": True,
            "write_requires_write_result_policy": True,
            "route_membership_alone_as_proof": False,
            "source_path_as_authority": False,
            "owner_name_as_proof": False,
            "row_count_as_proof": False,
        },
        "summary": {
            "remaining_surface_boundary_inventory_rerun": 1,
            "evaluated_surface_count": len(evaluated_surfaces),
            "selection_eligible_surface_count": 1,
            "selected_surface_id": "CollectionScalarI64Routes",
            "selected_contract_id": "CollectionLenScalarI64TypedDirectCloseoutContract",
            "collection_boundary_separated_from_map_load": 1,
            "write_result_policy_ready": 0,
            "direct_contract_materialized": 0,
            "collection_direct_closeout_ready": 0,
            "write_direct_closeout_ready": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectCollectionLenScalarI64ContractBasis",
            "reason_token": "ExactlyOneRemainingScalarKnownSurfaceBoundaryEligible",
            "selected_surface_id": "CollectionScalarI64Routes",
            "selected_contract_id": "CollectionLenScalarI64TypedDirectCloseoutContract",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "remaining_surface_boundary_inventory_rerun": 1,
            "collection_boundary_separated_from_map_load": 1,
            "direct_contract_selection": 1,
            "direct_contract_materialized": 0,
            "collection_direct_closeout_ready": 0,
            "write_direct_closeout_ready": 0,
            "write_result_policy_ready": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
            "hako_adoption": 0,
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
        print("mirbuilder-carrier-type-scalar-known-remaining-surface-boundary-inventory-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
