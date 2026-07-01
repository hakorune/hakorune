#!/usr/bin/env python3
"""Rerun ID scalar SourcePlanAndRecipe derivability after inventories."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-source-plan-and-recipe-derivability-resolution-002-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-DERIVABILITY-RESOLUTION-002"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

BASIS = FIXTURES / "mirbuilder-id-scalar-source-plan-derivation-basis-v0.json"
SURFACES = FIXTURES / "mirbuilder-id-scalar-source-surface-inventory-v0.json"
OPERATIONS = FIXTURES / "mirbuilder-id-scalar-operation-vocabulary-inventory-v0.json"

MISSING_BASIS = [
    "OwnerScopeBoundedNotProven",
    "BehaviorRecipeEffectCoverageNotProven",
    "IdDomainBoundaryNotDeclared",
    "StateMutationFrameNotDeclared",
    "ErrorSemanticsNotDeclared",
    "DeterministicOrderNotDeclared",
    "VerifierInputContractNotDeclared",
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    surfaces = read_json(SURFACES)
    operations = read_json(OPERATIONS)

    surface_by_owner = {row["owner_edge_id"]: row for row in surfaces.get("candidates") or []}
    rows = []
    for op_candidate in operations.get("candidates") or []:
        owner = op_candidate["owner_edge_id"]
        surface_candidate = surface_by_owner[owner]
        source_complete = bool(surface_candidate.get("required_source_surfaces_complete"))
        operation_complete = bool(op_candidate.get("operation_vocabulary_complete"))
        nominal_id_preserved = (
            operation_complete
            and all((row.get("raw_i64_interchangeability") == 0) for row in op_candidate.get("operation_rows") or [])
        )
        missing = list(MISSING_BASIS)
        source_plan_derivable = source_complete and operation_complete and nominal_id_preserved and not missing
        rows.append(
            {
                "owner_edge_id": owner,
                "source_surface_count": surface_candidate.get("required_source_surface_count"),
                "operation_surface_count": op_candidate.get("source_surface_count"),
                "required_source_surfaces_complete": source_complete,
                "operation_vocabulary_complete": operation_complete,
                "nominal_id_domain_isolation_preserved": nominal_id_preserved,
                "owner_scope_bounded": False,
                "behavior_recipe_effect_coverage_complete": False,
                "id_domain_boundary_declared": False,
                "state_mutation_frame_declared": False,
                "error_semantics_declared": False,
                "deterministic_order_declared": False,
                "verifier_input_contract_declared": False,
                "source_plan_derivable": source_plan_derivable,
                "behavior_recipe_derivable": False,
                "selection_eligible": False,
                "blocked_by": missing,
                "next_card": None,
            }
        )

    derivable = [row for row in rows if row["source_plan_derivable"]]
    decision = {
        "kind": "KeepStopped",
        "reason_token": "IdScalarSourcePlanDerivabilityRequiresScopeAndRecipeBasis",
        "selected_owner_edge_id": None,
        "selected_next_card": DESIGN_STOP,
    }
    if len(derivable) == 1:
        owner = derivable[0]["owner_edge_id"]
        decision = {
            "kind": "SelectSourcePlanAndRecipe",
            "reason_token": "ExactlyOneIdScalarSourcePlanAndRecipeDerivableCandidate",
            "selected_owner_edge_id": owner,
            "selected_next_card": "MIRBUILDER-" + owner.upper().replace("::", "-").replace("_", "-") + "-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-001",
        }
    elif len(derivable) > 1:
        decision["reason_token"] = "MultipleEqualIdScalarSourcePlanDerivabilityCandidates"

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarSourcePlanAndRecipeDerivabilityResolutionV2",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "source_plan_derivation_basis": rel(BASIS),
            "source_surface_inventory": rel(SURFACES),
            "operation_vocabulary_inventory": rel(OPERATIONS),
        },
        "provenance": {
            "source_plan_derivation_basis_hash": sha256_file(BASIS),
            "source_surface_inventory_hash": sha256_file(SURFACES),
            "operation_vocabulary_inventory_hash": sha256_file(OPERATIONS),
        },
        "previous_state": {
            "basis_token": basis.get("token"),
            "surface_inventory_token": surfaces.get("token"),
            "operation_vocabulary_token": operations.get("token"),
            "operation_vocabulary_reason_token": (operations.get("decision") or {}).get("reason_token"),
        },
        "candidates": rows,
        "candidate_pool": {
            "input_candidate_count": len(rows),
            "required_source_surfaces_complete_count": len([row for row in rows if row["required_source_surfaces_complete"]]),
            "operation_vocabulary_complete_count": len([row for row in rows if row["operation_vocabulary_complete"]]),
            "nominal_id_domain_preserved_count": len([row for row in rows if row["nominal_id_domain_isolation_preserved"]]),
            "owner_scope_bounded_count": 0,
            "behavior_recipe_effect_coverage_complete_count": 0,
            "source_plan_derivable_count": len(derivable),
            "behavior_recipe_derivable_count": 0,
            "selection_eligible_count": len([row for row in rows if row["selection_eligible"]]),
        },
        "decision": decision,
        "claims": {
            "source_plan_derivability_rerun_completed": 1,
            "source_plan_materialization": 0,
            "behavior_recipe_materialization": 0,
            "verifier_result_materialization": 0,
            "derived_artifact_seed_draft_materialization": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "source_selfhost_claim": 0,
            "manual_owner_selection": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "generated_artifact_as_native_edit_authority": 0,
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
        print("mirbuilder-id-scalar-source-plan-and-recipe-derivability-resolution-002 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
