#!/usr/bin/env python3
"""Resolve tied ID scalar derivable owners using refined proof axes."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-derivable-owner-discriminator-resolution-002-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-RESOLUTION-002"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

RESOLUTION_001 = FIXTURES / "mirbuilder-id-scalar-derivable-owner-discriminator-resolution-v0.json"
REFINEMENT = FIXTURES / "mirbuilder-id-scalar-derivable-owner-proof-axis-refinement-v0.json"
MUTATION = FIXTURES / "mirbuilder-id-scalar-state-mutation-frame-basis-v0.json"
EFFECT = FIXTURES / "mirbuilder-id-scalar-behavior-recipe-effect-coverage-basis-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def projection_fixtures_by_owner() -> dict[str, dict[str, Any]]:
    result: dict[str, dict[str, Any]] = {}
    for path in FIXTURES.glob("mirbuilder-*-projection-policy-v0.json"):
        fixture = read_json(path)
        selected = fixture.get("selected_policy") or {}
        owner = selected.get("owner_edge")
        if owner:
            result[owner] = {"path": path, "fixture": fixture}
    return result


def descriptor_object(fixture: dict[str, Any]) -> dict[str, Any]:
    for key, value in fixture.items():
        if key.endswith("_descriptor") and isinstance(value, dict):
            return value
    return {}


def standalone_projection_subject_established(owner: str, projection: dict[str, Any]) -> bool:
    fixture = projection.get("fixture") or {}
    selected = fixture.get("selected_policy") or {}
    claims = fixture.get("claims") or {}
    decision = fixture.get("decision") or {}
    return all(
        [
            bool(fixture),
            decision.get("kind") == "SelectProjectionPolicyDescriptor",
            claims.get("descriptor_selected") == 1,
            selected.get("descriptor_selected") is True,
            selected.get("owner_edge") == owner,
            selected.get("policy") != "KeepParentOwner",
            selected.get("reason_token") != "DiagnosticOnly",
            bool(descriptor_object(fixture).get("descriptor_id")),
        ]
    )


def lifecycle_contract_descriptor_complete(projection: dict[str, Any]) -> bool:
    descriptor = descriptor_object(projection.get("fixture") or {})
    return all(
        [
            bool(descriptor.get("descriptor_id")),
            bool(descriptor.get("return_contract")),
            bool(descriptor.get("mutation_entrypoints")),
            bool(descriptor.get("contract_validators") or descriptor.get("analysis_predicates")),
            bool(descriptor.get("diagnostic_formatters")),
            bool(descriptor.get("source_markers")),
        ]
    )


def mutation_frame_semantic_complete(owner: str, mutation: dict[str, Any]) -> bool:
    frames = [
        row for row in mutation.get("mutation_frames") or [] if row.get("owner_edge_id") == owner
    ]
    if not frames:
        return False
    for frame in frames:
        access = set(frame.get("access") or [])
        tokens = " ".join(frame.get("operation_tokens") or [])
        if "Read" in access and frame.get("read_set_declared") is not True:
            return False
        if "Write" in access and frame.get("write_set_declared") is not True:
            return False
        if "Append" in access and frame.get("append_semantics") is not True:
            return False
        if "Patch" in tokens and frame.get("replace_semantics") is not True:
            return False
        if not frame.get("owner_return_state"):
            return False
        if not frame.get("mutation_order"):
            return False
        if not frame.get("rollback_requirement"):
            return False
        if not frame.get("cleanup_requirement"):
            return False
    return True


def verifier_effect_class_coverage_complete(owner: str, effect: dict[str, Any]) -> bool:
    rows = [row for row in effect.get("effect_rows") or [] if row.get("owner_edge_id") == owner]
    if not rows:
        return False
    for row in rows:
        if not row.get("effect_class") or not row.get("operation_token"):
            return False
        if row.get("verifier_visible") is not True:
            return False
        for key in [
            "requires_deterministic_order",
            "requires_error_semantics",
            "requires_mutation_frame",
        ]:
            if key not in row:
                return False
    return True


def next_card(owner: str) -> str:
    suffix = owner.split("::", 1)[1].upper()
    return f"MIRBUILDER-{suffix}-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-001"


def build_fixture() -> dict[str, Any]:
    resolution_001 = read_json(RESOLUTION_001)
    refinement = read_json(REFINEMENT)
    mutation = read_json(MUTATION)
    effect = read_json(EFFECT)
    projections = projection_fixtures_by_owner()

    candidates = []
    for prior in resolution_001.get("candidates") or []:
        if not prior.get("selection_eligible"):
            continue
        owner = prior["owner_edge_id"]
        projection = projections.get(owner) or {}
        refined_axes = {
            "StandaloneProjectionSubjectEstablished": standalone_projection_subject_established(
                owner, projection
            ),
            "LifecycleContractDescriptorCompleteness": lifecycle_contract_descriptor_complete(
                projection
            ),
            "MutationFrameSemanticCompleteness": mutation_frame_semantic_complete(
                owner, mutation
            ),
            "VerifierEffectClassCoverageCompleteness": verifier_effect_class_coverage_complete(
                owner, effect
            ),
        }
        proof_tuple = [refined_axes[key] for key in sorted(refined_axes)]
        candidates.append(
            {
                "owner_edge_id": owner,
                "prior_proof_axes": prior.get("proof_axes") or {},
                "refined_proof_axes": refined_axes,
                "refined_proof_tuple": proof_tuple,
                "selection_eligible": all((prior.get("proof_axes") or {}).values()),
                "blocked_by": [key for key, value in refined_axes.items() if not value],
                "next_card": next_card(owner),
            }
        )

    eligible = [row for row in candidates if row.get("selection_eligible")]
    sorted_tuples = sorted({tuple(row["refined_proof_tuple"]) for row in eligible}, reverse=True)
    best_tuple = sorted_tuples[0] if sorted_tuples else ()
    best = [row for row in eligible if tuple(row["refined_proof_tuple"]) == best_tuple]

    if len(best) == 1 and len(sorted_tuples) > 1:
        selected = best[0]
        decision = {
            "kind": "SelectSourcePlanAndRecipe",
            "reason_token": "ExactlyOneIdScalarDerivableOwnerAfterRefinedProofAxes",
            "selected_owner_edge_id": selected["owner_edge_id"],
            "selected_next_card": selected["next_card"],
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "MultipleEqualIdScalarRefinedProofAxisCandidates",
            "selected_owner_edge_id": None,
            "selected_next_card": DESIGN_STOP,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarDerivableOwnerDiscriminatorResolutionV2",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "discriminator_resolution_001": rel(RESOLUTION_001),
            "proof_axis_refinement": rel(REFINEMENT),
            "state_mutation_frame_basis": rel(MUTATION),
            "behavior_recipe_effect_coverage_basis": rel(EFFECT),
        },
        "provenance": {
            "discriminator_resolution_001_hash": sha256_file(RESOLUTION_001),
            "proof_axis_refinement_hash": sha256_file(REFINEMENT),
            "state_mutation_frame_basis_hash": sha256_file(MUTATION),
            "behavior_recipe_effect_coverage_basis_hash": sha256_file(EFFECT),
        },
        "selection_policy": {
            "raw_prior_projection_policy_disposition_as_proof": False,
            "historical_descriptor_presence_as_preference": False,
            "lifecycle_richness_as_proof": False,
            "mutation_complexity_as_proof": False,
            "effect_class_count_as_proof": False,
            "owner_name_as_proof": False,
            "row_count_as_proof": False,
        },
        "candidates": candidates,
        "candidate_pool": {
            "input_derivable_owner_count": len(candidates),
            "selection_eligible_count": len(eligible),
            "unique_refined_proof_tuple_count": len(sorted_tuples),
            "selected_owner_count": 1 if decision.get("selected_owner_edge_id") else 0,
        },
        "decision": decision,
        "claims": {
            "owner_name_as_proof": 0,
            "historical_descriptor_presence_as_preference": 0,
            "lifecycle_richness_as_proof": 0,
            "mutation_complexity_as_proof": 0,
            "effect_class_count_as_proof": 0,
            "surface_count_as_proof": 0,
            "row_count_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "source_plan_materialization": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "source_selfhost_claim": 0,
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
        print("mirbuilder-id-scalar-derivable-owner-discriminator-resolution-002 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
