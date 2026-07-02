#!/usr/bin/env python3
"""Resolve tied ID scalar derivable owners using allowed proof axes only."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-derivable-owner-discriminator-resolution-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-RESOLUTION-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

TYPED_INDEX = FIXTURES / "mirbuilder-id-scalar-typed-evidence-index-policy-v0.json"
OP_AUTHORITY = FIXTURES / "mirbuilder-id-scalar-operation-vocabulary-authority-split-v0.json"
SELECTOR_GUARD = FIXTURES / "semantic-selector-no-lexical-tiebreak-guard-v0.json"
DERIVABILITY = FIXTURES / "mirbuilder-id-scalar-source-plan-and-recipe-derivability-resolution-003-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def has_entry(row: dict[str, Any], kind: str) -> bool:
    return any(
        entry.get("artifact_kind") == kind and entry.get("typed_refs_complete") is True
        for entry in row.get("evidence_entries") or []
    )


def build_fixture() -> dict[str, Any]:
    typed_index = read_json(TYPED_INDEX)
    op_authority = read_json(OP_AUTHORITY)
    selector_guard = read_json(SELECTOR_GUARD)
    derivability = read_json(DERIVABILITY)

    op_by_owner = {row["owner_edge_id"]: row for row in op_authority.get("owner_rows") or []}
    typed_by_owner = {row["owner_edge_id"]: row for row in typed_index.get("typed_evidence_rows") or []}
    derivable_owners = [
        row["owner_edge_id"]
        for row in derivability.get("candidates") or []
        if row.get("selection_eligible")
    ]

    selector_guard_clean = (
        (selector_guard.get("active_enforcement") or {}).get("forbidden_active_finding_count") == 0
    )

    candidates = []
    for owner in derivable_owners:
        typed = typed_by_owner[owner]
        op = op_by_owner[owner]
        proof_axes = {
            "TypedEvidenceIndexCompleteness": bool(typed.get("typed_evidence_complete")),
            "VerifierInputContractCompleteness": has_entry(typed, "VerifierInputContract"),
            "NativeSeedFileBoundaryDeterminism": has_entry(typed, "NativeSeedFileBoundary"),
            "StateTargetClosureQuality": has_entry(typed, "OwnerScopeBoundedness"),
            "OperationEffectClassCompleteness": has_entry(typed, "BehaviorRecipeEffectCoverage"),
            "SourcePlanRecipeComponentReadiness": all(
                has_entry(typed, kind)
                for kind in [
                    "SourceSurfaceInventory",
                    "StateMutationFrame",
                    "ErrorSemantics",
                    "DeterministicOrder",
                    "VerifierInputContract",
                ]
            ),
            "SemanticOperationAuthorityComplete": bool(
                op.get("semantic_operation_authority_complete")
            ),
            "SelectorGuardClean": selector_guard_clean,
        }
        proof_tuple = [proof_axes[key] for key in sorted(proof_axes)]
        candidates.append(
            {
                "owner_edge_id": owner,
                "proof_axes": proof_axes,
                "proof_tuple": proof_tuple,
                "selection_eligible": all(proof_tuple),
                "blocked_by": [key for key, value in proof_axes.items() if not value],
                "next_card": f"MIRBUILDER-{owner.split('::', 1)[1].upper()}-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-001"
                if all(proof_tuple)
                else None,
            }
        )

    eligible = [row for row in candidates if row["selection_eligible"]]
    unique_proof_tuples = {tuple(row["proof_tuple"]) for row in eligible}
    if len(eligible) == 1:
        selected = eligible[0]
        decision = {
            "kind": "SelectSourcePlanAndRecipe",
            "reason_token": "ExactlyOneIdScalarDerivableOwnerDiscriminatorCandidate",
            "selected_owner_edge_id": selected["owner_edge_id"],
            "selected_next_card": selected["next_card"],
        }
    elif len(unique_proof_tuples) > 1:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "IdScalarDiscriminatorProofAxisPriorityMissing",
            "selected_owner_edge_id": None,
            "selected_next_card": DESIGN_STOP,
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "MultipleEqualIdScalarDerivableOwnerDiscriminatorCandidates",
            "selected_owner_edge_id": None,
            "selected_next_card": DESIGN_STOP,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarDerivableOwnerDiscriminatorResolutionV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "typed_evidence_index_policy": rel(TYPED_INDEX),
            "operation_vocabulary_authority_split": rel(OP_AUTHORITY),
            "semantic_selector_no_lexical_tiebreak_guard": rel(SELECTOR_GUARD),
            "derivability_rerun_003": rel(DERIVABILITY),
        },
        "provenance": {
            "typed_evidence_index_policy_hash": sha256_file(TYPED_INDEX),
            "operation_vocabulary_authority_split_hash": sha256_file(OP_AUTHORITY),
            "semantic_selector_no_lexical_tiebreak_guard_hash": sha256_file(SELECTOR_GUARD),
            "derivability_rerun_003_hash": sha256_file(DERIVABILITY),
        },
        "selection_policy": {
            "manual_owner_selection": False,
            "owner_name_as_proof": False,
            "lexical_order_as_proof": False,
            "surface_count_as_proof": False,
            "row_count_as_proof": False,
            "coverage_percentage_as_proof": False,
            "route_membership_alone_as_proof": False,
            "proof_axes_only": True,
        },
        "candidates": candidates,
        "candidate_pool": {
            "input_derivable_owner_count": len(derivable_owners),
            "selection_eligible_count": len(eligible),
            "unique_proof_tuple_count": len(unique_proof_tuples),
            "selected_owner_count": 1 if decision.get("selected_owner_edge_id") else 0,
        },
        "decision": decision,
        "claims": {
            "manual_owner_selection": 0,
            "owner_name_as_proof": 0,
            "lexical_order_as_proof": 0,
            "surface_count_as_proof": 0,
            "row_count_as_proof": 0,
            "coverage_percentage_as_proof": 0,
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
        print("mirbuilder-id-scalar-derivable-owner-discriminator-resolution unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
