#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-var-rhs-producer-next-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-var-rhs-producer-next-selection-v0.json"
LOOP_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/loop_stmt_handler.hako"
RECIPE_VERIFIER="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/recipe_verifier_box.hako"
LOOP_DTO="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_loop_recipe_dto_snapshot.hako"
MIR_SHAPE="$ROOT_DIR/lang/src/compiler/mirbuilder/mir_json_v0_shape_box_recipe_control.hako"
PREV_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_loop_nested_if_var_rhs_bound_row_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$LOOP_HANDLER" "$RECIPE_VERIFIER" "$LOOP_DTO" "$MIR_SHAPE" "$PREV_GATE"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GATE")"
if ! grep -q '^loop_nested_if_var_rhs_row_implemented=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "Loop nested If Var rhs prerequisite is not green"
fi

python3 - "$FIXTURE" "$LOOP_HANDLER" "$RECIPE_VERIFIER" "$LOOP_DTO" "$MIR_SHAPE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
loop_impl = Path(sys.argv[2]).read_text(encoding="utf-8")
verifier = Path(sys.argv[3]).read_text(encoding="utf-8")
loop_dto = Path(sys.argv[4]).read_text(encoding="utf-8")
mir_shape = Path(sys.argv[5]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderProgramJsonVarRhsProducerNextSelectionV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-PROGRAMJSON-VAR-RHS-PRODUCER-NEXT-SELECTION-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-VAR-RHS-BOUND-ROW-001", "bad prerequisite")

selected = [c for c in fixture.get("candidates", []) if c.get("selected") is True]
need(len(selected) == 1, "selection must be exactly one")
need(selected[0].get("name") == "TopLevelLoopVarRhsOwnerDirectRow", "bad selected candidate")

boundary = fixture.get("selected_boundary") or {}
need(boundary.get("guard_route") == "ProgramJSON -> LoopStmtHandler.handle_state_values owner-direct AOT", "bad guard route")
need(boundary.get("legacy_loop_cond_rhs_int_consumers_unchanged") is True, "legacy consumers must remain")
need(boundary.get("full_phase_state_dispatcher_authority") is False, "dispatcher authority must stay false")

claims = fixture.get("claims") or {}
for key in [
    "var_rhs_producer_next_selection",
    "top_level_loop_var_rhs_row_selected",
    "owner_direct_observe_only_required",
    "legacy_loop_cond_rhs_int_consumers_preserved",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "top_level_loop_var_rhs_row_implemented",
    "length_bound_producer_selected",
    "full_phase_state_dispatcher_authority",
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

need('if BoxHelpers.same_token(rhs_type, "Int") != 1' in loop_impl, "top-level Loop still must require Int rhs before implementation")
need("BoolRecipeBox.from_numeric_compare_codes" in loop_impl, "Loop cond_recipe path missing")
need("_verify_cond_recipe(item, tag)" in verifier, "verifier cond_recipe boundary missing")
need("BoolRecipeBox.is_valid_compare(cond_recipe)" in verifier, "verifier must validate BoolRecipe compare")
need(";loop_cond_rhs_int=" in loop_dto, "Loop DTO still consumes loop_cond_rhs_int")
need('BoxHelpers.map_get(cond_facts, "cond_rhs_int")' in mir_shape, "MIR shape still consumes cond_rhs_int")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-var-rhs-producer-next-selection-guard-v0
token=MIRBUILDER-PROGRAMJSON-VAR-RHS-PRODUCER-NEXT-SELECTION-001
var_rhs_producer_next_selection=1
top_level_loop_var_rhs_row_selected=1
top_level_loop_var_rhs_row_implemented=0
owner_direct_observe_only_required=1
legacy_loop_cond_rhs_int_consumers_preserved=1
length_bound_producer_selected=0
full_phase_state_dispatcher_authority=0
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
selected_next_card=MIRBUILDER-PROGRAMJSON-TOP-LEVEL-LOOP-VAR-RHS-BOUND-ROW-001
summary=ok
REPORT
