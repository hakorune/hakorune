#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-source-selfhost-post-variable-context-explicit-mutation-resolution"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/1794-SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-EXPLICIT-MUTATION-RESOLUTION-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-post-variable-context-explicit-mutation-resolution-v0.json"
ADOPTION="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-explicit-mutation-api-hako-adoption-decision-v0.json"
RERUN="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-route-matrix-rerun-002-v0.json"
PROJECTION="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-explicit-mutation-api-projection-v0.json"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

guard_require_files "$TAG" \
  "$CARD" \
  "$FIXTURE" \
  "$ADOPTION" \
  "$RERUN" \
  "$PROJECTION" \
  "$STATE" \
  "$TASK_ORDER" \
  "$INDEX"

bash tools/checks/rust_lifecycle_variable_context_explicit_mutation_api_hako_adoption_decision_guard.sh >/tmp/hako_variable_context_explicit_mutation_api_hako_adoption_decision_guard.out

python3 - "$CARD" "$FIXTURE" "$ADOPTION" "$RERUN" "$PROJECTION" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
adoption_path = Path(sys.argv[3])
rerun_path = Path(sys.argv[4])
projection_path = Path(sys.argv[5])
state_path = Path(sys.argv[6])
task_order_path = Path(sys.argv[7])
index_path = Path(sys.argv[8])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
adoption = json.loads(adoption_path.read_text(encoding="utf-8"))
rerun = json.loads(rerun_path.read_text(encoding="utf-8"))
projection = json.loads(projection_path.read_text(encoding="utf-8"))
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-EXPLICIT-MUTATION-RESOLUTION-001"
output_contract = "rust-lifecycle-source-selfhost-post-variable-context-explicit-mutation-resolution-v0"
surface_id = "VariableContextNativeSurfaceExplicitMutationApiOnlyV1"

require(f"# {token}" in card, "card token drift")
require(f"output_contract={output_contract}" in card, "card output contract drift")
require("docs_only_closeout = forbidden" in card, "card docs-only drift")
require(f"last_adopted_surface:\n  {surface_id}" in card, "card surface drift")
require("next_action:\n  DesignConsultationRequired" in card, "card next action drift")
require("reason_token:\n  MachineDerivedRepairLaneOrNewEligibleRoute" in card, "card reason drift")

require(fixture.get("kind") == "SourceSelfhostPostVariableContextExplicitMutationResolutionV1", "fixture kind drift")
require(fixture.get("output_contract") == output_contract, "fixture output contract drift")

fixture_state = fixture.get("current_state") or {}
require(fixture_state.get("latest_card") == token, "fixture latest card drift")
require(fixture_state.get("current_blocker_token") == token, "fixture blocker drift")

last_adoption = fixture.get("last_adoption") or {}
require(last_adoption.get("token") == "VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-HAKO-ADOPTION-DECISION-001", "fixture adoption token drift")
require(last_adoption.get("decision") == "Adopt", "fixture adoption decision drift")
require(last_adoption.get("surface_id") == surface_id, "fixture adoption surface drift")

remaining = fixture.get("remaining_boundary") or {}
require(remaining.get("candidate_pool_state") == "Blocked", "fixture remaining candidate drift")
require(remaining.get("reason_token") == "MachineDerivedRepairLaneOrNewEligibleRoute", "fixture remaining reason drift")
require(remaining.get("parked_family") == "hakorune_mir_builder::variable_context", "fixture parked family drift")
require(remaining.get("parked_reason") == "ExplicitMutationApiOnly", "fixture parked reason drift")

recovery = fixture.get("recovery") or {}
require(recovery.get("next_action") == "DesignConsultationRequired", "fixture recovery next action drift")
require(recovery.get("allowed_next_owner_kinds") == ["NextRouteFamilySelectionPolicy", "ExplicitSourceSelfhostStopLine"], "fixture recovery allowed owners drift")

claims = fixture.get("claims") or {}
for key in [
    "manual_family_selection",
    "full_variable_context_claim",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
]:
    require(claims.get(key) == 0, f"fixture claim drift: {key}")

adoption_state = adoption.get("current_state") or {}
require(adoption_state.get("latest_card") == "VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-HAKO-ADOPTION-DECISION-001" or adoption_state.get("latest_card") == token, "adoption current state drift")

rerun_result = rerun.get("rerun_result") or {}
require(rerun_result.get("candidate_pool_state") == "CandidateEligible", "rerun candidate drift")
require(rerun_result.get("selected_surface_id") == surface_id, "rerun surface drift")

projection_state = projection.get("current_state") or {}
require(projection_state.get("latest_card") == "MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-PROJECTION-001", "projection current state drift")

allowed_current_tokens = {
    "VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-HAKO-ADOPTION-DECISION-001",
    token,
}
allowed_current_paths = {
    "docs/development/current/main/phases/phase-296x/1793-VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-HAKO-ADOPTION-DECISION-001.md",
    "docs/development/current/main/phases/phase-296x/1794-SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-EXPLICIT-MUTATION-RESOLUTION-001.md",
}
require(state.get("latest_card") in allowed_current_tokens, "current-state latest card drift")
require(state.get("current_blocker_token") in allowed_current_tokens, "current-state blocker drift")
require(state.get("latest_card_path") in allowed_current_paths, "current-state latest card path drift")

for needle in [
    token,
    output_contract,
]:
    require(needle in task_order, f"task-order missing {needle}")

require("tools/checks/rust_lifecycle_source_selfhost_post_variable_context_explicit_mutation_resolution_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print(f"last_adopted_surface={surface_id}")
print("next_action=DesignConsultationRequired")
print("reason_token=MachineDerivedRepairLaneOrNewEligibleRoute")
print("manual_family_selection=0")
print("full_variable_context_claim=0")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("new_backend_route=0")
print("new_abi=0")
print("new_python_semantic_projector=0")
print("summary=ok")
PY
