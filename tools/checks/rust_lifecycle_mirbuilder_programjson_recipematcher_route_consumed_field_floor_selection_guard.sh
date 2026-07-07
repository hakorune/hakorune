#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipematcher-route-consumed-field-floor-selection"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-route-consumed-field-floor-selection-v0.json"
AUTHORITY_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-authority-switch-coverage-floor-selection-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3249-MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ROUTE-CONSUMED-FIELD-FLOOR-SELECTION-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
PREV_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_missing_verified_recipe_reject_row_gate.sh"
SNAPSHOT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_canonical_loop_facts_input_snapshot.hako"
MATCHER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipematcher_execution_boundary.hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$AUTHORITY_FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$PREV_GUARD" "$SNAPSHOT_IMPL" "$MATCHER_IMPL"

PREV_OUT="$(HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 guard_cached_run "$TAG-prev" bash "$PREV_GUARD")"
if ! grep -q '^reject_floor_row_green=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "reject-floor prerequisite is not green"
fi

python3 - "$FIXTURE" "$AUTHORITY_FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$SNAPSHOT_IMPL" "$MATCHER_IMPL" "$PREV_OUT" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, authority_path, card_path, task_order_path, current_state_path, snapshot_path, matcher_path, prev_out = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
authority = json.loads(Path(authority_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")
current_state = Path(current_state_path).read_text(encoding="utf-8")
snapshot_impl = Path(snapshot_path).read_text(encoding="utf-8")
matcher_impl = Path(matcher_path).read_text(encoding="utf-8")

token = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ROUTE-CONSUMED-FIELD-FLOOR-SELECTION-001"
next_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ROUTE-CONSUMED-FIELD-FLOOR-PARITY-001"
if fixture.get("kind") != "MirBuilderProgramJsonRecipeMatcherRouteConsumedFieldFloorSelectionV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")

selected = fixture.get("selected_field_floor") or {}
authority_floor = authority.get("field_floor") or {}
if selected.get("hard_authority_candidate_fields") != authority_floor.get("hard_authority_candidate_fields"):
    raise SystemExit("hard field floor drift")
if selected.get("route_adjacent_facts_floor") != authority_floor.get("route_adjacent_facts_floor"):
    raise SystemExit("route-adjacent field floor drift")

snapshot_fields = {
    "has_nested_loop": "has_nested_loop=",
    "loop_cond_return_in_body_present": "loop_cond_return_in_body_present=",
    "cond_kind": "cond_kind=",
    "loop_var": "loop_var=",
    "loop_bound_int": "loop_bound_int=",
    "update_kind": "update_kind=",
    "update_target": "update_target=",
    "step_int": "step_int=",
}
for field, needle in snapshot_fields.items():
    if needle not in snapshot_impl:
        raise SystemExit(f"snapshot summary missing field: {field}")
matcher_fields = {
    "ok": ";ok=",
    "matched": ";matched=",
    "contract_kind": ";contract_kind=",
    "has_break": ";has_break=",
    "has_continue": ";has_continue=",
    "has_return": ";has_return=",
}
for field, needle in matcher_fields.items():
    if needle not in matcher_impl:
        raise SystemExit(f"matcher summary missing field: {field}")
if '"reason_code" => me._reason_code(reason)' not in matcher_impl:
    raise SystemExit("matcher result map missing reason_code")

selected_next = fixture.get("selected_next") or {}
if selected_next.get("card") != next_card:
    raise SystemExit("wrong selected next card")

claims = fixture.get("claims") or {}
if claims.get("route_consumed_field_floor_selection") != 1:
    raise SystemExit("missing positive claim")
for key, value in claims.items():
    if key == "route_consumed_field_floor_selection":
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [
    token,
    next_card,
    "field_floor_parity_green = 0",
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
for key in [
    "reject_floor_row_green=1",
    "matched=0",
    "programjson_runtime_route_authority=0",
    "runtime_route_switch=0",
    "source_selfhost_claim=0",
]:
    if key not in prev_out:
        raise SystemExit(f"previous guard output missing: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipematcher-route-consumed-field-floor-selection-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ROUTE-CONSUMED-FIELD-FLOOR-SELECTION-001
route_consumed_field_floor_selection=1
hard_authority_candidate_fields=ok,reason_code,matched,contract_kind,has_break,has_continue,has_return
route_adjacent_facts_floor=has_nested_loop,loop_cond_return_in_body_present,cond_kind,loop_var,loop_bound_int,update_kind,update_target,step_int
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ROUTE-CONSUMED-FIELD-FLOOR-PARITY-001
field_floor_parity_green=0
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
