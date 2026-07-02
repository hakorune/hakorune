#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-emission-ssa-phi-hako-adoption-decision-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_emission_ssa_phi_hako_adoption_decision.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2053-MIRBUILDER-EMISSION_SSA_PHI-HAKO-ADOPTION-DECISION-001.md"
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

token = "MIRBUILDER-EMISSION_SSA_PHI-HAKO-ADOPTION-DECISION-001"
next_card = "MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-010"

need(fixture.get("kind") == "MirBuilderEmissionSsaPhiHakoAdoptionDecisionV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(fixture.get("family_id") == "mirbuilder::emission_ssa_phi", "bad family")
decision = fixture.get("decision") or {}
need(decision.get("value") == "Adopt", "decision not adopt")
need(decision.get("reason_token") == "EmissionSsaPhiNativeSeedPresentAndSourcePlanVerified", "bad reason")
need((fixture.get("next_action") or {}).get("next_card") == next_card, "bad next card")

claims = fixture.get("claims") or {}
for key in ["hako_adopted", "native_hako_source_owner_present", "rust_bootstrap_retained", "rust_oracle_retained"]:
    need(claims.get(key) == 1, f"required claim missing: {key}")
for key in [
    "generated_artifact_as_edit_authority",
    "manual_family_selection",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "source_selfhost_claim",
    "rust_deletion",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_canonical_mir_instruction",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
for needle in [
    token,
    "reason_token = EmissionSsaPhiNativeSeedPresentAndSourcePlanVerified",
    "selected_next_card = MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-010",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-emission-ssa-phi-hako-adoption-decision")
print("decision=Adopt")
print("reason_token=EmissionSsaPhiNativeSeedPresentAndSourcePlanVerified")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
