#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

CARD="docs/development/current/main/phases/phase-296x/1789-SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-OWNED-SNAPSHOT-RESOLUTION-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-post-variable-context-owned-snapshot-resolution-v0.json"
ADOPTION="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-owned-read-snapshot-hako-adoption-decision-v0.json"
RERUN="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-route-matrix-rerun-v0.json"
ROUTE_MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/family_routes.json"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

python3 - "$CARD" "$FIXTURE" "$ADOPTION" "$RERUN" "$ROUTE_MANIFEST" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
adoption_path = Path(sys.argv[3])
rerun_path = Path(sys.argv[4])
route_manifest_path = Path(sys.argv[5])
state_path = Path(sys.argv[6])
task_order_path = Path(sys.argv[7])
index_path = Path(sys.argv[8])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
adoption = json.loads(adoption_path.read_text(encoding="utf-8"))
rerun = json.loads(rerun_path.read_text(encoding="utf-8"))
route_manifest = json.loads(route_manifest_path.read_text(encoding="utf-8"))
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-OWNED-SNAPSHOT-RESOLUTION-001"
output_contract = "rust-lifecycle-source-selfhost-post-variable-context-owned-snapshot-resolution-v0"
surface_id = "VariableContextNativeSurfaceOwnedReadSnapshotV1"
reason_token = "ReturnedMutableBorrowPolicyRequired"

require(f"# {token}" in card, "card token drift")
require(f"output_contract={output_contract}" in card, "card output contract drift")
require(f"last_adopted_surface={surface_id}" in card, "card surface drift")
require(f"reason_token={reason_token}" in card, "card reason drift")

require(fixture.get("kind") == "SourceSelfhostPostVariableContextOwnedSnapshotResolutionV1", "fixture kind drift")
require(fixture.get("output_contract") == output_contract, "fixture output contract drift")

fixture_state = fixture.get("current_state") or {}
require(fixture_state.get("latest_card") == token, "fixture latest card drift")
require(fixture_state.get("current_blocker_token") == token, "fixture blocker drift")

last = fixture.get("last_adopted_surface") or {}
require(last.get("surface_id") == surface_id, "fixture last surface drift")
require(last.get("decision") == "Adopt", "fixture last decision drift")
require(last.get("included_scope_count") == 5, "fixture included count drift")
require(last.get("full_variable_context_claim") == 0, "fixture full claim drift")

boundary = fixture.get("remaining_boundary") or {}
require(boundary.get("scope") == "VariableContext_mutable_returned_borrow", "fixture remaining boundary drift")
require(boundary.get("reason") == "ReturnedMutableBorrow", "fixture remaining reason drift")
require(boundary.get("selected") is False, "fixture remaining selected drift")
require(boundary.get("policy_required") is True, "fixture policy required drift")

resolution = fixture.get("resolution") or {}
require(resolution.get("next_action") == "DesignConsultationRequired", "fixture next action drift")
require(resolution.get("reason_token") == reason_token, "fixture reason token drift")
for owner_kind in [
    "ReturnedMutableBorrowPolicyDecision",
    "ExplicitSourceSelfhostStopLine",
    "NextRouteFamilySelectionPolicy",
]:
    require(owner_kind in resolution.get("allowed_next_owner_kinds", []), f"fixture missing owner kind: {owner_kind}")

claims = fixture.get("claims") or {}
for key in [
    "manual_family_selection",
    "full_variable_context_claim",
    "returned_mutable_borrow_selected",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
]:
    require(claims.get(key) == 0, f"fixture claim drift: {key}")

require(adoption.get("kind") == "VariableContextOwnedReadSnapshotHakoAdoptionDecisionV1", "adoption kind drift")
require((adoption.get("target") or {}).get("surface_id") == surface_id, "adoption surface drift")
require((adoption.get("decision") or {}).get("value") == "Adopt", "adoption decision drift")
adoption_claims = adoption.get("claims") or {}
require(adoption_claims.get("hako_adopted") == 1, "adoption hako adopted drift")
require(adoption_claims.get("full_variable_context_claim") == 0, "adoption full claim drift")
require(adoption_claims.get("returned_mutable_borrow_selected") == 0, "adoption mutable drift")

require(rerun.get("kind") == "VariableContextRouteMatrixRerunV1", "rerun kind drift")
require((rerun.get("rerun_result") or {}).get("selected_surface_id") == surface_id, "rerun surface drift")

rows = [
    row for row in route_manifest.get("routes", [])
    if row.get("family_id") == "hakorune_mir_builder::variable_context"
]
require(rows, "route manifest missing variable context rows")
require(route_manifest.get("claims", {}).get("source_selfhost_claim") == 0, "route manifest source selfhost drift")
require(route_manifest.get("claims", {}).get("runtime_try_hako_then_rust_fallback") == 0, "route manifest fallback drift")

require(state.get("latest_card") == token, "current-state latest card drift")
require(
    state.get("latest_card_path")
    == "docs/development/current/main/phases/phase-296x/1789-SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-OWNED-SNAPSHOT-RESOLUTION-001.md",
    "current-state latest card path drift",
)
require(state.get("current_blocker_token") == token, "current-state blocker drift")

for needle in [
    token,
    surface_id,
    "DesignConsultationRequired",
    reason_token,
]:
    require(needle in task_order, f"task-order missing {needle}")

require("tools/checks/rust_lifecycle_source_selfhost_post_variable_context_owned_snapshot_resolution_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print(f"last_adopted_surface={surface_id}")
print("remaining_boundary=VariableContext_mutable_returned_borrow")
print("next_action=DesignConsultationRequired")
print(f"reason_token={reason_token}")
print("manual_family_selection=0")
print("full_variable_context_claim=0")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("new_backend_route=0")
print("new_abi=0")
print("new_python_semantic_projector=0")
print("summary=ok")
PY
