#!/usr/bin/env python3
"""Decide HakoAdopted for the emission_ssa_phi native source seed."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-emission-ssa-phi-hako-adoption-decision-v0.json"

TOKEN = "MIRBUILDER-EMISSION_SSA_PHI-HAKO-ADOPTION-DECISION-001"
NEXT_CARD = "MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-010"
OWNER = "mirbuilder::emission_ssa_phi"

NATIVE_SEED = FIXTURES / "mirbuilder-emission-ssa-phi-hako-native-source-seed-v0.json"
VERIFIER_RESULT = FIXTURES / "mirbuilder-emission-ssa-phi-id-scalar-verifier-result-v0.json"
NATIVE_SOURCE = ROOT / "lang/src/compiler/lib/mirbuilder/emission_ssa_phi_native_seed.hako"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict:
    seed = read_json(NATIVE_SEED)
    verifier = read_json(VERIFIER_RESULT)
    seed_present = (seed.get("claims") or {}).get("native_seed_materialization") == 1
    verified = (verifier.get("claims") or {}).get("verified_source_plan_and_recipe") == 1
    adopt = seed_present and verified and NATIVE_SOURCE.exists()
    return {
        "schema_version": 0,
        "kind": "MirBuilderEmissionSsaPhiHakoAdoptionDecisionV1",
        "token": TOKEN,
        "family_id": OWNER,
        "input_authority": {
            "native_source_seed": rel(NATIVE_SEED),
            "verifier_result": rel(VERIFIER_RESULT),
            "native_source_path": rel(NATIVE_SOURCE),
        },
        "provenance": {
            "native_source_seed_hash": sha256_file(NATIVE_SEED),
            "verifier_result_hash": sha256_file(VERIFIER_RESULT),
            "native_source_hash": sha256_file(NATIVE_SOURCE),
        },
        "target": {
            "family_scope": "LeafSemanticOwner",
            "native_source_owner_present": 1 if seed_present else 0,
            "verified_source_plan_and_recipe": 1 if verified else 0,
            "generated_artifact_as_edit_authority": 0,
        },
        "decision": {
            "value": "Adopt" if adopt else "Defer",
            "reason_token": "EmissionSsaPhiNativeSeedPresentAndSourcePlanVerified"
            if adopt
            else "EmissionSsaPhiAdoptionRequiresNativeSeedAndVerifier",
            "selected_next_route": "native_hako_source_owner" if adopt else "defer",
        },
        "post_decision_state": {
            "hako_adopted": 1 if adopt else 0,
            "native_edit_authority": 1 if adopt else 0,
            "rust_role": "bootstrap_oracle_compat",
            "generated_artifact_role": "provenance_and_regeneration_reference",
        },
        "next_action": {
            "kind": "NativeOwnerSeedCapabilitySurveyRerun",
            "next_card": NEXT_CARD,
            "reason_token": "EmissionSsaPhiAdoptedSourceSelfhostStillStopped",
        },
        "claims": {
            "hako_adopted": 1 if adopt else 0,
            "native_hako_source_owner_present": 1 if adopt else 0,
            "rust_bootstrap_retained": 1,
            "rust_oracle_retained": 1,
            "generated_artifact_as_edit_authority": 0,
            "manual_family_selection": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "source_selfhost_claim": 0,
            "rust_deletion": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_canonical_mir_instruction": 0,
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
        print("mirbuilder-emission-ssa-phi-hako-adoption-decision unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
