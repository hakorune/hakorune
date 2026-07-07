#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipebodies-runtime-route-shadow-switch-consultation-002-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-runtime-route-shadow-switch-consultation-002-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3233-MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-ROUTE-SHADOW-SWITCH-CONSULTATION-002.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
EXPANDED_ROWS_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_shadow_parity_expanded_rows_gate.sh"
DUAL_RUN_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_runtime_dual_run_shadow_guard.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$EXPANDED_ROWS_GATE" "$DUAL_RUN_GUARD"

DUAL_OUT="$(guard_cached_run "$TAG" bash "$DUAL_RUN_GUARD")"
if ! grep -q '^dual_run_shadow_guard=1$' <<<"$DUAL_OUT"; then
  printf '%s\n' "$DUAL_OUT" >&2
  guard_fail "$TAG" "dual-run guard prerequisite is not green"
fi

EXPANDED_OUT="$(guard_cached_run "$TAG" bash "$EXPANDED_ROWS_GATE")"
if ! grep -q '^recipe_matcher_shadow_parity_expanded_rows=1$' <<<"$EXPANDED_OUT"; then
  printf '%s\n' "$EXPANDED_OUT" >&2
  guard_fail "$TAG" "expanded RecipeMatcher shadow parity prerequisite is not green"
fi
if ! grep -q '^row_count=4$' <<<"$EXPANDED_OUT"; then
  printf '%s\n' "$EXPANDED_OUT" >&2
  guard_fail "$TAG" "expanded RecipeMatcher shadow parity did not prove four rows"
fi

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$DUAL_OUT" "$EXPANDED_OUT" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, card_path, task_order_path, current_state_path, dual_out, expanded_out = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")
current_state = Path(current_state_path).read_text(encoding="utf-8")

token = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-ROUTE-SHADOW-SWITCH-CONSULTATION-002"
next_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-ROUTE-ADJACENT-SHADOW-GUARD-001"
if fixture.get("kind") != "MirBuilderProgramJsonRecipeBodiesRuntimeRouteShadowSwitchConsultationV2":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")

states = {row.get("id"): row for row in fixture.get("candidate_decisions") or []}
if states.get("B_RUNTIME_ROUTE_ADJACENT_SHADOW_GUARD", {}).get("state") != "Selected":
    raise SystemExit("B must be selected")
if states.get("B_RUNTIME_ROUTE_ADJACENT_SHADOW_GUARD", {}).get("selected_next_card") != next_card:
    raise SystemExit("B selected next card drift")
if states.get("C_LIMITED_PROGRAMJSON_AUTHORITY_SWITCH", {}).get("state") != "RejectedForNow":
    raise SystemExit("C must be rejected for now")
if states.get("D_MORE_COVERAGE_BEFORE_ANY_RUNTIME_ADJACENT_WORK", {}).get("state") != "DeferredAsCoverageFloorBeforeC":
    raise SystemExit("D must be deferred as coverage floor")

contract = fixture.get("selected_contract") or {}
expected = {
    "mode": "runtime_route_adjacent_shadow_only",
    "boundary": "after try_build_outcome(ctx), before registry candidate selection",
    "runtime_authority": "Rust ASTNode route",
    "shadow": "ProgramJSON matcher result",
    "mismatch_policy": "fail_fast",
    "writes_downstream": False,
    "runtime_route_switch": False,
    "programjson_runtime_route_authority": False,
    "recipe_matcher_input_authority": False,
    "route_selection": False,
    "mir_lowering": False,
    "mir_mutation": False,
    "id_allocation": False,
    "runtime_fallback": False,
}
for key, value in expected.items():
    if contract.get(key) != value:
        raise SystemExit(f"selected contract drift: {key}")

forbidden = fixture.get("forbidden_until_later_design_approval") or []
for needle in [
    "write ProgramJSON result into PlanBuildOutcome.recipe_contract",
    "pass ProgramJSON result to route registry or predicates",
    "fallback to Rust on ProgramJSON mismatch",
    "claim ProgramJSON runtime route authority",
    "claim runtime route switch",
]:
    if needle not in forbidden:
        raise SystemExit(f"missing forbidden item: {needle}")

coverage_floor = fixture.get("coverage_floor_before_c") or []
if len(coverage_floor) < 10:
    raise SystemExit("coverage floor before C is too small")

decision = fixture.get("decision") or {}
if decision.get("kind") != "SelectRuntimeRouteAdjacentShadowGuard":
    raise SystemExit("bad decision kind")
if decision.get("selected_next_card") != next_card:
    raise SystemExit("decision selected next card drift")

claims = fixture.get("claims") or {}
for key in [
    "consultation_decision_recorded",
    "selected_b_runtime_route_adjacent_shadow_guard",
    "runtime_authority_remains_rust_astnode",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"missing positive claim: {key}")
for key, value in claims.items():
    if key in {
        "consultation_decision_recorded",
        "selected_b_runtime_route_adjacent_shadow_guard",
        "runtime_authority_remains_rust_astnode",
    }:
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

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
    if f"{key}=0" not in dual_out or f"{key}=0" not in expanded_out:
        raise SystemExit(f"prerequisite output missing forbidden zero: {key}")

for needle in [
    token,
    "SELECT_B_RUNTIME_ROUTE_ADJACENT_SHADOW_GUARD",
    next_card,
    "programjson_runtime_route_authority=0",
    "runtime_route_switch=0",
]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
for needle in [token, next_card, "CONSULTATION_REQUIRED"]:
    if needle not in task_order:
        raise SystemExit(f"task-order missing: {needle}")
if f'latest_card = "{token}"' not in current_state:
    raise SystemExit("CURRENT_STATE latest card drift")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipebodies-runtime-route-shadow-switch-consultation-002-guard-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-ROUTE-SHADOW-SWITCH-CONSULTATION-002
consultation_decision_recorded=1
selected_b_runtime_route_adjacent_shadow_guard=1
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-ROUTE-ADJACENT-SHADOW-GUARD-001
runtime_authority=rust_astnode
boundary=after_try_build_outcome_before_route_candidate_selection
programjson_runtime_route_authority=0
runtime_route_switch=0
recipe_matcher_input_authority=0
limited_programjson_authority_switch=0
full_recipe_matcher_execution=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_fallback=0
source_selfhost_claim=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
