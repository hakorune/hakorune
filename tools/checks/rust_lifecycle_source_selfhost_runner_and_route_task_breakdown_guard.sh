#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-source-selfhost-runner-and-route-task-breakdown"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/1800-SOURCE-SELFHOST-RUNNER-AND-ROUTE-TASK-BREAKDOWN-001.md"
SSOT="docs/development/current/main/design/source-selfhost-runner-and-route-task-breakdown-ssot.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-runner-and-route-task-breakdown-v0.json"
DESIGN_STOP="docs/development/current/main/phases/phase-296x/1799-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001.md"
ARTIFACT_POLICY="docs/development/current/main/design/artifact-policy-ssot.md"
ROLE_SSOT="docs/development/current/main/design/rust-to-hako-converter-implementation-role-ssot.md"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"

guard_require_files "$TAG" \
  "$CARD" \
  "$SSOT" \
  "$FIXTURE" \
  "$DESIGN_STOP" \
  "$ARTIFACT_POLICY" \
  "$ROLE_SSOT" \
  "$TASK_ORDER" \
  "$STATE" \
  "$INDEX"

python3 - "$CARD" "$SSOT" "$FIXTURE" "$DESIGN_STOP" "$ARTIFACT_POLICY" "$ROLE_SSOT" "$TASK_ORDER" "$STATE" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
ssot_path = Path(sys.argv[2])
fixture_path = Path(sys.argv[3])
design_stop_path = Path(sys.argv[4])
artifact_policy_path = Path(sys.argv[5])
role_ssot_path = Path(sys.argv[6])
task_order_path = Path(sys.argv[7])
state_path = Path(sys.argv[8])
index_path = Path(sys.argv[9])

card = card_path.read_text(encoding="utf-8")
ssot = ssot_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
design_stop = design_stop_path.read_text(encoding="utf-8")
artifact_policy = artifact_policy_path.read_text(encoding="utf-8")
role_ssot = role_ssot_path.read_text(encoding="utf-8")
task_order = task_order_path.read_text(encoding="utf-8")
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
index = index_path.read_text(encoding="utf-8")

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "SOURCE-SELFHOST-RUNNER-AND-ROUTE-TASK-BREAKDOWN-001"
contract = "rust-lifecycle-source-selfhost-runner-and-route-task-breakdown-v0"
design_stop_token = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

require(f"# {token}" in card, "card token drift")
require(contract in card, "card output contract drift")
require(design_stop_token in card, "card must preserve current design stop")
require("runner_semantic_owner=0" in card, "card missing runner semantic-owner denial")
require("future_interpreter_required_for_projector_migration=0" in card, "card missing future interpreter denial")

require("# Source Selfhost Runner and Route Task Breakdown" in ssot, "SSOT title drift")
for needle in [
    "current vm-hako",
    "future interpreter",
    "EXE/AOT",
    "one .hako source/projector",
    "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-001",
    "<ROUTE-FAMILY>-ROUTE-MATRIX-REPAIR-001",
    "<SELECTED-FAMILY>-HAKO-ADOPTION-DECISION-001",
]:
    require(needle in ssot, f"SSOT missing {needle}")

require(fixture.get("kind") == "SourceSelfhostRunnerAndRouteTaskBreakdownV1", "fixture kind drift")
require(fixture.get("output_contract") == contract, "fixture contract drift")
require(fixture.get("current_blocker") == design_stop_token, "fixture current blocker drift")

candidate_pool = fixture.get("candidate_pool") or {}
require(candidate_pool.get("state") == "Blocked", "candidate pool state drift")
require(candidate_pool.get("reason_token") == "NoEligibleNativeAdoptionCandidate", "candidate pool reason drift")
require(candidate_pool.get("allowed_resume") == [
    "ConsultationGatedWiderRouteSelection",
    "MachineDerivedRouteRepair",
], "allowed resume drift")

runner_roles = fixture.get("runner_roles") or {}
require((runner_roles.get("shadow_reference_runner") or {}).get("may_own_semantics") is False, "shadow runner semantic-owner drift")
require((runner_roles.get("exe_aot") or {}).get("may_own_semantics") is False, "EXE/AOT semantic-owner drift")
require((runner_roles.get("current_vm_hako") or {}).get("co_mainline_product_lane") is False, "vm-hako co-mainline drift")
future = runner_roles.get("future_interpreter") or {}
require(future.get("active") is False, "future interpreter active drift")
require(future.get("required_for_python_to_hako_projector_migration") is False, "future interpreter migration prerequisite drift")

task_packs = fixture.get("task_packs") or []
require([row.get("order") for row in task_packs] == [0, 1, 2, 3, 4], "task pack order drift")
tokens = [row.get("token") for row in task_packs]
for expected in [
    token,
    "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-001",
    "<ROUTE-FAMILY>-ROUTE-MATRIX-REPAIR-001",
    "<SELECTED-FAMILY>-HAKO-ADOPTION-DECISION-001",
    "<PROJECTOR-FAMILY>-HAKO-SHADOW-PROMOTION-DECISION-001",
]:
    require(expected in tokens, f"missing task pack {expected}")

claims = fixture.get("claims") or {}
for key in [
    "single_hako_meaning_source",
    "task_packs_named",
    "consultation_gated_wider_route_selection",
    "machine_derived_route_repair_allowed",
]:
    require(claims.get(key) == 1, f"positive claim drift: {key}")
for key in [
    "manual_family_selection",
    "runner_semantic_owner",
    "exe_aot_gate_is_semantic_owner",
    "vm_hako_co_mainline_claim",
    "future_interpreter_activation",
    "future_interpreter_required_for_projector_migration",
    "support_lane_projector_as_adoption_candidate",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "source_selfhost_claim",
    "rust_deletion",
]:
    require(claims.get(key) == 0, f"forbidden claim drift: {key}")

require("current `vm-hako` is not a co-mainline candidate" in artifact_policy, "artifact policy vm-hako boundary drift")
require("Future Interpreter Reservation" in artifact_policy, "artifact policy missing interpreter reservation")
require("backend/interpreter directly interprets compiler policy facts as a second" in role_ssot, "role SSOT missing backend semantic-consumer denial")
require("KeepSourceSelfhostStopped" in design_stop, "design stop decision drift")

for needle in [
    token,
    "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-001",
    "runner_semantic_owner = 0",
    "future_interpreter_required_for_projector_migration = 0",
]:
    require(needle in task_order, f"task-order missing {needle}")

latest_card = state.get("latest_card")
latest_card_path = state.get("latest_card_path")
require(isinstance(latest_card, str) and latest_card, "CURRENT_STATE latest card missing")
require(isinstance(latest_card_path, str) and Path(latest_card_path).exists(), "CURRENT_STATE latest card path missing")
require(latest_card in latest_card_path, "CURRENT_STATE latest card/path mismatch")
require(state.get("current_blocker_token") == design_stop_token, "CURRENT_STATE blocker must remain design stop")
require("tools/checks/rust_lifecycle_source_selfhost_runner_and_route_task_breakdown_guard.sh" in index, "check index missing guard")

print(f"output_contract={contract}")
print(f"current_blocker_preserved={design_stop_token}")
print("single_hako_meaning_source=1")
print("runner_semantic_owner=0")
print("future_interpreter_required_for_projector_migration=0")
print("task_packs_named=1")
print("manual_family_selection=0")
print("runtime_fallback=0")
print("new_backend_route=0")
print("new_abi=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
