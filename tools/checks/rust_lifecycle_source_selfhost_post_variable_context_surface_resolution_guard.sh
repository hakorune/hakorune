#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

CARD="docs/development/current/main/phases/phase-296x/1784-SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-SURFACE-RESOLUTION-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-post-variable-context-surface-resolution-v0.json"
ADOPTION="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-native-surface-hako-adoption-decision-v0.json"
SELECTION="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-native-surface-adoption-selection-v0.json"
ROUTE_CLOSEOUT="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-route-matrix-closeout-v0.json"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

python3 - "$CARD" "$FIXTURE" "$ADOPTION" "$SELECTION" "$ROUTE_CLOSEOUT" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
adoption_path = Path(sys.argv[3])
selection_path = Path(sys.argv[4])
route_closeout_path = Path(sys.argv[5])
state_path = Path(sys.argv[6])
task_order_path = Path(sys.argv[7])
index_path = Path(sys.argv[8])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
adoption = json.loads(adoption_path.read_text(encoding="utf-8"))
selection = json.loads(selection_path.read_text(encoding="utf-8"))
route_closeout = json.loads(route_closeout_path.read_text(encoding="utf-8"))
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-SURFACE-RESOLUTION-001"
output_contract = "rust-lifecycle-source-selfhost-post-variable-context-surface-resolution-v0"
surface_id = "VariableContextNativeSurfaceNoReturnedBorrowV1"

require(f"# {token}" in card, "card token drift")
require(f"output_contract={output_contract}" in card, "card output contract drift")
require("next_action = DesignConsultationRequired" in card, "card next action drift")
require("manual_family_selection = 0" in card, "card manual selection drift")

require(fixture.get("kind") == "SourceSelfhostPostVariableContextSurfaceResolutionV1", "fixture kind drift")
require(fixture.get("output_contract") == output_contract, "fixture output contract drift")

fixture_state = fixture.get("current_state") or {}
require(fixture_state.get("latest_card") == token, "fixture latest card drift")
require(fixture_state.get("current_blocker_token") == token, "fixture blocker drift")

last = fixture.get("last_adoption") or {}
require(last.get("token") == "VARIABLE-CONTEXT-NATIVE-SURFACE-HAKO-ADOPTION-DECISION-001", "fixture last adoption token drift")
require(last.get("decision") == "Adopt", "fixture last adoption decision drift")
require(last.get("surface_id") == surface_id, "fixture surface drift")

remaining = fixture.get("remaining_boundary") or {}
require(remaining.get("candidate_pool_state") == "Blocked", "fixture candidate pool drift")
require(remaining.get("reason_token") == "NoRemainingMachineDerivedNativeSurfaceCandidate", "fixture reason drift")
require(remaining.get("parked_reason") == "ReturnedReadBorrow", "fixture parked reason drift")
require(remaining.get("full_variable_context_claim") == 0, "fixture full claim drift")
require(remaining.get("returned_borrow_selected") == 0, "fixture returned borrow drift")

recovery = fixture.get("recovery") or {}
require(recovery.get("next_action") == "DesignConsultationRequired", "fixture next action drift")
require("VariableContextReturnedBorrowRepairDecision" in recovery.get("allowed_next_owner_kinds", []), "fixture missing repair option")
require("ManualFamilySelection" in recovery.get("forbidden_next_owner_kinds", []), "fixture manual selection forbidden drift")

claims = fixture.get("claims") or {}
for key in [
    "manual_family_selection",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "source_selfhost_claim",
]:
    require(claims.get(key) == 0, f"fixture claim drift: {key}")

require((adoption.get("decision") or {}).get("value") == "Adopt", "adoption fixture decision drift")
require((adoption.get("target") or {}).get("surface_id") == surface_id, "adoption fixture surface drift")
require((adoption.get("claims") or {}).get("full_variable_context_claim") == 0, "adoption fixture full claim drift")
require((adoption.get("claims") or {}).get("returned_borrow_selected") == 0, "adoption fixture returned borrow drift")

require((selection.get("native_surface") or {}).get("surface_id") == surface_id, "selection fixture surface drift")
require((selection.get("native_surface") or {}).get("candidate_state") == "CandidateEligible", "selection fixture state drift")

require(route_closeout.get("family_state") == "Parked", "route closeout family state drift")
require(route_closeout.get("parked_reason") == "ReturnedReadBorrow", "route closeout parked reason drift")

current_latest = state.get("latest_card")
current_blocker = state.get("current_blocker_token")
allowed_current_tokens = {
    token,
    "MIRBUILDER-VARIABLE-CONTEXT-RETURNED-READ-SNAPSHOT-ROUTE-001",
    "MIRBUILDER-VARIABLE-CONTEXT-OWNED-READ-SNAPSHOT-PROJECTION-001",
    "MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-001",
}
require(current_latest in allowed_current_tokens, "current-state latest card drift")
require(current_blocker in allowed_current_tokens, "current-state blocker drift")
require(Path(state.get("latest_card_path", "")).exists(), "current-state latest card path missing")

for needle in [
    token,
    "DesignConsultationRequired",
    "NoRemainingMachineDerivedNativeSurfaceCandidate",
]:
    require(needle in task_order, f"task-order missing {needle}")

require("tools/checks/rust_lifecycle_source_selfhost_post_variable_context_surface_resolution_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print(f"last_adopted_surface={surface_id}")
print("candidate_pool_state=Blocked")
print("next_action=DesignConsultationRequired")
print("reason_token=NoRemainingMachineDerivedNativeSurfaceCandidate")
print("manual_family_selection=0")
print("runtime_fallback=0")
print("new_backend_route=0")
print("new_abi=0")
print("new_python_semantic_projector=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
