#!/usr/bin/env python3
"""Rerun ID scalar SourcePlanAndRecipe derivability after all basis contracts."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-source-plan-and-recipe-derivability-resolution-003-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-DERIVABILITY-RESOLUTION-003"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

SURFACES = FIXTURES / "mirbuilder-id-scalar-source-surface-inventory-v0.json"
OPERATIONS = FIXTURES / "mirbuilder-id-scalar-operation-vocabulary-inventory-v0.json"
OWNER_SCOPE = FIXTURES / "mirbuilder-id-scalar-owner-scope-boundedness-resolution-002-v0.json"
FILE_BOUNDARY = FIXTURES / "mirbuilder-id-scalar-native-seed-file-boundary-basis-v0.json"
ID_DOMAIN = FIXTURES / "mirbuilder-id-scalar-id-domain-boundary-basis-v0.json"
MUTATION = FIXTURES / "mirbuilder-id-scalar-state-mutation-frame-basis-v0.json"
ERROR_ORDER = FIXTURES / "mirbuilder-id-scalar-error-and-deterministic-order-basis-v0.json"
EFFECT = FIXTURES / "mirbuilder-id-scalar-behavior-recipe-effect-coverage-basis-v0.json"
VERIFIER = FIXTURES / "mirbuilder-id-scalar-verifier-input-contract-basis-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    surfaces = read_json(SURFACES)
    operations = read_json(OPERATIONS)
    owner_scope = read_json(OWNER_SCOPE)
    file_boundary = read_json(FILE_BOUNDARY)
    id_domain = read_json(ID_DOMAIN)
    mutation = read_json(MUTATION)
    error_order = read_json(ERROR_ORDER)
    effect = read_json(EFFECT)
    verifier = read_json(VERIFIER)

    surface_by_owner = {row["owner_edge_id"]: row for row in surfaces.get("candidates") or []}
    operation_by_owner = {row["owner_edge_id"]: row for row in operations.get("candidates") or []}
    scope_by_owner = {row["owner_edge_id"]: row for row in owner_scope.get("candidates") or []}
    boundary_by_owner = {row["owner_edge_id"]: row for row in file_boundary.get("boundary_rows") or []}

    candidates = []
    for owner in sorted(surface_by_owner):
        surface = surface_by_owner[owner]
        operation = operation_by_owner[owner]
        scope = scope_by_owner[owner]
        boundary = boundary_by_owner[owner]
        checks = {
            "required_source_surfaces_complete": bool(surface.get("required_source_surfaces_complete")),
            "operation_vocabulary_complete": bool(operation.get("operation_vocabulary_complete")),
            "owner_scope_bounded": bool(scope.get("owner_scope_bounded")),
            "native_seed_file_boundary_derivable": bool(
                boundary.get("native_seed_file_boundary_derivable")
            ),
            "id_domain_boundary_declared": bool(
                (id_domain.get("candidate_pool") or {}).get("id_domain_boundary_count")
            ),
            "state_mutation_frame_declared": bool(
                (mutation.get("candidate_pool") or {}).get("mutation_frame_count")
            ),
            "error_semantics_declared": bool(
                (error_order.get("candidate_pool") or {}).get("error_semantics_count")
            ),
            "deterministic_order_declared": bool(
                (error_order.get("candidate_pool") or {}).get("deterministic_order_count")
            ),
            "behavior_recipe_effect_coverage_complete": bool(
                (effect.get("candidate_pool") or {}).get("effect_class_count")
            ),
            "verifier_input_contract_declared": bool(
                (verifier.get("candidate_pool") or {}).get("input_fact_set_count")
            ),
        }
        derivable = all(checks.values())
        blocked_by = [name for name, ok in checks.items() if not ok]
        candidates.append(
            {
                "owner_edge_id": owner,
                **checks,
                "source_plan_derivable": derivable,
                "behavior_recipe_derivable": derivable,
                "selection_eligible": derivable,
                "blocked_by": blocked_by,
                "next_card": f"MIRBUILDER-{owner.split('::', 1)[1].upper()}-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-001"
                if derivable
                else None,
            }
        )

    eligible = [row for row in candidates if row["selection_eligible"]]
    if len(eligible) == 1:
        decision = {
            "kind": "SelectSourcePlanAndRecipe",
            "reason_token": "ExactlyOneIdScalarSourcePlanDerivableOwner",
            "selected_owner_edge_id": eligible[0]["owner_edge_id"],
            "selected_next_card": eligible[0]["next_card"],
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "MultipleEqualIdScalarSourcePlanDerivabilityCandidates",
            "selected_owner_edge_id": None,
            "selected_next_card": DESIGN_STOP,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarSourcePlanAndRecipeDerivabilityResolutionV3",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "source_surface_inventory": rel(SURFACES),
            "operation_vocabulary_inventory": rel(OPERATIONS),
            "owner_scope_boundedness_rerun_002": rel(OWNER_SCOPE),
            "native_seed_file_boundary_basis": rel(FILE_BOUNDARY),
            "id_domain_boundary_basis": rel(ID_DOMAIN),
            "state_mutation_frame_basis": rel(MUTATION),
            "error_and_deterministic_order_basis": rel(ERROR_ORDER),
            "behavior_recipe_effect_coverage_basis": rel(EFFECT),
            "verifier_input_contract_basis": rel(VERIFIER),
        },
        "provenance": {
            "source_surface_inventory_hash": sha256_file(SURFACES),
            "operation_vocabulary_inventory_hash": sha256_file(OPERATIONS),
            "owner_scope_boundedness_rerun_002_hash": sha256_file(OWNER_SCOPE),
            "native_seed_file_boundary_basis_hash": sha256_file(FILE_BOUNDARY),
            "id_domain_boundary_basis_hash": sha256_file(ID_DOMAIN),
            "state_mutation_frame_basis_hash": sha256_file(MUTATION),
            "error_and_deterministic_order_basis_hash": sha256_file(ERROR_ORDER),
            "behavior_recipe_effect_coverage_basis_hash": sha256_file(EFFECT),
            "verifier_input_contract_basis_hash": sha256_file(VERIFIER),
        },
        "candidates": candidates,
        "candidate_pool": {
            "input_candidate_count": len(candidates),
            "source_plan_derivable_count": len(eligible),
            "behavior_recipe_derivable_count": len(eligible),
            "selection_eligible_count": len(eligible),
            "ambiguous_derivable_count": len(eligible) if len(eligible) > 1 else 0,
        },
        "decision": decision,
        "claims": {
            "manual_owner_selection": 0,
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
        print("mirbuilder-id-scalar-source-plan-and-recipe-derivability-resolution-003 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
