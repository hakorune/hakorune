#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

CARD="docs/development/current/main/phases/phase-296x/1785-MIRBUILDER-VARIABLE-CONTEXT-RETURNED-READ-SNAPSHOT-ROUTE-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-returned-read-snapshot-route-v0.json"
POST_SURFACE="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-post-variable-context-surface-resolution-v0.json"
READ_VIEW_DECISION="docs/development/current/main/design/fixtures/rust-lifecycle/returned-read-borrow-read-view-decision-v0.json"
ROUTE_CLOSEOUT="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-route-matrix-closeout-v0.json"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

python3 - "$CARD" "$FIXTURE" "$POST_SURFACE" "$READ_VIEW_DECISION" "$ROUTE_CLOSEOUT" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
post_surface_path = Path(sys.argv[3])
read_view_path = Path(sys.argv[4])
route_closeout_path = Path(sys.argv[5])
state_path = Path(sys.argv[6])
task_order_path = Path(sys.argv[7])
index_path = Path(sys.argv[8])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
post_surface = json.loads(post_surface_path.read_text(encoding="utf-8"))
read_view = json.loads(read_view_path.read_text(encoding="utf-8"))
route_closeout = json.loads(route_closeout_path.read_text(encoding="utf-8"))
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "MIRBUILDER-VARIABLE-CONTEXT-RETURNED-READ-SNAPSHOT-ROUTE-001"
output_contract = "rust-lifecycle-variable-context-returned-read-snapshot-route-v0"
next_action = "MIRBUILDER-VARIABLE-CONTEXT-OWNED-READ-SNAPSHOT-PROJECTION-001"

require(f"# {token}" in card, "card token drift")
require(f"output_contract={output_contract}" in card, "card output contract drift")
require("selected_repair=OwnedReadSnapshotProjection" in card, "card selected repair drift")
require("candidate_pool_state_after_this_card=BlockedUntilRouteMatrixRerun" in card, "card candidate recovery drift")

require(fixture.get("kind") == "VariableContextReturnedReadSnapshotRouteV1", "fixture kind drift")
require(fixture.get("output_contract") == output_contract, "fixture output contract drift")
require(fixture.get("family_id") == "hakorune_mir_builder::variable_context", "fixture family drift")

fixture_state = fixture.get("current_state") or {}
require(fixture_state.get("latest_card") == token, "fixture latest card drift")
require(fixture_state.get("current_blocker_token") == token, "fixture blocker drift")

blocked_input = fixture.get("blocked_input") or {}
require(blocked_input.get("candidate_pool_state") == "Blocked", "fixture blocked input state drift")
require(blocked_input.get("reason_token") == "NoRemainingMachineDerivedNativeSurfaceCandidate", "fixture reason drift")
require(blocked_input.get("parked_reason") == "ReturnedReadBorrow", "fixture parked reason drift")
require(blocked_input.get("replacement_policy") == "OwnedReadSnapshotProjection", "fixture replacement policy drift")

methods = fixture.get("source_methods") or {}
variable_map = methods.get("variable_map") or {}
require(variable_map.get("rust_return") == "&BTreeMap<String, ValueId>", "variable_map return drift")
require(variable_map.get("selected") is False, "variable_map selected drift")
require(variable_map.get("deny_reason") == "ReturnedReadBorrow", "variable_map deny drift")
require(variable_map.get("replacement") == "VariableMapOwnedReadSnapshot", "variable_map replacement drift")

variable_map_mut = methods.get("variable_map_mut") or {}
require(variable_map_mut.get("rust_return") == "&mut BTreeMap<String, ValueId>", "variable_map_mut return drift")
require(variable_map_mut.get("selected") is False, "variable_map_mut selected drift")
require(variable_map_mut.get("deny_reason") == "ReturnedMutableBorrow", "variable_map_mut deny drift")
require(variable_map_mut.get("replacement") == "ExplicitMutationOperationsOnly", "variable_map_mut replacement drift")

snapshot = methods.get("snapshot") or {}
require(snapshot.get("selected") is True, "snapshot selected drift")
require(snapshot.get("hako_operation") == "CloneOwnedMap", "snapshot operation drift")

restore = methods.get("restore") or {}
require(restore.get("selected") is True, "restore selected drift")
require(restore.get("hako_operation") == "ReplaceOwnedMap", "restore operation drift")

surface = fixture.get("selected_replacement_surface") or {}
require("owned_read_snapshot" in surface.get("read", []), "surface missing owned read snapshot")
require("entries_snapshot" in surface.get("read", []), "surface missing entries snapshot")
require("restore" in surface.get("write", []), "surface missing restore")
require("raw_variable_map_alias" in surface.get("denied", []), "surface missing raw read alias deny")
require("raw_variable_map_mut_alias" in surface.get("denied", []), "surface missing raw mut alias deny")

recovery = fixture.get("candidate_recovery") or {}
require(recovery.get("next_action") == next_action, "fixture next action drift")
require(recovery.get("candidate_pool_state_after_this_card") == "BlockedUntilRouteMatrixRerun", "fixture candidate recovery drift")

claims = fixture.get("claims") or {}
for key in [
    "full_variable_context_claim",
    "returned_borrow_selected",
    "mutable_returned_borrow_selected",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "manual_family_selection",
]:
    require(claims.get(key) == 0, f"fixture claim drift: {key}")

remaining = post_surface.get("remaining_boundary") or {}
require(remaining.get("candidate_pool_state") == "Blocked", "post-surface state drift")
require(remaining.get("reason_token") == "NoRemainingMachineDerivedNativeSurfaceCandidate", "post-surface reason drift")
require(remaining.get("parked_reason") == "ReturnedReadBorrow", "post-surface parked reason drift")

require(read_view.get("current_contract") == "NoReturnedAlias + OwnedReadSnapshotProjection", "read-view contract drift")
require("keep OwnedReadSnapshotProjection for bulk read consumers" in read_view.get("decision", []), "read-view decision drift")
require("do not re-open variable_map() as a naked borrowed alias" in read_view.get("decision", []), "read-view alias decision drift")

require(route_closeout.get("parked_reason") == "ReturnedReadBorrow", "route closeout parked reason drift")
require(route_closeout.get("replacement_policy") == "OwnedReadSnapshotProjection", "route closeout replacement drift")

current_latest = state.get("latest_card")
current_blocker = state.get("current_blocker_token")
allowed_current_tokens = {
    token,
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
    token,
    "OwnedReadSnapshotProjection",
    next_action,
]:
    require(needle in task_order, f"task-order missing {needle}")

require("tools/checks/rust_lifecycle_variable_context_returned_read_snapshot_route_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print("selected_repair=OwnedReadSnapshotProjection")
print("variable_map_raw_alias_selected=0")
print("variable_map_replacement=VariableMapOwnedReadSnapshot")
print("variable_map_mut_selected=0")
print("variable_map_mut_deny_reason=ReturnedMutableBorrow")
print("candidate_pool_state_after_this_card=BlockedUntilRouteMatrixRerun")
print(f"next_action={next_action}")
print("manual_family_selection=0")
print("runtime_fallback=0")
print("new_backend_route=0")
print("new_abi=0")
print("new_python_semantic_projector=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
