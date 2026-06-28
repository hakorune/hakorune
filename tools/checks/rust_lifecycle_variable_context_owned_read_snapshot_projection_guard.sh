#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

CARD="docs/development/current/main/phases/phase-296x/1786-MIRBUILDER-VARIABLE-CONTEXT-OWNED-READ-SNAPSHOT-PROJECTION-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-owned-read-snapshot-projection-v0.json"
ROUTE_FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-returned-read-snapshot-route-v0.json"
NATIVE_SOURCE="apps/lib/hakorune_mir_builder/variable_context.hako"
NATIVE_TEST="apps/tests/phase296x_variable_context_native_snapshot_restore_min.hako"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

bash tools/checks/rust_mirbuilder_variable_context_native_snapshot_restore_guard.sh >/tmp/hako_variable_context_native_snapshot_restore_guard.out

python3 - "$CARD" "$FIXTURE" "$ROUTE_FIXTURE" "$NATIVE_SOURCE" "$NATIVE_TEST" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
route_fixture_path = Path(sys.argv[3])
native_source_path = Path(sys.argv[4])
native_test_path = Path(sys.argv[5])
state_path = Path(sys.argv[6])
task_order_path = Path(sys.argv[7])
index_path = Path(sys.argv[8])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
route_fixture = json.loads(route_fixture_path.read_text(encoding="utf-8"))
native_source = native_source_path.read_text(encoding="utf-8")
native_test = native_test_path.read_text(encoding="utf-8")
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "MIRBUILDER-VARIABLE-CONTEXT-OWNED-READ-SNAPSHOT-PROJECTION-001"
output_contract = "rust-lifecycle-variable-context-owned-read-snapshot-projection-v0"
next_action = "MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-001"

require(f"# {token}" in card, "card token drift")
require(f"output_contract={output_contract}" in card, "card output contract drift")
require("selected_projection=OwnedReadSnapshotProjection" in card, "card projection drift")
require("raw_variable_map_alias_emitted=0" in card, "card raw alias drift")

require(fixture.get("kind") == "VariableContextOwnedReadSnapshotProjectionV1", "fixture kind drift")
require(fixture.get("output_contract") == output_contract, "fixture output contract drift")

fixture_state = fixture.get("current_state") or {}
require(fixture_state.get("latest_card") == token, "fixture latest card drift")
require(fixture_state.get("current_blocker_token") == token, "fixture blocker drift")

input_route = fixture.get("input_route") or {}
require(input_route.get("token") == "MIRBUILDER-VARIABLE-CONTEXT-RETURNED-READ-SNAPSHOT-ROUTE-001", "fixture input route drift")
require(input_route.get("selected_repair") == "OwnedReadSnapshotProjection", "fixture selected repair drift")

projection = fixture.get("projection") or {}
require(projection.get("source_method") == "VariableContext::variable_map", "fixture source method drift")
require(projection.get("rust_return") == "&BTreeMap<String, ValueId>", "fixture rust return drift")
require(projection.get("selected_hako_api") == "VariableContextNativeApi.snapshot", "fixture api drift")
require(projection.get("result_transport") == "OrderedMapBox", "fixture transport drift")
require(projection.get("owned_clone_required") == 1, "fixture owned clone drift")

vectors = {row.get("name"): row for row in fixture.get("oracle_vectors", [])}
require(vectors.get("nonempty_snapshot", {}).get("snapshot") == [["a", 10], ["b", 20]], "oracle nonempty drift")
require(vectors.get("source_mutation_after_snapshot", {}).get("snapshot_after") == [["a", 10], ["b", 20]], "oracle source mutation alias drift")
require(vectors.get("snapshot_mutation_after_snapshot", {}).get("source_after") == [["a", 10], ["b", 20], ["c", 30]], "oracle snapshot mutation alias drift")

proof = fixture.get("proof") or {}
require(proof.get("source_to_snapshot_alias") == 0, "fixture source alias drift")
require(proof.get("snapshot_to_source_alias") == 0, "fixture snapshot alias drift")
require(proof.get("deterministic_order_preserved") == 1, "fixture order drift")
require(proof.get("raw_variable_map_alias_emitted") == 0, "fixture raw alias drift")
require(proof.get("variable_map_mut_selected") == 0, "fixture mutable selected drift")

recovery = fixture.get("candidate_recovery") or {}
require(recovery.get("next_action") == next_action, "fixture next action drift")
require(recovery.get("candidate_pool_state_after_this_card") == "BlockedUntilRouteMatrixRerun", "fixture recovery state drift")

claims = fixture.get("claims") or {}
for key in [
    "borrow_view_implementation",
    "returned_mutable_borrow_repair",
    "full_variable_context_claim",
    "candidate_pool_eligible",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "manual_family_selection",
]:
    require(claims.get(key) == 0, f"fixture claim drift: {key}")

route_recovery = route_fixture.get("candidate_recovery") or {}
require(route_recovery.get("next_action") == "MIRBUILDER-VARIABLE-CONTEXT-OWNED-READ-SNAPSHOT-PROJECTION-001", "route fixture next action drift")

require("snapshot(ctx: VariableContextNative): OrderedMapBox" in native_source, "native source snapshot signature drift")
require("return ctx.variable_map.clone_owned()" in native_source, "native source clone_owned drift")
require("variable_map()" not in native_source, "native source raw variable_map method drift")
require("variable_map_mut" not in native_source, "native source mutable raw map drift")

require("snapshot.remove(\"a\")" in native_test, "native test missing snapshot mutation proof")
require("source_has_a_after_snapshot_mutation" in native_test, "native test missing source alias proof")
require("snapshot_has_c_after_source_mutation" in native_test, "native test missing snapshot alias proof")

current_latest = state.get("latest_card")
current_blocker = state.get("current_blocker_token")
allowed_current_tokens = {
    token,
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

require("tools/checks/rust_lifecycle_variable_context_owned_read_snapshot_projection_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print("selected_projection=OwnedReadSnapshotProjection")
print("native_api=VariableContextNativeApi.snapshot")
print("owned_clone_required=1")
print("source_to_snapshot_alias=0")
print("snapshot_to_source_alias=0")
print("deterministic_order_preserved=1")
print("raw_variable_map_alias_emitted=0")
print("variable_map_mut_selected=0")
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
