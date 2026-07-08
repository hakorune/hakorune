#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-source-selfhost-post-rerun004-current-reentry-inventory"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/3340-SOURCE-SELFHOST-POST-RERUN004-CURRENT-REENTRY-INVENTORY-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-post-rerun004-current-reentry-inventory-v0.json"
BASIS_007="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-wider-route-selection-basis-007-v0.json"
RERUN_004_CARD="docs/development/current/main/phases/phase-296x/2057-MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-004.md"
CHECKPOINT_002="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-native-owner-checkpoint-rerun-002-v0.json"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$CARD" "$FIXTURE" "$BASIS_007" "$RERUN_004_CARD" "$CHECKPOINT_002" "$TASK_ORDER" "$STATE"

python3 - "$CARD" "$FIXTURE" "$BASIS_007" "$RERUN_004_CARD" "$CHECKPOINT_002" "$TASK_ORDER" "$STATE" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
basis_path = Path(sys.argv[3])
rerun_card_path = Path(sys.argv[4])
checkpoint_path = Path(sys.argv[5])
task_order_path = Path(sys.argv[6])
state_path = Path(sys.argv[7])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
basis = json.loads(basis_path.read_text(encoding="utf-8"))
rerun_card = rerun_card_path.read_text(encoding="utf-8")
checkpoint = json.loads(checkpoint_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
state = tomllib.loads(state_path.read_text(encoding="utf-8"))


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


token = "SOURCE-SELFHOST-POST-RERUN004-CURRENT-REENTRY-INVENTORY-001"
contract = "rust-lifecycle-source-selfhost-post-rerun004-current-reentry-inventory-v0"
prereq = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-ROUTE-ADJACENT-SHADOW-GUARD-REFRESH-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
basis_token = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-007"
rerun_token = "MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-004"
checkpoint_token = "SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-RERUN-002"
next_after_checkpoint = "MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V4"

require(token in card, "card missing token")
for needle in [
    "basis_007_valid_local_mechanical_selector = 1",
    "basis_007_consumed_by_rerun_004 = 1",
    "rerun_004_report_regenerated = 1",
    "rerun_004_result = KeepStopped",
    checkpoint_token,
    "machine_derived_route_repair_replay = 0",
    "programjson_runtime_route_authority = 0",
    next_after_checkpoint,
]:
    require(needle in card, f"card missing {needle}")

require(fixture.get("kind") == "SourceSelfhostPostRerun004CurrentReentryInventoryV1", "fixture kind drift")
require(fixture.get("token") == token, "fixture token drift")
require(fixture.get("output_contract") == contract, "fixture contract drift")
input_state = fixture.get("input_state") or {}
require(input_state.get("current_blocker") == design_stop, "input blocker drift")
require(input_state.get("current_prerequisite") == prereq, "input prerequisite drift")

basis_decision = basis.get("decision") or {}
basis_local = basis.get("local_authority") or {}
require(basis.get("token") == basis_token, "basis token drift")
require(basis_decision.get("selected_next_card") == rerun_token, "basis selected next drift")
require(basis_decision.get("kind") == "SelectUnconvertedSurfaceReportRerun", "basis decision kind drift")
require(basis_local.get("local_selection_authority") == "LocalMechanicalSelectorAuthorityV1", "basis local authority drift")
require((basis.get("freshness") or {}).get("unconverted_surface_report_fresh_after_emission_ssa_phi_adoption") is False, "basis must remain stale-report selector evidence")

consumed = fixture.get("consumed_selector") or {}
require(consumed.get("token") == basis_token, "consumed selector token drift")
require(consumed.get("status") == "historical_consumed", "consumed selector status drift")
require(consumed.get("selected_next_card") == rerun_token, "consumed selector next drift")
require(consumed.get("basis_007_valid_local_mechanical_selector") == 1, "basis validity claim drift")
require(consumed.get("basis_007_consumed_by_rerun_004") == 1, "basis consumed claim drift")

for needle in [
    "decision = KeepStopped",
    "report_regenerated = 1",
    "recommended_next_task =",
    checkpoint_token,
]:
    require(needle in rerun_card, f"rerun card missing {needle}")

rerun = fixture.get("rerun_004") or {}
require(rerun.get("token") == rerun_token, "rerun token drift")
require(rerun.get("report_regenerated") == 1, "rerun regenerated claim drift")
require(rerun.get("result") == "KeepStopped", "rerun result drift")
require(rerun.get("recommended_next") == checkpoint_token, "rerun recommended next drift")

checkpoint_decision = checkpoint.get("decision") or {}
require(checkpoint.get("token") == checkpoint_token, "checkpoint token drift")
require(checkpoint_decision.get("selected_next_card") == next_after_checkpoint, "checkpoint next drift")
require(fixture.get("next_after_selected") == next_after_checkpoint, "fixture checkpoint successor drift")

selection_rule = fixture.get("selection_rule") or {}
for key in [
    "basis_007_may_be_used_as_historical_evidence",
    "basis_007_may_not_be_replayed_as_current_unblock",
    "current_next_uses_rerun_004_recommended_next",
]:
    require(selection_rule.get(key) is True, f"selection rule drift: {key}")
for key in ["manual_family_selection", "route_membership_alone_as_proof"]:
    require(selection_rule.get(key) is False, f"forbidden selection rule drift: {key}")

decision = fixture.get("decision") or {}
require(decision.get("kind") == "SelectPostRerun004NativeOwnerCheckpointRerun", "decision kind drift")
require(decision.get("reason_token") == "Basis007ConsumedByRerun004UseRecommendedCheckpointRerun", "decision reason drift")
require(decision.get("selected_next_card") == checkpoint_token, "decision next drift")

claims = fixture.get("claims") or {}
for key in [
    "machine_derived_route_repair_replay",
    "manual_family_selection",
    "route_membership_alone_as_proof",
    "coverage_percentage_as_proof",
    "source_selfhost_claim",
    "hako_adopted_decision",
    "native_seed_materialization",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
]:
    require(claims.get(key) == 0, f"forbidden claim drift: {key}")

require(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
require(state.get("latest_card_path") == str(card_path), "CURRENT_STATE card path drift")
require(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")
require("next_documented_task =\n  " + checkpoint_token in task_order, "task-order next documented task drift")
require(checkpoint_token + " -> " + next_after_checkpoint in task_order, "task-order next chain drift")

print(f"output_contract={contract}")
print("basis_007_valid_local_mechanical_selector=1")
print("basis_007_consumed_by_rerun_004=1")
print("rerun_004_report_regenerated=1")
print("rerun_004_result=KeepStopped")
print(f"rerun_004_recommended_next={checkpoint_token}")
print("machine_derived_route_repair_replay=0")
print("manual_family_selection=0")
print("route_membership_alone_as_proof=0")
print("coverage_percentage_as_proof=0")
print("source_selfhost_claim=0")
print("hako_adopted_decision=0")
print("native_seed_materialization=0")
print("route_selection=0")
print("runtime_route_switch=0")
print("programjson_runtime_route_authority=0")
print("runtime_fallback=0")
print("new_backend_route=0")
print("new_abi=0")
print(f"selected_next_card={checkpoint_token}")
print("summary=ok")
PY
