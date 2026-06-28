#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-variable-context-reference-projection-contract"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/1795-MIRBUILDER-VARIABLE-CONTEXT-REFERENCE-PROJECTION-CONTRACT-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-reference-projection-contract-v0.json"
POST_RESOLUTION="docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-post-variable-context-explicit-mutation-resolution-v0.json"
READ_PROJECTION="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-owned-read-snapshot-projection-v0.json"
MUT_PROJECTION="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-explicit-mutation-api-projection-v0.json"
NATIVE_SOURCE="apps/lib/hakorune_mir_builder/variable_context.hako"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

guard_require_files "$TAG" \
  "$CARD" \
  "$FIXTURE" \
  "$POST_RESOLUTION" \
  "$READ_PROJECTION" \
  "$MUT_PROJECTION" \
  "$NATIVE_SOURCE" \
  "$STATE" \
  "$TASK_ORDER" \
  "$INDEX"

bash tools/checks/rust_lifecycle_source_selfhost_post_variable_context_explicit_mutation_resolution_guard.sh >/tmp/hako_source_selfhost_post_variable_context_explicit_mutation_resolution_guard.out

python3 - "$CARD" "$FIXTURE" "$POST_RESOLUTION" "$READ_PROJECTION" "$MUT_PROJECTION" "$NATIVE_SOURCE" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
post_resolution_path = Path(sys.argv[3])
read_projection_path = Path(sys.argv[4])
mut_projection_path = Path(sys.argv[5])
native_source_path = Path(sys.argv[6])
state_path = Path(sys.argv[7])
task_order_path = Path(sys.argv[8])
index_path = Path(sys.argv[9])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
post_resolution = json.loads(post_resolution_path.read_text(encoding="utf-8"))
read_projection = json.loads(read_projection_path.read_text(encoding="utf-8"))
mut_projection = json.loads(mut_projection_path.read_text(encoding="utf-8"))
native_source = native_source_path.read_text(encoding="utf-8")
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "MIRBUILDER-VARIABLE-CONTEXT-REFERENCE-PROJECTION-CONTRACT-001"
output_contract = "rust-lifecycle-variable-context-reference-projection-contract-v0"

require(f"# {token}" in card, "card token drift")
require(f"output_contract={output_contract}" in card, "card output contract drift")
require("projection_model=SemanticOneToOneVerifiedProjection" in card, "card projection model drift")
require("syntax_one_to_one_required=0" in card, "card syntax claim drift")

require(fixture.get("kind") == "VariableContextReferenceProjectionContractV1", "fixture kind drift")
require(fixture.get("output_contract") == output_contract, "fixture output contract drift")
require(fixture.get("family_id") == "hakorune_mir_builder::variable_context", "fixture family drift")

fixture_state = fixture.get("current_state") or {}
require(fixture_state.get("latest_card") == token, "fixture latest card drift")
require(fixture_state.get("current_blocker_token") == token, "fixture blocker drift")

input_state = fixture.get("input_state") or {}
require(input_state.get("prior_boundary") == "SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-EXPLICIT-MUTATION-RESOLUTION-001", "fixture prior boundary drift")
require(input_state.get("last_adopted_surface") == "VariableContextNativeSurfaceExplicitMutationApiOnlyV1", "fixture adopted surface drift")
require(input_state.get("projection_model") == "SemanticOneToOneVerifiedProjection", "fixture projection model drift")
require(input_state.get("syntax_one_to_one_required") == 0, "fixture syntax requirement drift")

surfaces = fixture.get("rust_surfaces") or {}
variable_map = surfaces.get("variable_map") or {}
require(variable_map.get("rust_return") == "&BTreeMap<String, ValueId>", "variable_map rust return drift")
require(variable_map.get("hako_projection") == "OwnedReadSnapshotProjection", "variable_map projection drift")
require(variable_map.get("raw_alias_selected") is False, "variable_map raw alias drift")
require(variable_map.get("borrow_view_required") is False, "variable_map borrow view drift")

variable_map_mut = surfaces.get("variable_map_mut") or {}
require(variable_map_mut.get("rust_return") == "&mut BTreeMap<String, ValueId>", "variable_map_mut rust return drift")
require(variable_map_mut.get("hako_projection") == "ExplicitMutationApiOnly", "variable_map_mut projection drift")
require(variable_map_mut.get("raw_mutable_alias_selected") is False, "variable_map_mut raw alias drift")
require(variable_map_mut.get("mut_lease_selected") is False, "variable_map_mut mut lease drift")

snapshot = surfaces.get("snapshot") or {}
require(snapshot.get("rust_operation") == "CloneOwnedMap", "snapshot rust operation drift")
require(snapshot.get("current_hako_api") == "snapshot", "snapshot current API drift")
require(snapshot.get("hako_operation") == "CloneOwnedMap", "snapshot Hako operation drift")

