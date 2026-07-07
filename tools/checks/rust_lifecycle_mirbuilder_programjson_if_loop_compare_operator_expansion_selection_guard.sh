#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-if-loop-compare-operator-expansion-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-if-loop-compare-operator-expansion-selection-v0.json"
READER="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_compare_reader_box.hako"
IF_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/if_stmt_handler.hako"
LOWERING_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_bool_recipe_compare_lowering_observe_only_pilot_gate.sh"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$READER" "$IF_HANDLER" "$LOWERING_GATE" "$TASK_ORDER"

LOWERING_OUT="$(guard_cached_run "$TAG" bash "$LOWERING_GATE")"
if ! grep -q '^observe_only_lowering_intent=1$' <<<"$LOWERING_OUT"; then
  printf '%s\n' "$LOWERING_OUT" >&2
  guard_fail "$TAG" "BoolRecipe lowering-intent prerequisite is not green"
fi

python3 - "$FIXTURE" "$READER" "$IF_HANDLER" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
reader = Path(sys.argv[2]).read_text(encoding="utf-8")
if_handler = Path(sys.argv[3]).read_text(encoding="utf-8")
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderProgramJsonIfLoopCompareOperatorExpansionSelectionV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-PROGRAMJSON-IF-LOOP-COMPARE-OPERATOR-EXPANSION-SELECTION-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-BOOL-RECIPE-COMPARE-LOWERING-OBSERVE-ONLY-PILOT-001", "bad prerequisite")

candidates = {row.get("name"): row for row in fixture.get("candidates") or []}
selected = candidates["TopLevelIfRelationalBatch"]
need(selected.get("selected") is True, "TopLevelIfRelationalBatch must be selected")
need(selected.get("selected_next_card") == "MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-RELATIONAL-ROW-BATCH-001", "bad selected next")
for name in ["LoopNestedIfRelationalExitBatch", "TopLevelLoopRelationalRuntimeRows", "MIRLoweringEmissionNow"]:
    need(candidates[name].get("selected") is False, f"{name} must not be selected")

batch = fixture.get("selected_batch") or {}
need(batch.get("owner") == "IfStmtHandler", "bad selected owner")
need(batch.get("rows") == [
    "if_var_lt_int_then_return_else_null",
    "if_var_le_int_then_return_else_null",
    "if_var_gt_int_then_return_else_null",
    "if_var_ge_int_then_return_else_null",
], "bad selected rows")
need(batch.get("lowering") is False, "batch must not claim lowering")
need(batch.get("route_selection") is False, "batch must not claim route selection")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectTopLevelIfRelationalBatch", "bad decision")
need(decision.get("selected_next_card") == "MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-RELATIONAL-ROW-BATCH-001", "bad decision next")

claims = fixture.get("claims") or {}
for key in ["operator_expansion_selection", "top_level_if_relational_batch_selected"]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "if_accepts_all_6_compare_operators",
    "loop_nested_if_operator_expansion",
    "top_level_loop_route_semantics_changed",
    "bool_recipe_lowering_executed",
    "mir_cmp_emission",
    "branch_emission",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

for op in ['"<"', '"<="', '">"', '">="', '"=="', '"!="']:
    need(op in reader, f"shared reader missing op {op}")
need("ProgramJsonCompareReaderBox.read_var_int_compare(program_json, cond_start)" in if_handler, "If handler missing shared reader")
need("_cond_kind_from_reader(cond_reader)" in if_handler, "If handler missing cond_kind reader bridge")
need("MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-RELATIONAL-ROW-BATCH-001" in task_order, "task-order missing selected next")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-if-loop-compare-operator-expansion-selection-guard-v0
token=MIRBUILDER-PROGRAMJSON-IF-LOOP-COMPARE-OPERATOR-EXPANSION-SELECTION-001
decision=SelectTopLevelIfRelationalBatch
operator_expansion_selection=1
top_level_if_relational_batch_selected=1
selected_next_card=MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-RELATIONAL-ROW-BATCH-001
if_accepts_all_6_compare_operators=0
loop_nested_if_operator_expansion=0
top_level_loop_route_semantics_changed=0
bool_recipe_lowering_executed=0
mir_cmp_emission=0
branch_emission=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
summary=ok
REPORT
