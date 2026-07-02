#!/usr/bin/env python3
"""Materialize the emission_ssa_phi ID scalar DerivedArtifactSeedDraftInput."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-emission-ssa-phi-id-scalar-derived-artifact-seed-draft-v0.json"

TOKEN = "MIRBUILDER-EMISSION_SSA_PHI-ID-SCALAR-DERIVED-ARTIFACT-SEED-DRAFT-001"
NEXT_CARD = "MIRBUILDER-ID-SCALAR-DOMAIN-SEED-READINESS-RESOLUTION-003"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

VERIFIER_RESULT = FIXTURES / "mirbuilder-emission-ssa-phi-id-scalar-verifier-result-v0.json"
SOURCE_PLAN = FIXTURES / "mirbuilder-emission-ssa-phi-id-scalar-source-plan-and-recipe-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict:
    verifier = read_json(VERIFIER_RESULT)
    source = read_json(SOURCE_PLAN)
    verified = (verifier.get("claims") or {}).get("verified_source_plan_and_recipe") == 1
    return {
        "schema_version": 0,
        "kind": "MirBuilderEmissionSsaPhiIdScalarDerivedArtifactSeedDraftV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "verifier_result": rel(VERIFIER_RESULT),
            "source_plan_and_recipe": rel(SOURCE_PLAN),
        },
        "provenance": {
            "verifier_result_hash": sha256_file(VERIFIER_RESULT),
            "source_plan_and_recipe_hash": sha256_file(SOURCE_PLAN),
        },
        "seed_draft_input": {
            "state": "DerivedArtifactSeedDraftInput",
            "owner_edge_id": (source.get("selected_owner") or {}).get("owner_edge_id"),
            "source_plan_id": (source.get("source_plan") or {}).get("plan_id"),
            "behavior_recipe_id": (source.get("behavior_recipe") or {}).get("recipe_id"),
            "verifier_result_kind": (verifier.get("verification_subject") or {}).get("result_kind"),
            "generated_artifact_as_native_edit_authority": False,
            "native_source_seed": False,
            "hako_adopted": False,
        },
        "decision": {
            "kind": "DerivedArtifactSeedDraftInputMaterialized" if verified else "KeepStopped",
            "reason_token": "EmissionSsaPhiIdScalarDerivedArtifactSeedDraftInputMaterialized"
            if verified
            else "EmissionSsaPhiIdScalarVerifierResultRequiredForSeedDraft",
            "selected_next_card": NEXT_CARD if verified else DESIGN_STOP,
        },
        "claims": {
            "derived_artifact_seed_draft_materialization": 1 if verified else 0,
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
        print("mirbuilder-emission-ssa-phi-id-scalar-derived-artifact-seed-draft unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
