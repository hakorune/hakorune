#!/usr/bin/env python3
"""Select the next ID scalar owner-scope blocker component by dependency order."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-owner-scope-blocker-priority-resolution-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-OWNER-SCOPE-BLOCKER-PRIORITY-RESOLUTION-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-ID-SCALAR-STATE-TARGET-ENUMERATION-BASIS-001"

OWNER_SCOPE = FIXTURES / "mirbuilder-id-scalar-owner-scope-boundedness-resolution-v0.json"
RERUN = FIXTURES / "mirbuilder-id-scalar-source-plan-and-recipe-derivability-resolution-002-v0.json"
SURFACES = FIXTURES / "mirbuilder-id-scalar-source-surface-inventory-v0.json"
OPERATIONS = FIXTURES / "mirbuilder-id-scalar-operation-vocabulary-inventory-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    owner_scope = read_json(OWNER_SCOPE)
    pool = owner_scope.get("candidate_pool") or {}
    decision = owner_scope.get("decision") or {}

    blocker_components = [
        {
            "component_id": "StateTargetEnumeration",
            "blocked_by": "StateTargetsNotEnumerated",
            "affected_candidate_count": pool.get("input_candidate_count", 0)
            - pool.get("state_targets_enumerated_count", 0),
            "dependency_rank": 0,
            "unblocks": [
                "OwnerScopeBoundedness",
                "NativeSeedFileBoundary",
                "StateMutationFrame",
                "CrossOwnerRecipeAuthority",
                "BehaviorRecipeEffectCoverage",
            ],
            "requires": [],
            "selection_eligible": True,
            "selected_next_card": NEXT_CARD,
        },
        {
            "component_id": "NativeSeedFileBoundary",
            "blocked_by": "NativeSeedFileBoundaryNotDerived",
            "affected_candidate_count": pool.get("input_candidate_count", 0)
            - pool.get("native_seed_file_boundary_derivable_count", 0),
            "dependency_rank": 2,
            "unblocks": ["NativeSourceSeedReadiness"],
            "requires": ["StateTargetEnumeration", "OwnerScopeBoundedness"],
            "selection_eligible": False,
            "selected_next_card": "MIRBUILDER-ID-SCALAR-NATIVE-SEED-FILE-BOUNDARY-BASIS-001",
        },
        {
            "component_id": "CrossOwnerRecipeAuthority",
            "blocked_by": "OperationTokensRequireCrossOwnerRecipeAuthority",
            "affected_candidate_count": pool.get("cross_owner_recipe_required_count", 0),
            "dependency_rank": 1,
            "unblocks": ["OwnerAuthorityRecipeAuthoritySplit", "BehaviorRecipeEffectCoverage"],
            "requires": ["StateTargetEnumeration"],
            "selection_eligible": False,
            "selected_next_card": "MIRBUILDER-ID-SCALAR-CROSS-OWNER-RECIPE-AUTHORITY-BASIS-001",
        },
    ]

    selected = min(
        [row for row in blocker_components if row["selection_eligible"]],
        key=lambda row: (
            row["dependency_rank"],
            0 if row["affected_candidate_count"] == pool.get("input_candidate_count") else 1,
            row["component_id"],
        ),
    )

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarOwnerScopeBlockerPriorityResolutionV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "owner_scope_boundedness_resolution": rel(OWNER_SCOPE),
            "source_plan_derivability_rerun_002": rel(RERUN),
            "source_surface_inventory": rel(SURFACES),
            "operation_vocabulary_inventory": rel(OPERATIONS),
        },
        "provenance": {
            "owner_scope_boundedness_resolution_hash": sha256_file(OWNER_SCOPE),
            "source_plan_derivability_rerun_002_hash": sha256_file(RERUN),
            "source_surface_inventory_hash": sha256_file(SURFACES),
            "operation_vocabulary_inventory_hash": sha256_file(OPERATIONS),
        },
        "previous_state": {
            "input_candidate_count": pool.get("input_candidate_count"),
            "owner_scope_bounded_count": pool.get("owner_scope_bounded_count"),
            "state_targets_enumerated_count": pool.get("state_targets_enumerated_count"),
            "native_seed_file_boundary_derivable_count": pool.get(
                "native_seed_file_boundary_derivable_count"
            ),
            "cross_owner_recipe_required_count": pool.get("cross_owner_recipe_required_count"),
            "selection_eligible_for_source_plan_count": pool.get(
                "selection_eligible_for_source_plan_count"
            ),
            "reason_token": decision.get("reason_token"),
        },
        "blocker_components": blocker_components,
        "selection_rule": {
            "manual_axis_selection": False,
            "select_lowest_dependency_rank": True,
            "prefer_common_root_blocker": True,
            "surface_count_as_proof": False,
            "cluster_size_as_proof": False,
            "source_file_path_as_authority": False,
            "route_membership_alone_as_proof": False,
        },
        "decision": {
            "kind": "SelectOwnerScopeBlockerComponent",
            "selected_component_id": selected["component_id"],
            "reason_token": "StateTargetEnumerationSelectedAsOwnerScopeRootBlocker",
            "selected_next_card": selected["selected_next_card"],
        },
        "claims": {
            "manual_owner_selection": 0,
            "manual_axis_selection": 0,
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
        print("mirbuilder-id-scalar-owner-scope-blocker-priority-resolution unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
