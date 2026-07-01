#!/usr/bin/env python3
"""Define the ID scalar SourcePlan derivation basis."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-source-plan-derivation-basis-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-SOURCE-PLAN-DERIVATION-BASIS-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-ID-SCALAR-SOURCE-SURFACE-INVENTORY-001"

DERIVABILITY = FIXTURES / "mirbuilder-id-scalar-source-plan-and-recipe-derivability-resolution-v0.json"
CONTRACT = FIXTURES / "mirbuilder-id-scalar-seed-evidence-contract-v0.json"
READINESS = FIXTURES / "mirbuilder-id-scalar-domain-seed-readiness-resolution-002-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    derivability = read_json(DERIVABILITY)
    pool = derivability.get("candidate_pool") or {}
    decision = derivability.get("decision") or {}

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarSourcePlanDerivationBasisV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "source_plan_derivability_resolution": rel(DERIVABILITY),
            "seed_evidence_contract": rel(CONTRACT),
            "seed_readiness_resolution_002": rel(READINESS),
        },
        "provenance": {
            "source_plan_derivability_resolution_hash": sha256_file(DERIVABILITY),
            "seed_evidence_contract_hash": sha256_file(CONTRACT),
            "seed_readiness_resolution_002_hash": sha256_file(READINESS),
        },
        "previous_state": {
            "input_candidate_count": pool.get("input_candidate_count"),
            "source_plan_derivable_count": pool.get("source_plan_derivable_count"),
            "behavior_recipe_derivable_count": pool.get("behavior_recipe_derivable_count"),
            "selection_eligible_count": pool.get("selection_eligible_count"),
            "previous_reason_token": decision.get("reason_token"),
            "blocked_by": [
                "SourcePlanDerivabilityNotProven",
                "BehaviorRecipeDerivabilityNotProven",
                "DescriptorOnlyIsNotSourcePlanAndRecipe",
            ],
        },
        "basis": {
            "directability_only_is_source_plan": False,
            "descriptor_only_is_source_plan": False,
            "source_plan_derivation_allowed": True,
            "source_plan_derivation_requires_machine_derived_surface_set": True,
            "source_plan_derivation_requires_operation_vocabulary": True,
            "source_plan_derivation_requires_behavior_recipe": True,
        },
        "source_plan_derivable_requires": [
            "owner_edge_confidence_exact_or_fixture",
            "owner_scope_bounded",
            "required_source_surfaces_complete",
            "operation_vocabulary_complete",
            "behavior_recipe_effect_coverage_complete",
            "nominal_id_domain_isolation_preserved",
            "id_domain_boundary_declared",
            "state_mutation_frame_declared",
            "error_semantics_declared",
            "deterministic_order_declared",
            "verifier_input_contract_declared",
            "no_borrow_policy_gap",
            "no_carrier_type_transport_gap",
            "no_runtime_fallback",
            "no_new_backend_route",
            "no_new_abi",
            "no_new_python_semantic_projector",
        ],
        "component_order": [
            "SourceSurfaceInventory",
            "OperationVocabularyInventory",
            "SourcePlanAndRecipeDerivabilityRerun",
            "SourcePlanAndRecipeMaterialization",
            "VerifierResultFixture",
            "DerivedArtifactSeedDraftInput",
            "SeedReadinessRerun",
            "NativeSourceSeed",
            "HakoAdoptionDecision",
        ],
        "decision": {
            "kind": "PolicyDefined",
            "reason_token": "IdScalarSourcePlanDerivationBasisDefined",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "directability_only_is_source_plan": 0,
            "descriptor_only_is_source_plan": 0,
            "source_plan_derivation_basis_defined": 1,
            "source_plan_implied_by_descriptor": 0,
            "source_plan_implied_by_directability": 0,
            "behavior_recipe_implied_by_descriptor": 0,
            "behavior_recipe_implied_by_directability": 0,
            "verifier_result_implied_by_source_plan": 0,
            "derived_artifact_seed_draft_implied_by_verifier": 0,
            "raw_i64_interchangeability": 0,
            "nominal_id_erasure": 0,
            "id_sentinel_semantics_inferred": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "source_selfhost_claim": 0,
            "manual_owner_selection": 0,
            "cluster_size_as_proof": 0,
            "directable_row_count_as_proof": 0,
            "lexical_order_as_seed_selection_proof": 0,
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
        print("mirbuilder-id-scalar-source-plan-derivation-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
