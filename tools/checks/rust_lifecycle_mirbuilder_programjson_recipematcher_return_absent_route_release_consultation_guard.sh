#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipematcher-return-absent-route-release-consultation"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-return-absent-route-release-consultation-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3243-MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-ROUTE-RELEASE-CONSULTATION-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
PREV_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_return_absent_decision_row_guard.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$PREV_GUARD"

PREV_OUT="$(HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 guard_cached_run "$TAG-prev" bash "$PREV_GUARD")"
if ! grep -q '^selected_next_card=CONSULTATION_REQUIRED$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "return-absent decision prerequisite did not stop for consultation"
fi

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$PREV_OUT" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, card_path, task_order_path, current_state_path, prev_out = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")
current_state = Path(current_state_path).read_text(encoding="utf-8")

token = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-ROUTE-RELEASE-CONSULTATION-001"
next_token = "MIRBUILDER-PROGRAMJSON-LOOP-BODY-RETURN-ABSENT-SCAN-ONLY-DIAGNOSTIC-001"
if fixture.get("kind") != "MirBuilderProgramJsonRecipeMatcherReturnAbsentRouteReleaseConsultationV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")

state = fixture.get("input_state") or {}
if state.get("consultation_result") != "B_DEFER_RETURN_ABSENT_TO_ROUTE_RELEASE_CONSULTATION":
    raise SystemExit("consultation result drift")
if state.get("return_absent_intersects_route_release_gating") is not True:
    raise SystemExit("route-release intersection must be true")
if state.get("final_top_level_return_is_not_loop_body_return_evidence") is not True:
    raise SystemExit("final return separation must be true")

decision = fixture.get("decision") or {}
if decision.get("kind") != "SelectDeferReturnAbsentAcceptedFloor":
    raise SystemExit("bad decision kind")
if decision.get("selected_option") != "B_DEFER_RETURN_ABSENT_TO_ROUTE_RELEASE_CONSULTATION":
    raise SystemExit("wrong selected option")
if decision.get("selected_next_card") != next_token:
    raise SystemExit("wrong selected next card")

sequence = fixture.get("next_sequence") or []
if [row.get("card") for row in sequence] != [
    next_token,
    "MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-FINAL-TOPLEVEL-RETURN-DECOUPLE-SNAPSHOT-BOUNDARY-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-ACCEPTED-FLOOR-001",
]:
    raise SystemExit("next sequence drift")

diag = fixture.get("scan_only_diagnostic_contract") or {}
required_diag = {
    "loop_body_has_break": 1,
    "loop_body_has_continue": 1,
    "loop_body_has_return": 0,
    "final_top_level_return_present": 1,
    "final_top_level_return_used_for_loop_body_has_return": 0,
    "matcher_result_equal": 0,
    "accepted_floor": 0,
}
for key, expected in required_diag.items():
    if diag.get(key) != expected:
        raise SystemExit(f"diagnostic contract drift: {key}")

claims = fixture.get("claims") or {}
for key in [
    "route_release_consultation_resolved",
    "selected_b_defer_return_absent",
    "return_absent_scan_only_diagnostic_selected_next",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"missing positive claim: {key}")
for key, value in claims.items():
    if key in {
        "route_release_consultation_resolved",
        "selected_b_defer_return_absent",
        "return_absent_scan_only_diagnostic_selected_next",
    }:
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [
    token,
    "B_DEFER_RETURN_ABSENT_TO_ROUTE_RELEASE_CONSULTATION",
    next_token,
    "return_absent_accepted_floor = 0",
    "runtime_route_switch = 0",
    "programjson_runtime_route_authority = 0",
    "Source Selfhost remains unclaimed",
]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
for needle in [token, next_token, "return_absent_scan_only_diagnostic"]:
    if needle not in task_order:
        raise SystemExit(f"task-order missing: {needle}")
allowed_latest = {
    token,
    next_token,
    "MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-FINAL-TOPLEVEL-RETURN-DECOUPLE-SNAPSHOT-BOUNDARY-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-ACCEPTED-FLOOR-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-REJECT-FLOOR-EXPANSION-SELECTION-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-UNSUPPORTED-CONDITION-OPERATOR-REJECT-ROW-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-REJECT-FLOOR-UPDATE-AXIS-SELECTION-001",
}
if not any(f'latest_card = "{allowed}"' in current_state for allowed in allowed_latest):
    raise SystemExit("CURRENT_STATE latest card drift")
for needle in [
    "return_absent_green=0",
    "return_absent_accepted_floor=0",
    "runtime_route_switch=0",
]:
    if needle not in prev_out:
        raise SystemExit(f"previous guard output missing: {needle}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipematcher-return-absent-route-release-consultation-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-ROUTE-RELEASE-CONSULTATION-001
route_release_consultation_resolved=1
selected_b_defer_return_absent=1
selected_next_card=MIRBUILDER-PROGRAMJSON-LOOP-BODY-RETURN-ABSENT-SCAN-ONLY-DIAGNOSTIC-001
return_absent_scan_only_diagnostic_selected_next=1
return_absent_green=0
return_absent_accepted_floor=0
matcher_result_equal=0
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
