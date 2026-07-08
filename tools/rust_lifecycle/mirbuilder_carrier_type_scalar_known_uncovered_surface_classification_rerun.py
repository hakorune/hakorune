#!/usr/bin/env python3
"""Rerun ScalarKnown uncovered surface classification."""

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
    / "mirbuilder-carrier-type-scalar-known-uncovered-surface-classification-rerun-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-UNCOVERED-SURFACE-"
    "CLASSIFICATION-RERUN-001"
)
NEXT_CARD = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-STRING-SEARCH-SCALAR-I64-"
    "TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001"
)

BASIS = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-uncovered-surface-classification-basis-v0.json"
)
STRING_SOURCE = ROOT / "src/mir/generic_method_route_plan/string_routes.rs"
STRING_TEST = ROOT / "src/mir/generic_method_route_plan/tests/string_routes/search_routes.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def classify(row: dict[str, Any]) -> dict[str, Any]:
    priority_hint = row.get("post_classification_priority_hint")
    effect_class = row.get("effect_class")
    tier = row.get("core_method_lowering_tier")
    value_demand = row.get("value_demand")
    publication_policy = row.get("publication_policy")
    test_anchor = row.get("test_anchor")

    selected = (
        priority_hint == "lowest_risk_candidate"
        and effect_class == "read"
        and tier == "WarmDirectAbi"
        and value_demand == "ScalarI64"
        and publication_policy == "NoPublication"
        and test_anchor == "src/mir/generic_method_route_plan/tests/string_routes/search_routes.rs"
    )
    blocked_by = []
    if row.get("surface_id") == "CollectionScalarI64Routes":
        blocked_by.append("MixedWithAlreadyClosedMapLoadScalarI64")
    if row.get("surface_id") == "WriteScalarI64Routes":
        blocked_by.append("WriteResultPolicyRequiredBeforeDirectCloseout")

    return {
        **row,
        "classification_eligible": selected,
        "selection_reason": "LowestRiskReadOnlyWarmDirectScalarI64NoPublication"
        if selected
        else None,
        "blocked_by": blocked_by,
    }


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    rows = [classify(row) for row in basis.get("surface_classes") or []]
    selected_rows = [row for row in rows if row.get("classification_eligible")]
    selected = selected_rows[0] if len(selected_rows) == 1 else None

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownUncoveredSurfaceClassificationRerunV1",
        "token": TOKEN,
        "input_state": {
            "classification_basis": rel(BASIS),
            "previous_decision": basis.get("decision", {}).get("kind"),
        },
        "provenance": {
            "classification_basis_hash": sha256_file(BASIS),
            "string_source_hash": sha256_file(STRING_SOURCE),
            "string_test_hash": sha256_file(STRING_TEST),
        },
        "classification_rows": rows,
        "summary": {
            "classified_surface_count": len(rows),
            "selection_eligible_surface_count": len(selected_rows),
            "selected_surface_count": 1 if selected else 0,
            "direct_contract_materialized": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectStringSearchScalarI64TypedDirectCloseoutContractBasis"
            if selected
            else "KeepStopped",
            "reason_token": "ExactlyOneScalarKnownUncoveredSurfaceClassified"
            if selected
            else "NoScalarKnownUncoveredSurfaceClassified",
            "selected_surface_id": selected.get("surface_id") if selected else None,
            "selected_contract_id": selected.get("candidate_contract_id") if selected else None,
            "selected_next_card": NEXT_CARD if selected else None,
        },
        "claims": {
            "scalar_known_uncovered_surface_classification_rerun": 1,
            "string_search_scalar_i64_contract_selected": 1 if selected else 0,
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
        print("mirbuilder-carrier-type-scalar-known-uncovered-surface-classification-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
