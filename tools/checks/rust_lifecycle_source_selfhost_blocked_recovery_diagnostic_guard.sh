#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

CARD="docs/development/current/main/phases/phase-296x/1781-SOURCE-SELFHOST-BLOCKED-RECOVERY-DIAGNOSTIC-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-blocked-recovery-diagnostic-v0.json"
CANDIDATE_FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/next-hako-adoption-candidate-selection-v0.json"
ROUTE_CLOSEOUT="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-route-matrix-closeout-v0.json"
SOURCE_PLAN="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-adoption-plan-v0.json"
ROUTE_MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/family_routes.json"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

python3 - "$CARD" "$FIXTURE" "$CANDIDATE_FIXTURE" "$ROUTE_CLOSEOUT" "$SOURCE_PLAN" "$ROUTE_MANIFEST" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
candidate_path = Path(sys.argv[3])
route_closeout_path = Path(sys.argv[4])
source_plan_path = Path(sys.argv[5])
route_manifest_path = Path(sys.argv[6])
state_path = Path(sys.argv[7])
task_order_path = Path(sys.argv[8])
index_path = Path(sys.argv[9])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
candidate = json.loads(candidate_path.read_text(encoding="utf-8"))
route_closeout = json.loads(route_closeout_path.read_text(encoding="utf-8"))
source_plan = json.loads(source_plan_path.read_text(encoding="utf-8"))
route_manifest = json.loads(route_manifest_path.read_text(encoding="utf-8"))
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "SOURCE-SELFHOST-BLOCKED-RECOVERY-DIAGNOSTIC-001"
output_contract = "rust-lifecycle-source-selfhost-blocked-recovery-diagnostic-v0"

require(f"# {token}" in card, "card token drift")
require(f"output_contract={output_contract}" in card, "card output contract drift")
require("candidate_pool_state = Blocked" in card, "card candidate pool drift")
require("next_action = DesignConsultationRequired" in card, "card next_action drift")
require("resume_condition = MachineDerivedRepairLaneOrNewEligibleRoute" in card, "card resume condition drift")
require("manual_family_selection = 0" in card, "card manual selection drift")

require(fixture.get("kind") == "SourceSelfhostBlockedRecoveryDiagnosticV1", "fixture kind drift")
require(fixture.get("output_contract") == output_contract, "fixture output contract drift")
require(fixture.get("source_selfhost_adoption_plan") == "SOURCE-SELFHOST-ADOPTION-PLAN-001", "fixture source plan drift")

fixture_state = fixture.get("current_state") or {}
require(fixture_state.get("latest_card") == token, "fixture latest card drift")
require(fixture_state.get("current_blocker_token") == token, "fixture blocker drift")

blocked = fixture.get("blocked_evidence") or {}
require(blocked.get("candidate_pool_state") == "Blocked", "fixture blocked state drift")
require(blocked.get("eligible_candidate_count") == 0, "fixture eligible count drift")
require(blocked.get("blocked_reason_token") == "NoEligibleDerivedMainlineRouteCandidate", "fixture blocked reason drift")

parked = fixture.get("parked_family") or {}
require(parked.get("family_id") == "hakorune_mir_builder::variable_context", "fixture parked family drift")
require(parked.get("family_state") == "Parked", "fixture parked state drift")
require(parked.get("parked_reason") == "ReturnedReadBorrow", "fixture parked reason drift")
require(parked.get("replacement_policy") == "OwnedReadSnapshotProjection", "fixture replacement policy drift")

recovery = fixture.get("recovery") or {}
require(recovery.get("next_action") == "DesignConsultationRequired", "fixture next action drift")
require(recovery.get("resume_condition") == "MachineDerivedRepairLaneOrNewEligibleRoute", "fixture resume condition drift")
require("ManualFamilySelection" in recovery.get("forbidden_next_owner_kinds", []), "fixture forbidden owner drift")

