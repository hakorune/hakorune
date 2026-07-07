#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-var-rhs-producer-closeout-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-var-rhs-producer-closeout-v0.json"
IF_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_if_cond_recipe_var_rhs_bound_row_gate.sh"
NESTED_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_loop_nested_if_var_rhs_bound_row_gate.sh"
LOOP_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_top_level_loop_var_rhs_bound_row_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$IF_GATE" "$NESTED_GATE" "$LOOP_GATE"

IF_OUT="$(guard_cached_run "$TAG" bash "$IF_GATE")"
NESTED_OUT="$(guard_cached_run "$TAG" bash "$NESTED_GATE")"
LOOP_OUT="$(guard_cached_run "$TAG" bash "$LOOP_GATE")"

if ! grep -q '^if_cond_recipe_var_rhs_bound_row=1$' <<<"$IF_OUT"; then
  printf '%s\n' "$IF_OUT" >&2
  guard_fail "$TAG" "top-level If Var rhs row is not green"
fi
if ! grep -q '^loop_nested_if_var_rhs_bound_row=1$' <<<"$NESTED_OUT"; then
  printf '%s\n' "$NESTED_OUT" >&2
  guard_fail "$TAG" "Loop nested If Var rhs row is not green"
fi
if ! grep -q '^top_level_loop_var_rhs_bound_row=1$' <<<"$LOOP_OUT"; then
  printf '%s\n' "$LOOP_OUT" >&2
  guard_fail "$TAG" "top-level Loop Var rhs row is not green"
fi

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderProgramJsonVarRhsProducerCloseoutV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-PROGRAMJSON-VAR-RHS-PRODUCER-CLOSEOUT-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-PROGRAMJSON-TOP-LEVEL-LOOP-VAR-RHS-BOUND-ROW-001", "bad prerequisite")
need(len(fixture.get("covered_rows") or []) == 3, "covered row count drift")

contract = fixture.get("closeout_contract") or {}
need(contract.get("var_rhs_producer_surface_closed") is True, "surface must close")
need(contract.get("guard_route") == "owner-direct AOT rows only", "bad guard route")
need(contract.get("full_phase_state_dispatcher_authority") is False, "dispatcher authority must stay false")

claims = fixture.get("claims") or {}
for key in [
    "var_rhs_producer_closeout",
    "var_rhs_producer_surface_closed",
    "top_level_if_var_rhs_row_green",
    "loop_nested_if_var_rhs_row_green",
    "top_level_loop_var_rhs_row_green",
    "owner_direct_observe_only",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "full_phase_state_dispatcher_authority",
    "legacy_loop_dto_lowering_updated",
    "length_bound_producer_selected",
    "reversed_var_var_context_aware",
    "bool_recipe_lowering_executed",
    "mir_cmp_emission",
    "branch_emission",
    "basic_block_mutation",
    "value_id_allocation",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")
need(fixture.get("decision", {}).get("selected_next_card") == "MIRBUILDER-COMPARE-LOWERING-MUTATION-OWNER-SELECTION-001", "bad selected next")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-var-rhs-producer-closeout-guard-v0
token=MIRBUILDER-PROGRAMJSON-VAR-RHS-PRODUCER-CLOSEOUT-001
var_rhs_producer_closeout=1
var_rhs_producer_surface_closed=1
top_level_if_var_rhs_row_green=1
loop_nested_if_var_rhs_row_green=1
top_level_loop_var_rhs_row_green=1
owner_direct_observe_only=1
full_phase_state_dispatcher_authority=0
legacy_loop_dto_lowering_updated=0
length_bound_producer_selected=0
reversed_var_var_context_aware=0
bool_recipe_lowering_executed=0
mir_cmp_emission=0
branch_emission=0
basic_block_mutation=0
value_id_allocation=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-COMPARE-LOWERING-MUTATION-OWNER-SELECTION-001
summary=ok
REPORT
