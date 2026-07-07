#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipematcher-accepted-floor-matrix"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-accepted-floor-matrix-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3237-MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ACCEPTED-FLOOR-MATRIX-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
COVERAGE_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_authority_switch_coverage_floor_selection_guard.sh"
EXPANDED_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_shadow_parity_expanded_rows_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$COVERAGE_GUARD" "$EXPANDED_GUARD"

COVERAGE_OUT="$(HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 guard_cached_run "$TAG-coverage" bash "$COVERAGE_GUARD")"
if ! grep -q '^coverage_floor_selection=1$' <<<"$COVERAGE_OUT"; then
  printf '%s\n' "$COVERAGE_OUT" >&2
  guard_fail "$TAG" "coverage floor selection prerequisite is not green"
fi

EXPANDED_OUT="$(HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 guard_cached_run "$TAG-expanded" bash "$EXPANDED_GUARD")"
if ! grep -q '^row_count=4$' <<<"$EXPANDED_OUT"; then
  printf '%s\n' "$EXPANDED_OUT" >&2
  guard_fail "$TAG" "expanded shadow parity prerequisite does not report 4 rows"
fi

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$COVERAGE_OUT" "$EXPANDED_OUT" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, card_path, task_order_path, current_state_path, coverage_out, expanded_out = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")
current_state = Path(current_state_path).read_text(encoding="utf-8")

token = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ACCEPTED-FLOOR-MATRIX-001"
next_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-CONTINUE-PRESENT-VERIFIED-RECIPE-SUPPORT-001"
if fixture.get("kind") != "MirBuilderProgramJsonRecipeMatcherAcceptedFloorMatrixV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")
if fixture.get("input_state", {}).get("current_green_row_count") != 4:
    raise SystemExit("current green row count drift")

matrix = {row.get("axis"): row for row in fixture.get("accepted_floor_matrix", [])}
expected_status = {
    "current_return_only_shape": "green",
    "continue_present": "blocked_verified_recipe_missing",
    "break_present": "blocked_verified_recipe_missing",
    "break_and_continue_present": "blocked_on_break_and_continue_verified_recipe_support",
    "return_absent_decision_row": "decision_required",
    "nested_loop_decision_row": "decision_required",
}
for axis, status in expected_status.items():
    row = matrix.get(axis)
    if row is None:
        raise SystemExit(f"missing matrix axis: {axis}")
    if row.get("status") != status:
        raise SystemExit(f"bad status for {axis}: {row.get('status')}")

selected = fixture.get("selected_next") or {}
if selected.get("selected_next_card") != next_card:
    raise SystemExit("bad selected next card")
if selected.get("box_count_slice") is not True:
    raise SystemExit("selected next must be BoxCount slice")
if selected.get("single_axis_only") != "continue_present":
    raise SystemExit("selected next must be continue_present only")

claims = fixture.get("claims") or {}
if claims.get("accepted_floor_matrix") != 1:
    raise SystemExit("missing accepted floor matrix claim")
if claims.get("current_green_return_only_rows") != 4:
    raise SystemExit("bad green return-only row claim")
for key in [
    "continue_present_green",
    "break_present_green",
    "break_and_continue_present_green",
    "return_absent_decision_green",
    "nested_loop_decision_green",
    "programjson_runtime_route_authority",
    "runtime_route_switch",
    "recipe_matcher_input_authority",
    "route_selection",
    "mir_lowering",
    "mir_mutation",
    "id_allocation",
    "runtime_fallback",
    "source_selfhost_claim",
    "new_backend_route",
    "new_abi",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [
    token,
    next_card,
    "continue_present:",
    "break_present:",
    "runtime_route_switch = 0",
    "programjson_runtime_route_authority = 0",
    "Source Selfhost remains unclaimed",
]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
for needle in [token, next_card]:
    if needle not in task_order:
        raise SystemExit(f"task-order missing: {needle}")
if f'latest_card = "{token}"' not in current_state:
    raise SystemExit("CURRENT_STATE latest card drift")
for key in ["coverage_floor_selection=1", "selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ACCEPTED-FLOOR-MATRIX-001"]:
    if key not in coverage_out:
        raise SystemExit(f"coverage prerequisite missing: {key}")
for key in ["row_count=4", "programjson_runtime_route_authority=0", "runtime_route_switch=0"]:
    if key not in expanded_out:
        raise SystemExit(f"expanded prerequisite missing: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipematcher-accepted-floor-matrix-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ACCEPTED-FLOOR-MATRIX-001
accepted_floor_matrix=1
current_green_return_only_rows=4
continue_present_status=blocked_verified_recipe_missing
break_present_status=blocked_verified_recipe_missing
break_and_continue_present_status=blocked_on_break_and_continue_verified_recipe_support
return_absent_decision_status=decision_required
nested_loop_decision_status=decision_required
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-CONTINUE-PRESENT-VERIFIED-RECIPE-SUPPORT-001
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
