#!/usr/bin/env python3
"""Enumerate ID scalar semantic state targets from operation vocabulary rows."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-state-target-enumeration-basis-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-STATE-TARGET-ENUMERATION-BASIS-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-ID-SCALAR-OWNER-SCOPE-BOUNDEDNESS-RESOLUTION-002"

PRIORITY = FIXTURES / "mirbuilder-id-scalar-owner-scope-blocker-priority-resolution-v0.json"
OWNER_SCOPE = FIXTURES / "mirbuilder-id-scalar-owner-scope-boundedness-resolution-v0.json"
SURFACES = FIXTURES / "mirbuilder-id-scalar-source-surface-inventory-v0.json"
OPERATIONS = FIXTURES / "mirbuilder-id-scalar-operation-vocabulary-inventory-v0.json"


TOKEN_RULES: dict[str, tuple[str, str, tuple[str, ...], bool, bool]] = {
    "ContextRegistryConstruct": ("ContextRegistryState", "OwnerField", ("Write",), False, True),
    "DiagnosticStringBuild": ("DiagnosticState", "DiagnosticState", ("Observe",), False, False),
    "EdgeCfgCompose": ("RouteVerificationState", "ExternalOwnerState", ("Read", "Observe"), True, False),
    "JoinPayloadBuild": ("JoinPayloadState", "OutputPlanList", ("Append",), False, True),
    "LoopBindingBuild": ("LoopBindingState", "LocalAccumulator", ("Write",), False, True),
    "MergeContractOrDiagnostic": ("RouteVerificationState", "ExternalOwnerState", ("Read", "Observe"), True, False),
    "MergeRewriteDecision": ("RouteVerificationState", "ExternalOwnerState", ("Read", "Observe"), True, False),
    "PhiInfoBuild": ("PhiInfoState", "LocalAccumulator", ("Write",), False, True),
    "PhiInstructionDefine": ("PhiInstructionState", "OwnerField", ("Append",), False, True),
    "PhiInstructionDefineCurrentBlock": ("PhiInstructionState", "OwnerField", ("Append",), False, True),
    "PhiInstructionDefineFunction": ("PhiInstructionState", "OwnerField", ("Append",), False, True),
    "PhiInstructionPatch": ("PhiInstructionState", "OwnerField", ("Read", "Write"), False, True),
    "PhiLifecyclePatch": ("PhiLifecycleState", "OwnerField", ("Read", "Write"), False, True),
    "PlanListBuild": ("JoinIRPlanState", "OutputPlanList", ("Append",), False, True),
    "PlannerGateOrFactRead": ("PlannerFactState", "ExternalDependency", ("Read",), True, False),
    "PlannerSessionDispatch": ("PlannerSessionState", "ExternalDependency", ("Read", "Observe"), True, False),
    "PredicateRead": ("PredicateObservationState", "VerifierObservation", ("Read",), False, False),
    "ReadOnlyLength": ("PredicateObservationState", "VerifierObservation", ("Read",), False, False),
    "RecipeIndexRead": ("RecipeAuthorityState", "ExternalDependency", ("Read",), True, False),
    "RouteLocalCleanupMutationFrame": ("RouteLocalCleanupState", "LocalAccumulator", ("Read", "Write"), False, True),
    "RouteLocalClosureConstruct": ("RouteLocalClosureState", "LocalAccumulator", ("Write",), False, True),
    "RouteRewriteBuild": ("RouteRewriteState", "OutputPlanList", ("Append",), False, True),
    "RouteVerifyPredicate": ("RouteVerificationState", "VerifierObservation", ("Read",), False, False),
    "SkeletonOrWiringBuild": ("JoinIRPlanState", "OutputPlanList", ("Append",), False, True),
    "TraceDiagnosticEmission": ("DiagnosticState", "DiagnosticState", ("Append",), False, False),
    "VerifierContractCheck": ("VerifierObservationState", "VerifierObservation", ("Observe",), False, False),
    "VerifierDiagnosticState": ("DiagnosticState", "DiagnosticState", ("Read", "Append"), False, False),
    "VerifierObservabilityEmit": ("VerifierObservationState", "VerifierObservation", ("Observe",), False, False),
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def target_id(owner_edge_id: str, semantic_resource: str) -> str:
    snake = []
    for ch in semantic_resource:
        if ch.isupper() and snake:
            snake.append("_")
        snake.append(ch.lower())
    return f"{owner_edge_id}::{''.join(snake)}"


def build_owner_targets(candidate: dict[str, Any]) -> dict[str, Any]:
    owner_edge_id = candidate["owner_edge_id"]
    grouped: dict[str, dict[str, Any]] = {}
    for row in candidate.get("operation_rows") or []:
        token = row["operation_token"]
        semantic_resource, target_kind, access, cross_owner, mutation_frame = TOKEN_RULES[token]
        tid = target_id(owner_edge_id, semantic_resource)
        target = grouped.setdefault(
            tid,
            {
                "state_target_id": tid,
                "semantic_resource": semantic_resource,
                "target_kind": target_kind,
                "access": [],
                "operation_tokens": [],
                "source_surfaces": [],
                "inside_owner_scope": not cross_owner,
                "requires_cross_owner_recipe_authority": cross_owner,
                "mutation_frame_required": mutation_frame,
            },
        )
        target["inside_owner_scope"] = target["inside_owner_scope"] and not cross_owner
        target["requires_cross_owner_recipe_authority"] = (
            target["requires_cross_owner_recipe_authority"] or cross_owner
        )
        target["mutation_frame_required"] = target["mutation_frame_required"] or mutation_frame
        target["access"] = sorted(set(target["access"]) | set(access))
        target["operation_tokens"] = sorted(set(target["operation_tokens"]) | {token})
        target["source_surfaces"] = sorted(set(target["source_surfaces"]) | {row["source_id"]})

    targets = sorted(grouped.values(), key=lambda row: row["state_target_id"])
    return {
        "owner_edge_id": owner_edge_id,
        "state_targets": targets,
        "state_targets_enumerated": bool(targets),
        "all_state_targets_inside_owner_scope": all(row["inside_owner_scope"] for row in targets),
        "cross_owner_state_target_count": sum(
            1 for row in targets if row["requires_cross_owner_recipe_authority"]
        ),
        "mutation_frame_required_count": sum(1 for row in targets if row["mutation_frame_required"]),
    }


def build_fixture() -> dict[str, Any]:
    priority = read_json(PRIORITY)
    owner_scope = read_json(OWNER_SCOPE)
    operations = read_json(OPERATIONS)
    owner_targets = [build_owner_targets(row) for row in operations.get("candidates") or []]
    state_target_count = sum(len(row["state_targets"]) for row in owner_targets)
    cross_owner_count = sum(row["cross_owner_state_target_count"] for row in owner_targets)

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarStateTargetEnumerationBasisV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "owner_scope_blocker_priority": rel(PRIORITY),
            "owner_scope_boundedness_resolution": rel(OWNER_SCOPE),
            "source_surface_inventory": rel(SURFACES),
            "operation_vocabulary_inventory": rel(OPERATIONS),
        },
        "provenance": {
            "owner_scope_blocker_priority_hash": sha256_file(PRIORITY),
            "owner_scope_boundedness_resolution_hash": sha256_file(OWNER_SCOPE),
            "source_surface_inventory_hash": sha256_file(SURFACES),
            "operation_vocabulary_inventory_hash": sha256_file(OPERATIONS),
        },
        "previous_state": {
            "selected_component_id": (priority.get("decision") or {}).get("selected_component_id"),
            "selected_next_card": (priority.get("decision") or {}).get("selected_next_card"),
            "input_candidate_count": (owner_scope.get("candidate_pool") or {}).get("input_candidate_count"),
            "state_targets_enumerated_count": (owner_scope.get("candidate_pool") or {}).get(
                "state_targets_enumerated_count"
            ),
        },
        "enumeration_policy": {
            "primary_unit": "semantic_state_target",
            "grouped_by": "owner_edge",
            "validated_by": [
                "source_surface",
                "operation_token",
                "mutation_frame",
                "native_seed_file_boundary",
            ],
            "source_file_path_as_authority": False,
            "surface_count_as_proof": False,
            "manual_owner_selection": False,
        },
        "owner_edge_targets": owner_targets,
        "candidate_pool": {
            "input_candidate_count": len(owner_targets),
            "state_targets_enumerated_owner_edge_count": sum(
                1 for row in owner_targets if row["state_targets_enumerated"]
            ),
            "state_target_count": state_target_count,
            "cross_owner_state_target_count": cross_owner_count,
            "all_targets_inside_owner_scope_count": sum(
                1 for row in owner_targets if row["all_state_targets_inside_owner_scope"]
            ),
            "mutation_frame_required_owner_edge_count": sum(
                1 for row in owner_targets if row["mutation_frame_required_count"] > 0
            ),
        },
        "decision": {
            "kind": "StateTargetBasisDefined",
            "reason_token": "IdScalarStateTargetsEnumerated",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "manual_owner_selection": 0,
            "manual_axis_selection": 0,
            "surface_count_as_proof": 0,
            "cluster_size_as_proof": 0,
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
        print("mirbuilder-id-scalar-state-target-enumeration-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
