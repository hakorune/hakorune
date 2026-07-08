#!/usr/bin/env python3
"""Define the ScalarKnown transport closeout basis after rerun-003."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = (
    FIXTURES / "mirbuilder-carrier-type-scalar-known-transport-closeout-basis-v0.json"
)

TOKEN = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-BASIS-001"
PREVIOUS_RERUN_TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-"
    "REQUIREMENT-RERUN-003"
)
NEXT_CARD = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-RERUN-001"

PREVIOUS_RERUN = (
    FIXTURES
    / "mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun-003-v0.json"
)
CONTRACT_BASIS = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-map-load-i64-typed-direct-closeout-contract-basis-v0.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def selected_scalar_row(rerun: dict[str, Any]) -> dict[str, Any]:
    for row in rerun.get("component_requirement_rows") or []:
        if row.get("requirement_id") == "ScalarKnownCloseoutAuthority":
            return row
    raise SystemExit("missing ScalarKnownCloseoutAuthority row")


def build_fixture() -> dict[str, Any]:
    rerun = read_json(PREVIOUS_RERUN)
    contract_basis = read_json(CONTRACT_BASIS)
    scalar_row = selected_scalar_row(rerun)

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownTransportCloseoutBasisV1",
        "token": TOKEN,
        "input_state": {
            "previous_rerun_token": PREVIOUS_RERUN_TOKEN,
            "previous_rerun": rel(PREVIOUS_RERUN),
            "typed_direct_closeout_contract_basis": rel(CONTRACT_BASIS),
        },
        "provenance": {
            "previous_rerun_hash": sha256_file(PREVIOUS_RERUN),
            "typed_direct_closeout_contract_basis_hash": sha256_file(CONTRACT_BASIS),
        },
        "previous_state": {
            "accepted_component_evidence_source_count": rerun.get("summary", {}).get(
                "accepted_component_evidence_source_count"
            ),
            "root_component_requirement_count": rerun.get("summary", {}).get(
                "root_component_requirement_count"
            ),
            "selected_component_requirement": rerun.get("decision", {}).get(
                "selected_component_requirement"
            ),
            "selected_next_card": rerun.get("decision", {}).get("selected_next_card"),
            "scalar_known_root_authority_status": scalar_row.get(
                "root_authority_status"
            ),
            "scalar_known_accepted_source_count": len(
                scalar_row.get("accepted_sources") or []
            ),
        },
        "closeout_basis": {
            "target_axis": "ScalarKnownTransportAxis",
            "target_requirement": "ScalarKnownCloseoutAuthority",
            "accepted_contracts": [
                {
                    "contract_id": contract_basis.get("contract", {}).get(
                        "contract_id"
                    ),
                    "source_kind": contract_basis.get("target", {}).get(
                        "accepted_source_kind"
                    ),
                    "route_kind": contract_basis.get("contract", {}).get("route_kind"),
                    "return_shape": contract_basis.get("contract", {}).get(
                        "return_shape"
                    ),
                    "value_demand": contract_basis.get("contract", {}).get(
                        "value_demand"
                    ),
                    "publication_policy": contract_basis.get("contract", {}).get(
                        "publication_policy"
                    ),
                }
            ],
            "basis_only": True,
            "rerun_required_before_axis_closeout": True,
        },
        "selection_rule": {
            "name": "ScalarKnownTransportCloseoutBasisOnlyV1",
            "basis_only": True,
            "axis_closeout_forbidden_at_basis": True,
            "concrete_carrier_type_axis_selection": False,
            "source_selfhost_claim": False,
            "rerun_required_before_closeout": True,
        },
        "claims": {
            "scalar_known_transport_closeout_basis": 1,
            "scalar_known_closeout_authority_root_consumed": 1,
            "map_load_scalar_i64_typed_direct_closeout_contract_consumed": 1,
            "basis_only": 1,
            "rerun_required_before_axis_closeout": 1,
            "scalar_known_transport_axis_closeout": 0,
            "concrete_carrier_type_axis_selection": 0,
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
        "decision": {
            "kind": "SelectScalarKnownTransportCloseoutRerun",
            "reason_token": "ScalarKnownTransportCloseoutBasisDefined",
            "selected_carrier_type_axis": None,
            "selected_component_requirement": "ScalarKnownCloseoutAuthority",
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
        print("mirbuilder-carrier-type-scalar-known-transport-closeout-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
