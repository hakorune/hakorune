#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-010-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_crate_wide_native_owner_seed_capability_survey_rerun_010.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2054-MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-010.md"
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


token = "MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-010"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
reason = "NoRemainingIdScalarOwnerWithCompleteRefinedProofAxesAfterEmissionSsaPhiAdoption"

need(fixture.get("kind") == "MirBuilderCrateWideNativeOwnerSeedCapabilitySurveyRerunV10", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
for key in [
    "emission_ssa_phi_adoption_decision",
    "id_scalar_derivable_owner_discriminator_resolution_002",
    "id_scalar_seed_readiness_resolution_003",
]:
    need(inputs.get(key, "").endswith(".json"), f"input missing: {key}")

adoption = fixture.get("adoption_delta") or {}
need(adoption.get("owner_edge_id") == "mirbuilder::emission_ssa_phi", "adopted owner drift")
need(adoption.get("decision") == "Adopt", "adoption decision drift")
need(adoption.get("hako_adopted") == 1, "hako adopted drift")
need(adoption.get("source_selfhost_claim") == 0, "source selfhost claim drift")

pool = fixture.get("candidate_pool") or {}
need(pool.get("input_derivable_owner_count") == 2, "input derivable owner count drift")
need(pool.get("adopted_owner_excluded_count") == 1, "adopted exclusion count drift")
need(pool.get("remaining_owner_count") == 1, "remaining owner count drift")
need(pool.get("remaining_refined_proof_complete_count") == 0, "remaining proof complete drift")
need(pool.get("selection_eligible_count") == 0, "selection eligible drift")
need(pool.get("native_seed_candidate_count") == 0, "seed candidate drift")

remaining = fixture.get("remaining_candidates") or []
need(len(remaining) == 1, "remaining candidate count drift")
candidate = remaining[0]
need(candidate.get("owner_edge_id") == "mirbuilder::context_registry", "remaining owner drift")
need(candidate.get("selection_eligible") is False, "remaining candidate must not be eligible")
need(candidate.get("refined_proof_axis_missing_count") == 2, "missing axis count drift")
need("StandaloneProjectionSubjectEstablished" in candidate.get("blocked_by", []), "missing standalone blocker")
need("LifecycleContractDescriptorCompleteness" in candidate.get("blocked_by", []), "missing lifecycle blocker")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == reason, "reason drift")
need(decision.get("selected_next_card") == design_stop, "next card drift")
need(decision.get("selected_owner_edge_id") is None, "selected owner drift")

claims = fixture.get("claims") or {}
for key in [
    "emission_ssa_phi_adoption_consumed",
    "id_scalar_discriminator_resolution_002_consumed",
    "id_scalar_seed_readiness_resolution_003_consumed",
]:
    need(claims.get(key) == 1, f"required claim missing: {key}")
for key in [
    "manual_owner_selection",
    "owner_name_as_proof",
    "row_count_as_proof",
    "surface_count_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "generated_artifact_as_native_edit_authority",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")
for needle in [
    token,
    "reason_token = " + reason,
    "selected_next_card = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-native-owner-seed-capability-survey-rerun-010")
print("decision=KeepStopped")
print("reason_token=" + reason)
print("selected_next_card=" + design_stop)
print("source_selfhost_claim=0")
print("summary=ok")
PY
