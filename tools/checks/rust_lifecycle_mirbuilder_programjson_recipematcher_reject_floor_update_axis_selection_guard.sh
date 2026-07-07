#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipematcher-reject-floor-update-axis-selection"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-reject-floor-update-axis-selection-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3256-MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-REJECT-FLOOR-UPDATE-AXIS-SELECTION-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
PREV_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_unsupported_condition_operator_reject_row_gate.sh"
SNAPSHOT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_canonical_loop_facts_input_snapshot.hako"
LOOP_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/loop_stmt_handler.hako"
MATCHER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipematcher_execution_boundary.hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$PREV_GUARD" "$SNAPSHOT_IMPL" "$LOOP_HANDLER" "$MATCHER_IMPL"

PREV_OUT="$(HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 guard_cached_run "$TAG-prev" bash "$PREV_GUARD")"
if ! grep -q '^unsupported_condition_operator_reject_row_green=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "unsupported-condition reject prerequisite is not green"
fi

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$SNAPSHOT_IMPL" "$LOOP_HANDLER" "$MATCHER_IMPL" "$PREV_OUT" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, card_path, task_order_path, current_state_path, snapshot_path, loop_handler_path, matcher_path, prev_out = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")
current_state = Path(current_state_path).read_text(encoding="utf-8")
snapshot_impl = Path(snapshot_path).read_text(encoding="utf-8")
loop_handler = Path(loop_handler_path).read_text(encoding="utf-8")
matcher_impl = Path(matcher_path).read_text(encoding="utf-8")

token = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-REJECT-FLOOR-UPDATE-AXIS-SELECTION-001"
next_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-UNSUPPORTED-UPDATE-OPERATOR-REJECT-ROW-001"
if fixture.get("kind") != "MirBuilderProgramJsonRecipeMatcherRejectFloorUpdateAxisSelectionV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")

axes = fixture.get("candidate_axes") or []
selected = [axis for axis in axes if axis.get("selected_next") is True]
if len(selected) != 1 or selected[0].get("axis") != "unsupported_update_operator":
    raise SystemExit("wrong update-axis selection")
for required in [
    "unsupported_update_operator",
    "update_target_mismatch",
    "extra_statement_or_swapped_body_order_or_non_null_else",
]:
    if not any(axis.get("axis") == required for axis in axes):
        raise SystemExit(f"missing candidate axis: {required}")

selected_next = fixture.get("selected_next") or {}
if selected_next.get("card") != next_card:
    raise SystemExit("wrong selected next card")
if selected_next.get("row_id") != "unsupported_update_operator_reject":
    raise SystemExit("wrong selected row")
shape_delta = selected_next.get("programjson_shape_delta") or {}
if shape_delta.get("loop_update_operator") != "-":
    raise SystemExit("wrong update operator delta")
if shape_delta.get("expected_snapshot_reason") != "unsupported_loop_update":
    raise SystemExit("wrong expected snapshot reason")
if shape_delta.get("expected_matcher_reason") != "snapshot_not_ok":
    raise SystemExit("wrong expected matcher reason")

for needle in [
    'if me._i(update_snap, "ok") != 1 { return me._err("unsupported_loop_update") }',
    'if reason == "unsupported_loop_update" { return 15 }',
]:
    if needle not in snapshot_impl:
        raise SystemExit(f"snapshot impl missing: {needle}")
for needle in [
    'if BoxHelpers.same_token(op, "+") != 1',
    'Loop body assignment Binary op must be',
]:
    if needle not in loop_handler:
        raise SystemExit(f"loop handler missing current update boundary marker: {needle}")
for needle in [
    'return me._err("snapshot_not_ok")',
    '"matched" => 0',
]:
    if needle not in matcher_impl:
        raise SystemExit(f"matcher impl missing: {needle}")

claims = fixture.get("claims") or {}
for key in ["reject_floor_update_axis_selection", "selected_unsupported_update_operator_next"]:
    if claims.get(key) != 1:
        raise SystemExit(f"missing positive claim: {key}")
for key, value in claims.items():
    if key in {"reject_floor_update_axis_selection", "selected_unsupported_update_operator_next"}:
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [
    token,
    next_card,
    "unsupported_loop_update",
    "snapshot_not_ok",
    "unsupported_update_operator_reject_row_green = 0",
    "runtime_route_switch = 0",
    "programjson_runtime_route_authority = 0",
    "Source Selfhost remains unclaimed",
]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
for needle in [token, next_card, "unsupported_update_operator"]:
    if needle not in task_order:
        raise SystemExit(f"task-order missing: {needle}")
if f'latest_card = "{token}"' not in current_state:
    raise SystemExit("CURRENT_STATE latest card drift")
for key in [
    "unsupported_condition_operator_reject_row_green=1",
    "snapshot_reason=unsupported_loop_cond",
    "programjson_runtime_route_authority=0",
    "runtime_route_switch=0",
    "source_selfhost_claim=0",
]:
    if key not in prev_out:
        raise SystemExit(f"previous guard output missing: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipematcher-reject-floor-update-axis-selection-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-REJECT-FLOOR-UPDATE-AXIS-SELECTION-001
reject_floor_update_axis_selection=1
selected_next_axis=unsupported_update_operator
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-UNSUPPORTED-UPDATE-OPERATOR-REJECT-ROW-001
selected_row_id=unsupported_update_operator_reject
expected_snapshot_reason=unsupported_loop_update
expected_matcher_reason=snapshot_not_ok
unsupported_update_operator_reject_row_green=0
programjson_runtime_route_authority=0
runtime_route_switch=0
recipe_matcher_input_authority=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_fallback=0
source_selfhost_claim=0
summary=ok
REPORT
