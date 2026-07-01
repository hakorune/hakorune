#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-source-selfhost-native-owner-checkpoint"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/2003-SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-native-owner-checkpoint-v0.json"
REPORT="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json"
MANIFEST="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TOOL="tools/rust_lifecycle/source_selfhost_native_owner_checkpoint.py"

guard_require_files "$TAG" "$CARD" "$FIXTURE" "$REPORT" "$MANIFEST" "$TASK_ORDER" "$STATE" "$TOOL"

python3 "$TOOL" --check

python3 - "$CARD" "$FIXTURE" "$REPORT" "$TASK_ORDER" "$STATE" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
report_path = Path(sys.argv[3])
task_order_path = Path(sys.argv[4])
state_path = Path(sys.argv[5])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
report = json.loads(report_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
state = tomllib.loads(state_path.read_text(encoding="utf-8"))


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


token = "SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-001"
next_card = "MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V2"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

require(token in card, "card missing token")
for needle in [
    "native_owner_count = 11",
    "missing_projection_policy_count = 1384",
    "missing_projection_evidence_quality_count = 1199",
    "borrow_surface_evidence_quality_count = 0",
    next_card,
    "source_selfhost_claim = 0",
]:
    require(needle in card, f"card missing {needle}")

require(fixture.get("kind") == "SourceSelfhostNativeOwnerCheckpointV1", "fixture kind drift")
require(fixture.get("token") == token, "fixture token drift")
require((fixture.get("input_state") or {}).get("current_blocker") == design_stop, "blocker drift")

native_map = fixture.get("native_owner_map") or {}
require(native_map.get("native_owner_count") == 11, "native owner count drift")
for owner in native_map.get("owners") or []:
    require(owner.get("source_selfhost_claim") == 0, "native owner row must not claim Source Selfhost")

report_summary = report.get("summary") or {}
blockers = fixture.get("blocker_class_evidence") or {}
missing = blockers.get("MissingProjectionPolicy") or {}
borrow = blockers.get("BorrowSurfaceNeedsPolicy") or {}
route = blockers.get("RouteRepairNeeded") or {}
require(missing.get("candidate_count") == report_summary.get("missing_projection_policy_count") == 1384, "missing projection count drift")
require(missing.get("evidence_quality_count") == 1199, "missing projection evidence quality drift")
require(missing.get("selection_eligible") is True, "missing projection must be eligible")
require(borrow.get("candidate_count") == report_summary.get("borrow_policy_needed_count") == 112, "borrow count drift")
require(borrow.get("evidence_quality_count") == 0, "borrow evidence quality drift")
require(borrow.get("selection_eligible") is False, "borrow lane must not be selected")
require(route.get("candidate_count") == 0, "route repair count drift")
require(route.get("selection_eligible") is False, "route repair must not be selected")

selection_rule = fixture.get("selection_rule") or {}
for key in [
    "route_repair_precedes_policy_lanes",
    "fresh_report_required",
    "evidence_quality_precedes_candidate_count",
    "missing_projection_requires_fixture_mapped_known_shape",
    "borrow_surface_requires_owner_confidence",
]:
    require(selection_rule.get(key) is True, f"selection rule drift: {key}")
for key in ["source_selfhost_claim_allowed", "manual_blocker_class_selection"]:
    require(selection_rule.get(key) is False, f"forbidden selection rule drift: {key}")

decision = fixture.get("decision") or {}
require(decision.get("kind") == "SelectMissingProjectionPolicyClusterResolutionV2", "decision kind drift")
require(decision.get("reason_token") == "MissingProjectionPolicyEvidenceQualityWinsCheckpoint", "decision reason drift")
require(decision.get("selected_blocker_class") == "MissingProjectionPolicy", "selected blocker drift")
require(decision.get("selected_next_card") == next_card, "decision next drift")

claims = fixture.get("claims") or {}
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
    require(claims.get(key) == 0, f"forbidden claim drift: {key}")

require(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
require(state.get("latest_card_path") == str(card_path), "CURRENT_STATE latest path drift")
require(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

for needle in [
    token,
    next_card,
    "native_owner_count = 11",
    "missing_projection_evidence_quality_count = 1199",
]:
    require(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-source-selfhost-native-owner-checkpoint-v0")
print("native_owner_count=11")
print("selected_blocker_class=MissingProjectionPolicy")
print("missing_projection_evidence_quality_count=1199")
print("borrow_surface_evidence_quality_count=0")
print(f"selected_next_card={next_card}")
print("source_selfhost_claim=0")
print("summary=ok")
PY
