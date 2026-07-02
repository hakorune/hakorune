#!/usr/bin/env python3
"""Materialize the emission_ssa_phi ID scalar SourcePlanAndRecipe component."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-emission-ssa-phi-id-scalar-source-plan-and-recipe-v0.json"

TOKEN = "MIRBUILDER-EMISSION_SSA_PHI-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-001"
NEXT_CARD = "MIRBUILDER-EMISSION_SSA_PHI-ID-SCALAR-VERIFIER-RESULT-001"
OWNER = "mirbuilder::emission_ssa_phi"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

RESOLUTION = FIXTURES / "mirbuilder-id-scalar-derivable-owner-discriminator-resolution-002-v0.json"
PROJECTION = FIXTURES / "mirbuilder-emission-ssa-phi-projection-policy-v0.json"
MUTATION = FIXTURES / "mirbuilder-id-scalar-state-mutation-frame-basis-v0.json"
EFFECT = FIXTURES / "mirbuilder-id-scalar-behavior-recipe-effect-coverage-basis-v0.json"
VERIFIER = FIXTURES / "mirbuilder-id-scalar-verifier-input-contract-basis-v0.json"
ID_DOMAIN = FIXTURES / "mirbuilder-id-scalar-id-domain-boundary-basis-v0.json"
ERROR_ORDER = FIXTURES / "mirbuilder-id-scalar-error-and-deterministic-order-basis-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def selected_resolution() -> dict[str, Any]:
    resolution = read_json(RESOLUTION)
    decision = resolution.get("decision") or {}
    if decision.get("selected_owner_edge_id") != OWNER:
        raise SystemExit("resolution 002 no longer selects emission_ssa_phi")
    return resolution


def build_fixture() -> dict[str, Any]:
    resolution = selected_resolution()
    projection = read_json(PROJECTION)
    mutation = read_json(MUTATION)
    effect = read_json(EFFECT)
    verifier = read_json(VERIFIER)
    id_domain = read_json(ID_DOMAIN)
    error_order = read_json(ERROR_ORDER)

    descriptor = projection.get("emission_ssa_phi_descriptor") or {}
    source_surfaces = projection.get("source_surfaces") or []
    mutation_frames = [
        row for row in mutation.get("mutation_frames") or [] if row.get("owner_edge_id") == OWNER
    ]
    effect_rows = [
        row for row in effect.get("effect_rows") or [] if row.get("owner_edge_id") == OWNER
    ]

    plan = {
        "plan_id": "MirBuilderEmissionSsaPhiIdScalarSourcePlanV1",
        "owner_edge_id": OWNER,
        "source_surfaces": [
            {
                "source_id": row.get("source_id"),
                "role": row.get("role"),
                "borrow_axis": row.get("borrow_axis"),
                "return_type": row.get("return_type"),
            }
            for row in source_surfaces
        ],
        "standalone_projection_subject": True,
        "descriptor_id": descriptor.get("descriptor_id"),
        "state_targets": [
            {
                "state_target_id": row.get("state_target_id"),
                "semantic_resource": row.get("semantic_resource"),
                "target_kind": row.get("target_kind"),
            }
            for row in mutation_frames
        ],
        "id_domain_boundaries": [
            row.get("domain")
            for row in id_domain.get("domain_boundaries") or []
            if OWNER in (row.get("owner_edge_counts") or {})
        ],
    }

    recipe = {
        "recipe_id": "MirBuilderEmissionSsaPhiIdScalarBehaviorRecipeV1",
        "owner_edge_id": OWNER,
        "effect_rows": [
            {
                "source_id": row.get("source_id"),
                "operation_token": row.get("operation_token"),
                "effect_class": row.get("effect_class"),
                "requires_mutation_frame": row.get("requires_mutation_frame"),
                "requires_error_semantics": row.get("requires_error_semantics"),
                "requires_deterministic_order": row.get("requires_deterministic_order"),
            }
            for row in effect_rows
        ],
        "mutation_frames": [
            {
                "mutation_frame_id": row.get("mutation_frame_id"),
                "access": row.get("access"),
                "owner_return_state": row.get("owner_return_state"),
                "mutation_order": row.get("mutation_order"),
                "rollback_requirement": row.get("rollback_requirement"),
                "cleanup_requirement": row.get("cleanup_requirement"),
            }
            for row in mutation_frames
        ],
        "return_contract": descriptor.get("return_contract"),
        "verifier_observable_effects_declared": True,
    }

    return {
        "schema_version": 0,
        "kind": "MirBuilderEmissionSsaPhiIdScalarSourcePlanAndRecipeV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "discriminator_resolution_002": rel(RESOLUTION),
            "projection_policy": rel(PROJECTION),
            "state_mutation_frame_basis": rel(MUTATION),
            "behavior_recipe_effect_coverage_basis": rel(EFFECT),
            "verifier_input_contract_basis": rel(VERIFIER),
            "id_domain_boundary_basis": rel(ID_DOMAIN),
            "error_and_deterministic_order_basis": rel(ERROR_ORDER),
        },
        "provenance": {
            "discriminator_resolution_002_hash": sha256_file(RESOLUTION),
            "projection_policy_hash": sha256_file(PROJECTION),
            "state_mutation_frame_basis_hash": sha256_file(MUTATION),
            "behavior_recipe_effect_coverage_basis_hash": sha256_file(EFFECT),
            "verifier_input_contract_basis_hash": sha256_file(VERIFIER),
            "id_domain_boundary_basis_hash": sha256_file(ID_DOMAIN),
            "error_and_deterministic_order_basis_hash": sha256_file(ERROR_ORDER),
        },
        "selected_owner": {
            "owner_edge_id": OWNER,
            "selection_reason": (resolution.get("decision") or {}).get("reason_token"),
            "selected_by_owner_name": False,
            "selected_by_count": False,
        },
        "source_plan": plan,
        "behavior_recipe": recipe,
        "verifier_preconditions": verifier.get("input_fact_sets") or [],
        "decision": {
            "kind": "SourcePlanAndRecipeMaterialized",
            "reason_token": "EmissionSsaPhiIdScalarSourcePlanAndRecipeMaterialized",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "source_plan_materialization": 1,
            "behavior_recipe_materialization": 1,
            "verifier_result_materialization": 0,
            "derived_artifact_seed_draft_materialization": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "source_selfhost_claim": 0,
            "owner_name_as_proof": 0,
            "surface_count_as_proof": 0,
            "row_count_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
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
        print("mirbuilder-emission-ssa-phi-id-scalar-source-plan-and-recipe unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
