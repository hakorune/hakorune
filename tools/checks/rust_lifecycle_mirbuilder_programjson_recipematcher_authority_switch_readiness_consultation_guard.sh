#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipematcher-authority-switch-readiness-consultation"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-authority-switch-readiness-consultation-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3251-MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-AUTHORITY-SWITCH-READINESS-CONSULTATION-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
FIELD_FLOOR_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_route_consumed_field_floor_parity_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$FIELD_FLOOR_GUARD"

FIELD_OUT="$(HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 guard_cached_run "$TAG-field-floor" bash "$FIELD_FLOOR_GUARD")"
if ! grep -q '^route_consumed_field_floor_parity_green=1$' <<<"$FIELD_OUT"; then
  printf '%s\n' "$FIELD_OUT" >&2
  guard_fail "$TAG" "route-consumed field-floor parity prerequisite is not green"
fi

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$FIELD_OUT" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, card_path, task_order_path, current_state_path, field_out = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")
current_state = Path(current_state_path).read_text(encoding="utf-8")

token = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-AUTHORITY-SWITCH-READINESS-CONSULTATION-001"
next_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-NESTED-LOOP-DECISION-ROW-CONSULTATION-001"
if fixture.get("kind") != "MirBuilderProgramJsonRecipeMatcherAuthoritySwitchReadinessConsultationV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")

state = fixture.get("readiness_state") or {}
accepted = state.get("accepted_floor") or {}
for axis in [
    "current_return_only_shape",
    "continue_present",
    "break_present",
    "break_and_continue_present",
    "return_absent_decision_row",
]:
    if accepted.get(axis) != "green":
        raise SystemExit(f"accepted floor axis not green: {axis}")
if accepted.get("nested_loop_decision_row") != "decision_required":
    raise SystemExit("nested-loop decision axis must remain decision_required")

reject = state.get("reject_floor") or {}
if reject.get("missing_verified_recipe") != "green":
    raise SystemExit("missing_verified_recipe reject row must be green")
for axis, value in reject.items():
    if axis == "missing_verified_recipe":
        continue
    if value != "pending":
        raise SystemExit(f"reject floor axis should remain pending: {axis}")

field = state.get("field_floor") or {}
if field.get("route_consumed_field_floor_parity") != "green":
    raise SystemExit("field floor parity must be green")

options = {row.get("option"): row for row in fixture.get("candidate_next_steps") or []}
if options.get("A_LIMITED_AUTHORITY_SWITCH_NOW", {}).get("eligible") is not False:
    raise SystemExit("limited authority switch must be ineligible")
selected = options.get("B_NESTED_LOOP_DECISION_ROW_NEXT") or {}
if selected.get("eligible") is not True or selected.get("selected_next_card") != next_card:
    raise SystemExit("nested-loop decision must be selected next")

decision = fixture.get("decision") or {}
if decision.get("kind") != "DeferAuthoritySwitchSelectNestedLoopDecision":
    raise SystemExit("bad decision kind")
if decision.get("selected_next_card") != next_card:
    raise SystemExit("bad selected next card")

claims = fixture.get("claims") or {}
for key in ["authority_switch_readiness_consultation", "selected_nested_loop_decision_next"]:
    if claims.get(key) != 1:
        raise SystemExit(f"missing positive claim: {key}")
for key, value in claims.items():
    if key in {"authority_switch_readiness_consultation", "selected_nested_loop_decision_next"}:
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [
    token,
    next_card,
    "A_LIMITED_AUTHORITY_SWITCH_NOW",
    "authority_switch_ready = 0",
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
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-NESTED-LOOP-DECISION-ROW-CONSULTATION-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-NESTED-LOOP-REJECT-BOUNDARY-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-REJECT-FLOOR-EXPANSION-SELECTION-001",
}
if not any(f'latest_card = "{allowed}"' in current_state for allowed in allowed_latest):
    raise SystemExit("CURRENT_STATE latest card drift")
for key in [
    "route_consumed_field_floor_parity_green=1",
    "programjson_runtime_route_authority=0",
    "runtime_route_switch=0",
    "source_selfhost_claim=0",
]:
    if key not in field_out:
        raise SystemExit(f"field-floor prerequisite missing: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipematcher-authority-switch-readiness-consultation-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-AUTHORITY-SWITCH-READINESS-CONSULTATION-001
authority_switch_readiness_consultation=1
authority_switch_ready=0
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-NESTED-LOOP-DECISION-ROW-CONSULTATION-001
selected_nested_loop_decision_next=1
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
