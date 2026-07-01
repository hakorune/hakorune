#!/usr/bin/env python3
"""Rerun ID scalar owner-scope boundedness after state target enumeration."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-owner-scope-boundedness-resolution-002-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-OWNER-SCOPE-BOUNDEDNESS-RESOLUTION-002"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-ID-SCALAR-NATIVE-SEED-FILE-BOUNDARY-BASIS-001"

STATE_TARGETS = FIXTURES / "mirbuilder-id-scalar-state-target-enumeration-basis-v0.json"
OWNER_SCOPE_001 = FIXTURES / "mirbuilder-id-scalar-owner-scope-boundedness-resolution-v0.json"
SURFACES = FIXTURES / "mirbuilder-id-scalar-source-surface-inventory-v0.json"
OPERATIONS = FIXTURES / "mirbuilder-id-scalar-operation-vocabulary-inventory-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    state_targets = read_json(STATE_TARGETS)
    owner_scope_001 = read_json(OWNER_SCOPE_001)
    previous_by_owner = {row["owner_edge_id"]: row for row in owner_scope_001.get("candidates") or []}
    candidates = []

    for row in state_targets.get("owner_edge_targets") or []:
        owner = row["owner_edge_id"]
        previous = previous_by_owner[owner]
        state_targets_enumerated = bool(row.get("state_targets_enumerated"))
        all_targets_inside = bool(row.get("all_state_targets_inside_owner_scope"))
        cross_owner_required = row.get("cross_owner_state_target_count", 0) > 0
        native_seed_file_boundary_derivable = False
        owner_scope_bounded = (
            state_targets_enumerated
            and all_targets_inside
            and not cross_owner_required
        )
        blocked_by = []
        if cross_owner_required:
            blocked_by.append("OperationTokensRequireCrossOwnerRecipeAuthority")
        if not state_targets_enumerated:
            blocked_by.append("StateTargetsNotEnumerated")
        if not native_seed_file_boundary_derivable:
            blocked_by.append("NativeSeedFileBoundaryNotDerived")
        if not owner_scope_bounded:
            blocked_by.append("OwnerScopeBoundedNotProven")
        if owner_scope_bounded and not native_seed_file_boundary_derivable:
            blocked_by.append("BoundedOwnerScopeRequiresNativeSeedFileBoundary")

        candidates.append(
            {
                "owner_edge_id": owner,
                "source_surface_count": previous.get("source_surface_count"),
                "source_path_count": previous.get("source_path_count"),
                "operation_tokens": previous.get("operation_tokens"),
                "state_target_count": len(row.get("state_targets") or []),
                "state_targets_enumerated": state_targets_enumerated,
                "all_state_targets_inside_owner_scope": all_targets_inside,
                "cross_owner_state_target_count": row.get("cross_owner_state_target_count"),
                "operation_tokens_need_cross_owner_recipe_authority": cross_owner_required,
                "native_seed_file_boundary_derivable": native_seed_file_boundary_derivable,
                "owner_scope_bounded": owner_scope_bounded,
                "selection_eligible_for_source_plan": False,
                "blocked_by": blocked_by,
                "next_card": None,
            }
        )

    bounded = [row for row in candidates if row["owner_scope_bounded"]]
    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarOwnerScopeBoundednessResolutionV2",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "state_target_enumeration_basis": rel(STATE_TARGETS),
            "owner_scope_boundedness_resolution_001": rel(OWNER_SCOPE_001),
            "source_surface_inventory": rel(SURFACES),
            "operation_vocabulary_inventory": rel(OPERATIONS),
        },
        "provenance": {
            "state_target_enumeration_basis_hash": sha256_file(STATE_TARGETS),
            "owner_scope_boundedness_resolution_001_hash": sha256_file(OWNER_SCOPE_001),
            "source_surface_inventory_hash": sha256_file(SURFACES),
            "operation_vocabulary_inventory_hash": sha256_file(OPERATIONS),
        },
        "previous_state": {
            "state_target_count": (state_targets.get("candidate_pool") or {}).get("state_target_count"),
            "cross_owner_state_target_count": (state_targets.get("candidate_pool") or {}).get(
                "cross_owner_state_target_count"
            ),
            "previous_selected_next_card": (state_targets.get("decision") or {}).get("selected_next_card"),
        },
        "boundedness_policy": {
            "primary_unit": "owner_edge",
            "bounded_requires_state_targets_inside_owner_scope": True,
            "native_seed_file_boundary_required_for_source_plan": True,
            "cross_owner_recipe_authority_required_for_external_targets": True,
            "source_file_path_as_authority": False,
            "surface_count_as_proof": False,
            "manual_owner_selection": False,
        },
        "candidates": candidates,
        "candidate_pool": {
            "input_candidate_count": len(candidates),
            "owner_scope_bounded_count": len(bounded),
            "state_targets_enumerated_count": len(
                [row for row in candidates if row["state_targets_enumerated"]]
            ),
            "native_seed_file_boundary_derivable_count": len(
                [row for row in candidates if row["native_seed_file_boundary_derivable"]]
            ),
            "cross_owner_recipe_required_count": len(
                [row for row in candidates if row["operation_tokens_need_cross_owner_recipe_authority"]]
            ),
            "selection_eligible_for_source_plan_count": 0,
        },
        "decision": {
            "kind": "SelectNativeSeedFileBoundaryBasis",
            "reason_token": "BoundedOwnerScopeRequiresNativeSeedFileBoundary",
            "selected_owner_edge_id": None,
            "selected_next_card": NEXT_CARD,
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
        print("mirbuilder-id-scalar-owner-scope-boundedness-resolution-002 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
