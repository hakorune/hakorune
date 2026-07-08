#!/usr/bin/env python3
"""Define the StringSearch ScalarI64 typed direct closeout contract basis."""

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
    / "mirbuilder-carrier-type-scalar-known-string-search-scalar-i64-typed-direct-closeout-contract-basis-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-STRING-SEARCH-SCALAR-I64-"
    "TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001"
)
NEXT_CARD = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-STRING-SEARCH-SCALAR-I64-"
    "TYPED-DIRECT-CLOSEOUT-CONTRACT-RERUN-001"
)

PREVIOUS_RERUN = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-uncovered-surface-classification-rerun-v0.json"
)
STRING_SOURCE = ROOT / "src/mir/generic_method_route_plan/string_routes.rs"
STRING_TEST = ROOT / "src/mir/generic_method_route_plan/tests/string_routes/search_routes.rs"

STRING_SEARCH_ROUTES = [
    {
        "route_kind": "StringIndexOf",
        "method_surface": "indexOf",
        "proof_or_policy_source": "IndexOfSurfacePolicy",
        "core_method_op": "StringIndexOf",
    },
    {
        "route_kind": "StringLastIndexOf",
        "method_surface": "lastIndexOf",
        "proof_or_policy_source": "LastIndexOfSurfacePolicy",
        "core_method_op": "StringLastIndexOf",
    },
    {
        "route_kind": "StringContains",
        "method_surface": "contains",
        "proof_or_policy_source": "ContainsSurfacePolicy",
        "core_method_op": "StringContains",
    },
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    previous = read_json(PREVIOUS_RERUN)

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownStringSearchScalarI64TypedDirectCloseoutContractBasisV1",
        "token": TOKEN,
        "input_state": {
            "classification_rerun": rel(PREVIOUS_RERUN),
            "selected_surface_id": previous.get("decision", {}).get("selected_surface_id"),
            "selected_contract_id": previous.get("decision", {}).get("selected_contract_id"),
        },
        "provenance": {
            "classification_rerun_hash": sha256_file(PREVIOUS_RERUN),
            "string_source_hash": sha256_file(STRING_SOURCE),
            "string_test_hash": sha256_file(STRING_TEST),
        },
        "contract": {
            "contract_id": "StringSearchScalarI64TypedDirectCloseoutContract",
            "source_kind": "TypedDirectCloseoutContract",
            "target_axis": "ScalarKnownTransportAxis",
            "surface_id": "StringScalarI64Routes",
            "routes": STRING_SEARCH_ROUTES,
            "return_shape": "ScalarI64",
            "value_demand": "ScalarI64",
            "publication_policy": "NoPublication",
            "core_method_lowering_tier": "WarmDirectAbi",
            "effect_class": "read",
            "test_anchor": rel(STRING_TEST),
            "all_rows_join_contract": True,
            "no_carrier_boundary_required_or_already_covered": True,
        },
        "selection_rule": {
            "name": "StringSearchScalarI64TypedDirectCloseoutContractBasisOnlyV1",
            "basis_only": True,
            "contract_materialization_requires_rerun": True,
            "axis_closeout_forbidden_at_basis": True,
            "source_path_as_authority": False,
            "owner_name_as_proof": False,
            "row_count_as_proof": False,
            "route_membership_alone_as_proof": False,
        },
        "summary": {
            "typed_direct_closeout_contract_basis": 1,
            "string_search_route_count": len(STRING_SEARCH_ROUTES),
            "direct_contract_materialized": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectStringSearchScalarI64TypedDirectCloseoutContractRerun",
            "reason_token": "StringSearchScalarI64TypedDirectCloseoutContractBasisDefined",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "string_search_scalar_i64_typed_direct_closeout_contract_basis": 1,
            "basis_only": 1,
            "direct_contract_materialized": 0,
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
        print("mirbuilder-carrier-type-scalar-known-string-search-scalar-i64-typed-direct-closeout-contract-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
