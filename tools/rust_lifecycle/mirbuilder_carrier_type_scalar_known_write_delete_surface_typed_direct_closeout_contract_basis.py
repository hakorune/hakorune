#!/usr/bin/env python3
"""Define the Write Delete surface typed direct closeout contract basis."""

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
    / "mirbuilder-carrier-type-scalar-known-write-delete-surface-typed-direct-closeout-contract-basis-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-"
    "TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001"
)
NEXT_CARD = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-"
    "DIRECT-CLOSEOUT-RERUN-001"
)

POST_DELETE_RERUN = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-remaining-subsurface-post-delete-adoption-rerun-v0.json"
)
ADOPTION = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-delete-surface-hako-adoption-decision-v0.json"
)
WRITE_SOURCE = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"
HAKO_SOURCE = ROOT / "lang/src/compiler/lib/write_delete_surface_policy_classifier.hako"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    rerun = read_json(POST_DELETE_RERUN)
    adoption = read_json(ADOPTION)

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownWriteDeleteSurfaceTypedDirectCloseoutContractBasisV1",
        "token": TOKEN,
        "input_state": {
            "post_delete_adoption_rerun": rel(POST_DELETE_RERUN),
            "post_delete_adoption_rerun_hash": sha256_file(POST_DELETE_RERUN),
            "write_delete_surface_adoption_decision": rel(ADOPTION),
            "write_delete_surface_adoption_hash": sha256_file(ADOPTION),
            "selected_write_subsurface": rerun.get("decision", {}).get("selected_subsurface"),
            "selected_next_card_from_rerun": rerun.get("decision", {}).get("selected_next_card"),
            "adoption_decision": adoption.get("adoption_decision", {}).get("decision"),
        },
        "provenance": {
            "write_source_hash": sha256_file(WRITE_SOURCE),
            "hako_source_hash": sha256_file(HAKO_SOURCE),
        },
        "contract": {
            "contract_id": "WriteDeleteSurfaceTypedDirectCloseoutContract",
            "source_kind": "TypedDirectCloseoutContract",
            "target_axis": "ScalarKnownTransportAxis",
            "surface_id": "WriteScalarI64Routes",
            "subsurface_id": "DeleteSurfacePolicy",
            "route_kind_set": ["MapDeleteAny"],
            "proof_or_policy_source": ["DeleteSurfacePolicy"],
            "core_method_op": "MapDelete",
            "core_method_lowering_tier": "ColdFallback",
            "result_class": "ScalarI64Result",
            "return_shape": "ScalarI64",
            "value_demand": "WriteAny",
            "publication_policy": "NonePublication",
            "effect_class": "mutate",
            "mutation_class": "MutatesReceiverOrContainer",
            "hako_owner": "write_delete_surface_policy_classifier",
            "hako_adopted_source": rel(HAKO_SOURCE),
            "runtime_mutation_authority": False,
            "publication_execution": False,
            "push_surface_policy_included": False,
            "set_surface_policy_included": False,
        },
        "selection_rule": {
            "name": "WriteDeleteSurfaceTypedDirectCloseoutContractBasisOnlyV1",
            "basis_only": True,
            "contract_materialization_requires_rerun": True,
            "whole_write_surface_closeout_forbidden_at_basis": True,
            "axis_closeout_forbidden_at_basis": True,
            "source_path_as_authority": False,
            "owner_name_as_proof": False,
            "row_count_as_proof": False,
            "route_count_as_proof": False,
            "route_membership_alone_as_proof": False,
        },
        "summary": {
            "write_delete_surface_typed_direct_closeout_contract_basis": 1,
            "write_delete_route_count": 1,
            "direct_contract_materialized": 0,
            "write_delete_direct_closeout_ready": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectWriteDeleteSurfaceDirectCloseoutRerun",
            "reason_token": "WriteDeleteSurfaceTypedDirectCloseoutContractBasisDefined",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "write_delete_surface_typed_direct_closeout_contract_basis": 1,
            "basis_only": 1,
            "direct_contract_materialized": 0,
            "write_delete_direct_closeout_ready": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
            "hako_generation": 0,
            "new_route_authority": 0,
            "behavior_change": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "native_seed_materialization": 0,
            "new_python_semantic_projector": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
            "manual_subsurface_selection": 0,
            "row_count_as_proof": 0,
            "route_count_as_proof": 0,
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
        print("mirbuilder-carrier-type-scalar-known-write-delete-surface-typed-direct-closeout-contract-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
