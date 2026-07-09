#!/usr/bin/env python3
"""Define the Collection `.hako` route-authority pilot basis."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-collection-hako-authority-pilot-basis-v0.json"

TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-HAKO-AUTHORITY-PILOT-BASIS-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001"

DESIGN_STOP = (
    ROOT
    / "docs/development/current/main/phases/phase-296x/3395-MIRBUILDER-SCALAR-KNOWN-FASTPATH-NEXT-HAKO-AUTHORITY-SURFACE-DESIGN-STOP-002.md"
)
COLLECTION_ARTIFACT = ROOT / "src/mir/generic_method_route_plan/generated/collection_len_scalar_i64_hako_policy.rs"
SHADOW_SOURCE = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
COLLECTION_ROUTES = ROOT / "src/mir/generic_method_route_plan/collection_read_routes.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def file_entry(path: Path) -> dict[str, str]:
    return {"path": rel(path), "sha256": sha256_file(path)}


def build_fixture() -> dict[str, Any]:
    route_rows = [
        {
            "route_kind": "MapEntryCount",
            "core_op": "MapLen",
            "receiver_domain": "MapBox",
        },
        {
            "route_kind": "ArraySlotLen",
            "core_op": "ArrayLen",
            "receiver_domain": "ArrayBox",
        },
        {
            "route_kind": "StringLen",
            "core_op": "StringLen",
            "receiver_domain": "StringBox",
        },
        {
            "route_kind": "AnyLength",
            "core_op": "AnyLen",
            "receiver_domain": "Box",
        },
    ]
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathCollectionHakoAuthorityPilotBasisV1",
        "token": TOKEN,
        "input_state": {
            "design_stop_card": rel(DESIGN_STOP),
            "design_stop_card_hash": sha256_file(DESIGN_STOP),
            "mapload_authority_pilot_precedes_collection_basis": True,
            "string_authority_pilot_precedes_collection_basis": True,
        },
        "provenance": {
            "collection_generated_typed_artifact": file_entry(COLLECTION_ARTIFACT),
            "shadow_consumer": file_entry(SHADOW_SOURCE),
            "collection_routes": file_entry(COLLECTION_ROUTES),
        },
        "basis": {
            "basis_only": True,
            "surface": "CollectionScalarI64Routes",
            "route_kind_family": [
                "MapEntryCount",
                "ArraySlotLen",
                "StringLen",
                "AnyLength",
            ],
            "route_rows": route_rows,
            "proof_axis": [
                "PriorScopedReadAuthorityContinuation",
                "LenSurfacePolicyHomogeneousScalarI64NoPublicationObserve",
                "ExplicitEnumeratedMixedReceiverDomainBoundary",
                "AnyLengthBoxDomainIsExplicitRowNotWildcardSelector",
                "GeneratedTypedArtifactMismatchGateCurrent",
                "RustOracleCompatFailFastRetained",
            ],
            "authority_source": "COLLECTION_LEN_SCALAR_I64_HAKO_POLICIES",
            "rust_oracle_compat_checker_retained": True,
            "mismatch_policy": "FailFast",
            "implementation_deferred": True,
            "selected_next_card": NEXT_CARD,
        },
        "homogeneous_axes": {
            "lowering_tier": "WarmDirectAbi",
            "return_shape": "ScalarI64",
            "value_demand": "ScalarI64",
            "publication_policy": "NoPublication",
            "effect_class": "observe",
            "proof_or_policy_source": "LenSurfacePolicy",
        },
        "mixed_receiver_domain_boundary": {
            "explicit_mixed_receiver_domain_enumeration": True,
            "receiver_domain_not_used_as_selection_proof": True,
            "any_length_box_domain_is_explicit_row_not_wildcard_selector": True,
            "any_length_wildcard_selector": False,
            "runtime_box_domain_fallback": False,
        },
        "decision": {
            "kind": "SelectCollectionRouteDecisionAuthorityPilotImplementation",
            "reason_token": "CollectionLenSurfacePolicyHomogeneousAxesMixedDomainBoundaryDeclared",
            "selected_next_card": NEXT_CARD,
        },
        "summary": {
            "collection_hako_authority_pilot_basis": 1,
            "selected_surface": "CollectionScalarI64Routes",
            "selected_route_family": "MapEntryCount_ArraySlotLen_StringLen_AnyLength",
            "prior_scoped_read_authority_continuation": 1,
            "len_surface_policy_homogeneous_scalar_i64_no_publication_observe": 1,
            "generated_typed_artifact_mismatch_gate_current": 1,
            "mixed_receiver_domain_boundary_declared": 1,
            "explicit_mixed_receiver_domain_enumeration": 1,
            "receiver_domain_not_used_as_selection_proof": 1,
            "any_length_box_domain_is_explicit_row_not_wildcard_selector": 1,
            "rust_oracle_compat_checker_retained": 1,
            "mismatch_fail_fast_required": 1,
            "basis_only": 1,
            "authority_pilot_implementation_deferred": 1,
            "collection_hako_route_decision_authority_pilot": 0,
            "collection_hako_authority_result_consumed": 0,
            "collection_live_route_calls_authority_pilot": 0,
            "scalar_known_hako_runtime_route_authority": 0,
            "source_selfhost_claim": 0,
        },
        "claims": {
            "collection_hako_authority_pilot_basis": 1,
            "prior_scoped_read_authority_continuation": 1,
            "len_surface_policy_homogeneous_scalar_i64_no_publication_observe": 1,
            "generated_typed_artifact_mismatch_gate_current": 1,
            "mixed_receiver_domain_boundary_declared": 1,
            "explicit_mixed_receiver_domain_enumeration": 1,
            "receiver_domain_not_used_as_selection_proof": 1,
            "any_length_box_domain_is_explicit_row_not_wildcard_selector": 1,
            "rust_oracle_compat_checker_retained": 1,
            "mismatch_fail_fast_required": 1,
            "basis_only": 1,
            "authority_pilot_implementation_deferred": 1,
            "collection_hako_route_decision_authority_pilot": 0,
            "collection_hako_authority_result_consumed": 0,
            "collection_live_route_calls_authority_pilot": 0,
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
            "apparent_simplicity_as_proof": 0,
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
        print("mirbuilder-scalar-known-fastpath-collection-hako-authority-pilot-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