claims = fixture.get("claims") or {}
for key in [
    "manual_family_selection",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "source_selfhost_claim",
    "hako_adopted",
]:
    require(claims.get(key) == 0, f"fixture claim drift: {key}")

require(candidate.get("candidate_pool_state") == "Blocked", "candidate fixture state drift")
require(candidate.get("eligible_candidate_count") == 0, "candidate fixture eligible count drift")
require((candidate.get("decision") or {}).get("reason_token") == "NoEligibleDerivedMainlineRouteCandidate", "candidate fixture reason drift")

require(route_closeout.get("family_state") == "Parked", "route closeout family state drift")
require(route_closeout.get("parked_reason") == "ReturnedReadBorrow", "route closeout reason drift")
require(route_closeout.get("replacement_policy") == "OwnedReadSnapshotProjection", "route closeout replacement drift")

source_plan_plan = source_plan.get("plan") or {}
require(source_plan_plan.get("candidate_pool_state") == "Blocked", "source plan candidate state drift")
require(source_plan_plan.get("manual_family_selection") == 0, "source plan manual selection drift")

routes = route_manifest.get("routes") or []
variable_rows = [row for row in routes if row.get("family_id") == "hakorune_mir_builder::variable_context"]
require(variable_rows, "route manifest missing variable context rows")
require(any(row.get("state") == "Denied" and row.get("deny_reason") == "ReturnedReadBorrow" for row in variable_rows), "route manifest missing returned-borrow denied row")

current_latest = state.get("latest_card")
current_blocker = state.get("current_blocker_token")
allowed_current_tokens = {
    token,
    "VARIABLE-CONTEXT-NATIVE-SURFACE-ADOPTION-SELECTION-001",
    "VARIABLE-CONTEXT-NATIVE-SURFACE-HAKO-ADOPTION-DECISION-001",
    "SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-SURFACE-RESOLUTION-001",
    "MIRBUILDER-VARIABLE-CONTEXT-RETURNED-READ-SNAPSHOT-ROUTE-001",
    "MIRBUILDER-VARIABLE-CONTEXT-OWNED-READ-SNAPSHOT-PROJECTION-001",
    "MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-001",
    "VARIABLE-CONTEXT-OWNED-READ-SNAPSHOT-HAKO-ADOPTION-DECISION-001",
    "SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-OWNED-SNAPSHOT-RESOLUTION-001",
    "MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-SURFACE-SELECTION-001",
}
require(current_latest in allowed_current_tokens, "current-state latest card drift")
require(current_blocker in allowed_current_tokens, "current-state blocker drift")
require(Path(state.get("latest_card_path", "")).exists(), "current-state latest card path missing")

for needle in [
    "SOURCE-SELFHOST-BLOCKED-RECOVERY-DIAGNOSTIC-001",
    "candidate_pool_state = Blocked",
]:
    require(needle in task_order, f"task-order missing {needle}")
require(
    "next_action = DesignConsultationRequired" in task_order
    or "VARIABLE-CONTEXT-NATIVE-SURFACE-HAKO-ADOPTION-DECISION-001" in task_order,
    "task-order missing blocked recovery next action",
)

require("tools/checks/rust_lifecycle_source_selfhost_blocked_recovery_diagnostic_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print("candidate_pool_state=Blocked")
print("eligible_candidate_count=0")
print("blocked_reason_token=NoEligibleDerivedMainlineRouteCandidate")
print("parked_family=hakorune_mir_builder::variable_context")
print("parked_reason=ReturnedReadBorrow")
print("replacement_policy=OwnedReadSnapshotProjection")
print("next_action=DesignConsultationRequired")
print("resume_condition=MachineDerivedRepairLaneOrNewEligibleRoute")
print("manual_family_selection=0")
print("runtime_fallback=0")
print("new_backend_route=0")
print("new_abi=0")
print("new_python_semantic_projector=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
