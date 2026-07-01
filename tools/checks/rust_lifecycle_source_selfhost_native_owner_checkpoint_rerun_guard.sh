#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-native-owner-checkpoint-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/source_selfhost_native_owner_checkpoint_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2007-SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-RERUN-001.md"
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


token = "SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-RERUN-001"
next_card = "MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V3"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "SourceSelfhostNativeOwnerCheckpointRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("seed_capability_rerun_008", "").endswith("mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-008-v0.json"), "seed rerun input drift")

seed_state = fixture.get("seed_rerun_state") or {}
need(seed_state.get("kind") == "SelectNativeOwnerCheckpointRerun", "seed rerun decision drift")

native_map = fixture.get("native_owner_map") or {}
need(native_map.get("native_owner_count") == 11, "native owner count drift")
for owner in native_map.get("owners") or []:
    need(owner.get("source_selfhost_claim") == 0, "native owner must not claim Source Selfhost")

blockers = fixture.get("blocker_class_evidence") or {}
missing = blockers.get("MissingProjectionPolicy") or {}
borrow = blockers.get("BorrowSurfaceNeedsPolicy") or {}
route = blockers.get("RouteRepairNeeded") or {}
need(missing.get("candidate_count") == 1004, "missing count drift")
need(missing.get("evidence_quality_count") == 819, "missing evidence quality drift")
need(missing.get("selection_eligible") is True, "missing projection must be eligible")
need(missing.get("next_card") == next_card, "missing projection next drift")
need(borrow.get("candidate_count") == 112, "borrow count drift")
need(borrow.get("evidence_quality_count") == 0, "borrow evidence drift")
need(borrow.get("selection_eligible") is False, "borrow must not be selected")
need(route.get("candidate_count") == 0, "route repair count drift")
need(route.get("selection_eligible") is False, "route repair must not be selected")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectMissingProjectionPolicyClusterResolutionV3", "decision kind drift")
need(decision.get("reason_token") == "MissingProjectionPolicyEvidenceQualityWinsAfterCoverageReclassification", "reason drift")
need(decision.get("selected_blocker_class") == "MissingProjectionPolicy", "selected blocker drift")
need(decision.get("selected_next_card") == next_card, "decision next drift")

claims = fixture.get("claims") or {}
need(claims.get("native_owner_checkpoint_rerun") == 1, "checkpoint rerun claim drift")
for key in [
    "source_selfhost_claim",
    "rust_deletion",
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "candidate_count_as_proof",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "generated_artifact_as_native_edit_authority",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_canonical_mir_instruction",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")
for needle in [
    token,
    next_card,
    "native_owner_count = 11",
    "missing_projection_evidence_quality_count = 819",
    "borrow_surface_evidence_quality_count = 0",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-source-selfhost-native-owner-checkpoint-rerun")
print("native_owner_count=11")
print("selected_blocker_class=MissingProjectionPolicy")
print("missing_projection_evidence_quality_count=819")
print("borrow_surface_evidence_quality_count=0")
print(f"selected_next_card={next_card}")
print("source_selfhost_claim=0")
print("summary=ok")
PY
