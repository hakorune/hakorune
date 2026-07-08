#!/usr/bin/env python3
"""Rerun StringSearch ScalarI64 typed direct closeout contract."""

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
    / "mirbuilder-carrier-type-scalar-known-string-search-scalar-i64-typed-direct-closeout-contract-rerun-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-STRING-SEARCH-SCALAR-I64-"
    "TYPED-DIRECT-CLOSEOUT-CONTRACT-RERUN-001"
)
NEXT_CARD = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

BASIS = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-string-search-scalar-i64-typed-direct-closeout-contract-basis-v0.json"
)
PRIOR_CLOSEOUT = (
    FIXTURES / "mirbuilder-carrier-type-scalar-known-transport-closeout-rerun-v0.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    prior = read_json(PRIOR_CLOSEOUT)
    contract = basis.get("contract") or {}

    scoped_closeouts = [
        {
            "contract_id": "MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract",
            "surface_id": "MapLoadScalarI64",
            "source": "prior_scoped_closeout",
        },
        {
            "contract_id": contract.get("contract_id"),
            "surface_id": contract.get("surface_id"),
            "routes": contract.get("routes") or [],
            "return_shape": contract.get("return_shape"),
            "value_demand": contract.get("value_demand"),
            "publication_policy": contract.get("publication_policy"),
            "core_method_lowering_tier": contract.get("core_method_lowering_tier"),
            "effect_class": contract.get("effect_class"),
        },
    ]

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownStringSearchScalarI64TypedDirectCloseoutContractRerunV1",
        "token": TOKEN,
        "input_state": {
            "contract_basis": rel(BASIS),
            "prior_closeout_rerun": rel(PRIOR_CLOSEOUT),
        },
        "provenance": {
            "contract_basis_hash": sha256_file(BASIS),
            "prior_closeout_rerun_hash": sha256_file(PRIOR_CLOSEOUT),
        },
        "accepted_scoped_closeouts": scoped_closeouts,
        "remaining_uncovered_surface_ids": [
            "CollectionScalarI64Routes",
            "WriteScalarI64Routes",
        ],
        "summary": {
            "string_search_scalar_i64_typed_direct_closeout_contract_materialized": 1,
            "accepted_scoped_closeout_count": len(scoped_closeouts),
            "remaining_uncovered_scalar_known_surface_count": 2,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "KeepScopedCloseout",
            "reason_token": "ScalarKnownTransportAxisStillHasUncoveredSurfaces",
            "selected_next_card": NEXT_CARD,
            "consultation_required": True,
        },
        "claims": {
            "string_search_scalar_i64_typed_direct_closeout_contract_materialized": 1,
            "accepted_scoped_closeout_count": len(scoped_closeouts),
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
        print("mirbuilder-carrier-type-scalar-known-string-search-scalar-i64-typed-direct-closeout-contract-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
