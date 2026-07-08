#!/usr/bin/env python3
"""Define the narrow MapLoadScalarI64 typed direct closeout contract basis."""

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
    / "mirbuilder-carrier-type-scalar-known-map-load-i64-typed-direct-closeout-contract-basis-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-MAP-LOAD-I64-TYPED-DIRECT-"
    "CLOSEOUT-CONTRACT-BASIS-001"
)
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
PREVIOUS_RERUN_TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-"
    "REQUIREMENT-RERUN-002"
)
NEXT_CARD = (
    "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-"
    "REQUIREMENT-RERUN-003"
)

PREVIOUS_RERUN = (
    FIXTURES
    / "mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun-002-v0.json"
)
SOURCE_DISCOVERY_BASIS = (
    FIXTURES
    / "mirbuilder-carrier-type-transport-component-evidence-source-discovery-basis-v0.json"
)
SOURCE_DISCOVERY_INVENTORY = (
    FIXTURES
    / "mirbuilder-carrier-type-transport-component-evidence-source-discovery-inventory-v0.json"
)

SOURCE_FILES = [
    ROOT / "src/mir/generic_method_route_plan/map_set_scalar_proof.rs",
    ROOT / "src/mir/generic_method_route_plan/collection_read_routes.rs",
    ROOT / "src/mir/generic_method_route_plan/tests/scalar_proof.rs",
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    previous_rerun = read_json(PREVIOUS_RERUN)
    source_basis = read_json(SOURCE_DISCOVERY_BASIS)
    source_inventory = read_json(SOURCE_DISCOVERY_INVENTORY)

    return {
        "schema_version": 0,
        "kind": (
            "MirBuilderCarrierTypeScalarKnownMapLoadI64"
            "TypedDirectCloseoutContractBasisV1"
        ),
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "previous_rerun_token": PREVIOUS_RERUN_TOKEN,
            "previous_rerun": rel(PREVIOUS_RERUN),
            "component_evidence_source_discovery_basis": rel(
                SOURCE_DISCOVERY_BASIS
            ),
            "component_evidence_source_discovery_inventory": rel(
                SOURCE_DISCOVERY_INVENTORY
            ),
        },
        "provenance": {
            "previous_rerun_hash": sha256_file(PREVIOUS_RERUN),
            "component_evidence_source_discovery_basis_hash": sha256_file(
                SOURCE_DISCOVERY_BASIS
            ),
            "component_evidence_source_discovery_inventory_hash": sha256_file(
                SOURCE_DISCOVERY_INVENTORY
            ),
            "source_file_hashes": [
                {"path": rel(path), "sha256": sha256_file(path)}
                for path in SOURCE_FILES
            ],
        },
        "previous_state": {
            "accepted_component_evidence_source_count": previous_rerun.get(
                "summary", {}
            ).get("accepted_component_evidence_source_count"),
            "root_component_requirement_count": previous_rerun.get(
                "summary", {}
            ).get("root_component_requirement_count"),
            "selection_eligible_component_requirement_count": previous_rerun.get(
                "summary", {}
            ).get("selection_eligible_component_requirement_count"),
            "decision": previous_rerun.get("decision", {}).get("kind"),
            "reason_token": previous_rerun.get("decision", {}).get(
                "reason_token"
            ),
            "selected_next_card": previous_rerun.get("decision", {}).get(
                "selected_next_card"
            ),
            "source_inventory_typed_direct_closeout_contract_count": source_inventory.get(
                "summary", {}
            ).get(
                "typed_direct_closeout_contract_count"
            ),
            "allowed_source_kind_count": len(
                source_basis.get("allowed_evidence_source_kinds") or []
            ),
        },
        "target": {
            "component_requirement": "ScalarKnownCloseoutAuthority",
            "candidate_axis": "ScalarKnownTransportAxis",
            "accepted_source_kind": "TypedDirectCloseoutContract",
            "target_requirement_acceptance_claim": 0,
        },
        "contract": {
            "contract_id": (
                "MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract"
            ),
            "route_kind": "MapLoadScalarI64",
            "return_shape": "ScalarI64OrMissingZero",
            "proof_function": "prove_scalar_i64_map_get_store_fact",
            "value_demand": "ScalarI64",
            "publication_policy": "NoPublication",
            "all_rows_join_contract": True,
            "no_carrier_boundary_required_or_already_covered": True,
            "scope_note": (
                "Narrow MapBox scalar i64 get route evidence only; this does "
                "not close ScalarKnownTransportAxis as a whole."
            ),
        },
        "selection_rule": {
            "name": (
                "MapLoadScalarI64TypedDirectCloseoutContractBasisOnlyV1"
            ),
            "basis_only": True,
            "component_specific_card_selection": False,
            "concrete_carrier_type_axis_selection": False,
            "rerun_required_before_selection": True,
            "direct_component_selection_from_zero_root_forbidden": True,
            "source_path_as_authority": False,
            "owner_name_as_proof": False,
            "row_count_as_proof": False,
            "route_membership_alone_as_proof": False,
        },
        "claims": {
            "typed_direct_closeout_contract_basis": 1,
            "map_load_scalar_i64_existing_rust_owner_evidence": 1,
            "scalar_i64_or_missing_zero_return_shape_evidence": 1,
            "scalar_i64_value_demand_evidence": 1,
            "no_publication_policy_evidence": 1,
            "basis_only": 1,
            "rerun_required_before_component_selection": 1,
            "scalar_known_transport_axis_closeout": 0,
            "scalar_known_closeout_authority_accepted_root": 0,
            "target_requirement_acceptance_claim": 0,
            "root_component_requirement_selected": 0,
            "component_specific_card_selection": 0,
            "concrete_carrier_type_axis_selection": 0,
            "source_selfhost_claim": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "source_path_as_authority": 0,
            "owner_name_as_proof": 0,
            "row_count_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "return_type_string_mapping_as_proof": 0,
            "observed_subaxis_set_as_proof": 0,
            "hardcoded_carrier_axis_priority": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
        },
        "decision": {
            "kind": "SelectCarrierTypeRemainingAxisComponentRequirementRerun003",
            "reason_token": "MapLoadScalarI64TypedDirectCloseoutContractBasisDefined",
            "selected_carrier_type_axis": None,
            "selected_component_requirement": None,
            "selected_next_card": NEXT_CARD,
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
        print("mirbuilder-carrier-type-scalar-known-map-load-i64-typed-direct-closeout-contract-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
