#!/usr/bin/env python3
"""Resolve ID scalar SourcePlanAndRecipe derivability."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-source-plan-and-recipe-derivability-resolution-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-DERIVABILITY-RESOLUTION-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

SELECTION = FIXTURES / "mirbuilder-id-scalar-seed-packet-candidate-selection-v0.json"
CONTRACT = FIXTURES / "mirbuilder-id-scalar-seed-evidence-contract-v0.json"
READINESS = FIXTURES / "mirbuilder-id-scalar-domain-seed-readiness-resolution-002-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def projection_policy_refs(owner_edge_id: str) -> list[str]:
    refs: list[str] = []
    for path in sorted(FIXTURES.glob("*projection-policy-v0.json")):
        if owner_edge_id in path.read_text(encoding="utf-8", errors="ignore"):
            refs.append(rel(path))
    return refs


def build_fixture() -> dict[str, Any]:
    selection = read_json(SELECTION)
    contract = read_json(CONTRACT)
    readiness = read_json(READINESS)

    candidates = []
    for row in selection.get("candidate_rows", []):
        if row.get("owner_edge_confidence") != "FixtureMapped":
            continue
        refs = projection_policy_refs(row["owner_edge_id"])
        candidates.append(
            {
                "owner_edge_id": row["owner_edge_id"],
                "owner_edge_confidence": row.get("owner_edge_confidence"),
                "source_plan_derivable": False,
                "behavior_recipe_derivable": False,
                "derivation_authority": "projection_policy_descriptor_refs_only",
                "owner_scope": "Unknown",
                "required_source_surfaces": [],
                "missing_surfaces": ["SourcePlanAndRecipe"],
                "source_plan_kind": "NotDerived",
                "behavior_recipe_kind": "NotDerived",
                "source_surface_confidence": "DescriptorOnly",
                "operation_vocabulary_complete": False,
                "nominal_id_types_used": [],
                "nominal_id_domain_isolation": row.get("nominal_id_domain_isolation"),
                "verifier_preconditions": ["SourcePlanAndRecipe"],
                "draft_input_preconditions": ["VerifierResultFixture"],
                "native_seed_file_boundary": "NotEvaluatedAtThisStage",
                "raw_i64_interchangeability": 0,
                "blocked_by": [
                    "SourcePlanDerivabilityNotProven",
                    "BehaviorRecipeDerivabilityNotProven",
                    "DescriptorOnlyIsNotSourcePlanAndRecipe",
                ],
                "selection_eligible": False,
                "projection_policy_refs": refs,
                "next_card": None,
            }
        )

    eligible = [row for row in candidates if row["selection_eligible"]]
    if len(eligible) == 1:
        decision = {
            "kind": "SelectSourcePlanAndRecipe",
            "reason_token": "ExactlyOneIdScalarSourcePlanAndRecipeDerivabilityCandidate",
            "selected_owner_edge_id": eligible[0]["owner_edge_id"],
            "selected_next_card": "MIRBUILDER-" + eligible[0]["owner_edge_id"].upper().replace("::", "-").replace("_", "-") + "-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-001",
        }
    else:
        reason = "NoIdScalarSourcePlanAndRecipeDerivabilityCandidate"
        if len(eligible) > 1:
            reason = "MultipleEqualIdScalarSourcePlanDerivabilityCandidates"
        decision = {
            "kind": "KeepStopped",
            "reason_token": reason,
            "selected_owner_edge_id": None,
            "selected_next_card": DESIGN_STOP,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarSourcePlanAndRecipeDerivabilityResolutionV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "seed_packet_candidate_selection": rel(SELECTION),
            "seed_evidence_contract": rel(CONTRACT),
            "seed_readiness_resolution_002": rel(READINESS),
        },
        "provenance": {
            "seed_packet_candidate_selection_hash": sha256_file(SELECTION),
            "seed_evidence_contract_hash": sha256_file(CONTRACT),
            "seed_readiness_resolution_002_hash": sha256_file(READINESS),
        },
        "previous_state": {
            "input_owner_edge_count": (selection.get("candidate_pool") or {}).get("input_owner_edge_count"),
            "packet_generation_candidate_count": (selection.get("candidate_pool") or {}).get("packet_generation_candidate_count"),
            "selected_candidate_count": (selection.get("candidate_pool") or {}).get("selected_candidate_count"),
            "ambiguous_candidate_count": (selection.get("candidate_pool") or {}).get("ambiguous_candidate_count"),
            "previous_reason_token": (selection.get("decision") or {}).get("reason_token"),
        },
        "component": {
            "component_id": "SourcePlanAndRecipe",
            "component_order": 1,
            "directability_only_is_seed_evidence": False,
            "directability_may_feed_component_generation": True,
        },
        "derivability_rules": {
            "source_plan_derivable_requires": [
                "owner_edge_confidence_exact_or_fixture",
                "owner_scope_bounded",
                "required_source_surfaces_machine_derived",
                "nominal_id_domain_isolation_preserved",
                "no_borrow_policy_gap",
                "no_carrier_type_transport_gap",
                "no_runtime_fallback",
                "no_new_backend_route",
                "no_new_abi",
                "no_new_python_semantic_projector",
            ],
            "behavior_recipe_derivable_requires": [
                "direct_operations_enumerated",
                "nominal_id_transport_operations_use_nominal_domain_wrappers",
                "no_raw_i64_interchangeability",
                "no_generated_artifact_as_native_edit_authority",
                "no_hako_generation_inside_resolver",
            ],
        },
        "candidates": candidates,
        "candidate_pool": {
            "input_candidate_count": len(candidates),
            "source_plan_derivable_count": len([row for row in candidates if row["source_plan_derivable"]]),
            "behavior_recipe_derivable_count": len([row for row in candidates if row["behavior_recipe_derivable"]]),
            "selection_eligible_count": len(eligible),
            "ambiguous_derivable_count": len(eligible) if len(eligible) > 1 else 0,
        },
        "decision": decision,
        "claims": {
            "seed_packet_candidate_selection_consumed": 1,
            "seed_evidence_contract_consumed": 1,
            "seed_readiness_resolution_002_consumed": 1,
            "manual_owner_selection": 0,
            "cluster_size_as_proof": 0,
            "directable_row_count_as_proof": 0,
            "lexical_order_as_seed_selection_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "generated_artifact_as_native_edit_authority": 0,
            "source_plan_implied_by_directability": 0,
            "behavior_recipe_implied_by_directability": 0,
            "verifier_result_implied_by_source_plan": 0,
            "derived_artifact_seed_draft_implied_by_verifier": 0,
            "raw_i64_interchangeability": 0,
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
        print("mirbuilder-id-scalar-source-plan-and-recipe-derivability-resolution unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
