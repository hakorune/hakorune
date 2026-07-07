#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-compare-reader-var-rhs-producer-row-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-compare-reader-var-rhs-producer-row-selection-v0.json"
IF_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/if_stmt_handler.hako"
LOOP_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/loop_stmt_handler.hako"
PREV_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_compare_reader_var_rhs_bound_parity_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$IF_HANDLER" "$LOOP_HANDLER" "$PREV_GATE"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GATE")"
if ! grep -q '^compare_reader_var_rhs_bound_parity=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "Var rhs bound parity prerequisite is not green"
fi

python3 - "$FIXTURE" "$IF_HANDLER" "$LOOP_HANDLER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
if_handler = Path(sys.argv[2]).read_text(encoding="utf-8")
loop_handler = Path(sys.argv[3]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderProgramJsonCompareReaderVarRhsProducerRowSelectionV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-PROGRAMJSON-COMPARE-READER-VAR-RHS-PRODUCER-ROW-SELECTION-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-PROGRAMJSON-COMPARE-READER-VAR-RHS-BOUND-PARITY-001", "bad prerequisite")

state = fixture.get("input_state") or {}
need(state.get("reader_bound_kind_code") == 2, "bad reader bound kind")
need(state.get("producer_change_available") is True, "producer change must be available")
need(state.get("lowering_emission_deferred") is True, "lowering must remain deferred")

candidates = {row.get("name"): row for row in fixture.get("candidates") or []}
need(candidates["TopLevelIfVarRhsCondRecipeRow"].get("selected") is True, "top-level If row must be selected")
need(candidates["TopLevelIfVarRhsCondRecipeRow"].get("selected_next_card") == "MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-VAR-RHS-BOUND-ROW-001", "bad selected next")
need(candidates["LoopNestedIfVarRhsCondRecipeRow"].get("selected") is False, "Loop nested If must wait")
need(candidates["TopLevelLoopVarRhsCondRecipeRow"].get("selected") is False, "Top-level Loop must wait")
need(candidates["LengthBoundProducerRows"].get("selected") is False, "Length bound producers must wait")
need(candidates["MutationBearingLoweringOwner"].get("selected") is False, "Lowering owner must wait")

boundary = fixture.get("selected_boundary") or {}
need(boundary.get("owner") == "IfStmtHandler", "bad selected owner")
need(boundary.get("producer_scope") == "one exact top-level If row", "bad producer scope")
need(boundary.get("lowering") is False, "lowering must be false")
need(boundary.get("route_selection") is False, "route selection must be false")

claims = fixture.get("claims") or {}
for key in ["var_rhs_producer_row_selection", "top_level_if_var_rhs_row_selected"]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "top_level_if_var_rhs_row_implemented",
    "loop_nested_if_var_rhs_row_selected",
    "top_level_loop_var_rhs_row_selected",
    "length_bound_producer_selected",
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

need("ProgramJsonCompareReaderBox.read_var_int_compare(program_json, cond_start)" in if_handler, "If handler must use shared reader")
need("Loop If Compare rhs must be Int" in loop_handler, "Loop nested If must remain Int-bound before its card")
need("Loop Compare rhs must be Int" in loop_handler, "Top-level Loop must remain Int-bound before its card")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-compare-reader-var-rhs-producer-row-selection-guard-v0
token=MIRBUILDER-PROGRAMJSON-COMPARE-READER-VAR-RHS-PRODUCER-ROW-SELECTION-001
decision=SelectTopLevelIfVarRhsCondRecipeProducerRow
var_rhs_producer_row_selection=1
top_level_if_var_rhs_row_selected=1
top_level_if_var_rhs_row_implemented=0
loop_nested_if_var_rhs_row_selected=0
top_level_loop_var_rhs_row_selected=0
length_bound_producer_selected=0
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
selected_next_card=MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-VAR-RHS-BOUND-ROW-001
summary=ok
REPORT
