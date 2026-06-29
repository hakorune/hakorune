#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-source-selfhost-wider-route-selection-design-stop"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/1799-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-wider-route-selection-design-stop-v0.json"
POLICY="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-next-route-family-selection-policy-v0.json"
RECOVERY="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-blocked-recovery-diagnostic-v0.json"
ADOPTION_PLAN="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-adoption-plan-v0.json"
CANDIDATE_SELECTION="docs/development/current/main/design/fixtures/rust-lifecycle/next-hako-adoption-candidate-selection-v0.json"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

guard_require_files "$TAG" \
  "$CARD" \
  "$FIXTURE" \
  "$POLICY" \
  "$RECOVERY" \
  "$ADOPTION_PLAN" \
  "$CANDIDATE_SELECTION" \
  "$STATE" \
  "$TASK_ORDER" \
  "$INDEX"

python3 - "$CARD" "$FIXTURE" "$POLICY" "$RECOVERY" "$ADOPTION_PLAN" "$CANDIDATE_SELECTION" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
policy_path = Path(sys.argv[3])
recovery_path = Path(sys.argv[4])
adoption_plan_path = Path(sys.argv[5])
candidate_selection_path = Path(sys.argv[6])
state_path = Path(sys.argv[7])
task_order_path = Path(sys.argv[8])
index_path = Path(sys.argv[9])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
policy = json.loads(policy_path.read_text(encoding="utf-8"))
recovery = json.loads(recovery_path.read_text(encoding="utf-8"))
adoption_plan = json.loads(adoption_plan_path.read_text(encoding="utf-8"))
candidate_selection = json.loads(candidate_selection_path.read_text(encoding="utf-8"))
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
output_contract = "rust-lifecycle-source-selfhost-wider-route-selection-design-stop-v0"
decision_kind = "KeepSourceSelfhostStopped"
reason_token = "NoEligibleNativeAdoptionCandidate"
next_action = "DesignConsultationRequired"
resume_condition = "ConsultationGatedWiderRouteSelectionOrMachineDerivedRouteRepair"

require(f"# {token}" in card, "card token drift")
require(f"output_contract:\n  {output_contract}" in card, "card output contract drift")
require(f"decision:\n  {decision_kind}" in card, "card decision drift")
require(f"reason_token:\n  {reason_token}" in card, "card reason drift")
require(f"next_action:\n  {next_action}" in card, "card next action drift")
require(f"resume_condition:\n  {resume_condition}" in card, "card resume condition drift")

require(fixture.get("kind") == "SourceSelfhostWiderRouteSelectionDesignStopV1", "fixture kind drift")
require(fixture.get("output_contract") == output_contract, "fixture output contract drift")
require((fixture.get("current_state") or {}).get("latest_card") == token, "fixture latest card drift")
require((fixture.get("current_state") or {}).get("current_blocker_token") == token, "fixture blocker drift")

decision = fixture.get("decision") or {}
require(decision.get("kind") == decision_kind, "fixture decision drift")
require(decision.get("reason_token") == reason_token, "fixture reason token drift")
require(decision.get("next_action") == next_action, "fixture next action drift")

recovery_section = fixture.get("recovery") or {}
require(recovery_section.get("resume_condition") == resume_condition, "fixture resume condition drift")
require(recovery_section.get("manual_family_selection") == 0, "manual family selection drift")
require(recovery_section.get("route_membership_alone_as_proof") == 0, "route membership proof drift")
require(recovery_section.get("coverage_percentage_as_proof") == 0, "coverage proof drift")
require(recovery_section.get("bundle_size_as_proof") == 0, "bundle size proof drift")
require(recovery_section.get("support_lane_projector_as_adoption_candidate") == 0, "support-lane proof drift")

claims = fixture.get("claims") or {}
for key in [
    "new_hako_adopted_family",
    "new_route_family_selected",
    "full_variable_context_claim",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "rust_deletion",
]:
    require(claims.get(key) == 0, f"claim drift: {key}")

require((policy.get("decision") or {}).get("kind") == "KeepSourceSelfhostStopped", "policy decision drift")
require((policy.get("decision") or {}).get("reason_token") == reason_token, "policy reason drift")
require((policy.get("decision") or {}).get("next_action") == token, "policy handoff drift")

require((recovery.get("blocked_evidence") or {}).get("candidate_pool_state") == "Blocked", "recovery state drift")
recovery_block = recovery.get("recovery") or {}
require(recovery_block.get("next_action") == "DesignConsultationRequired", "recovery next action drift")

require(adoption_plan.get("plan", {}).get("candidate_pool_state") == "Blocked", "adoption plan candidate pool drift")
require(adoption_plan.get("plan", {}).get("manual_family_selection") == 0, "adoption plan manual selection drift")

require((candidate_selection.get("decision") or {}).get("kind") == "Blocked", "candidate selection should remain blocked")
require((candidate_selection.get("eligible_candidate_count")) == 0, "candidate selection eligible count drift")

require(state.get("current_blocker_token") == token, "current-state blocker drift")
latest_card = state.get("latest_card")
latest_card_path = state.get("latest_card_path")
require(isinstance(latest_card, str) and latest_card, "current-state latest card missing")
require(isinstance(latest_card_path, str) and Path(latest_card_path).exists(), "current-state latest card path missing")
require(latest_card in latest_card_path, "current-state latest card/path mismatch")

for needle in [
    token,
    output_contract,
    decision_kind,
    reason_token,
    next_action,
    resume_condition,
    "manual_family_selection = 0",
    "consultation_gated_wider_route_selection = 1",
    "machine_derived_route_repair_allowed = 1",
]:
    require(needle in task_order, f"task-order missing {needle}")

require("tools/checks/rust_lifecycle_source_selfhost_wider_route_selection_design_stop_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print(f"decision={decision_kind}")
print(f"reason_token={reason_token}")
print(f"next_action={next_action}")
print(f"resume_condition={resume_condition}")
print("candidate_pool_state=Blocked")
print("manual_family_selection=0")
print("route_membership_alone_as_proof=0")
print("support_lane_projector_as_adoption_candidate=0")
print("runtime_fallback=0")
print("new_backend_route=0")
print("new_abi=0")
print("summary=ok")
PY
