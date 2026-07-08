#!/usr/bin/env python3
"""Define the Write Set MapStoreAny direct closeout contract basis."""

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
    / "mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-direct-closeout-contract-basis-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-"
    "DIRECT-CLOSEOUT-CONTRACT-BASIS-001"
)
NEXT_CARD = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-"
    "DIRECT-CLOSEOUT-RERUN-001"
)

POST_ADOPTION_RERUN = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-post-adoption-rerun-v0.json"
)
ADOPTION = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-hako-adoption-decision-v0.json"
)
WRITE_SOURCE = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"
HAKO_SOURCE = ROOT / "lang/src/compiler/lib/write_set_mapstore_any_policy_classifier.hako"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    rerun = read_json(POST_ADOPTION_RERUN)
    adoption = read_json(ADOPTION)

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownWriteSetMapStoreAnyDirectCloseoutContractBasisV1",
        "token": TOKEN,
        "input_state": {
            "post_adoption_rerun": rel(POST_ADOPTION_RERUN),
            "post_adoption_rerun_hash": sha256_file(POST_ADOPTION_RERUN),
            "write_set_mapstore_any_adoption_decision": rel(ADOPTION),
            "write_set_mapstore_any_adoption_hash": sha256_file(ADOPTION),
            "selected_scoped_surface": rerun.get("decision", {}).get("selected_scoped_surface"),
            "selected_next_card_from_rerun": rerun.get("decision", {}).get("selected_next_card"),
            "adoption_decision": adoption.get("adoption_decision", {}).get("decision"),
        },
        "provenance": {
            "write_source_hash": sha256_file(WRITE_SOURCE),
            "hako_source_hash": sha256_file(HAKO_SOURCE),
        },
        "contract": {
            "contract_id": "WriteSetMapStoreAnyDirectCloseoutContract",
            "source_kind": "AnyWriteDirectCloseoutContract",
            "target_axis": "ScalarKnownTransportAxis",
            "surface_id": "WriteScalarI64Routes",
            "subsurface_id": "SetSurfacePolicy/MapStoreAny",
            "route_kind_set": ["MapStoreAny"],
            "proof_or_policy_source": ["SetSurfacePolicy", "AnyWriteBoundaryDeclared"],
            "core_method_op": "MapSet",
            "core_method_lowering_tier": "ColdFallback",
            "result_class": "NoneResult",
            "return_shape": "None",
            "value_demand": "WriteAny",
            "write_value_boundary": "Any",
            "publication_policy": "NonePublication",
            "effect_class": "mutate",
            "mutation_class": "MutatesReceiverOrContainer",
            "hako_owner": "write_set_mapstore_any_policy_classifier",
            "hako_adopted_source": rel(HAKO_SOURCE),
            "runtime_mutation_authority": False,
            "publication_execution": False,
            "mapstore_any_included": True,
            "any_write_boundary_declared": True,
            "any_write_boundary_opened": False,
        },
        "selection_rule": {
            "name": "WriteSetMapStoreAnyDirectCloseoutContractBasisOnlyV1",
            "basis_only": True,
            "contract_materialization_requires_rerun": True,
            "whole_set_surface_closeout_forbidden_at_basis": True,
            "whole_write_surface_closeout_forbidden_at_basis": True,
            "axis_closeout_forbidden_at_basis": True,
            "source_path_as_authority": False,
            "owner_name_as_proof": False,
            "row_count_as_proof": False,
            "route_count_as_proof": False,
            "route_membership_alone_as_proof": False,
        },
        "summary": {
            "write_set_mapstore_any_direct_closeout_contract_basis": 1,
            "write_set_mapstore_any_route_count": 1,
            "any_write_boundary_declared": 1,
            "any_write_boundary_opened": 0,
            "direct_contract_materialized": 0,
            "write_set_mapstore_any_direct_closeout_ready": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectWriteSetMapStoreAnyDirectCloseoutRerun",
            "reason_token": "WriteSetMapStoreAnyDirectCloseoutContractBasisDefined",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "write_set_mapstore_any_direct_closeout_contract_basis": 1,
            "basis_only": 1,
            "any_write_boundary_declared": 1,
            "any_write_boundary_opened": 0,
            "direct_contract_materialized": 0,
            "write_set_mapstore_any_direct_closeout_ready": 0,
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
        print("mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-direct-closeout-contract-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