restore = surfaces.get("restore") or {}
require(restore.get("rust_operation") == "ReplaceOwnedMap", "restore rust operation drift")
require(restore.get("current_hako_api") == "restore", "restore current API drift")
require(restore.get("hako_operation") == "ReplaceOwnedMap", "restore Hako operation drift")

for name in ["insert", "remove", "replace_owned_map"]:
    row = surfaces.get(name) or {}
    require(row.get("hako_operation") == name, f"{name} operation drift")
    require(row.get("mutation_frame") == ["variable_map"], f"{name} mutation frame drift")

current_api = set(fixture.get("current_native_api", []))
for name in ["lookup", "contains", "len", "is_empty", "snapshot", "restore", "replace_owned_map", "insert", "remove"]:
    require(name in current_api, f"current native API missing {name}")

future_api = set(fixture.get("future_native_api_candidates", []))
for name in ["entries_snapshot", "snapshot_owned", "restore_owned"]:
    require(name in future_api, f"future API candidate missing {name}")

denied = set(fixture.get("denied", []))
for name in [
    "raw_variable_map_alias",
    "raw_variable_map_mut_alias",
    "returned_mutable_borrow_escape",
    "rust_lifetime_syntax_in_hako",
    "runtime_fallback",
]:
    require(name in denied, f"denied surface missing {name}")

acceptance = fixture.get("acceptance") or {}
for key in [
    "selected_rust_surfaces_classified",
    "replacement_policies_explicit",
    "native_hako_api_has_no_raw_borrow_alias",
    "alias_isolation_guarded",
    "mutation_frame_guarded",
    "restore_replace_not_merge",
    "deterministic_iteration_preserved",
    "emitter_policy_free",
]:
    require(acceptance.get(key) == 1, f"acceptance drift: {key}")

claims = fixture.get("claims") or {}
for key in [
    "syntax_one_to_one_claim",
    "full_variable_context_claim",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "manual_family_selection",
]:
    require(claims.get(key) == 0, f"claim drift: {key}")

post_state = post_resolution.get("last_adoption") or {}
require(post_state.get("surface_id") == "VariableContextNativeSurfaceExplicitMutationApiOnlyV1", "post-resolution surface drift")
require(read_projection.get("kind") == "VariableContextOwnedReadSnapshotProjectionV1", "read projection kind drift")
require(mut_projection.get("kind") == "VariableContextExplicitMutationApiProjectionV1", "mutation projection kind drift")
require((mut_projection.get("projection") or {}).get("selected_policy") == "ExplicitMutationApiOnly", "mutation projection policy drift")

for source_token in [
    "snapshot(ctx: VariableContextNative): OrderedMapBox",
    "restore(ctx: VariableContextNative, snapshot: OrderedMapBox)",
    "replace_owned_map(ctx: VariableContextNative, owned_map: OrderedMapBox)",
    "insert(ctx, name, value_id): i64",
    "remove(ctx, name)",
]:
    require(source_token in native_source, f"native source missing {source_token}")
require("variable_map_mut" not in native_source, "native source must not expose variable_map_mut")

allowed_current_tokens = {
    token,
    "MIRBUILDER-VARIABLE-CONTEXT-BOUNDED-NATIVE-SURFACE-READINESS-RESOLVER-001",
}
allowed_current_paths = {
    "docs/development/current/main/phases/phase-296x/1795-MIRBUILDER-VARIABLE-CONTEXT-REFERENCE-PROJECTION-CONTRACT-001.md",
    "docs/development/current/main/phases/phase-296x/1796-MIRBUILDER-VARIABLE-CONTEXT-BOUNDED-NATIVE-SURFACE-READINESS-RESOLVER-001.md",
}
require(state.get("latest_card") in allowed_current_tokens, "current-state latest card drift")
require(state.get("current_blocker_token") in allowed_current_tokens, "current-state blocker drift")
require(state.get("latest_card_path") in allowed_current_paths, "current-state latest card path drift")

for needle in [
    token,
    output_contract,
    "SemanticOneToOneVerifiedProjection",
    "OwnedReadSnapshotProjection",
    "ExplicitMutationApiOnly",
]:
    require(needle in task_order, f"task-order missing {needle}")

require("tools/checks/rust_lifecycle_variable_context_reference_projection_contract_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print("projection_model=SemanticOneToOneVerifiedProjection")
print("syntax_one_to_one_required=0")
print("variable_map_projection=OwnedReadSnapshotProjection")
print("variable_map_mut_projection=ExplicitMutationApiOnly")
print("raw_variable_map_alias_selected=0")
print("raw_variable_map_mut_alias_selected=0")
print("mut_lease_selected=0")
print("full_variable_context_claim=0")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("new_backend_route=0")
print("new_abi=0")
print("new_python_semantic_projector=0")
print("summary=ok")
PY
