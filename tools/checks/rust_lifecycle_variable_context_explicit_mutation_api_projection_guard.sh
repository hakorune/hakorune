#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-variable-context-explicit-mutation-api-projection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1791-MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-PROJECTION-001.md"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-explicit-mutation-api-projection-v0.json"
NATIVE_SOURCE="$ROOT_DIR/apps/lib/hakorune_mir_builder/variable_context.hako"
NATIVE_TEST="$ROOT_DIR/apps/tests/phase296x_variable_context_native_explicit_mutation_min.hako"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
EXE="/tmp/phase296x_variable_context_native_explicit_mutation_min.exe"
BUILD_LOG="/tmp/phase296x_variable_context_native_explicit_mutation_min.build.log"
RUN_LOG="/tmp/phase296x_variable_context_native_explicit_mutation_min.run.log"

guard_require_command "$TAG" python3
guard_require_files "$TAG" \
  "$CARD" \
  "$FIXTURE" \
  "$NATIVE_SOURCE" \
  "$NATIVE_TEST" \
  "$STATE" \
  "$TASK_ORDER" \
  "$INDEX" \
  "$ROOT_DIR/tools/bin/hako"

bash "$ROOT_DIR/tools/build_hako_llvmc_ffi.sh" >/dev/null
rm -f "$EXE" "$BUILD_LOG" "$RUN_LOG"

if ! ./target/release/hakorune --emit-exe "$EXE" "$NATIVE_TEST" >"$BUILD_LOG" 2>&1; then
  echo "emit_exe=fail" >&2
  echo "source=apps/tests/phase296x_variable_context_native_explicit_mutation_min.hako" >&2
  sed -n '1,120p' "$BUILD_LOG" >&2
  exit 1
fi

if ! "$EXE" >"$RUN_LOG" 2>&1; then
  echo "runtime_smoke=fail" >&2
  echo "source=apps/tests/phase296x_variable_context_native_explicit_mutation_min.hako" >&2
  sed -n '1,120p' "$RUN_LOG" >&2
  exit 1
fi

if ! grep -Fq "variable_context_native_explicit_mutation=ok" "$RUN_LOG"; then
  echo "runtime_marker=fail" >&2
  echo "expected=variable_context_native_explicit_mutation=ok" >&2
  sed -n '1,120p' "$RUN_LOG" >&2
  exit 1
fi

python3 - "$CARD" "$FIXTURE" "$NATIVE_SOURCE" "$NATIVE_TEST" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
native_source_path = Path(sys.argv[3])
native_test_path = Path(sys.argv[4])
state_path = Path(sys.argv[5])
task_order_path = Path(sys.argv[6])
index_path = Path(sys.argv[7])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
native_source = native_source_path.read_text(encoding="utf-8")
native_test = native_test_path.read_text(encoding="utf-8")
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-PROJECTION-001"
output_contract = "rust-lifecycle-variable-context-explicit-mutation-api-projection-v0"
next_action = "MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-002"
owner_kind = "VariableContextReturnedMutableBorrowPolicyDecision"
policy = "ExplicitMutationApiOnly"

require(f"# {token}" in card, "card token drift")
require(f"output_contract={output_contract}" in card, "card output contract drift")
require(f"selected_policy={policy}" in card, "card policy drift")
require(f"owner_kind={owner_kind}" in card, "card owner kind drift")
require("replace_owned_map_native_api=1" in card, "card replace_owned_map claim drift")
require("candidate_pool_state_after_this_card=BlockedUntilRouteMatrixRerun" in card, "card recovery drift")
require(f"next_action={next_action}" in card, "card next action drift")

require(fixture.get("kind") == "VariableContextExplicitMutationApiProjectionV1", "fixture kind drift")
require(fixture.get("output_contract") == output_contract, "fixture output contract drift")

fixture_state = fixture.get("current_state") or {}
require(fixture_state.get("latest_card") == token, "fixture latest card drift")
require(fixture_state.get("current_blocker_token") == token, "fixture blocker drift")

input_state = fixture.get("input_state") or {}
require(input_state.get("current_blocker") == "MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-SURFACE-SELECTION-001", "fixture input blocker drift")
require(input_state.get("last_adopted_surface") == "VariableContextNativeSurfaceOwnedReadSnapshotV1", "fixture adopted surface drift")
require(input_state.get("remaining_boundary") == "VariableContext_mutable_returned_borrow", "fixture remaining boundary drift")
require(input_state.get("reason_token") == "ReturnedMutableBorrowPolicyRequired", "fixture reason token drift")
require(input_state.get("selected_policy") == policy, "fixture selected policy drift")

