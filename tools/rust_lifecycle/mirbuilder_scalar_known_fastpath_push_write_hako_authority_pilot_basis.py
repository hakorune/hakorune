#!/usr/bin/env python3
"""Define the Push Write `.hako` authority pilot basis."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-push-write-hako-authority-pilot-basis-v0.json"

TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-PUSH-WRITE-HAKO-AUTHORITY-PILOT-BASIS-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-PUSH-WRITE-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001"

DESIGN_STOP = (
    ROOT
    / "docs/development/current/main/phases/phase-296x/3406-MIRBUILDER-SCALAR-KNOWN-FASTPATH-NEXT-WRITE-HAKO-AUTHORITY-SURFACE-DESIGN-STOP-001.md"
)
SHADOW_FIXTURE = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-write-push-v0.json"
)
PUSH_ARTIFACT = ROOT / "src/mir/generic_method_route_plan/generated/write_push_hako_policy.rs"
SHADOW_SOURCE = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
WRITE_ROUTES = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def file_entry(path: Path) -> dict[str, str]:
    return {"path": rel(path), "sha256": sha256_file(path)}


def build_fixture() -> dict[str, Any]:
    shadow = read_json(SHADOW_FIXTURE)
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathPushWriteHakoAuthorityPilotBasisV1",
        "token": TOKEN,
        "input_state": {
            "design_stop_card": rel(DESIGN_STOP),
            "design_stop_card_hash": sha256_file(DESIGN_STOP),
            "prior_shadow_consume_fixture": rel(SHADOW_FIXTURE),
            "prior_shadow_consume_fixture_hash": sha256_file(SHADOW_FIXTURE),
            "prior_shadow_consumed": (shadow.get("claims") or {}).get(
                "generated_typed_hako_artifact_shadow_consumed"
            ),
        },
        "provenance": {
            "push_generated_typed_artifact": file_entry(PUSH_ARTIFACT),
            "shadow_consumer": file_entry(SHADOW_SOURCE),
            "write_routes": file_entry(WRITE_ROUTES),
        },
        "basis": {
            "basis_only": True,
            "selected_write_surface": "PushSurfacePolicy",
            "selected_route_family": "ArrayAppendAny",
            "proof_axis": [
                "PriorScopedWriteCloseoutEvidenceContinuation",
                "ExistingGeneratedTypedArtifactShadowConsumed",
                "PushSurfacePolicyScalarI64NoPublicationMutationMetadataOnly",
                "NoAnyWriteBoundaryOpened",
                "RustOracleCompatFailFastRetained",
            ],
            "authority_source": "WRITE_PUSH_HAKO_POLICY",
            "rust_oracle_compat_checker_retained": True,
            "mismatch_policy": "FailFast",
            "implementation_deferred": True,
            "selected_next_card": NEXT_CARD,
        },
        "write_shape": {
            "surface": "PushSurfacePolicy",
            "route_kind": "ArrayAppendAny",
            "core_op": "ArrayPush",
            "lowering_tier": "ColdFallback",
            "result_class": "ScalarI64Result",
            "return_shape": "ScalarI64",
            "value_demand": "WriteAny",
            "publication_policy": "NoPublication",
            "effect_class": "mutate",
            "mutation_class": "MutatesReceiverOrContainer",
            "mutation_authority": "metadata_only",
            "any_write_boundary_opened": False,
        },
        "decision": {
            "kind": "SelectPushWriteRouteDecisionAuthorityPilotImplementation",
            "reason_token": "PushHasGeneratedArtifactNoAnyWriteBoundaryAndMutationMetadataOnly",
            "selected_next_card": NEXT_CARD,
        },
        "summary": {
            "push_write_hako_authority_pilot_basis": 1,
            "prior_scoped_write_closeout_evidence_continuation": 1,
            "existing_generated_typed_artifact_shadow_consumed": 1,
            "push_surface_policy_scalar_i64_no_publication_mutation_metadata_only": 1,
            "no_any_write_boundary_opened": 1,
            "rust_oracle_compat_checker_retained": 1,
            "mismatch_fail_fast_required": 1,
            "basis_only": 1,
            "push_hako_route_decision_authority_pilot": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "source_selfhost_claim": 0,
        },
        "claims": {
            "push_write_hako_authority_pilot_basis": 1,
            "prior_scoped_write_closeout_evidence_continuation": 1,
            "existing_generated_typed_artifact_shadow_consumed": 1,
            "push_surface_policy_scalar_i64_no_publication_mutation_metadata_only": 1,
            "no_any_write_boundary_opened": 1,
            "rust_oracle_compat_checker_retained": 1,
            "mismatch_fail_fast_required": 1,
            "basis_only": 1,
            "push_hako_route_decision_authority_pilot": 0,
            "push_hako_authority_result_consumed": 0,
            "push_live_route_calls_authority_pilot": 0,
            "write_surface_authority_closeout": 0,
            "write_wide_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "scalar_known_hako_runtime_route_authority": 0,
            "source_selfhost_claim": 0,
            "any_write_boundary_opened": 0,
            "mapstoreany_authority": 0,
            "mapdeleteany_authority": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "route_count_as_proof": 0,
            "row_count_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "owner_name_as_proof": 0,
            "source_path_as_authority": 0,
            "route_membership_alone_as_proof": 0,
            "manual_surface_selection": 0,
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
        print("mirbuilder-scalar-known-fastpath-push-write-hako-authority-pilot-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
