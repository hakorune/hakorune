#!/usr/bin/env python3
"""Define ScalarKnown uncovered surface classification basis."""

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
    / "mirbuilder-carrier-type-scalar-known-uncovered-surface-classification-basis-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-UNCOVERED-SURFACE-"
    "CLASSIFICATION-BASIS-001"
)
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-UNCOVERED-SURFACE-"
    "CLASSIFICATION-RERUN-001"
)

PREVIOUS_RERUN = (
    FIXTURES / "mirbuilder-carrier-type-scalar-known-transport-closeout-rerun-v0.json"
)
SOURCE_FILES = [
    ROOT / "src/mir/generic_method_route_plan/string_routes.rs",
    ROOT / "src/mir/generic_method_route_plan/collection_read_routes.rs",
    ROOT / "src/mir/generic_method_route_plan/write_routes.rs",
    ROOT / "src/mir/generic_method_route_plan/tests/string_routes/search_routes.rs",
    ROOT / "src/mir/generic_method_route_plan/tests/string_routes/len_routes.rs",
    ROOT / "src/mir/generic_method_route_plan/tests/map_set_routes/collection_routes.rs",
]

CLASSIFICATION_DIMENSIONS = [
    "surface_id",
    "route_kind_set",
    "method_surface",
    "return_shape",
    "value_demand",
    "publication_policy",
    "proof_or_policy_source",
    "core_method_op",
    "core_method_lowering_tier",
    "effect_class",
    "receiver_key_value_result_origin_evidence",
    "test_anchor",
]

SURFACE_CLASSES = [
    {
        "surface_id": "StringScalarI64Routes",
        "candidate_contract_id": "StringSearchScalarI64TypedDirectCloseoutContract",
        "route_kind_set": [
            "StringIndexOf",
            "StringLastIndexOf",
            "StringContains",
        ],
        "method_surface": ["indexOf", "lastIndexOf", "contains"],
        "return_shape": "ScalarI64",
        "value_demand": "ScalarI64",
        "publication_policy": "NoPublication",
        "proof_or_policy_source": [
            "IndexOfSurfacePolicy",
            "LastIndexOfSurfacePolicy",
            "ContainsSurfacePolicy",
        ],
        "core_method_op": [
            "StringIndexOf",
            "StringLastIndexOf",
            "StringContains",
        ],
        "core_method_lowering_tier": "WarmDirectAbi",
        "effect_class": "read",
        "test_anchor": "src/mir/generic_method_route_plan/tests/string_routes/search_routes.rs",
        "post_classification_priority_hint": "lowest_risk_candidate",
    },
    {
        "surface_id": "CollectionScalarI64Routes",
        "candidate_contract_id": "CollectionLenScalarI64TypedDirectCloseoutContract",
        "route_kind_set": ["MapEntryCount", "ArraySlotLen", "StringLen", "AnyLength"],
        "method_surface": ["len", "length", "count"],
        "return_shape": "ScalarI64",
        "value_demand": "ScalarI64",
        "publication_policy": "NoPublication",
        "proof_or_policy_source": ["LenSurfacePolicy"],
        "core_method_op": ["MapLen", "ArrayLen", "StringLen", "AnyLen"],
        "core_method_lowering_tier": "WarmDirectAbi",
        "effect_class": "observe",
        "test_anchor": "src/mir/generic_method_route_plan/tests/string_routes/len_routes.rs",
        "post_classification_priority_hint": "mixed_with_already_closed_map_load",
    },
    {
        "surface_id": "WriteScalarI64Routes",
        "candidate_contract_id": "WriteResultScalarI64ClassificationOnly",
        "route_kind_set": ["ArrayAppendAny", "MapDeleteAny", "MapStoreI64", "MapStoreAny"],
        "method_surface": ["push", "delete", "set"],
        "return_shape": "ScalarI64OrNoneMixed",
        "value_demand": "WriteAny",
        "publication_policy": "MixedNoPublicationAndNone",
        "proof_or_policy_source": ["PushSurfacePolicy", "DeleteSurfacePolicy", "SetSurfacePolicy"],
        "core_method_op": ["ArrayPush", "MapDelete", "MapSet"],
        "core_method_lowering_tier": "ColdFallbackOrWarmDirectAbiMixed",
        "effect_class": "mutate",
        "test_anchor": "src/mir/generic_method_route_plan/tests/map_set_routes/collection_routes.rs",
        "post_classification_priority_hint": "do_not_select_before_write_result_policy",
    },
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    previous = read_json(PREVIOUS_RERUN)
    uncovered = previous.get("uncovered_scalar_known_surfaces") or []

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownUncoveredSurfaceClassificationBasisV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "previous_rerun": rel(PREVIOUS_RERUN),
            "previous_reason_token": previous.get("decision", {}).get("reason_token"),
        },
        "provenance": {
            "previous_rerun_hash": sha256_file(PREVIOUS_RERUN),
            "source_file_hashes": [
                {"path": rel(path), "sha256": sha256_file(path)}
                for path in SOURCE_FILES
            ],
        },
        "previous_state": {
            "uncovered_scalar_known_surface_count": previous.get("summary", {}).get(
                "uncovered_scalar_known_surface_count"
            ),
            "scalar_known_transport_axis_closeout": previous.get("summary", {}).get(
                "scalar_known_transport_axis_closeout"
            ),
            "scoped_map_load_scalar_i64_closeout": previous.get("summary", {}).get(
                "scoped_map_load_scalar_i64_closeout"
            ),
            "selected_next_card": previous.get("decision", {}).get("selected_next_card"),
        },
        "uncovered_input_surfaces": uncovered,
        "classification_dimensions": CLASSIFICATION_DIMENSIONS,
        "surface_classes": SURFACE_CLASSES,
        "selection_rule": {
            "name": "ScalarKnownUncoveredSurfaceClassificationBasisOnlyV1",
            "basis_only": True,
            "direct_surface_selection_allowed": False,
            "classification_rerun_required_before_contract_selection": True,
            "route_membership_alone_as_proof": False,
            "source_path_as_authority": False,
            "owner_name_as_proof": False,
            "row_count_as_proof": False,
        },
        "summary": {
            "classification_basis": 1,
            "classified_surface_count": len(SURFACE_CLASSES),
            "direct_contract_selection": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectScalarKnownUncoveredSurfaceClassificationRerun",
            "reason_token": "ScalarKnownUncoveredSurfaceClassificationBasisDefined",
            "selected_surface_id": None,
            "selected_contract_id": None,
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "scalar_known_uncovered_surface_classification_basis": 1,
            "classification_dimensions_defined": 1,
            "basis_only": 1,
            "direct_contract_selection": 0,
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
        print("mirbuilder-carrier-type-scalar-known-uncovered-surface-classification-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
