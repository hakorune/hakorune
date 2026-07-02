#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-emission-ssa-phi-id-scalar-verifier-result-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_emission_ssa_phi_id_scalar_verifier_result.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2049-MIRBUILDER-EMISSION_SSA_PHI-ID-SCALAR-VERIFIER-RESULT-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

token = "MIRBUILDER-EMISSION_SSA_PHI-ID-SCALAR-VERIFIER-RESULT-001"
next_card = "MIRBUILDER-EMISSION_SSA_PHI-ID-SCALAR-DERIVED-ARTIFACT-SEED-DRAFT-001"

need(fixture.get("kind") == "MirBuilderEmissionSsaPhiIdScalarVerifierResultV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
pool = fixture.get("candidate_pool") or {}
need(pool.get("failed_check_count") == 0, "verifier failure count drift")
need(pool.get("passed_check_count") == pool.get("check_count"), "not all checks passed")
need((fixture.get("verification_subject") or {}).get("result_kind") == "VerifiedSourcePlanAndRecipe", "bad result kind")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "VerifierResultFixtureMaterialized", "bad decision kind")
need(decision.get("reason_token") == "EmissionSsaPhiIdScalarSourcePlanAndRecipeVerified", "bad reason")
need(decision.get("selected_next_card") == next_card, "bad next card")

claims = fixture.get("claims") or {}
need(claims.get("verifier_result_materialization") == 1, "verifier result not materialized")
need(claims.get("verified_source_plan_and_recipe") == 1, "source plan not verified")
for key in [
    "derived_artifact_seed_draft_materialization",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
for needle in [
    token,
    "reason_token = EmissionSsaPhiIdScalarSourcePlanAndRecipeVerified",
    "selected_next_card = MIRBUILDER-EMISSION_SSA_PHI-ID-SCALAR-DERIVED-ARTIFACT-SEED-DRAFT-001",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-emission-ssa-phi-id-scalar-verifier-result")
print("result_kind=VerifiedSourcePlanAndRecipe")
print("reason_token=EmissionSsaPhiIdScalarSourcePlanAndRecipeVerified")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
