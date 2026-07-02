#!/usr/bin/env python3
"""Verify the emission_ssa_phi ID scalar SourcePlanAndRecipe component."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-emission-ssa-phi-id-scalar-verifier-result-v0.json"

TOKEN = "MIRBUILDER-EMISSION_SSA_PHI-ID-SCALAR-VERIFIER-RESULT-001"
NEXT_CARD = "MIRBUILDER-EMISSION_SSA_PHI-ID-SCALAR-DERIVED-ARTIFACT-SEED-DRAFT-001"
SOURCE_PLAN = FIXTURES / "mirbuilder-emission-ssa-phi-id-scalar-source-plan-and-recipe-v0.json"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict:
    source = read_json(SOURCE_PLAN)
    plan = source.get("source_plan") or {}
    recipe = source.get("behavior_recipe") or {}
    preconditions = source.get("verifier_preconditions") or []

    checks = [
        {
            "check_id": "SourcePlanOwnerMatchesRecipeOwner",
            "passed": plan.get("owner_edge_id") == recipe.get("owner_edge_id"),
        },
        {
            "check_id": "StandaloneProjectionSubjectPresent",
            "passed": plan.get("standalone_projection_subject") is True,
        },
        {
            "check_id": "EffectRowsPresent",
            "passed": bool(recipe.get("effect_rows")),
        },
        {
            "check_id": "MutationFramesPresent",
            "passed": bool(recipe.get("mutation_frames")),
        },
        {
            "check_id": "VerifierPreconditionsPresent",
            "passed": bool(preconditions),
        },
        {
            "check_id": "NoNativeSeedOrHakoClaimInSourcePlan",
            "passed": all(
                (source.get("claims") or {}).get(key) == 0
                for key in [
                    "native_seed_materialization",
                    "hako_generation",
                    "hako_adopted_decision",
                    "source_selfhost_claim",
                ]
            ),
        },
    ]
    passed = all(row["passed"] for row in checks)

    return {
        "schema_version": 0,
        "kind": "MirBuilderEmissionSsaPhiIdScalarVerifierResultV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "source_plan_and_recipe": rel(SOURCE_PLAN),
        },
        "provenance": {
            "source_plan_and_recipe_hash": sha256_file(SOURCE_PLAN),
        },
        "verification_subject": {
            "owner_edge_id": plan.get("owner_edge_id"),
            "plan_id": plan.get("plan_id"),
            "recipe_id": recipe.get("recipe_id"),
            "result_kind": "VerifiedSourcePlanAndRecipe" if passed else "RejectedSourcePlanAndRecipe",
        },
        "checks": checks,
        "candidate_pool": {
            "check_count": len(checks),
            "passed_check_count": sum(1 for row in checks if row["passed"]),
            "failed_check_count": sum(1 for row in checks if not row["passed"]),
        },
        "decision": {
            "kind": "VerifierResultFixtureMaterialized" if passed else "KeepStopped",
            "reason_token": "EmissionSsaPhiIdScalarSourcePlanAndRecipeVerified"
            if passed
            else "EmissionSsaPhiIdScalarSourcePlanAndRecipeVerifierFailed",
            "selected_next_card": NEXT_CARD if passed else DESIGN_STOP,
        },
        "claims": {
            "verifier_result_materialization": 1 if passed else 0,
            "verified_source_plan_and_recipe": 1 if passed else 0,
            "derived_artifact_seed_draft_materialization": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "source_selfhost_claim": 0,
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
        print("mirbuilder-emission-ssa-phi-id-scalar-verifier-result unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
