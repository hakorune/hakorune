#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipematcher-authority-switch-coverage-floor-selection"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-authority-switch-coverage-floor-selection-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3236-MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-AUTHORITY-SWITCH-COVERAGE-FLOOR-SELECTION-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
RUNTIME_ADJACENT_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_runtime_route_adjacent_shadow_guard.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$RUNTIME_ADJACENT_GUARD"

RUNTIME_OUT="$(HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 guard_cached_run "$TAG" bash "$RUNTIME_ADJACENT_GUARD")"
if ! grep -q '^runtime_route_adjacent_shadow_guard=1$' <<<"$RUNTIME_OUT"; then
  printf '%s\n' "$RUNTIME_OUT" >&2
  guard_fail "$TAG" "runtime-adjacent shadow guard prerequisite is not green"
fi

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$RUNTIME_OUT" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, card_path, task_order_path, current_state_path, runtime_out = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")
current_state = Path(current_state_path).read_text(encoding="utf-8")

token = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-AUTHORITY-SWITCH-COVERAGE-FLOOR-SELECTION-001"
next_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ACCEPTED-FLOOR-MATRIX-001"
if fixture.get("kind") != "MirBuilderProgramJsonRecipeMatcherAuthoritySwitchCoverageFloorSelectionV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")

selection = fixture.get("coverage_floor_selection") or {}
if selection.get("selected_kind") != "accepted_matrix_then_reject_floor":
    raise SystemExit("bad selected floor kind")
if selection.get("accepted_floor_card") != next_card:
    raise SystemExit("bad accepted floor card")
if selection.get("reject_floor_card") != "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-REJECT-FLOOR-001":
    raise SystemExit("bad reject floor card")

accepted = fixture.get("accepted_floor_required_axes") or []
for axis in [
    "current_return_only_shape",
    "break_present",
    "continue_present",
    "break_and_continue_present",
    "return_absent_decision_row",
    "nested_loop_decision_row",
]:
    if axis not in accepted:
        raise SystemExit(f"missing accepted floor axis: {axis}")

reject = fixture.get("reject_floor_required_axes") or []
for axis in [
    "unsupported_condition_operator",
    "unsupported_condition_shape",
    "unsupported_update_operator",
    "update_target_mismatch",
    "unsupported_variable_name",
    "malformed_verified_recipe",
    "missing_verified_recipe",
    "extra_statement",
    "swapped_body_order",
    "non_null_else_branch",
    "no_final_return",
    "no_in_body_return",
]:
    if axis not in reject:
        raise SystemExit(f"missing reject floor axis: {axis}")

fields = fixture.get("field_floor") or {}
for field in ["ok", "reason_code", "matched", "contract_kind", "has_break", "has_continue", "has_return"]:
    if field not in fields.get("hard_authority_candidate_fields", []):
        raise SystemExit(f"missing authority candidate field: {field}")
for field in [
    "has_nested_loop",
    "loop_cond_return_in_body_present",
    "cond_kind",
    "loop_var",
    "loop_bound_int",
    "update_kind",
    "update_target",
    "step_int",
]:
    if field not in fields.get("route_adjacent_facts_floor", []):
        raise SystemExit(f"missing route-adjacent field: {field}")

switch = fixture.get("switch_preconditions") or {}
for key in ["accepted_floor_green", "reject_floor_green", "route_consumed_fields_named"]:
    if switch.get(key) is not True:
        raise SystemExit(f"missing switch precondition: {key}")
for key in ["programjson_may_write_plan_build_outcome_recipe_contract", "runtime_route_switch", "programjson_runtime_route_authority"]:
    if switch.get(key) is not False:
        raise SystemExit(f"forbidden switch precondition drift: {key}")

claims = fixture.get("claims") or {}
for key in ["coverage_floor_selection", "accepted_floor_required", "reject_floor_required", "field_floor_required"]:
    if claims.get(key) != 1:
        raise SystemExit(f"missing positive claim: {key}")
for key, value in claims.items():
    if key in {"coverage_floor_selection", "accepted_floor_required", "reject_floor_required", "field_floor_required"}:
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [
    token,
    next_card,
    "programjson_runtime_route_authority = 0",
    "runtime_route_switch = 0",
    "Source Selfhost remains unclaimed",
]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
for needle in [token, next_card]:
    if needle not in task_order:
        raise SystemExit(f"task-order missing: {needle}")
allowed_latest = {
    token,
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ACCEPTED-FLOOR-MATRIX-001",
}
if not any(f'latest_card = "{allowed}"' in current_state for allowed in allowed_latest):
    raise SystemExit("CURRENT_STATE latest card drift")
for key in [
    "programjson_runtime_route_authority",
    "runtime_route_switch",
    "recipe_matcher_input_authority",
    "route_selection",
    "mir_lowering",
    "mir_mutation",
    "id_allocation",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    if f"{key}=0" not in runtime_out:
        raise SystemExit(f"runtime prerequisite missing zero claim: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipematcher-authority-switch-coverage-floor-selection-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-AUTHORITY-SWITCH-COVERAGE-FLOOR-SELECTION-001
coverage_floor_selection=1
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ACCEPTED-FLOOR-MATRIX-001
accepted_floor_required=1
reject_floor_required=1
field_floor_required=1
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
