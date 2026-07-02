#!/usr/bin/env python3
"""Resolve parent-owned ID scalar subject boundary for context_registry."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-parent-owned-subject-boundary-resolution-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-PARENT-OWNED-SUBJECT-BOUNDARY-RESOLUTION-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_LIFECYCLE = "MIRBUILDER-CONTEXT_REGISTRY-LIFECYCLE-CONTRACT-DESCRIPTOR-BASIS-001"
NEXT_WIDER = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-007"
OWNER = "mirbuilder::context_registry"

RERUN_010 = FIXTURES / "mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-010-v0.json"
CONTEXT_POLICY = FIXTURES / "mirbuilder-context-registry-projection-policy-v0.json"
DISCRIMINATOR_002 = FIXTURES / "mirbuilder-id-scalar-derivable-owner-discriminator-resolution-002-v0.json"
SOURCE_SURFACES = FIXTURES / "mirbuilder-id-scalar-source-surface-inventory-v0.json"
STATE_TARGETS = FIXTURES / "mirbuilder-id-scalar-state-target-enumeration-basis-v0.json"
EFFECT_COVERAGE = FIXTURES / "mirbuilder-id-scalar-behavior-recipe-effect-coverage-basis-v0.json"
NATIVE_BOUNDARY = FIXTURES / "mirbuilder-id-scalar-native-seed-file-boundary-basis-v0.json"
TYPED_EVIDENCE = FIXTURES / "mirbuilder-id-scalar-typed-evidence-index-policy-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def owner_row(rows: list[dict[str, Any]], owner: str) -> dict[str, Any]:
    for row in rows:
        if row.get("owner_edge_id") == owner:
            return row
    return {}


def build_fixture() -> dict[str, Any]:
    rerun = read_json(RERUN_010)
    policy = read_json(CONTEXT_POLICY)
    discriminator = read_json(DISCRIMINATOR_002)
    surfaces = read_json(SOURCE_SURFACES)
    targets = read_json(STATE_TARGETS)
    effects = read_json(EFFECT_COVERAGE)
    boundary = read_json(NATIVE_BOUNDARY)
    typed = read_json(TYPED_EVIDENCE)

    remaining = owner_row(rerun.get("remaining_candidates") or [], OWNER)
    source_row = owner_row(surfaces.get("candidates") or [], OWNER)
    target_row = owner_row(targets.get("owner_edge_targets") or [], OWNER)
    boundary_row = owner_row(boundary.get("boundary_rows") or [], OWNER)
    typed_row = owner_row(typed.get("typed_evidence_rows") or [], OWNER)
    effect_rows = [
        row for row in effects.get("effect_rows") or [] if row.get("owner_edge_id") == OWNER
    ]

    source_surfaces = source_row.get("surfaces") or []
    state_targets = target_row.get("state_targets") or []
    operation_effect_classes = sorted(
        {
            row.get("effect_class")
            for row in effect_rows
            if row.get("effect_class") and row.get("operation_token")
        }
    )

    tests = {
        "keep_parent_owner_as_standalone_proof": False,
        "source_symbol_as_proof": False,
        "source_path_as_authority": False,
        "shape_name_as_semantic_policy": False,
        "route_membership_alone_as_proof": False,
        "standalone_subject_id_declared": False,
        "parent_owner_id_declared": False,
        "owned_semantic_resource_declared": bool(
            {target.get("semantic_resource") for target in state_targets if target.get("semantic_resource")}
        ),
        "source_surface_set_declared": bool(source_surfaces),
        "state_target_set_declared": bool(state_targets),
        "operation_effect_class_set_declared": bool(operation_effect_classes),
        "native_seed_file_boundary_candidate_declared": boundary_row.get(
            "native_seed_file_boundary_derivable"
        )
        is True
        and bool(boundary_row.get("native_source_seed_path")),
        "module_export_candidate_declared": bool(boundary_row.get("module_export")),
        "generator_overwrite_guard_candidate_declared": bool(
            boundary_row.get("generator_overwrite_guard_path")
        ),
        "parent_semantics_not_copied": False,
        "external_parent_dependencies_declared": False,
    }
    required_positive = [
        "standalone_subject_id_declared",
        "parent_owner_id_declared",
        "owned_semantic_resource_declared",
        "source_surface_set_declared",
        "state_target_set_declared",
        "operation_effect_class_set_declared",
        "native_seed_file_boundary_candidate_declared",
        "module_export_candidate_declared",
        "generator_overwrite_guard_candidate_declared",
        "parent_semantics_not_copied",
        "external_parent_dependencies_declared",
    ]
    missing = [key for key in required_positive if tests.get(key) is not True]
    standalone = not missing

    if standalone:
        classification_kind = "StandaloneSubjectBoundaryCandidate"
        decision = {
            "kind": "SelectLifecycleContractDescriptorBasis",
            "reason_token": "ContextRegistryStandaloneSubjectBoundaryCandidateEstablished",
            "selected_next_card": NEXT_LIFECYCLE,
        }
    else:
        classification_kind = "RemainParentOwned"
        decision = {
            "kind": "SelectWiderRouteSelectionBasis",
            "reason_token": "ContextRegistryRemainsParentOwnedNotSeedEligible",
            "selected_next_card": NEXT_WIDER,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarParentOwnedSubjectBoundaryResolutionV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "latest_candidate_rerun": rel(RERUN_010),
            "context_registry_projection_policy": rel(CONTEXT_POLICY),
            "id_scalar_derivable_owner_discriminator_resolution": rel(DISCRIMINATOR_002),
            "source_surface_inventory": rel(SOURCE_SURFACES),
            "state_target_enumeration_basis": rel(STATE_TARGETS),
            "behavior_recipe_effect_coverage_basis": rel(EFFECT_COVERAGE),
            "native_seed_file_boundary_basis": rel(NATIVE_BOUNDARY),
            "typed_evidence_index_policy": rel(TYPED_EVIDENCE),
        },
        "provenance": {
            "latest_candidate_rerun_hash": sha256_file(RERUN_010),
            "context_registry_projection_policy_hash": sha256_file(CONTEXT_POLICY),
            "id_scalar_derivable_owner_discriminator_resolution_hash": sha256_file(
                DISCRIMINATOR_002
            ),
            "source_surface_inventory_hash": sha256_file(SOURCE_SURFACES),
            "state_target_enumeration_basis_hash": sha256_file(STATE_TARGETS),
            "behavior_recipe_effect_coverage_basis_hash": sha256_file(EFFECT_COVERAGE),
            "native_seed_file_boundary_basis_hash": sha256_file(NATIVE_BOUNDARY),
            "typed_evidence_index_policy_hash": sha256_file(TYPED_EVIDENCE),
        },
        "input_candidate": {
            "owner_edge_id": OWNER,
            "projection_disposition": (policy.get("selected_policy") or {}).get("policy"),
            "projection_surface_selected": (policy.get("selected_policy") or {}).get(
                "projection_surface_selected"
            ),
            "current_reason_token": (policy.get("selected_policy") or {}).get("reason_token"),
            "remaining_owner_count_as_proof": False,
            "existing_blocked_by": remaining.get("blocked_by") or [],
        },
        "typed_subject_boundary_evidence": {
            "source_surface_count": len(source_surfaces),
            "state_target_count": len(state_targets),
            "operation_effect_classes": operation_effect_classes,
            "native_source_seed_path_candidate": boundary_row.get("native_source_seed_path"),
            "module_export_candidate": boundary_row.get("module_export"),
            "typed_evidence_complete": typed_row.get("typed_evidence_complete") is True,
        },
        "subject_boundary_tests": tests,
        "classification": {
            "kind": classification_kind,
            "standalone_projection_subject_established": standalone,
            "lifecycle_contract_descriptor_allowed_next": standalone,
            "source_plan_materialization_allowed": False,
            "blocked_by": missing,
        },
        "decision": decision,
        "claims": {
            "latest_candidate_rerun_consumed": 1,
            "context_registry_projection_policy_consumed": 1,
            "id_scalar_derivable_owner_discriminator_resolution_consumed": 1,
            "remaining_owner_count_as_proof": 0,
            "owner_name_as_proof": 0,
            "source_symbol_as_proof": 0,
            "source_path_as_authority": 0,
            "with_plugin_sigs_symbol_name_as_proof": 0,
            "keep_parent_owner_as_standalone_proof": 0,
            "projection_descriptor_coverage_as_standalone_proof": 0,
            "lifecycle_contract_descriptor_completeness": 1 if standalone else 0,
            "source_plan_materialization": 0,
            "behavior_recipe_materialization": 0,
            "verifier_result_materialization": 0,
            "derived_artifact_seed_draft_materialization": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
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
        print("mirbuilder-id-scalar-parent-owned-subject-boundary-resolution unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
