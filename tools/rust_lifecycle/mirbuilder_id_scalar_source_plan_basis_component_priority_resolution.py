#!/usr/bin/env python3
"""Select the next ID scalar SourcePlan basis component by dependency order."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-source-plan-basis-component-priority-resolution-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-SOURCE-PLAN-BASIS-COMPONENT-PRIORITY-RESOLUTION-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-ID-SCALAR-OWNER-SCOPE-BOUNDEDNESS-RESOLUTION-001"

RERUN = FIXTURES / "mirbuilder-id-scalar-source-plan-and-recipe-derivability-resolution-002-v0.json"
SURFACES = FIXTURES / "mirbuilder-id-scalar-source-surface-inventory-v0.json"
OPERATIONS = FIXTURES / "mirbuilder-id-scalar-operation-vocabulary-inventory-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    rerun = read_json(RERUN)
    pool = rerun.get("candidate_pool") or {}
    decision = rerun.get("decision") or {}

    unresolved_components = [
        {
            "component_id": "OwnerScopeBoundedness",
            "blocked_by_token": "OwnerScopeBoundedNotProven",
            "dependency_rank": 0,
            "unblocks": [
                "StateMutationFrame",
                "BehaviorRecipeEffectCoverage",
                "VerifierInputContract",
                "NativeSeedFileBoundary",
            ],
            "requires": [],
            "selection_eligible": True,
            "next_card": NEXT_CARD,
        },
        {
            "component_id": "IdDomainBoundary",
            "blocked_by_token": "IdDomainBoundaryNotDeclared",
            "dependency_rank": 1,
            "unblocks": ["BehaviorRecipeEffectCoverage", "VerifierInputContract"],
            "requires": [],
            "selection_eligible": True,
            "next_card": "MIRBUILDER-ID-SCALAR-ID-DOMAIN-BOUNDARY-BASIS-001",
        },
        {
            "component_id": "ErrorSemantics",
            "blocked_by_token": "ErrorSemanticsNotDeclared",
            "dependency_rank": 1,
            "unblocks": ["BehaviorRecipeEffectCoverage", "VerifierInputContract"],
            "requires": [],
            "selection_eligible": True,
            "next_card": "MIRBUILDER-ID-SCALAR-ERROR-SEMANTICS-BASIS-001",
        },
        {
            "component_id": "DeterministicOrder",
            "blocked_by_token": "DeterministicOrderNotDeclared",
            "dependency_rank": 1,
            "unblocks": ["BehaviorRecipeEffectCoverage", "VerifierInputContract"],
            "requires": [],
            "selection_eligible": True,
            "next_card": "MIRBUILDER-ID-SCALAR-DETERMINISTIC-ORDER-BASIS-001",
        },
        {
            "component_id": "StateMutationFrame",
            "blocked_by_token": "StateMutationFrameNotDeclared",
            "dependency_rank": 2,
            "unblocks": ["BehaviorRecipeEffectCoverage", "VerifierInputContract"],
            "requires": ["OwnerScopeBoundedness"],
            "selection_eligible": False,
            "next_card": "MIRBUILDER-ID-SCALAR-STATE-MUTATION-FRAME-BASIS-001",
        },
        {
            "component_id": "BehaviorRecipeEffectCoverage",
            "blocked_by_token": "BehaviorRecipeEffectCoverageNotProven",
            "dependency_rank": 3,
            "unblocks": ["VerifierInputContract", "SourcePlanAndRecipeDerivability"],
            "requires": [
                "OwnerScopeBoundedness",
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
            "requires": ["SourcePlan", "BehaviorRecipeEffectCoverage"],
            "selection_eligible": False,
            "next_card": "MIRBUILDER-ID-SCALAR-VERIFIER-INPUT-CONTRACT-BASIS-001",
        },
    ]

    selected = min(
        [row for row in unresolved_components if row["selection_eligible"]],
        key=lambda row: (row["dependency_rank"], 0 if row["component_id"] == "OwnerScopeBoundedness" else 1),
    )

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarSourcePlanBasisComponentPriorityResolutionV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "source_plan_derivability_rerun_002": rel(RERUN),
            "source_surface_inventory": rel(SURFACES),
            "operation_vocabulary_inventory": rel(OPERATIONS),
        },
        "provenance": {
            "source_plan_derivability_rerun_002_hash": sha256_file(RERUN),
            "source_surface_inventory_hash": sha256_file(SURFACES),
            "operation_vocabulary_inventory_hash": sha256_file(OPERATIONS),
        },
        "previous_state": {
            "input_candidate_count": pool.get("input_candidate_count"),
            "required_source_surfaces_complete_count": pool.get("required_source_surfaces_complete_count"),
            "operation_vocabulary_complete_count": pool.get("operation_vocabulary_complete_count"),
            "nominal_id_domain_preserved_count": pool.get("nominal_id_domain_preserved_count"),
            "owner_scope_bounded_count": pool.get("owner_scope_bounded_count"),
            "behavior_recipe_effect_coverage_complete_count": pool.get("behavior_recipe_effect_coverage_complete_count"),
            "source_plan_derivable_count": pool.get("source_plan_derivable_count"),
            "selection_eligible_count": pool.get("selection_eligible_count"),
            "reason_token": decision.get("reason_token"),
        },
        "unresolved_components": unresolved_components,
        "selection_rule": {
            "manual_component_selection": False,
            "select_lowest_dependency_rank": True,
            "prefer_components_that_define_owner_subject": True,
            "cluster_size_as_proof": False,
            "surface_count_as_proof": False,
            "lexical_order_as_proof": False,
        },
        "decision": {
            "kind": "SelectBasisComponent",
            "selected_component_id": selected["component_id"],
            "reason_token": "OwnerScopeBoundednessSelectedAsSourcePlanRootComponent",
            "selected_next_card": selected["next_card"],
        },
        "claims": {
            "manual_owner_selection": 0,
            "manual_component_selection": 0,
            "cluster_size_as_proof": 0,
            "surface_count_as_proof": 0,
            "directable_row_count_as_proof": 0,
            "route_membership_alone_as_proof": 0,
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
        print("mirbuilder-id-scalar-source-plan-basis-component-priority-resolution unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
