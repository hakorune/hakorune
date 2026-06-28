#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

CARD="docs/development/current/main/phases/phase-296x/1790-MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-SURFACE-SELECTION-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-explicit-mutation-surface-selection-v0.json"
POST_RESOLUTION="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-post-variable-context-owned-snapshot-resolution-v0.json"
NATIVE_SOURCE="apps/lib/hakorune_mir_builder/variable_context.hako"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

python3 - "$CARD" "$FIXTURE" "$POST_RESOLUTION" "$NATIVE_SOURCE" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
post_resolution_path = Path(sys.argv[3])
native_source_path = Path(sys.argv[4])
state_path = Path(sys.argv[5])
task_order_path = Path(sys.argv[6])
index_path = Path(sys.argv[7])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
post_resolution = json.loads(post_resolution_path.read_text(encoding="utf-8"))
native_source = native_source_path.read_text(encoding="utf-8")
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-SURFACE-SELECTION-001"
output_contract = "rust-lifecycle-variable-context-explicit-mutation-surface-selection-v0"
owner_kind = "VariableContextReturnedMutableBorrowPolicyDecision"
policy = "ExplicitMutationApiOnly"
next_action = "MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-PROJECTION-001"
ops = ["insert", "remove", "restore", "replace_owned_map"]

require(f"# {token}" in card, "card token drift")
require(f"output_contract={output_contract}" in card, "card output contract drift")
require(f"selected_policy={policy}" in card, "card policy drift")
require(f"owner_kind={owner_kind}" in card, "card owner kind drift")
require("variable_map_mut_selected=0" in card, "card mutable selection drift")
require(f"next_action={next_action}" in card, "card next action drift")

require(fixture.get("kind") == "VariableContextExplicitMutationSurfaceSelectionV1", "fixture kind drift")
require(fixture.get("output_contract") == output_contract, "fixture output contract drift")
require(fixture.get("family_id") == "hakorune_mir_builder::variable_context", "fixture family drift")

fixture_state = fixture.get("current_state") or {}
require(fixture_state.get("latest_card") == token, "fixture latest card drift")
require(fixture_state.get("current_blocker_token") == token, "fixture blocker drift")

input_state = fixture.get("input_state") or {}
require(input_state.get("current_blocker") == "SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-OWNED-SNAPSHOT-RESOLUTION-001", "fixture input blocker drift")
require(input_state.get("last_adopted_surface") == "VariableContextNativeSurfaceOwnedReadSnapshotV1", "fixture adopted surface drift")
require(input_state.get("remaining_boundary") == "VariableContext_mutable_returned_borrow", "fixture remaining boundary drift")
require(input_state.get("reason_token") == "ReturnedMutableBorrowPolicyRequired", "fixture reason drift")

selected = fixture.get("selected_policy") or {}
require(selected.get("owner_kind") == owner_kind, "fixture owner kind drift")
require(selected.get("policy") == policy, "fixture policy drift")
variable_map_mut = selected.get("variable_map_mut") or {}
require(variable_map_mut.get("rust_return") == "&mut BTreeMap<String, ValueId>", "fixture rust return drift")
require(variable_map_mut.get("selected") is False, "fixture mutable selected drift")
require(variable_map_mut.get("deny_reason") == "ReturnedMutableBorrow", "fixture mutable deny drift")
require(variable_map_mut.get("replacement") == "ExplicitMutationOperations", "fixture mutable replacement drift")

op_rows = {row.get("name"): row for row in fixture.get("selected_mutation_operations", [])}
require(sorted(op_rows) == sorted(ops), "fixture mutation op set drift")
require(op_rows["insert"].get("operation") == "MapSet", "insert operation drift")
require(op_rows["remove"].get("operation") == "MapRemove", "remove operation drift")
require(op_rows["restore"].get("operation") == "ReplaceOwnedMap", "restore operation drift")
require(op_rows["replace_owned_map"].get("operation") == "ReplaceOwnedMap", "replace_owned_map operation drift")
for row in op_rows.values():
    require(row.get("mutates") == ["variable_map"], f"mutation frame drift: {row.get('name')}")

denied = set(fixture.get("explicitly_denied", []))
for item in [
    "raw_variable_map_mut_alias",
    "returned_mutable_borrow_escape",
    "borrow_lifetime_inference",
    "implicit_commit_discard_mut_lease",
]:
    require(item in denied, f"fixture missing denial: {item}")

recovery = fixture.get("candidate_recovery") or {}
require(recovery.get("candidate_pool_state_after_this_card") == "BlockedUntilExplicitMutationProjection", "fixture recovery state drift")
require(recovery.get("next_action") == next_action, "fixture recovery next action drift")

claims = fixture.get("claims") or {}
for key in [
    "full_variable_context_claim",
    "returned_mutable_borrow_selected",
    "mut_lease_selected",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "manual_family_selection",
]:
    require(claims.get(key) == 0, f"fixture claim drift: {key}")

require(post_resolution.get("kind") == "SourceSelfhostPostVariableContextOwnedSnapshotResolutionV1", "post resolution kind drift")
boundary = post_resolution.get("remaining_boundary") or {}
require(boundary.get("scope") == "VariableContext_mutable_returned_borrow", "post resolution boundary drift")
require(boundary.get("reason") == "ReturnedMutableBorrow", "post resolution reason drift")
require((post_resolution.get("resolution") or {}).get("reason_token") == "ReturnedMutableBorrowPolicyRequired", "post resolution reason token drift")

for needle in [
    "insert(ctx, name, value_id)",
    "remove(ctx, name)",
    "restore(ctx: VariableContextNative, snapshot: OrderedMapBox)",
]:
    require(needle in native_source, f"native source missing existing API: {needle}")
require("variable_map_mut" not in native_source, "native source must not expose variable_map_mut")
require("replace_owned_map" not in native_source, "replace_owned_map must remain for projection card")

require(state.get("latest_card") == token, "current-state latest card drift")
require(
    state.get("latest_card_path")
    == "docs/development/current/main/phases/phase-296x/1790-MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-SURFACE-SELECTION-001.md",
    "current-state latest card path drift",
)
require(state.get("current_blocker_token") == token, "current-state blocker drift")

for needle in [
    token,
    owner_kind,
    policy,
    next_action,
]:
    require(needle in task_order, f"task-order missing {needle}")

require("tools/checks/rust_lifecycle_variable_context_explicit_mutation_surface_selection_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print(f"selected_policy={policy}")
print(f"owner_kind={owner_kind}")
print("variable_map_mut_selected=0")
print("variable_map_mut_deny_reason=ReturnedMutableBorrow")
print("selected_mutation_ops=insert,remove,restore,replace_owned_map")
print("candidate_pool_state_after_this_card=BlockedUntilExplicitMutationProjection")
print(f"next_action={next_action}")
print("manual_family_selection=0")
print("full_variable_context_claim=0")
print("returned_mutable_borrow_selected=0")
print("mut_lease_selected=0")
print("runtime_fallback=0")
print("new_backend_route=0")
print("new_abi=0")
print("new_python_semantic_projector=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
