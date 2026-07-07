#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-compare-reader-followon-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-compare-reader-followon-selection-v0.json"
READER="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_compare_reader_box.hako"
BOOL_RECIPE="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/bool_recipe_box.hako"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CLOSEOUT_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_if_loop_compare_row_batch_closeout_guard.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$READER" "$BOOL_RECIPE" "$TASK_ORDER" "$CLOSEOUT_GATE"

CLOSEOUT_OUT="$(guard_cached_run "$TAG" bash "$CLOSEOUT_GATE")"
if ! grep -q '^if_loop_compare_row_batch_closeout=1$' <<<"$CLOSEOUT_OUT"; then
  printf '%s\n' "$CLOSEOUT_OUT" >&2
  guard_fail "$TAG" "If/Loop compare row batch closeout prerequisite is not green"
fi

python3 - "$FIXTURE" "$READER" "$BOOL_RECIPE" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
reader = Path(sys.argv[2]).read_text(encoding="utf-8")
bool_recipe = Path(sys.argv[3]).read_text(encoding="utf-8")
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderProgramJsonCompareReaderFollowonSelectionV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-PROGRAMJSON-COMPARE-READER-FOLLOWON-SELECTION-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-PROGRAMJSON-IF-LOOP-COMPARE-ROW-BATCH-CLOSEOUT-001", "bad prerequisite")

current = fixture.get("current_reader_surface") or {}
need(current.get("owner") == "ProgramJsonCompareReaderBox.read_var_int_compare", "bad current owner")
need(current.get("bound_kind_code") == 1, "current reader must still be LiteralI64")
need(current.get("analysis_only") is True, "reader must stay analysis-only")

capacity = fixture.get("downstream_capacity") or {}
need(capacity.get("bool_recipe_bound_expr_supports_symbol_ref") is True, "BoolRecipe SymbolRef capacity missing")
need(capacity.get("lowering_emission_deferred") is True, "lowering emission must remain deferred")

candidates = {row.get("name"): row for row in fixture.get("candidates") or []}
need(candidates["VarRhsBound"].get("selected") is True, "Var rhs bound must be selected")
need(candidates["VarRhsBound"].get("selected_next_card") == "MIRBUILDER-PROGRAMJSON-COMPARE-READER-VAR-RHS-BOUND-PARITY-001", "bad selected next")
need(candidates["LengthBound"].get("selected") is False, "Length bound must wait")
need(candidates["ReversedVarVarContextAware"].get("selected") is False, "context-aware reversed Var/Var must wait")
need(candidates["MutationBearingLoweringOwner"].get("selected") is False, "mutation-bearing lowering owner must wait")

boundary = fixture.get("selected_boundary") or {}
need(boundary.get("name") == "ProgramJsonCompareReaderVarRhsBoundParityV1", "bad selected boundary")
need(boundary.get("output_shape") == "ProgramJsonCompareReaderCodeMapV1 with bound_kind_code=2 and bound_symbol_id>0", "bad output shape")
need(boundary.get("reversed_var_var_without_context") == "not_claimed", "reversed Var/Var must be not claimed")
need(boundary.get("length_bounds") == "not_claimed", "length bounds must be not claimed")
need(boundary.get("lowering") is False, "lowering must be false")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectProgramJsonCompareReaderVarRhsBoundParity", "bad decision")
need(decision.get("selected_next_card") == "MIRBUILDER-PROGRAMJSON-COMPARE-READER-VAR-RHS-BOUND-PARITY-001", "bad decision next")

claims = fixture.get("claims") or {}
for key in ["compare_reader_followon_selection", "var_rhs_bound_selected"]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "var_rhs_bound_implemented",
    "length_bound_selected",
    "reversed_var_var_context_aware_selected",
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

need("read_var_int_compare(program_json, compare_start): MapBox" in reader, "current reader entry missing")
need('"rhs_not_int"' in reader, "current reader must still reject non-Int rhs before implementation")
need('"bound_kind_code" => 1' in reader, "current reader must still publish LiteralI64")
need("symbol_ref(symbol_id)" in bool_recipe, "BoolRecipe SymbolRef boundary missing")
need("if kind == 2 { return BoundExprBox.symbol_ref" in bool_recipe, "BoolRecipe code-map SymbolRef support missing")
need("MIRBUILDER-PROGRAMJSON-COMPARE-READER-FOLLOWON-SELECTION-001; status=landed" in task_order, "task-order must mark selection landed")
need("MIRBUILDER-PROGRAMJSON-COMPARE-READER-VAR-RHS-BOUND-PARITY-001; status=next" in task_order, "task-order missing selected implementation next")
need("MIRBUILDER-COMPARE-LOWERING-MUTATION-OWNER-SELECTION-001" in task_order, "future lowering owner selection missing")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-compare-reader-followon-selection-guard-v0
token=MIRBUILDER-PROGRAMJSON-COMPARE-READER-FOLLOWON-SELECTION-001
decision=SelectProgramJsonCompareReaderVarRhsBoundParity
compare_reader_followon_selection=1
var_rhs_bound_selected=1
var_rhs_bound_implemented=0
length_bound_selected=0
reversed_var_var_context_aware_selected=0
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
selected_next_card=MIRBUILDER-PROGRAMJSON-COMPARE-READER-VAR-RHS-BOUND-PARITY-001
summary=ok
REPORT
