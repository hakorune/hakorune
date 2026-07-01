#!/usr/bin/env python3
"""Resolve ID scalar owner-scope boundedness evidence."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-owner-scope-boundedness-resolution-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-OWNER-SCOPE-BOUNDEDNESS-RESOLUTION-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

PRIORITY = FIXTURES / "mirbuilder-id-scalar-source-plan-basis-component-priority-resolution-v0.json"
SURFACES = FIXTURES / "mirbuilder-id-scalar-source-surface-inventory-v0.json"
OPERATIONS = FIXTURES / "mirbuilder-id-scalar-operation-vocabulary-inventory-v0.json"

COMPOSITE_OPERATION_TOKENS = {
    "EdgeCfgCompose",
    "JoinPayloadBuild",
    "MergeContractOrDiagnostic",
    "MergeRewriteDecision",
    "PlanListBuild",
    "PlannerSessionDispatch",
    "RouteRewriteBuild",
    "RouteLocalCleanupMutationFrame",
    "SkeletonOrWiringBuild",
}

MUTATING_OPERATION_TOKENS = {
    "ContextRegistryConstruct",
    "PhiInstructionDefine",
    "PhiInstructionDefineCurrentBlock",
    "PhiInstructionDefineFunction",
    "PhiInstructionPatch",
    "PhiLifecyclePatch",
    "RouteLocalCleanupMutationFrame",
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def module_prefix(paths: list[str]) -> str:
    if not paths:
        return "Unknown"
    common = Path(paths[0]).parent
    for path in paths[1:]:
        parent = Path(path).parent
        while common != parent and common not in parent.parents:
            if common == common.parent:
                return "."
            common = common.parent
    return str(common)


def build_fixture() -> dict[str, Any]:
    priority = read_json(PRIORITY)
    surfaces = read_json(SURFACES)
    operations = read_json(OPERATIONS)

    surface_by_owner = {row["owner_edge_id"]: row for row in surfaces.get("candidates") or []}
    candidates = []
    for row in operations.get("candidates") or []:
        owner = row["owner_edge_id"]
        surface_row = surface_by_owner[owner]
        op_tokens = sorted({op["operation_token"] for op in row.get("operation_rows") or []})
        paths = sorted({op["source_path"] for op in row.get("operation_rows") or []})
        composite_tokens = sorted(set(op_tokens) & COMPOSITE_OPERATION_TOKENS)
        mutating_tokens = sorted(set(op_tokens) & MUTATING_OPERATION_TOKENS)
        all_surfaces_same_owner = bool(surface_row.get("required_source_surfaces_complete"))
        operation_tokens_need_cross_owner_recipe = bool(composite_tokens)
        state_targets_enumerated = False
        native_seed_file_boundary_derivable = False
        owner_scope_bounded = (
            all_surfaces_same_owner
            and not operation_tokens_need_cross_owner_recipe
            and state_targets_enumerated
            and native_seed_file_boundary_derivable
        )
        blocked_by = []
        if operation_tokens_need_cross_owner_recipe:
            blocked_by.append("OperationTokensRequireCrossOwnerRecipeAuthority")
        if mutating_tokens and not state_targets_enumerated:
            blocked_by.append("StateTargetsNotEnumerated")
        if not native_seed_file_boundary_derivable:
            blocked_by.append("NativeSeedFileBoundaryNotDerived")
        if not owner_scope_bounded:
            blocked_by.append("OwnerScopeBoundedNotProven")
        candidates.append(
            {
                "owner_edge_id": owner,
                "source_surface_count": row.get("source_surface_count"),
                "source_path_count": len(paths),
                "source_module_prefix": module_prefix(paths),
                "all_required_source_surfaces_same_owner_edge": all_surfaces_same_owner,
                "operation_tokens": op_tokens,
                "composite_operation_tokens": composite_tokens,
                "mutating_operation_tokens": mutating_tokens,
                "operation_tokens_need_cross_owner_recipe_authority": operation_tokens_need_cross_owner_recipe,
                "state_targets_enumerated": state_targets_enumerated,
                "native_seed_file_boundary_derivable": native_seed_file_boundary_derivable,
                "source_module_boundary_is_evidence_not_authority": True,
                "owner_scope_bounded": owner_scope_bounded,
                "selection_eligible_for_source_plan": False,
                "blocked_by": blocked_by,
                "next_card": None,
            }
        )

    bounded = [row for row in candidates if row["owner_scope_bounded"]]
    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarOwnerScopeBoundednessResolutionV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "basis_component_priority_resolution": rel(PRIORITY),
            "source_surface_inventory": rel(SURFACES),
            "operation_vocabulary_inventory": rel(OPERATIONS),
        },
        "provenance": {
            "basis_component_priority_resolution_hash": sha256_file(PRIORITY),
            "source_surface_inventory_hash": sha256_file(SURFACES),
            "operation_vocabulary_inventory_hash": sha256_file(OPERATIONS),
        },
        "previous_state": {
            "selected_component_id": (priority.get("decision") or {}).get("selected_component_id"),
            "previous_reason_token": (priority.get("decision") or {}).get("reason_token"),
            "previous_selected_next_card": (priority.get("decision") or {}).get("selected_next_card"),
        },
        "boundedness_policy": {
            "primary_unit": "owner_edge",
            "validated_by": [
                "required_source_surface_set",
                "operation_token_set",
                "state_target_set",
                "future_native_seed_file_boundary",
            ],
            "source_file_path_as_authority": False,
            "surface_count_as_proof": False,
            "route_membership_alone_as_proof": False,
            "manual_owner_selection": False,
        },
        "candidates": candidates,
        "candidate_pool": {
            "input_candidate_count": len(candidates),
            "owner_scope_bounded_count": len(bounded),
            "state_targets_enumerated_count": len([row for row in candidates if row["state_targets_enumerated"]]),
            "native_seed_file_boundary_derivable_count": len(
                [row for row in candidates if row["native_seed_file_boundary_derivable"]]
            ),
            "cross_owner_recipe_required_count": len(
                [row for row in candidates if row["operation_tokens_need_cross_owner_recipe_authority"]]
            ),
            "selection_eligible_for_source_plan_count": 0,
        },
        "decision": {
            "kind": "KeepStopped",
            "reason_token": "IdScalarOwnerScopeBoundednessNotProven",
            "selected_owner_edge_id": None,
            "selected_next_card": DESIGN_STOP,
        },
        "claims": {
            "owner_scope_boundedness_resolution_completed": 1,
            "manual_owner_selection": 0,
            "surface_count_as_proof": 0,
            "cluster_size_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "source_file_path_as_authority": 0,
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
        print("mirbuilder-id-scalar-owner-scope-boundedness-resolution unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
