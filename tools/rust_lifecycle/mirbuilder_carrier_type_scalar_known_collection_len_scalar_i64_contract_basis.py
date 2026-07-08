#!/usr/bin/env python3
"""Define the CollectionLen ScalarI64 typed direct closeout contract basis."""

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
    / "mirbuilder-carrier-type-scalar-known-collection-len-scalar-i64-contract-basis-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-COLLECTION-LEN-SCALAR-I64-"
    "CONTRACT-BASIS-001"
)
NEXT_CARD = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-COLLECTION-LEN-SCALAR-I64-"
    "CONTRACT-RERUN-001"
)

PREVIOUS_RERUN = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-remaining-surface-boundary-inventory-rerun-v0.json"
)
RUST_REFACTOR = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-rust-typed-direct-closeout-contract-boundary-refactor-v0.json"
)
RUST_BOUNDARY_SOURCE = (
    ROOT / "src/mir/generic_method_route_plan/scalar_known_typed_direct_closeout_contract.rs"
)
COLLECTION_SOURCE = ROOT / "src/mir/generic_method_route_plan/collection_read_routes.rs"
COLLECTION_TEST = ROOT / "src/mir/generic_method_route_plan/tests/string_routes/len_routes.rs"

COLLECTION_LEN_ROUTES = [
    {"route_kind": "MapEntryCount", "method_surface": "len|length|size", "core_method_op": "MapLen"},
    {"route_kind": "ArraySlotLen", "method_surface": "len|length|size", "core_method_op": "ArrayLen"},
    {"route_kind": "StringLen", "method_surface": "len|length|size", "core_method_op": "StringLen"},
    {"route_kind": "AnyLength", "method_surface": "len|length|size", "core_method_op": "AnyLen"},
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    previous = read_json(PREVIOUS_RERUN)
    rust_refactor = read_json(RUST_REFACTOR)

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownCollectionLenScalarI64ContractBasisV1",
        "token": TOKEN,
        "input_state": {
            "boundary_inventory_rerun": rel(PREVIOUS_RERUN),
            "rust_boundary_refactor": rel(RUST_REFACTOR),
            "selected_surface_id": previous.get("decision", {}).get("selected_surface_id"),
            "selected_contract_id": previous.get("decision", {}).get("selected_contract_id"),
            "accepted_contracts_before_basis": rust_refactor.get("accepted_contracts"),
        },
        "provenance": {
            "boundary_inventory_rerun_hash": sha256_file(PREVIOUS_RERUN),
            "rust_boundary_refactor_hash": sha256_file(RUST_REFACTOR),
            "rust_boundary_source_hash": sha256_file(RUST_BOUNDARY_SOURCE),
            "collection_source_hash": sha256_file(COLLECTION_SOURCE),
            "collection_test_hash": sha256_file(COLLECTION_TEST),
        },
        "contract": {
            "contract_id": "CollectionLenScalarI64TypedDirectCloseoutContract",
            "source_kind": "TypedDirectCloseoutContract",
            "target_axis": "ScalarKnownTransportAxis",
            "surface_id": "CollectionScalarI64Routes",
            "rust_boundary_status": "CandidateNeedsPolicy",
            "routes": COLLECTION_LEN_ROUTES,
            "proof_or_policy_source": ["LenSurfacePolicy"],
            "return_shape": "ScalarI64",
            "value_demand": "ScalarI64",
            "publication_policy": "NoPublication",
            "core_method_lowering_tier": "WarmDirectAbi",
            "effect_class": "observe",
            "test_anchor": rel(COLLECTION_TEST),
            "separate_from_map_load_contract": True,
            "write_result_policy_required": False,
        },
        "selection_rule": {
            "name": "CollectionLenScalarI64TypedDirectCloseoutContractBasisOnlyV1",
            "basis_only": True,
            "contract_materialization_requires_rerun": True,
            "axis_closeout_forbidden_at_basis": True,
            "source_path_as_authority": False,
            "owner_name_as_proof": False,
            "row_count_as_proof": False,
            "route_membership_alone_as_proof": False,
        },
        "summary": {
            "collection_len_scalar_i64_contract_basis": 1,
            "collection_len_route_count": len(COLLECTION_LEN_ROUTES),
            "direct_contract_materialized": 0,
            "collection_direct_closeout_ready": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectCollectionLenScalarI64ContractRerun",
            "reason_token": "CollectionLenScalarI64ContractBasisDefined",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "collection_len_scalar_i64_contract_basis": 1,
            "basis_only": 1,
            "direct_contract_materialized": 0,
            "collection_direct_closeout_ready": 0,
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
        print("mirbuilder-carrier-type-scalar-known-collection-len-scalar-i64-contract-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
