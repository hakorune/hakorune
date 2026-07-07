#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipematcher-reject-floor-selection"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-reject-floor-selection-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3247-MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-REJECT-FLOOR-SELECTION-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
PREV_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_return_absent_accepted_floor_gate.sh"
SNAPSHOT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_canonical_loop_facts_input_snapshot.hako"
MATCHER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipematcher_execution_boundary.hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$PREV_GUARD" "$SNAPSHOT_IMPL" "$MATCHER_IMPL"

PREV_OUT="$(HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 guard_cached_run "$TAG-prev" bash "$PREV_GUARD")"
if ! grep -q '^return_absent_accepted_floor=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "return-absent accepted-floor prerequisite is not green"
fi

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$SNAPSHOT_IMPL" "$MATCHER_IMPL" "$PREV_OUT" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, card_path, task_order_path, current_state_path, snapshot_path, matcher_path, prev_out = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")
current_state = Path(current_state_path).read_text(encoding="utf-8")
snapshot_impl = Path(snapshot_path).read_text(encoding="utf-8")
matcher_impl = Path(matcher_path).read_text(encoding="utf-8")

token = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-REJECT-FLOOR-SELECTION-001"
next_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-MISSING-VERIFIED-RECIPE-REJECT-ROW-001"
if fixture.get("kind") != "MirBuilderProgramJsonRecipeMatcherRejectFloorSelectionV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")

axes = fixture.get("reject_floor_axes") or []
selected = [axis for axis in axes if axis.get("selected_first") is True]
if len(selected) != 1 or selected[0].get("axis") != "malformed_or_missing_verified_recipe":
    raise SystemExit("wrong first reject-floor selection")
for required in [
    "unsupported_condition_operator",
    "unsupported_update_operator",
    "update_target_mismatch",
    "malformed_or_missing_verified_recipe",
    "extra_statement_or_swapped_body_order_or_non_null_else",
]:
    if not any(axis.get("axis") == required for axis in axes):
        raise SystemExit(f"missing reject axis: {required}")

selected_next = fixture.get("selected_next") or {}
if selected_next.get("card") != next_card:
    raise SystemExit("wrong selected next card")
if selected_next.get("row_id") != "missing_verified_recipe_reject":
    raise SystemExit("wrong selected row")
if selected_next.get("expected_snapshot", {}).get("reason") != "verified_recipe_missing":
    raise SystemExit("wrong expected snapshot reason")
if selected_next.get("expected_matcher_result", {}).get("reason") != "snapshot_not_ok":
    raise SystemExit("wrong expected matcher reason")

for needle in [
    'return me._err("verified_recipe_missing")',
    'if reason == "verified_recipe_missing" { return 1 }',
    '"matcher_input_present" => 0',
]:
    if needle not in snapshot_impl:
        raise SystemExit(f"snapshot impl missing: {needle}")
for needle in [
    'return me._err("snapshot_not_ok")',
    'if reason == "snapshot_not_ok" { return 2 }',
    '"matched" => 0',
]:
    if needle not in matcher_impl:
        raise SystemExit(f"matcher impl missing: {needle}")

claims = fixture.get("claims") or {}
for key in ["reject_floor_selection", "selected_missing_verified_recipe_first"]:
    if claims.get(key) != 1:
        raise SystemExit(f"missing positive claim: {key}")
for key, value in claims.items():
    if key in {"reject_floor_selection", "selected_missing_verified_recipe_first"}:
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [
    token,
    next_card,
    "reject_row_green = 0",
    "runtime_route_switch = 0",
    "programjson_runtime_route_authority = 0",
    "Source Selfhost remains unclaimed",
]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
for needle in [token, next_card, "malformed_or_missing_verified_recipe"]:
    if needle not in task_order:
        raise SystemExit(f"task-order missing: {needle}")
if f'latest_card = "{token}"' not in current_state:
    raise SystemExit("CURRENT_STATE latest card drift")
for key in [
    "return_absent_accepted_floor",
    "matcher_result_equal",
    "programjson_runtime_route_authority=0",
    "runtime_route_switch=0",
    "source_selfhost_claim=0",
]:
    if key not in prev_out:
        raise SystemExit(f"previous guard output missing: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipematcher-reject-floor-selection-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-REJECT-FLOOR-SELECTION-001
reject_floor_selection=1
selected_first_axis=malformed_or_missing_verified_recipe
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-MISSING-VERIFIED-RECIPE-REJECT-ROW-001
selected_row_id=missing_verified_recipe_reject
expected_snapshot_reason=verified_recipe_missing
expected_matcher_reason=snapshot_not_ok
reject_row_green=0
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
