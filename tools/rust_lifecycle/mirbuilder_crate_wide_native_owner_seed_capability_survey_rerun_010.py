#!/usr/bin/env python3
"""Rerun native-owner seed capability after emission_ssa_phi adoption."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-010-v0.json"

TOKEN = "MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-010"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

ADOPTION = FIXTURES / "mirbuilder-emission-ssa-phi-hako-adoption-decision-v0.json"
DISCRIMINATOR = FIXTURES / "mirbuilder-id-scalar-derivable-owner-discriminator-resolution-002-v0.json"
SEED_READINESS = FIXTURES / "mirbuilder-id-scalar-domain-seed-readiness-resolution-003-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    adoption = read_json(ADOPTION)
    discriminator = read_json(DISCRIMINATOR)
    readiness = read_json(SEED_READINESS)

    adopted_owner = adoption.get("family_id")
    adopted = (adoption.get("claims") or {}).get("hako_adopted") == 1
    candidates = discriminator.get("candidates") or []

    remaining = []
    for row in candidates:
        if row.get("owner_edge_id") == adopted_owner and adopted:
            continue
        refined = row.get("refined_proof_axes") or {}
        missing = [key for key, value in refined.items() if value is not True]
        remaining.append(
            {
                "owner_edge_id": row.get("owner_edge_id"),
                "prior_next_card": row.get("next_card"),
                "refined_proof_axes": refined,
                "refined_proof_axis_missing_count": len(missing),
                "blocked_by": missing,
                "selection_eligible": False,
                "reason_token": "RemainingIdScalarOwnerMissingRefinedProofAxes"
                if missing
                else "RemainingOwnerRequiresPostAdoptionSelectionBasis",
            }
        )

    eligible = [row for row in remaining if row["selection_eligible"]]

    return {
        "schema_version": 0,
        "kind": "MirBuilderCrateWideNativeOwnerSeedCapabilitySurveyRerunV10",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "emission_ssa_phi_adoption_decision": rel(ADOPTION),
            "id_scalar_derivable_owner_discriminator_resolution_002": rel(DISCRIMINATOR),
            "id_scalar_seed_readiness_resolution_003": rel(SEED_READINESS),
        },
        "provenance": {
            "emission_ssa_phi_adoption_decision_hash": sha256_file(ADOPTION),
            "id_scalar_derivable_owner_discriminator_resolution_002_hash": sha256_file(DISCRIMINATOR),
            "id_scalar_seed_readiness_resolution_003_hash": sha256_file(SEED_READINESS),
        },
        "adoption_delta": {
            "owner_edge_id": adopted_owner,
            "decision": (adoption.get("decision") or {}).get("value"),
            "hako_adopted": 1 if adopted else 0,
            "source_selfhost_claim": (adoption.get("claims") or {}).get("source_selfhost_claim"),
        },
        "previous_seed_readiness": {
            "decision": (readiness.get("decision") or {}).get("kind"),
            "selected_owner_edge_id": (readiness.get("decision") or {}).get("selected_owner_edge_id"),
            "selected_next_card": (readiness.get("decision") or {}).get("selected_next_card"),
            "seed_materialization_ready_count": (readiness.get("candidate_pool") or {}).get(
                "seed_materialization_ready_count"
            ),
        },
        "remaining_candidates": remaining,
        "candidate_pool": {
            "input_derivable_owner_count": len(candidates),
            "adopted_owner_excluded_count": 1 if adopted_owner and adopted else 0,
            "remaining_owner_count": len(remaining),
            "remaining_refined_proof_complete_count": 0,
            "selection_eligible_count": len(eligible),
            "native_seed_candidate_count": 0,
        },
        "selection_rule": {
            "exclude_already_hako_adopted_owner": True,
            "remaining_owner_requires_complete_refined_proof_axes": True,
            "manual_owner_selection": False,
            "owner_name_as_proof": False,
            "row_count_as_proof": False,
            "surface_count_as_proof": False,
            "source_selfhost_claim_allowed": False,
        },
        "decision": {
            "kind": "KeepStopped",
            "selected_owner_edge_id": None,
            "selected_next_card": DESIGN_STOP,
            "reason_token": "NoRemainingIdScalarOwnerWithCompleteRefinedProofAxesAfterEmissionSsaPhiAdoption",
        },
        "claims": {
            "emission_ssa_phi_adoption_consumed": 1,
            "id_scalar_discriminator_resolution_002_consumed": 1,
            "id_scalar_seed_readiness_resolution_003_consumed": 1,
            "manual_owner_selection": 0,
            "owner_name_as_proof": 0,
            "row_count_as_proof": 0,
            "surface_count_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "generated_artifact_as_native_edit_authority": 0,
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
        print("mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-010 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
