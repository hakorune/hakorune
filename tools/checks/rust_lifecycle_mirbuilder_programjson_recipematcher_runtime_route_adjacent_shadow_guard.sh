#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipematcher-runtime-route-adjacent-shadow-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-runtime-route-adjacent-shadow-guard-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3234-MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-ROUTE-ADJACENT-SHADOW-GUARD-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
ROUTER="$ROOT_DIR/src/mir/builder/control_flow/joinir/route_entry/router.rs"
GUARD_IMPL="$ROOT_DIR/src/mir/builder/control_flow/joinir/route_entry/runtime_adjacent_shadow_guard.rs"
CONSULTATION_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_runtime_route_shadow_switch_consultation_002_guard.sh"
EXPANDED_ROWS_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_shadow_parity_expanded_rows_gate.sh"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$ROUTER" "$GUARD_IMPL" "$CONSULTATION_GUARD" "$EXPANDED_ROWS_GATE"

CONSULTATION_OUT="$(guard_cached_run "$TAG" bash "$CONSULTATION_GUARD")"
if ! grep -q '^selected_b_runtime_route_adjacent_shadow_guard=1$' <<<"$CONSULTATION_OUT"; then
  printf '%s\n' "$CONSULTATION_OUT" >&2
  guard_fail "$TAG" "consultation guard did not select runtime-adjacent shadow guard"
fi

EXPANDED_OUT="$(guard_cached_run "$TAG" bash "$EXPANDED_ROWS_GATE")"
if ! grep -q '^recipe_matcher_shadow_parity_expanded_rows=1$' <<<"$EXPANDED_OUT"; then
  printf '%s\n' "$EXPANDED_OUT" >&2
  guard_fail "$TAG" "expanded rows prerequisite is not green"
fi

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$ROUTER" "$GUARD_IMPL" "$CONSULTATION_OUT" "$EXPANDED_OUT" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, card_path, task_order_path, current_state_path, router_path, guard_path, consultation_out, expanded_out = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")
current_state = Path(current_state_path).read_text(encoding="utf-8")
router = Path(router_path).read_text(encoding="utf-8")
guard_impl = Path(guard_path).read_text(encoding="utf-8")

token = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-ROUTE-ADJACENT-SHADOW-GUARD-001"
next_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-AUTHORITY-SWITCH-COVERAGE-FLOOR-SELECTION-001"
if fixture.get("kind") != "MirBuilderProgramJsonRecipeMatcherRuntimeRouteAdjacentShadowGuardV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")

contract = fixture.get("runtime_adjacent_contract") or {}
expected = {
    "mode": "runtime_route_adjacent_shadow_only",
    "boundary": "after try_build_outcome(ctx), before registry candidate selection",
    "runtime_authority": "Rust ASTNode route",
    "mismatch_policy": "fail_fast_gate_only",
    "writes_downstream": False,
    "programjson_input_in_loop_route_context": False,
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
        raise SystemExit(f"runtime-adjacent contract drift: {key}")

order = fixture.get("static_order_contract") or {}
for key in ["must_appear_after", "must_call", "must_appear_before_any"]:
    if key not in order:
        raise SystemExit(f"missing static order key: {key}")

after = router.find(order["must_appear_after"])
call = router.find(order["must_call"])
if after < 0 or call < 0 or not (after < call):
    raise SystemExit("shadow guard is not after try_build_outcome")
for forbidden_after in order["must_appear_before_any"]:
    pos = router.find(forbidden_after)
    if pos < 0:
        raise SystemExit(f"missing downstream marker: {forbidden_after}")
    if not (call < pos):
        raise SystemExit(f"shadow guard is not before {forbidden_after}")

for needle in [
    "observe_after_try_build_outcome_before_registry",
    "outcome: &PlanBuildOutcome",
    "runtime_authority_is_rust_astnode: true",
    "programjson_runtime_route_authority: false",
    "runtime_route_switch: false",
    "recipe_matcher_input_authority: false",
    "writes_downstream: false",
    "runtime_fallback: false",
]:
    if needle not in guard_impl:
        raise SystemExit(f"guard implementation missing token: {needle}")
for forbidden in [
    "outcome.recipe_contract =",
    "registry::",
    "PlanLowerer",
    "CorePlan",
    "MirBuilder",
    "IdAllocator",
    "runtime_route_switch: true",
    "programjson_runtime_route_authority: true",
    "runtime_fallback: true",
]:
    if forbidden in guard_impl:
        raise SystemExit(f"forbidden guard implementation token: {forbidden}")

report = fixture.get("report_contract") or {}
for key, value in report.items():
    if key in {
        "runtime_authority_is_rust_astnode",
        "boundary_after_try_build_outcome",
        "boundary_before_route_candidate_selection",
    }:
        if value is not True:
            raise SystemExit(f"report positive drift: {key}")
    else:
        if value is not False:
            raise SystemExit(f"report forbidden drift: {key}")

claims = fixture.get("claims") or {}
for key in [
    "runtime_route_adjacent_shadow_guard",
    "boundary_after_try_build_outcome_before_route_candidate_selection",
    "runtime_authority_remains_rust_astnode",
    "programjson_shadow_checked_by_lifecycle_gate",
    "no_downstream_write",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"missing positive claim: {key}")
for key, value in claims.items():
    if key in {
        "runtime_route_adjacent_shadow_guard",
        "boundary_after_try_build_outcome_before_route_candidate_selection",
        "runtime_authority_remains_rust_astnode",
        "programjson_shadow_checked_by_lifecycle_gate",
        "no_downstream_write",
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
    if f"{key}=0" not in consultation_out or f"{key}=0" not in expanded_out:
        raise SystemExit(f"prerequisite output missing forbidden zero: {key}")

for needle in [
    token,
    "programjson_runtime_route_authority=0",
    "runtime_route_switch=0",
    "recipe_matcher_input_authority=0",
    next_card,
]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
for needle in [token, next_card]:
    if needle not in task_order:
        raise SystemExit(f"task-order missing: {needle}")
allowed_latest = [
    f'latest_card = "{token}"',
    'latest_card = "GUARD-CACHE-EMIT-EXE-AND-DIRTY-MEMO-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-AUTHORITY-SWITCH-COVERAGE-FLOOR-SELECTION-001"',
]
if not any(needle in current_state for needle in allowed_latest):
    raise SystemExit("CURRENT_STATE latest card drift")
PY

cargo check --quiet

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipematcher-runtime-route-adjacent-shadow-guard-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-ROUTE-ADJACENT-SHADOW-GUARD-001
runtime_route_adjacent_shadow_guard=1
boundary_after_try_build_outcome_before_route_candidate_selection=1
runtime_authority=rust_astnode
programjson_shadow_checked_by_lifecycle_gate=1
no_downstream_write=1
programjson_input_in_loop_route_context=0
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
