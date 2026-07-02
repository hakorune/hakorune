#!/usr/bin/env python3
"""Rerun ID-scalar seed readiness after emission_ssa_phi seed packet components."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-domain-seed-readiness-resolution-003-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-DOMAIN-SEED-READINESS-RESOLUTION-003"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
OWNER = "mirbuilder::emission_ssa_phi"
NEXT_CARD = "MIRBUILDER-EMISSION_SSA_PHI-HAKO-NATIVE-SOURCE-SEED-001"

READINESS_002 = FIXTURES / "mirbuilder-id-scalar-domain-seed-readiness-resolution-002-v0.json"
SOURCE_PLAN = FIXTURES / "mirbuilder-emission-ssa-phi-id-scalar-source-plan-and-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-emission-ssa-phi-id-scalar-verifier-result-v0.json"
SEED_DRAFT = FIXTURES / "mirbuilder-emission-ssa-phi-id-scalar-derived-artifact-seed-draft-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict:
    previous = read_json(READINESS_002)
    source_plan = read_json(SOURCE_PLAN)
    verifier = read_json(VERIFIER)
    seed_draft = read_json(SEED_DRAFT)

    owner_rows = []
    ready_count = 0
    for row in previous.get("owner_edge_readiness") or []:
        current = dict(row)
        if row.get("owner_edge_id") == OWNER:
            current["derived_artifact_seed_draft_input_available"] = (
                (seed_draft.get("claims") or {}).get("derived_artifact_seed_draft_materialization")
                == 1
            )
            current["verifier_result_fixture_present"] = (
                (verifier.get("claims") or {}).get("verifier_result_materialization") == 1
            )
            current["source_plan_and_recipe_present"] = (
                (source_plan.get("claims") or {}).get("source_plan_materialization") == 1
                and (source_plan.get("claims") or {}).get("behavior_recipe_materialization") == 1
            )
            current["evidence_refs"] = {
                "source_plan_and_recipe": [rel(SOURCE_PLAN)],
                "verifier_result": [rel(VERIFIER)],
                "derived_artifact_seed_draft_input": [rel(SEED_DRAFT)],
            }

        blocked_by = []
        if not current.get("derived_artifact_seed_draft_input_available"):
            blocked_by.append("MissingDerivedArtifactSeedDraftInput")
        if not current.get("verifier_result_fixture_present"):
            blocked_by.append("MissingVerifierResultFixture")
        if not current.get("source_plan_and_recipe_present"):
            blocked_by.append("MissingSourcePlanAndRecipe")
        if blocked_by:
            blocked_by.append("DirectabilityOnlyIsNotSeedEvidence")
        current["blocked_by"] = blocked_by
        current["selection_eligible_for_seed_materialization"] = not blocked_by
        current["next_card"] = NEXT_CARD if current["selection_eligible_for_seed_materialization"] else None
        if current["selection_eligible_for_seed_materialization"]:
            ready_count += 1
        owner_rows.append(current)

    if ready_count == 1:
        selected = next(row for row in owner_rows if row["selection_eligible_for_seed_materialization"])
        decision = {
            "kind": "SelectNativeSeedMaterialization",
            "reason_token": "ExactlyOneIdScalarSeedMaterializationReadyOwnerEdgeAfterSeedPacket",
            "selected_owner_edge_id": selected["owner_edge_id"],
            "selected_next_card": selected["next_card"],
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "NoUniqueIdScalarSeedMaterializationReadyOwnerEdgeAfterSeedPacket",
            "selected_owner_edge_id": None,
            "selected_next_card": DESIGN_STOP,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarDomainSeedReadinessResolutionV3",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "previous_seed_readiness_resolution": rel(READINESS_002),
            "source_plan_and_recipe": rel(SOURCE_PLAN),
            "verifier_result": rel(VERIFIER),
            "derived_artifact_seed_draft_input": rel(SEED_DRAFT),
        },
        "provenance": {
            "previous_seed_readiness_resolution_hash": sha256_file(READINESS_002),
            "source_plan_and_recipe_hash": sha256_file(SOURCE_PLAN),
            "verifier_result_hash": sha256_file(VERIFIER),
            "derived_artifact_seed_draft_input_hash": sha256_file(SEED_DRAFT),
        },
        "owner_edge_readiness": owner_rows,
        "candidate_pool": {
            "readiness_input_owner_edge_count": len(owner_rows),
            "seed_materialization_ready_count": ready_count,
            "selected_owner_count": 1 if decision.get("selected_owner_edge_id") else 0,
            "missing_seed_evidence_owner_edge_count": len(
                [row for row in owner_rows if not row["selection_eligible_for_seed_materialization"]]
            ),
        },
        "decision": decision,
        "claims": {
            "manual_owner_selection": 0,
            "cluster_size_as_proof": 0,
            "directable_row_count_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "generated_artifact_as_native_edit_authority": 0,
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
        print("mirbuilder-id-scalar-domain-seed-readiness-resolution-003 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
