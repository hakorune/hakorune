#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-007-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_crate_wide_native_owner_seed_capability_survey_rerun_007.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/1977-MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-007.md"
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

token = "MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-007"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
reason = "NoBridgeEligibleStrictEmissionNativeSeedCandidateAfterFreshUnconvertedSurfaceReport"

need(fixture.get("kind") == "MirBuilderCrateWideNativeOwnerSeedCapabilitySurveyRerunV7", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

fresh = fixture.get("fresh_report_state") or {}
need(fresh.get("decision") == "KeepStopped", "fresh report decision drift")
need(fresh.get("scanned_surface_count") == 1584, "fresh report scanned count drift")
need(fresh.get("missing_projection_policy_count") == 1384, "fresh report missing projection count drift")
need(fresh.get("projection_descriptor_ledger_hash") == fresh.get("source_selfhost_family_guard_manifest_hash"), "report ledger hash boundary drift")

pool = fixture.get("candidate_pool") or {}
need(pool.get("verified_hako_family_ir_count") == 47, "verified count drift")
need(pool.get("bridge_eligible_count") == 0, "eligible count drift")
need(pool.get("already_adopted_count") == 12, "already adopted count drift")

selected = fixture.get("selected_candidate") or {}
need(selected.get("owner_edge_id") is None, "selected owner must be null")
need(selected.get("selected_next_card") == design_stop, "bad selected next")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "bad decision kind")
need(decision.get("selected_owner_edge_id") is None, "bad decision owner")
need(decision.get("selected_next_card") == design_stop, "bad decision next")
need(decision.get("reason_token") == reason, "bad reason")

claims = fixture.get("claims") or {}
for key in ["fresh_unconverted_surface_report_consumed", "bridge_policy_consumed", "strict_converter_emission_probe_consumed"]:
    need(claims.get(key) == 1, f"{key} must be 1")
for key in [
    "manual_family_selection",
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
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
]:
    need(claims.get(key) == 0, f"{key} must be 0")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")
need(reason in task_order, "task-order missing reason")
need("design consultation stop reached" in task_order, "task-order missing stop marker")

print("output_contract=rust-lifecycle-mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-007")
print("fresh_unconverted_surface_report_consumed=1")
print("bridge_eligible_count=0")
print("decision=KeepStopped")
print(f"selected_next_card={design_stop}")
print("manual_family_selection=0")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("summary=ok")
PY
