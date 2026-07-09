#!/usr/bin/env python3
"""Select the first ScalarKnown read surface typed artifact pilot."""

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
    / "mirbuilder-scalar-known-fastpath-read-surface-generated-typed-artifact-selection-design-consultation-v0.json"
)

TOKEN = (
    "MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-SURFACE-GENERATED-TYPED-ARTIFACT-"
    "SELECTION-DESIGN-CONSULTATION-001"
)
NEXT_CARD = (
    "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-BASIS-"
    "MAPLOAD-SCALAR-I64-001"
)

RERUN_003 = (
    FIXTURES / "mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-003-v0.json"
)
SCALAR_CONTRACT = ROOT / "src/mir/generic_method_route_plan/scalar_known_typed_direct_closeout_contract.rs"
COLLECTION_READ_ROUTES = ROOT / "src/mir/generic_method_route_plan/collection_read_routes.rs"
STRING_ROUTES = ROOT / "src/mir/generic_method_route_plan/string_routes.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    rerun = read_json(RERUN_003)
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathReadSurfaceGeneratedTypedArtifactSelectionDesignConsultationV1",
        "token": TOKEN,
        "input_state": {
            "connected_closeout_inventory_rerun_003": rel(RERUN_003),
            "connected_closeout_inventory_rerun_003_hash": sha256_file(RERUN_003),
            "rerun_003_decision": (rerun.get("decision") or {}).get("kind"),
            "rerun_003_reason": (rerun.get("decision") or {}).get("reason_token"),
        },
        "provenance": {
            "scalar_known_contract": rel(SCALAR_CONTRACT),
            "scalar_known_contract_hash": sha256_file(SCALAR_CONTRACT),
            "collection_read_routes": rel(COLLECTION_READ_ROUTES),
            "collection_read_routes_hash": sha256_file(COLLECTION_READ_ROUTES),
            "string_routes": rel(STRING_ROUTES),
            "string_routes_hash": sha256_file(STRING_ROUTES),
        },
        "proof_axis": {
            "name": "ReadSurfaceGeneratedArtifactMinimalityAxis",
            "artifact_shape_complexity": True,
            "live_decision_insertion_locality": True,
            "policy_homogeneity": True,
            "semantic_authority_non_broadening": True,
            "route_count_as_proof": False,
            "owner_name_as_proof": False,
            "source_path_as_authority": False,
            "route_membership_alone_as_proof": False,
            "manual_surface_selection": False,
        },
        "candidate_assessment": [
            {
                "surface_id": "MapLoadScalarI64Routes",
                "selected_first": True,
                "route_kind_family": ["MapLoadScalarI64"],
                "core_op_family": ["MapGet"],
                "return_shape": "ScalarI64OrMissingZero",
                "value_demand": "ScalarI64",
                "publication_policy": "NoPublication",
                "effect_class": "read",
                "proof_family": "ScalarI64MapGetStoreFact",
                "allowed_existing_proofs": [
                    "MapSetScalarI64SameKeyNoEscape",
                    "MapSetScalarI64DominatesNoEscape",
                    "MapSetScalarI64CoveredDynamicI64KeyNoEscape",
                ],
                "selection_reason": "Narrowest read generated artifact family that mirrors an existing scalar-map proof branch without broadening authority.",
            },
            {
                "surface_id": "StringScalarI64Routes",
                "selected_first": False,
                "blocked_by": ["ThreeRouteKindsAndThreeCoreOpsBeforeFirstReadArtifact"],
            },
            {
                "surface_id": "CollectionScalarI64Routes",
                "selected_first": False,
                "blocked_by": ["MixedReceiverDomainFamiliesAndObserveEffectBeforeFirstReadArtifact"],
            },
        ],
        "summary": {
            "read_surface_generated_typed_artifact_selection_consultation": 1,
            "read_surface_generated_artifact_minimality_axis": 1,
            "mapload_scalar_i64_routes_selected_first": 1,
            "mapload_generated_artifact_basis_selected": 1,
            "basis_only": 1,
            "implementation_deferred_to_next_card": 1,
            "generated_typed_hako_artifact_created": 0,
            "mapload_fastpath_shadow_consumed": 0,
            "fastpath_connected_closeout": 0,
            "hako_runtime_route_authority": 0,
            "rust_fastpath_rewired": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectMapLoadScalarI64RoutesFirst",
            "reason_token": "ReadSurfaceGeneratedArtifactMinimalityAxis",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "read_surface_generated_typed_artifact_selection_consultation": 1,
            "read_surface_generated_artifact_minimality_axis": 1,
            "mapload_scalar_i64_routes_selected_first": 1,
            "mapload_generated_artifact_basis_selected": 1,
            "basis_only": 1,
            "implementation_deferred_to_next_card": 1,
            "generated_typed_hako_artifact_created": 0,
            "mapload_fastpath_shadow_consumed": 0,
            "read_surface_connection_complete": 0,
            "fastpath_connected_closeout": 0,
            "hako_runtime_route_authority": 0,
            "rust_fastpath_rewired": 0,
            "route_selection_authority_switch": 0,
            "backend_lowering_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "build_rs_hako_compiler_invocation": 0,
            "live_hako_authority": 0,
            "caller_orientation_runtime_path": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "runtime_fallback": 0,
            "source_selfhost_claim": 0,
            "manual_surface_selection": 0,
            "route_count_as_proof": 0,
            "owner_name_as_proof": 0,
            "source_path_as_authority": 0,
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
        print("mirbuilder-scalar-known-fastpath-read-surface-selection-consultation unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
