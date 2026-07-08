#!/usr/bin/env python3
"""Record the ScalarKnown Rust typed direct closeout contract boundary refactor."""

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
    / "mirbuilder-carrier-type-scalar-known-rust-typed-direct-closeout-contract-boundary-refactor-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-RUST-TYPED-DIRECT-CLOSEOUT-"
    "CONTRACT-BOUNDARY-REFACTOR-001"
)
NEXT_CARD = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-REMAINING-SURFACE-"
    "BOUNDARY-INVENTORY-RERUN-001"
)

PREVIOUS_BASIS = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-remaining-surface-boundary-inventory-basis-v0.json"
)
RUST_BOUNDARY_SOURCE = (
    ROOT / "src/mir/generic_method_route_plan/scalar_known_typed_direct_closeout_contract.rs"
)
RUST_ROUTE_PLAN_MOD = ROOT / "src/mir/generic_method_route_plan.rs"

ACCEPTED_CONTRACTS = [
    "MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract",
    "StringSearchScalarI64TypedDirectCloseoutContract",
]
REMAINING_CANDIDATE_SURFACES = [
    "CollectionScalarI64Routes",
    "WriteScalarI64Routes",
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    previous = read_json(PREVIOUS_BASIS)

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownRustTypedDirectCloseoutContractBoundaryRefactorV1",
        "token": TOKEN,
        "input_state": {
            "previous_basis": rel(PREVIOUS_BASIS),
            "previous_selected_next_card": previous.get("decision", {}).get("selected_next_card"),
        },
        "provenance": {
            "previous_basis_hash": sha256_file(PREVIOUS_BASIS),
            "rust_boundary_source_hash": sha256_file(RUST_BOUNDARY_SOURCE),
            "rust_route_plan_mod_hash": sha256_file(RUST_ROUTE_PLAN_MOD),
        },
        "rust_boundary": {
            "source": rel(RUST_BOUNDARY_SOURCE),
            "module_registered_from": rel(RUST_ROUTE_PLAN_MOD),
            "struct_name": "ScalarKnownTypedDirectCloseoutContract",
            "status_enum": "ScalarKnownContractStatus",
            "accepted_status": "AcceptedScopedCloseout",
            "candidate_status": "CandidateNeedsPolicy",
        },
        "accepted_contracts": ACCEPTED_CONTRACTS,
        "remaining_candidate_surfaces": REMAINING_CANDIDATE_SURFACES,
        "behavior_preservation": {
            "route_selection_changed": False,
            "route_kind_semantics_changed": False,
            "return_shape_semantics_changed": False,
            "value_demand_semantics_changed": False,
            "publication_policy_semantics_changed": False,
            "effect_semantics_changed": False,
            "lowering_path_changed": False,
        },
        "summary": {
            "rust_contract_boundary_refactor": 1,
            "scalar_known_typed_direct_closeout_contract_boundary": 1,
            "accepted_scoped_closeout_contract_count": len(ACCEPTED_CONTRACTS),
            "remaining_candidate_surface_count": len(REMAINING_CANDIDATE_SURFACES),
            "behavior_preserved": 1,
            "existing_rust_owner_evidence_repackaged": 1,
            "direct_contract_selection": 0,
            "collection_direct_closeout_ready": 0,
            "write_direct_closeout_ready": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectRemainingSurfaceBoundaryInventoryRerunAfterRustBoundaryRefactor",
            "reason_token": "ScalarKnownTypedDirectCloseoutContractBoundaryRepackaged",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "rust_contract_boundary_refactor": 1,
            "scalar_known_typed_direct_closeout_contract_boundary": 1,
            "behavior_preserved": 1,
            "existing_rust_owner_evidence_repackaged": 1,
            "direct_contract_selection": 0,
            "collection_direct_closeout_ready": 0,
            "write_direct_closeout_ready": 0,
            "scalar_known_transport_axis_closeout": 0,
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
        print("mirbuilder-carrier-type-scalar-known-rust-typed-direct-closeout-contract-boundary-refactor unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
