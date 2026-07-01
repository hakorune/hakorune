#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-source-selfhost-wider-route-selection-basis-003"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/2001-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-003.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-wider-route-selection-basis-003-v0.json"
RERUN="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun-005-v0.json"
REPORT="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json"
MANIFEST="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TOOL="tools/rust_lifecycle/source_selfhost_wider_route_selection_basis_003.py"

guard_require_files "$TAG" "$CARD" "$FIXTURE" "$RERUN" "$REPORT" "$MANIFEST" "$TASK_ORDER" "$STATE" "$TOOL"

python3 "$TOOL" --check

python3 - "$CARD" "$FIXTURE" "$RERUN" "$REPORT" "$MANIFEST" "$TASK_ORDER" "$STATE" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
rerun_path = Path(sys.argv[3])
report_path = Path(sys.argv[4])
manifest_path = Path(sys.argv[5])
task_order_path = Path(sys.argv[6])
state_path = Path(sys.argv[7])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
rerun = json.loads(rerun_path.read_text(encoding="utf-8"))
report = json.loads(report_path.read_text(encoding="utf-8"))
manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
state = tomllib.loads(state_path.read_text(encoding="utf-8"))


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


token = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-003"
contract = "rust-lifecycle-source-selfhost-wider-route-selection-basis-003-v0"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
next_card = "MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-003"
reason = "SourceSurfaceReportStaleAfterNativeOwnerAdoption"
rerun_reason = "NoBridgeEligibleCandidateAfterTypedObjectPlanAdoption"

require(token in card, "card token drift")
for needle in [
    contract,
    "PostBridgePolicyV2ExhaustionLaneSelector",
    "bridge_eligible_remaining_count = 0",
    "already_hako_adopted_count = 3",
    "native_owner_adoption_delta_count = 3",
    next_card,
    "manual_family_selection = 0",
    "source_selfhost_claim = 0",
]:
    require(needle in card, f"card missing {needle}")

require(fixture.get("kind") == "SourceSelfhostWiderRouteSelectionBasisV3", "fixture kind drift")
require(fixture.get("token") == token, "fixture token drift")
require(fixture.get("output_contract") == contract, "fixture contract drift")

rerun_pool = rerun.get("candidate_pool") or {}
rerun_decision = rerun.get("decision") or {}
exhaustion = fixture.get("bridge_policy_v2_exhaustion") or {}
require(rerun_pool.get("input_owner_edge_count") == 3, "rerun input owner count drift")
require(rerun_pool.get("already_hako_adopted_count") == 3, "rerun adopted count drift")
require(rerun_pool.get("bridge_eligible_remaining_count") == 0, "rerun bridge remaining drift")
require(rerun_pool.get("selected_candidate_count") == 0, "rerun selected count drift")
require(rerun_decision.get("kind") == "KeepStopped", "rerun decision drift")
require(rerun_decision.get("reason_token") == rerun_reason, "rerun reason drift")
require(rerun_decision.get("selected_next_card") == design_stop, "rerun next drift")
require(exhaustion.get("already_hako_adopted_count") == 3, "fixture adopted count drift")
require(exhaustion.get("bridge_eligible_remaining_count") == 0, "fixture bridge remaining drift")
require(exhaustion.get("selected_candidate_count") == 0, "fixture selected count drift")
require(exhaustion.get("reason_token") == rerun_reason, "fixture rerun reason drift")

freshness = fixture.get("freshness") or {}
delta = freshness.get("latest_native_owner_delta_tokens") or []
require(freshness.get("unconverted_surface_report_fresh") is False, "report freshness must be stale")
require(freshness.get("native_owner_adoption_delta_count") == 3, "adoption delta count drift")
require(delta == [
    "MIRBUILDER-DIRECT-STATE-PLAN-REFRESH-HAKO-ADOPTION-DECISION-001",
    "MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-HAKO-ADOPTION-DECISION-001",
    "MIRBUILDER-TYPED-OBJECT-PLAN-REFRESH-HAKO-ADOPTION-DECISION-001",
], "adoption delta tokens drift")
require(freshness.get("unconverted_surface_report_token") == report.get("token"), "report token drift")

lanes = fixture.get("candidate_lanes") or []
eligible = [lane for lane in lanes if lane.get("selection_eligible") is True]
require(len(eligible) == 1, "exactly one lane must be selected")
require(eligible[0].get("lane") == "UnconvertedSurfaceReportRerun", "selected lane drift")
require(eligible[0].get("next_card") == next_card, "selected lane next drift")

selection_rule = fixture.get("selection_rule") or {}
for key in [
    "consume_rerun_005",
    "bridge_policy_v2_remaining_candidates_must_be_zero",
    "report_freshness_precedes_checkpoint",
    "native_owner_checkpoint_precedes_blocker_class_selection",
    "exactly_one_lane_or_keep_stopped",
]:
    require(selection_rule.get(key) is True, f"selection rule drift: {key}")
for key in ["cluster_size_as_proof", "coverage_percentage_as_proof", "manual_lane_selection"]:
    require(selection_rule.get(key) is False, f"selection rule forbidden drift: {key}")

decision = fixture.get("decision") or {}
require(decision.get("kind") == "SelectUnconvertedSurfaceReportRerun", "decision kind drift")
require(decision.get("reason_token") == reason, "decision reason drift")
require(decision.get("selected_next_card") == next_card, "decision next drift")
require(decision.get("selected_owner_edge_id") is None, "selector must not choose owner")

claims = fixture.get("claims") or {}
for key in [
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "cluster_size_as_proof",
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
    "new_canonical_mir_instruction",
    "new_python_semantic_projector",
    "runner_semantic_owner",
    "family_name_based_policy",
]:
    require(claims.get(key) == 0, f"forbidden claim drift: {key}")

manifest_tokens = [row.get("token") for row in manifest.get("rows", [])]
require(token in manifest_tokens, "family guard manifest missing BASIS-003")

require(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
require(state.get("latest_card_path") == str(card_path), "CURRENT_STATE latest path drift")
require(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

for needle in [
    token,
    contract,
    "post_bridge_policy_v2_basis_003_result",
    next_card,
    reason,
]:
    require(needle in task_order, f"task-order missing {needle}")

print(f"output_contract={contract}")
print(f"current_blocker_preserved={design_stop}")
print("decision=SelectUnconvertedSurfaceReportRerun")
print(f"reason_token={reason}")
print(f"selected_next_card={next_card}")
print("native_owner_adoption_delta_count=3")
print("manual_family_selection=0")
print("manual_shape_selection=0")
print("manual_axis_selection=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
