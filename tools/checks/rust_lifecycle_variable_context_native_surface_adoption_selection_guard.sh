#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

CARD="docs/development/current/main/phases/phase-296x/1782-VARIABLE-CONTEXT-NATIVE-SURFACE-ADOPTION-SELECTION-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-native-surface-adoption-selection-v0.json"
ROUTE_CLOSEOUT="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-route-matrix-closeout-v0.json"
BLOCKED_DIAGNOSTIC="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-blocked-recovery-diagnostic-v0.json"
ROUTE_MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/family_routes.json"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

python3 - "$CARD" "$FIXTURE" "$ROUTE_CLOSEOUT" "$BLOCKED_DIAGNOSTIC" "$ROUTE_MANIFEST" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
route_closeout_path = Path(sys.argv[3])
blocked_diagnostic_path = Path(sys.argv[4])
route_manifest_path = Path(sys.argv[5])
state_path = Path(sys.argv[6])
task_order_path = Path(sys.argv[7])
index_path = Path(sys.argv[8])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
route_closeout = json.loads(route_closeout_path.read_text(encoding="utf-8"))
blocked_diagnostic = json.loads(blocked_diagnostic_path.read_text(encoding="utf-8"))
route_manifest = json.loads(route_manifest_path.read_text(encoding="utf-8"))
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "VARIABLE-CONTEXT-NATIVE-SURFACE-ADOPTION-SELECTION-001"
output_contract = "rust-lifecycle-variable-context-native-surface-adoption-selection-v0"
next_action = "VARIABLE-CONTEXT-NATIVE-SURFACE-HAKO-ADOPTION-DECISION-001"
included_scopes = [
    "VariableContext_simple_map_only",
    "VariableContext_snapshot_restore_only",
    "VariableContext_carrier_snapshot_only",
    "VariableContext_explicit_carrier_snapshot_only",
]

require(f"# {token}" in card, "card token drift")
require(f"output_contract={output_contract}" in card, "card output contract drift")
require("full_variable_context_claim = 0" in card, "card full family claim drift")
require("native_surface_candidate_state = CandidateEligible" in card, "card surface candidate drift")
require(f"next_action={next_action}" in card, "card closeout next action drift")

require(fixture.get("kind") == "VariableContextNativeSurfaceAdoptionSelectionV1", "fixture kind drift")
require(fixture.get("output_contract") == output_contract, "fixture output contract drift")

fixture_state = fixture.get("current_state") or {}
require(fixture_state.get("latest_card") == token, "fixture latest card drift")
require(fixture_state.get("current_blocker_token") == token, "fixture blocker drift")

pool = fixture.get("source_selfhost_candidate_pool") or {}
require(pool.get("state") == "Blocked", "fixture source-selfhost pool drift")
require(pool.get("blocked_reason_token") == "NoEligibleDerivedMainlineRouteCandidate", "fixture blocked reason drift")

family = fixture.get("family") or {}
require(family.get("family_id") == "hakorune_mir_builder::variable_context", "fixture family drift")
require(family.get("full_family_state") == "Parked", "fixture full family state drift")
require(family.get("parked_reason") == "ReturnedReadBorrow", "fixture parked reason drift")
require(family.get("replacement_policy") == "OwnedReadSnapshotProjection", "fixture replacement drift")
require(family.get("full_variable_context_claim") == 0, "fixture full claim drift")

surface = fixture.get("native_surface") or {}
require(surface.get("surface_id") == "VariableContextNativeSurfaceNoReturnedBorrowV1", "fixture surface id drift")
require(surface.get("candidate_state") == "CandidateEligible", "fixture surface state drift")
require(surface.get("included_scopes") == included_scopes, "fixture included scopes drift")
excluded = {item.get("scope"): item.get("reason") for item in surface.get("excluded_scopes", [])}
require(excluded.get("VariableContext_immutable_borrow_only") == "ReturnedReadBorrow", "fixture immutable borrow exclusion drift")
require(excluded.get("VariableContext_mutable_returned_borrow") == "ReturnedMutableBorrow", "fixture mutable borrow exclusion drift")

decision = fixture.get("decision") or {}
require(decision.get("next_action") == next_action, "fixture next action drift")
require(decision.get("reason_token") == "EligibleNoReturnedBorrowNativeSurface", "fixture reason drift")
require(decision.get("manual_family_selection") == 0, "fixture manual selection drift")

claims = fixture.get("claims") or {}
for key in [
    "returned_borrow_selected",
    "borrow_view_implementation",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "source_selfhost_claim",
    "hako_adopted",
]:
    require(claims.get(key) == 0, f"fixture claim drift: {key}")

require(route_closeout.get("family_state") == "Parked", "route closeout family state drift")
require(route_closeout.get("parked_reason") == "ReturnedReadBorrow", "route closeout reason drift")
require(route_closeout.get("replacement_policy") == "OwnedReadSnapshotProjection", "route closeout replacement drift")
require(route_closeout.get("selected_mainline_routes") == included_scopes, "route closeout included scopes drift")

blocked = blocked_diagnostic.get("blocked_evidence") or {}
require(blocked.get("candidate_pool_state") == "Blocked", "blocked diagnostic pool drift")
require(blocked.get("eligible_candidate_count") == 0, "blocked diagnostic eligible count drift")

rows = route_manifest.get("routes") or []
rows_by_scope = {row.get("pilot_scope") or row.get("mainline_selection_scope"): row for row in rows}
for scope in included_scopes:
    row = rows_by_scope.get(scope)
    require(row is not None, f"route manifest missing included scope: {scope}")
    require(row.get("family_id") == "hakorune_mir_builder::variable_context", f"route manifest family drift: {scope}")
    require(row.get("state") == "DerivedMainline", f"route manifest state drift: {scope}")
    require(row.get("selected_on_mainline") is True, f"route manifest selection drift: {scope}")
    require(row.get("fallback_policy") == "forbidden", f"route manifest fallback drift: {scope}")

denied = rows_by_scope.get("VariableContext_immutable_borrow_only")
require(denied is not None, "route manifest missing immutable borrow row")
require(denied.get("state") == "Denied", "route manifest immutable borrow state drift")
require(denied.get("deny_reason") == "ReturnedReadBorrow", "route manifest immutable borrow reason drift")
require(denied.get("replacement_policy") == "OwnedReadSnapshotProjection", "route manifest immutable borrow replacement drift")

current_latest = state.get("latest_card")
current_blocker = state.get("current_blocker_token")
allowed_current_tokens = {
    token,
    "VARIABLE-CONTEXT-NATIVE-SURFACE-HAKO-ADOPTION-DECISION-001",
    "SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-SURFACE-RESOLUTION-001",
}
require(current_latest in allowed_current_tokens, "current-state latest card drift")
require(current_blocker in allowed_current_tokens, "current-state blocker drift")
require(Path(state.get("latest_card_path", "")).exists(), "current-state latest card path missing")

for needle in [
    token,
    "VariableContextNativeSurfaceNoReturnedBorrowV1",
    next_action,
]:
    require(needle in task_order, f"task-order missing {needle}")

require("tools/checks/rust_lifecycle_variable_context_native_surface_adoption_selection_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print("source_selfhost_candidate_pool_state=Blocked")
print("full_variable_context_family_state=Parked")
print("native_surface_candidate_state=CandidateEligible")
print("included_scope_count=4")
print("excluded_returned_borrow_count=2")
print(f"next_action={next_action}")
print("manual_family_selection=0")
print("runtime_fallback=0")
print("new_backend_route=0")
print("new_abi=0")
print("new_python_semantic_projector=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
