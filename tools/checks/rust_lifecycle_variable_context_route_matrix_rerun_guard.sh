#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

CARD="docs/development/current/main/phases/phase-296x/1787-MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-route-matrix-rerun-v0.json"
ROUTE_MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/family_routes.json"
PRIOR_CLOSEOUT="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-route-matrix-closeout-v0.json"
ROUTE_FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-returned-read-snapshot-route-v0.json"
PROJECTION_FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-owned-read-snapshot-projection-v0.json"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

python3 - "$CARD" "$FIXTURE" "$ROUTE_MANIFEST" "$PRIOR_CLOSEOUT" "$ROUTE_FIXTURE" "$PROJECTION_FIXTURE" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
route_manifest_path = Path(sys.argv[3])
prior_closeout_path = Path(sys.argv[4])
route_fixture_path = Path(sys.argv[5])
projection_fixture_path = Path(sys.argv[6])
state_path = Path(sys.argv[7])
task_order_path = Path(sys.argv[8])
index_path = Path(sys.argv[9])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
route_manifest = json.loads(route_manifest_path.read_text(encoding="utf-8"))
prior_closeout = json.loads(prior_closeout_path.read_text(encoding="utf-8"))
route_fixture = json.loads(route_fixture_path.read_text(encoding="utf-8"))
projection_fixture = json.loads(projection_fixture_path.read_text(encoding="utf-8"))
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-001"
output_contract = "rust-lifecycle-variable-context-route-matrix-rerun-v0"
selected_surface = "VariableContextNativeSurfaceOwnedReadSnapshotV1"
next_action = "VARIABLE-CONTEXT-OWNED-READ-SNAPSHOT-HAKO-ADOPTION-DECISION-001"

require(f"# {token}" in card, "card token drift")
require(f"output_contract={output_contract}" in card, "card output contract drift")
require(f"selected_surface_id={selected_surface}" in card, "card selected surface drift")
require(f"next_action={next_action}" in card, "card next action drift")

require(fixture.get("kind") == "VariableContextRouteMatrixRerunV1", "fixture kind drift")
require(fixture.get("output_contract") == output_contract, "fixture output contract drift")

fixture_state = fixture.get("current_state") or {}
require(fixture_state.get("latest_card") == token, "fixture latest card drift")
require(fixture_state.get("current_blocker_token") == token, "fixture blocker drift")

require(route_manifest.get("kind") == "RustDerivedHakoFamilyRouteManifest", "route manifest kind drift")
require(route_manifest.get("claims", {}).get("source_selfhost_claim") == 0, "route manifest source selfhost drift")
require(route_manifest.get("claims", {}).get("runtime_try_hako_then_rust_fallback") == 0, "route manifest fallback drift")

rows = [
    row for row in route_manifest.get("routes", [])
    if row.get("family_id") == "hakorune_mir_builder::variable_context"
]
require(len(rows) == 5, "route manifest variable context row count drift")
denied = [row for row in rows if row.get("pilot_scope") == "VariableContext_immutable_borrow_only"]
require(len(denied) == 1, "route manifest immutable borrow row drift")
denied_row = denied[0]
require(denied_row.get("state") == "Denied", "immutable borrow state drift")
require(denied_row.get("deny_reason") == "ReturnedReadBorrow", "immutable borrow reason drift")
require(denied_row.get("replacement_policy") == "OwnedReadSnapshotProjection", "immutable borrow replacement drift")
require(denied_row.get("selected_on_mainline") is False, "immutable borrow selected drift")

require(prior_closeout.get("kind") == "VariableContextRouteMatrixCloseoutV1", "prior closeout kind drift")
require(prior_closeout.get("candidate_pool_state") == "Parked", "prior closeout pool drift")
require(prior_closeout.get("parked_reason") == "ReturnedReadBorrow", "prior closeout parked reason drift")
require(prior_closeout.get("replacement_policy") == "OwnedReadSnapshotProjection", "prior closeout replacement drift")

require(route_fixture.get("kind") == "VariableContextReturnedReadSnapshotRouteV1", "route fixture kind drift")
require((route_fixture.get("candidate_recovery") or {}).get("candidate_pool_state_after_this_card") == "BlockedUntilRouteMatrixRerun", "route fixture recovery drift")
require((route_fixture.get("candidate_recovery") or {}).get("next_action") == "MIRBUILDER-VARIABLE-CONTEXT-OWNED-READ-SNAPSHOT-PROJECTION-001", "route fixture next action drift")