projection = fixture.get("projection") or {}
require(projection.get("source_method") == "VariableContext::variable_map_mut", "fixture source method drift")
require(projection.get("rust_return") == "&mut BTreeMap<String, ValueId>", "fixture rust return drift")
require(projection.get("selected_policy") == policy, "fixture projection policy drift")
require(projection.get("owned_map_transport") == "OrderedMapBox", "fixture transport drift")
require(projection.get("raw_variable_map_mut_alias_emitted") == 0, "fixture raw alias drift")
require(projection.get("variable_map_mut_selected") == 0, "fixture mutable selection drift")
require(projection.get("replace_owned_map_native_api") == 1, "fixture replace_owned_map drift")
require(projection.get("insert_native_api") == 1, "fixture insert drift")
require(projection.get("remove_native_api") == 1, "fixture remove drift")
require(projection.get("restore_native_api") == 1, "fixture restore drift")

vectors = {row.get("name"): row for row in fixture.get("oracle_vectors", [])}
require(vectors.get("replace_owned_map_overwrites_seed", {}).get("source_after") == [["a", 10], ["b", 20]], "oracle replace_owned_map drift")
require(vectors.get("owned_alias_isolation_after_replace", {}).get("source_after") == [["a", 10], ["b", 20]], "oracle owned alias drift")
require(vectors.get("restore_alias_isolation_after_snapshot", {}).get("source_after_restore") == [["a", 10], ["b", 20], ["e", 50]], "oracle restore drift")

proof = fixture.get("proof") or {}
require(proof.get("source_to_owned_alias") == 0, "fixture source alias drift")
require(proof.get("owned_to_source_alias") == 0, "fixture owned alias drift")
require(proof.get("raw_variable_map_mut_alias_emitted") == 0, "fixture raw alias proof drift")
require(proof.get("variable_map_mut_selected") == 0, "fixture mutable proof drift")
require(proof.get("replace_owned_map_native_api") == 1, "fixture replace_owned_map proof drift")
require(proof.get("insert_native_api") == 1, "fixture insert proof drift")
require(proof.get("remove_native_api") == 1, "fixture remove proof drift")
require(proof.get("restore_native_api") == 1, "fixture restore proof drift")
require(proof.get("deterministic_order_preserved") == 1, "fixture order proof drift")

recovery = fixture.get("candidate_recovery") or {}
require(recovery.get("candidate_pool_state_after_this_card") == "BlockedUntilRouteMatrixRerun", "fixture recovery state drift")
require(recovery.get("next_action") == next_action, "fixture next action drift")

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

require("replace_owned_map(ctx: VariableContextNative, owned_map: OrderedMapBox)" in native_source, "native source missing replace_owned_map")
require("ctx.variable_map = owned_map.clone_owned()" in native_source, "native source replace_owned_map body drift")
require("restore(ctx: VariableContextNative, snapshot: OrderedMapBox)" in native_source, "native source restore signature drift")
require("snapshot(ctx: VariableContextNative): OrderedMapBox" in native_source, "native source snapshot signature drift")
require("variable_map_mut" not in native_source, "native source must not expose variable_map_mut")

require("variable_context_native_explicit_mutation=ok" in native_test, "native test marker drift")
require("replace_owned_map(ctx, owned)" in native_test, "native test missing replace_owned_map coverage")
require("VariableContextNativeApi.restore(ctx, snapshot)" in native_test, "native test missing restore coverage")

allowed_current_tokens = {
    token,
    "MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-002",
    "VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-HAKO-ADOPTION-DECISION-001",
    "SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-EXPLICIT-MUTATION-RESOLUTION-001",
    "MIRBUILDER-VARIABLE-CONTEXT-REFERENCE-PROJECTION-CONTRACT-001",
}
allowed_current_paths = {
    "docs/development/current/main/phases/phase-296x/1791-MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-PROJECTION-001.md",
    "docs/development/current/main/phases/phase-296x/1792-MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-002.md",
    "docs/development/current/main/phases/phase-296x/1793-VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-HAKO-ADOPTION-DECISION-001.md",
    "docs/development/current/main/phases/phase-296x/1794-SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-EXPLICIT-MUTATION-RESOLUTION-001.md",
    "docs/development/current/main/phases/phase-296x/1795-MIRBUILDER-VARIABLE-CONTEXT-REFERENCE-PROJECTION-CONTRACT-001.md",
}
require(state.get("latest_card") in allowed_current_tokens, "current-state latest card drift")
require(state.get("latest_card_path") in allowed_current_paths, "current-state latest card path drift")
require(state.get("current_blocker_token") in allowed_current_tokens, "current-state blocker drift")

for needle in [
    token,
    policy,
    owner_kind,
    next_action,
]:
    require(needle in task_order, f"task-order missing {needle}")

require("tools/checks/rust_lifecycle_variable_context_explicit_mutation_api_projection_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print(f"selected_policy={policy}")
print(f"owner_kind={owner_kind}")
print("replace_owned_map_native_api=1")
print("raw_variable_map_mut_alias_emitted=0")
print("variable_map_mut_selected=0")
print("candidate_pool_state_after_this_card=BlockedUntilRouteMatrixRerun")
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
