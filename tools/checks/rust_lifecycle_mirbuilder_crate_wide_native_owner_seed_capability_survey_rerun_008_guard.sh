#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-008-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_crate_wide_native_owner_seed_capability_survey_rerun_008.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2006-MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-008.md"
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


token = "MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-008"
next_card = "SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-RERUN-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
reason = "NoSeedCandidateAfterCoverageReclassificationNeedsCheckpointRerun"

need(fixture.get("kind") == "MirBuilderCrateWideNativeOwnerSeedCapabilitySurveyRerunV8", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

report = fixture.get("reclassified_report_state") or {}
need(report.get("decision") == "KeepStopped", "report decision drift")
need(report.get("scanned_surface_count") == 1584, "surface count drift")
need(report.get("projection_descriptor_coverage_reclassified_count") == 380, "coverage count drift")
need(report.get("missing_projection_policy_count") == 1004, "missing projection count drift")
need(report.get("mapped_to_known_owner_count") == 398, "mapped owner count drift")
need(report.get("borrow_policy_needed_count") == 112, "borrow policy count drift")

pool = fixture.get("candidate_pool") or {}
need(pool.get("verified_hako_family_ir_count") == 47, "verified count drift")
need(pool.get("bridge_eligible_count") == 0, "eligible count drift")
need(pool.get("already_adopted_count") == 15, "already adopted count drift")
need(pool.get("gap_blocked_count") == 36, "gap blocked count drift")

selected = fixture.get("selected_candidate") or {}
need(selected.get("owner_edge_id") is None, "selected owner must be null")
need(selected.get("selected_next_card") == design_stop, "raw selection should remain design stop")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectNativeOwnerCheckpointRerun", "bad decision kind")
need(decision.get("selected_owner_edge_id") is None, "bad decision owner")
need(decision.get("selected_next_card") == next_card, "bad next card")
need(decision.get("reason_token") == reason, "bad reason")

claims = fixture.get("claims") or {}
for key in [
    "reclassified_unconverted_surface_report_consumed",
    "projection_descriptor_coverage_reclassification_consumed",
    "bridge_policy_consumed",
    "strict_converter_emission_probe_consumed",
]:
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
for needle in [
    token,
    next_card,
    "bridge_eligible_count = 0",
    "already_adopted_count = 15",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-008")
print("projection_descriptor_coverage_reclassified_count=380")
print("bridge_eligible_count=0")
print("already_adopted_count=15")
print(f"selected_next_card={next_card}")
print("manual_family_selection=0")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("summary=ok")
PY