require(projection_fixture.get("kind") == "VariableContextOwnedReadSnapshotProjectionV1", "projection fixture kind drift")
projection = projection_fixture.get("projection") or {}
proof = projection_fixture.get("proof") or {}
recovery = projection_fixture.get("candidate_recovery") or {}
require(projection.get("selected_hako_api") == "VariableContextNativeApi.snapshot", "projection api drift")
require(projection.get("owned_clone_required") == 1, "projection clone drift")
require(proof.get("source_to_snapshot_alias") == 0, "projection source alias drift")
require(proof.get("snapshot_to_source_alias") == 0, "projection snapshot alias drift")
require(proof.get("raw_variable_map_alias_emitted") == 0, "projection raw alias drift")
require(proof.get("variable_map_mut_selected") == 0, "projection mutable selected drift")
require(recovery.get("candidate_pool_state_after_this_card") == "BlockedUntilRouteMatrixRerun", "projection recovery state drift")
require(recovery.get("next_action") == token, "projection next action drift")

prior_state = fixture.get("prior_state") or {}
require(prior_state.get("candidate_pool_state") == "Blocked", "fixture prior pool drift")
require(prior_state.get("parked_reason") == "ReturnedReadBorrow", "fixture prior parked reason drift")
require(prior_state.get("replacement_policy") == "OwnedReadSnapshotProjection", "fixture prior replacement drift")
require(prior_state.get("denied_route") == "VariableContext_immutable_borrow_only", "fixture prior denied route drift")

repair = fixture.get("repair_evidence") or {}
require(repair.get("projection") == "OwnedReadSnapshotProjection", "fixture repair projection drift")
require(repair.get("native_api") == "VariableContextNativeApi.snapshot", "fixture repair api drift")
require(repair.get("source_to_snapshot_alias") == 0, "fixture repair source alias drift")
require(repair.get("snapshot_to_source_alias") == 0, "fixture repair snapshot alias drift")
require(repair.get("raw_variable_map_alias_emitted") == 0, "fixture repair raw alias drift")
require(repair.get("variable_map_mut_selected") == 0, "fixture repair mutable drift")

result = fixture.get("rerun_result") or {}
require(result.get("candidate_pool_state") == "CandidateEligible", "fixture result pool drift")
require(result.get("selected_surface_id") == selected_surface, "fixture selected surface drift")
require(result.get("reason_token") == "ReturnedReadBorrowRepairedByOwnedReadSnapshotProjection", "fixture reason drift")
require(result.get("next_action") == next_action, "fixture next action drift")

included = fixture.get("included_scopes") or []
for scope in [
    "VariableContext_simple_map_only",
    "VariableContext_snapshot_restore_only",
    "VariableContext_carrier_snapshot_only",
    "VariableContext_explicit_carrier_snapshot_only",
    "VariableContext_immutable_borrow_repaired_as_owned_snapshot",
]:
    require(scope in included, f"fixture missing included scope: {scope}")
excluded = {row.get("scope"): row.get("reason") for row in fixture.get("excluded_scopes", [])}
require(excluded.get("VariableContext_mutable_returned_borrow") == "ReturnedMutableBorrow", "fixture mutable exclusion drift")

claims = fixture.get("claims") or {}
require(claims.get("route_matrix_rerun") == 1, "fixture rerun claim drift")
for key in [
    "manual_family_selection",
    "raw_variable_map_alias_selected",
    "variable_map_mut_selected",
    "full_variable_context_claim",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "hako_adopted",
]:
    require(claims.get(key) == 0, f"fixture claim drift: {key}")

current_latest = state.get("latest_card")
current_blocker = state.get("current_blocker_token")
allowed_current_tokens = {
    token,
    "VARIABLE-CONTEXT-OWNED-READ-SNAPSHOT-HAKO-ADOPTION-DECISION-001",
    "SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-OWNED-SNAPSHOT-RESOLUTION-001",
}
require(current_latest in allowed_current_tokens, "current-state latest card drift")
require(current_blocker in allowed_current_tokens, "current-state blocker drift")
require(Path(state.get("latest_card_path", "")).exists(), "current-state latest card path missing")

for needle in [
    token,
    selected_surface,
    next_action,
]:
    require(needle in task_order, f"task-order missing {needle}")

require("tools/checks/rust_lifecycle_variable_context_route_matrix_rerun_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print("candidate_pool_state=CandidateEligible")
print(f"selected_surface_id={selected_surface}")
print("owned_read_snapshot_projection=green")
print("raw_variable_map_alias_selected=0")
print("variable_map_mut_selected=0")
print("full_variable_context_claim=0")
print("manual_family_selection=0")
print("runtime_fallback=0")
print("new_backend_route=0")
print("new_abi=0")
print("new_python_semantic_projector=0")
print("source_selfhost_claim=0")
print(f"next_action={next_action}")
print("summary=ok")
PY
