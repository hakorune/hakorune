#!/usr/bin/env python3
"""Rerun ID scalar SourcePlan basis component priority after file boundaries."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-source-plan-basis-component-priority-resolution-002-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-SOURCE-PLAN-BASIS-COMPONENT-PRIORITY-RESOLUTION-002"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-ID-SCALAR-ID-DOMAIN-BOUNDARY-BASIS-001"

DERIVABILITY = FIXTURES / "mirbuilder-id-scalar-source-plan-and-recipe-derivability-resolution-002-v0.json"
STATE_TARGETS = FIXTURES / "mirbuilder-id-scalar-state-target-enumeration-basis-v0.json"
OWNER_SCOPE = FIXTURES / "mirbuilder-id-scalar-owner-scope-boundedness-resolution-002-v0.json"
FILE_BOUNDARY = FIXTURES / "mirbuilder-id-scalar-native-seed-file-boundary-basis-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    derivability = read_json(DERIVABILITY)
    state_targets = read_json(STATE_TARGETS)
    owner_scope = read_json(OWNER_SCOPE)
    file_boundary = read_json(FILE_BOUNDARY)
    bounded_count = (owner_scope.get("candidate_pool") or {}).get("owner_scope_bounded_count")
    boundary_count = (file_boundary.get("candidate_pool") or {}).get(
        "native_seed_file_boundary_derivable_count"
    )

    components = [
        {
            "component_id": "IdDomainBoundary",
            "blocked_by_token": "IdDomainBoundaryNotDeclared",
            "dependency_rank": 0,
            "unblocks": ["BehaviorRecipeEffectCoverage", "VerifierInputContract"],
            "requires": ["OwnerScopeBoundedness", "NativeSeedFileBoundary"],
            "selection_eligible": bounded_count == boundary_count and bounded_count > 0,
            "next_card": NEXT_CARD,
        },
        {
            "component_id": "StateMutationFrame",
            "blocked_by_token": "StateMutationFrameNotDeclared",
            "dependency_rank": 1,
            "unblocks": ["BehaviorRecipeEffectCoverage", "VerifierInputContract"],
            "requires": ["OwnerScopeBoundedness", "StateTargetEnumeration"],
            "selection_eligible": False,
            "next_card": "MIRBUILDER-ID-SCALAR-STATE-MUTATION-FRAME-BASIS-001",
        },
        {
            "component_id": "ErrorSemantics",
            "blocked_by_token": "ErrorSemanticsNotDeclared",
            "dependency_rank": 2,
            "unblocks": ["BehaviorRecipeEffectCoverage", "VerifierInputContract"],
            "requires": ["IdDomainBoundary"],
            "selection_eligible": False,
            "next_card": "MIRBUILDER-ID-SCALAR-ERROR-SEMANTICS-BASIS-001",
        },
        {
            "component_id": "DeterministicOrder",
            "blocked_by_token": "DeterministicOrderNotDeclared",
            "dependency_rank": 2,
            "unblocks": ["BehaviorRecipeEffectCoverage", "VerifierInputContract"],
            "requires": ["IdDomainBoundary"],
            "selection_eligible": False,
            "next_card": "MIRBUILDER-ID-SCALAR-DETERMINISTIC-ORDER-BASIS-001",
        },
        {
            "component_id": "BehaviorRecipeEffectCoverage",
            "blocked_by_token": "BehaviorRecipeEffectCoverageNotProven",
            "dependency_rank": 3,
            "unblocks": ["VerifierInputContract", "SourcePlanAndRecipeDerivability"],
            "requires": [
                "IdDomainBoundary",
                "StateMutationFrame",
                "ErrorSemantics",
                "DeterministicOrder",
            ],
            "selection_eligible": False,
            "next_card": "MIRBUILDER-ID-SCALAR-BEHAVIOR-RECIPE-EFFECT-COVERAGE-BASIS-001",
        },
        {
            "component_id": "VerifierInputContract",
            "blocked_by_token": "VerifierInputContractNotDeclared",
            "dependency_rank": 4,
            "unblocks": ["VerifierResultFixture"],
            "requires": ["BehaviorRecipeEffectCoverage"],
            "selection_eligible": False,
            "next_card": "MIRBUILDER-ID-SCALAR-VERIFIER-INPUT-CONTRACT-BASIS-001",
        },
    ]
    selected = min(
        [row for row in components if row["selection_eligible"]],
        key=lambda row: (row["dependency_rank"], row["component_id"]),
    )

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarSourcePlanBasisComponentPriorityResolutionV2",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "source_plan_derivability_rerun_002": rel(DERIVABILITY),
            "state_target_enumeration_basis": rel(STATE_TARGETS),
            "owner_scope_boundedness_rerun_002": rel(OWNER_SCOPE),
            "native_seed_file_boundary_basis": rel(FILE_BOUNDARY),
        },
        "provenance": {
            "source_plan_derivability_rerun_002_hash": sha256_file(DERIVABILITY),
            "state_target_enumeration_basis_hash": sha256_file(STATE_TARGETS),
            "owner_scope_boundedness_rerun_002_hash": sha256_file(OWNER_SCOPE),
            "native_seed_file_boundary_basis_hash": sha256_file(FILE_BOUNDARY),
        },
        "previous_state": {
            "bounded_owner_count": bounded_count,
            "native_seed_file_boundary_derivable_count": boundary_count,
            "source_plan_derivable_count": (derivability.get("candidate_pool") or {}).get(
                "source_plan_derivable_count"
            ),
            "state_target_count": (state_targets.get("candidate_pool") or {}).get("state_target_count"),
        },
        "unresolved_components": components,
        "selection_rule": {
            "manual_component_selection": False,
            "select_lowest_dependency_rank": True,
            "prefer_nominal_id_boundary_before_recipe_effects": True,
            "cluster_size_as_proof": False,
            "surface_count_as_proof": False,
            "lexical_order_as_seed_selection_proof": False,
        },
        "decision": {
            "kind": "SelectBasisComponent",
            "selected_component_id": selected["component_id"],
            "reason_token": "IdDomainBoundarySelectedAfterOwnerScopeAndFileBoundary",
            "selected_next_card": selected["next_card"],
        },
        "claims": {
            "manual_component_selection": 0,
            "manual_owner_selection": 0,
            "cluster_size_as_proof": 0,
            "surface_count_as_proof": 0,
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
        print("mirbuilder-id-scalar-source-plan-basis-component-priority-resolution-002 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
