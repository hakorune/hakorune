#!/usr/bin/env python3
"""Materialize the scoped Collection `.hako` route-decision authority pilot."""

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
    / "mirbuilder-scalar-known-fastpath-collection-hako-route-decision-authority-pilot-v0.json"
)

TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-HAKO-AUTHORITY-PILOT-RERUN-001"

BASIS = FIXTURES / "mirbuilder-scalar-known-fastpath-collection-hako-authority-pilot-basis-v0.json"
COLLECTION_ARTIFACT = ROOT / "src/mir/generic_method_route_plan/generated/collection_len_scalar_i64_hako_policy.rs"
SHADOW_SOURCE = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
COLLECTION_ROUTES = ROOT / "src/mir/generic_method_route_plan/collection_read_routes.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def file_entry(path: Path) -> dict[str, str]:
    return {"path": rel(path), "sha256": sha256_file(path)}


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathCollectionHakoRouteDecisionAuthorityPilotV1",
        "token": TOKEN,
        "input_state": {
            "basis_fixture": rel(BASIS),
            "basis_fixture_hash": sha256_file(BASIS),
            "basis_selected_next_card": (basis.get("decision") or {}).get("selected_next_card"),
            "basis_defined": (basis.get("summary") or {}).get(
                "collection_hako_authority_pilot_basis"
            ),
        },
        "provenance": {
            "collection_generated_typed_artifact": file_entry(COLLECTION_ARTIFACT),
            "shadow_consumer": file_entry(SHADOW_SOURCE),
            "collection_routes": file_entry(COLLECTION_ROUTES),
        },
        "implementation": {
            "surface": "CollectionScalarI64Routes",
            "route_kind_family": [
                "MapEntryCount",
                "ArraySlotLen",
                "StringLen",
                "AnyLength",
            ],
            "authority_function": "collection_scalar_i64_hako_route_authority_pilot_decision",
            "legacy_shadow_wrapper_retained": "collection_scalar_i64_shadow_consumed_decision",
            "live_route_calls_authority_function": True,
            "hako_decision_constructed_from": "COLLECTION_LEN_SCALAR_I64_HAKO_POLICIES",
            "rust_oracle_decision_constructed": True,
            "rust_oracle_compat_checker": True,
            "mismatch_policy": "FailFast",
            "runtime_source_text_parsing": False,
            "authority_scope": "CollectionScalarI64RoutesOnly",
        },
        "collection_shape": {
            "route_rows": [
                "MapEntryCount:MapLen:MapBox",
                "ArraySlotLen:ArrayLen:ArrayBox",
                "StringLen:StringLen:StringBox",
                "AnyLength:AnyLen:Box",
            ],
            "lowering_tier": "WarmDirectAbi",
            "return_shape": "ScalarI64",
            "value_demand": "ScalarI64",
            "publication_policy": "NoPublication",
            "effect_class": "observe",
            "proof_or_policy_source": "LenSurfacePolicy",
            "any_length_box_domain_is_explicit_row_not_wildcard_selector": True,
        },
        "decision": {
            "kind": "SelectCollectionAuthorityPilotRerun",
            "reason_token": "CollectionHakoRouteDecisionAuthorityPilotMaterialized",
            "selected_next_card": NEXT_CARD,
        },
        "summary": {
            "collection_hako_route_decision_authority_pilot": 1,
            "collection_hako_authority_result_consumed": 1,
            "collection_rust_oracle_compat_checker": 1,
            "collection_mismatch_fail_fast": 1,
            "collection_live_route_calls_authority_pilot": 1,
            "collection_mixed_receiver_domain_guarded": 1,
            "collection_anylength_box_domain_guarded": 1,
            "collection_anylength_global_box_authority": 0,
            "any_length_wildcard_selector": 0,
            "runtime_box_domain_fallback": 0,
            "scalar_known_hako_runtime_route_authority": 0,
            "source_selfhost_claim": 0,
        },
        "claims": {
            "collection_hako_route_decision_authority_pilot": 1,
            "collection_hako_authority_result_consumed": 1,
            "collection_rust_oracle_compat_checker": 1,
            "collection_mismatch_fail_fast": 1,
            "collection_live_route_calls_authority_pilot": 1,
            "collection_mixed_receiver_domain_guarded": 1,
            "collection_anylength_box_domain_guarded": 1,
            "collection_anylength_global_box_authority": 0,
            "receiver_domain_authority_switch": 0,
            "receiver_domain_widening_authority": 0,
            "receiver_domain_projection": 0,
            "any_length_wildcard_selector": 0,
            "runtime_box_domain_fallback": 0,
            "read_surface_authority_closeout": 0,
            "scalar_known_hako_runtime_route_authority": 0,
            "scalar_known_transport_axis_authority_switch": 0,
            "rust_fastpath_rewired": 0,
            "route_selection_authority_switch": 0,
            "backend_lowering_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "caller_orientation_runtime_path": 0,
            "build_rs_hako_compiler_invocation": 0,
            "live_hako_authority": 0,
            "source_selfhost_claim": 0,
            "hako_generation": 0,
            "new_route_authority": 0,
            "behavior_change": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "native_seed_materialization": 0,
            "new_python_semantic_projector": 0,
            "manual_surface_selection": 0,
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
        print("mirbuilder-scalar-known-fastpath-collection-hako-route-decision-authority-pilot unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
