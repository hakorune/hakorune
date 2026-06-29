#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-source-selfhost-next-route-family-selection-policy"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/1798-SOURCE-SELFHOST-NEXT-ROUTE-FAMILY-SELECTION-POLICY-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-next-route-family-selection-policy-v0.json"
PRIOR_SELECTION="docs/development/current/main/design/fixtures/rust-lifecycle/next-hako-adoption-candidate-selection-v0.json"
VARIABLE_CONTEXT_ADOPTION="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-explicit-mutation-api-hako-adoption-decision-v0.json"
ENTRIES_RESOLUTION="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-entries-snapshot-need-resolution-v0.json"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

guard_require_files "$TAG" \
  "$CARD" \
  "$FIXTURE" \
  "$PRIOR_SELECTION" \
  "$VARIABLE_CONTEXT_ADOPTION" \
  "$ENTRIES_RESOLUTION" \
  "$STATE" \
  "$TASK_ORDER" \
  "$INDEX"

python3 - "$CARD" "$FIXTURE" "$PRIOR_SELECTION" "$VARIABLE_CONTEXT_ADOPTION" "$ENTRIES_RESOLUTION" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
prior_selection_path = Path(sys.argv[3])
variable_context_adoption_path = Path(sys.argv[4])
entries_resolution_path = Path(sys.argv[5])
state_path = Path(sys.argv[6])
task_order_path = Path(sys.argv[7])
index_path = Path(sys.argv[8])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
prior_selection = json.loads(prior_selection_path.read_text(encoding="utf-8"))
variable_context_adoption = json.loads(variable_context_adoption_path.read_text(encoding="utf-8"))
entries_resolution = json.loads(entries_resolution_path.read_text(encoding="utf-8"))
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "SOURCE-SELFHOST-NEXT-ROUTE-FAMILY-SELECTION-POLICY-001"
output_contract = "rust-lifecycle-source-selfhost-next-route-family-selection-policy-v0"
decision_kind = "KeepSourceSelfhostStopped"
reason_token = "NoEligibleNativeAdoptionCandidate"
next_action = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

require(f"# {token}" in card, "card token drift")
require(f"output_contract:\n  {output_contract}" in card, "card output contract drift")
require(f"current_decision:\n  {decision_kind}" in card, "card decision drift")
require(f"reason_token:\n  {reason_token}" in card, "card reason drift")
require(f"next_action:\n  {next_action}" in card, "card next action drift")

require(fixture.get("kind") == "SourceSelfhostNextRouteFamilySelectionPolicyV1", "fixture kind drift")
require(fixture.get("output_contract") == output_contract, "fixture output contract drift")
require((fixture.get("current_state") or {}).get("latest_card") == token, "fixture latest card drift")
require((fixture.get("current_state") or {}).get("current_blocker_token") == token, "fixture blocker drift")

classifications = set((fixture.get("classification_policy") or {}).get("classifications", []))
required_classes = {
    "AlreadyAdopted",
    "BoundedSurfaceAdopted",
    "SupportLaneOnly",
    "NeedsRouteRepair",
    "NeedsNativeAdoptionDecision",
    "NeedsHakoProjectorPromotion",
    "ConsultationGated",
    "NoEligibleCandidate",
}
require(classifications == required_classes, "classification enum drift")

families = fixture.get("family_classifications") or []
require(families, "missing family classifications")
by_family = {row.get("family_id"): row for row in families}
require(len(by_family) == len(families), "duplicate family classifications")

prior_tokens = set(prior_selection.get("selected_mainline_route_tokens", []))
prior_tokens.update(prior_selection.get("excluded_route_tokens", []))
missing = sorted(token for token in prior_tokens if token not in by_family)
require(not missing, f"classification partition missing prior route token(s): {missing}")

require(by_family["variable_context"].get("classification") == "BoundedSurfaceAdopted", "variable_context classification drift")
require(by_family["variable_context"].get("subclassification") == "ParkedFullClaim", "variable_context subclass drift")
require(by_family["variable_context"].get("eligible_for_next_adoption") is False, "variable_context eligibility drift")

for family_id in ["ReturnEmission", "FunctionRegionStackPop", "SlotRegistryRelease", "compiler-library helpers"]:
    row = by_family.get(family_id)
    require(row is not None, f"missing support lane family {family_id}")
    require(row.get("classification") == "SupportLaneOnly", f"support lane classification drift: {family_id}")
    require(row.get("eligible_for_next_adoption") is False, f"support lane eligibility drift: {family_id}")

decision = fixture.get("decision") or {}
require(decision.get("kind") == decision_kind, "fixture decision kind drift")
require(decision.get("reason_token") == reason_token, "fixture reason token drift")
require(decision.get("next_action") == next_action, "fixture next action drift")
require("recovery_message" in decision and decision["recovery_message"], "missing recovery message")

candidate_pool = fixture.get("candidate_pool") or {}
require(candidate_pool.get("state") == "Blocked", "candidate pool state drift")
require(candidate_pool.get("eligible_count") == 0, "eligible count drift")

selection_rules = fixture.get("selection_rules") or {}
for key in [
    "manual_family_selection",
    "route_membership_alone_as_proof",
    "coverage_percentage_as_proof",
    "bundle_size_as_proof",
    "support_lane_projector_as_adoption_candidate",
]:
    require(selection_rules.get(key) == 0, f"selection rule drift: {key}")
require(selection_rules.get("route_repair_precedes_adoption_selection") == 1, "route repair priority drift")

claims = fixture.get("claims") or {}
for key in [
    "classification_partition_complete",
    "exactly_one_decision",
    "blocked_result_has_recovery_message",
]:
    require(claims.get(key) == 1, f"positive claim drift: {key}")
for key in [
    "new_hako_adopted_family",
    "full_variable_context_claim",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "rust_deletion",
]:
    require(claims.get(key) == 0, f"forbidden claim drift: {key}")

require(prior_selection.get("candidate_pool_state") == "Blocked", "prior candidate pool state drift")
require(prior_selection.get("eligible_candidate_count") == 0, "prior candidate count drift")
require((variable_context_adoption.get("decision") or {}).get("value") == "Adopt", "VariableContext adoption decision drift")
require((variable_context_adoption.get("target") or {}).get("full_variable_context_claim") == 0, "VariableContext full claim drift")
require((entries_resolution.get("decision") or {}).get("next_action") == "NextRouteFamilySelectionPolicy", "entries resolver handoff drift")
require((entries_resolution.get("decision") or {}).get("need_state") == "NotNeeded", "entries need state drift")

latest_card_path = state.get("latest_card_path")
require(isinstance(latest_card_path, str) and Path(latest_card_path).exists(), "current-state latest card path missing")
landed_tail = state.get("landed_tail") or []
require(any("1798 fixes Source Selfhost next route-family selection policy" in row for row in landed_tail), "current-state missing 1798 provenance")

for needle in [
    token,
    decision_kind,
    reason_token,
    next_action,
    "classification_partition_complete = 1",
    "support_lane_projector_as_adoption_candidate = 0",
    "NextRouteFamilySelectionPolicy",
]:
    require(needle in task_order, f"task-order missing {needle}")

require("tools/checks/rust_lifecycle_source_selfhost_next_route_family_selection_policy_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print(f"decision={decision_kind}")
print(f"reason_token={reason_token}")
print(f"next_action={next_action}")
print("classification_partition_complete=1")
print("eligible_count=0")
print("manual_family_selection=0")
print("route_membership_alone_as_proof=0")
print("support_lane_projector_as_adoption_candidate=0")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("new_backend_route=0")
print("new_abi=0")
print("summary=ok")
PY
