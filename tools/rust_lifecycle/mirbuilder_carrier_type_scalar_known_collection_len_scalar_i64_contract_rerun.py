#!/usr/bin/env python3
"""Rerun the CollectionLen ScalarI64 typed direct closeout contract."""

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
    / "mirbuilder-carrier-type-scalar-known-collection-len-scalar-i64-contract-rerun-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-COLLECTION-LEN-SCALAR-I64-"
    "CONTRACT-RERUN-001"
)
NEXT_CARD = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-RESULT-POLICY-BASIS-001"

COLLECTION_BASIS = (
    FIXTURES / "mirbuilder-carrier-type-scalar-known-collection-len-scalar-i64-contract-basis-v0.json"
)
STRING_RERUN = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-string-search-scalar-i64-typed-direct-closeout-contract-rerun-v0.json"
)
RUST_REFACTOR = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-rust-typed-direct-closeout-contract-boundary-refactor-v0.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    basis = read_json(COLLECTION_BASIS)
    string_rerun = read_json(STRING_RERUN)
    rust_refactor = read_json(RUST_REFACTOR)
    prior_closeouts = string_rerun.get("accepted_scoped_closeouts") or []
    collection_contract = {
        "contract_id": "CollectionLenScalarI64TypedDirectCloseoutContract",
        "surface_id": "CollectionScalarI64Routes",
        "routes": ["MapEntryCount", "ArraySlotLen", "StringLen", "AnyLength"],
        "proof_or_policy_source": ["LenSurfacePolicy"],
        "return_shape": "ScalarI64",
        "value_demand": "ScalarI64",
        "publication_policy": "NoPublication",
        "core_method_lowering_tier": "WarmDirectAbi",
        "effect_class": "observe",
    }
    accepted_closeouts = prior_closeouts + [collection_contract]

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownCollectionLenScalarI64ContractRerunV1",
        "token": TOKEN,
        "input_state": {
            "collection_len_basis": rel(COLLECTION_BASIS),
            "prior_string_search_rerun": rel(STRING_RERUN),
            "rust_boundary_refactor": rel(RUST_REFACTOR),
            "rust_candidate_surfaces": rust_refactor.get("remaining_candidate_surfaces"),
        },
        "provenance": {
            "collection_len_basis_hash": sha256_file(COLLECTION_BASIS),
            "prior_string_search_rerun_hash": sha256_file(STRING_RERUN),
            "rust_boundary_refactor_hash": sha256_file(RUST_REFACTOR),
        },
        "accepted_scoped_closeouts": accepted_closeouts,
        "materialized_contract": collection_contract,
        "remaining_candidate_surfaces": ["WriteScalarI64Routes"],
        "write_blocker": "WriteResultPolicyRequiredBeforeDirectCloseout",
        "summary": {
            "collection_len_scalar_i64_contract_materialized": 1,
            "accepted_scoped_closeout_count": len(accepted_closeouts),
            "remaining_candidate_surface_count": 1,
            "remaining_candidate_surface_id": "WriteScalarI64Routes",
            "write_result_policy_ready": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectWriteResultPolicyBasis",
            "reason_token": "CollectionLenScopedCloseoutMaterializedWritePolicyRemains",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "collection_len_scalar_i64_contract_materialized": 1,
            "accepted_scoped_closeout_count": len(accepted_closeouts),
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
        "basis_summary": basis.get("summary"),
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-carrier-type-scalar-known-collection-len-scalar-i64-contract-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
