#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-bool-recipe-compare-lowering-emission-consultation-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-bool-recipe-compare-lowering-emission-consultation-v0.json"
INTENT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/bool_recipe_compare_lowering_intent_snapshot.hako"
BOOL_RECIPE="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/bool_recipe_box.hako"
COMPARE_RS="$ROOT_DIR/src/mir/builder/ops/comparison.rs"
EMIT_COMPARE_RS="$ROOT_DIR/src/mir/builder/emission/compare.rs"
EMIT_BRANCH_RS="$ROOT_DIR/src/mir/builder/emission/branch.rs"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INTENT_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_bool_recipe_compare_lowering_observe_only_pilot_gate.sh"
LOOP_REL_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_loop_nested_if_cond_recipe_relational_row_batch_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$INTENT_IMPL" "$BOOL_RECIPE" "$COMPARE_RS" "$EMIT_COMPARE_RS" "$EMIT_BRANCH_RS" "$TASK_ORDER" "$INTENT_GATE" "$LOOP_REL_GATE"

INTENT_OUT="$(guard_cached_run "$TAG" bash "$INTENT_GATE")"
if ! grep -q '^observe_only_lowering_intent=1$' <<<"$INTENT_OUT"; then
  printf '%s\n' "$INTENT_OUT" >&2
  guard_fail "$TAG" "BoolRecipe lowering intent prerequisite is not green"
fi

LOOP_REL_OUT="$(guard_cached_run "$TAG" bash "$LOOP_REL_GATE")"
if ! grep -q '^loop_nested_if_cond_recipe_relational_row_batch=1$' <<<"$LOOP_REL_OUT"; then
  printf '%s\n' "$LOOP_REL_OUT" >&2
  guard_fail "$TAG" "Loop nested If relational row prerequisite is not green"
fi

python3 - "$FIXTURE" "$INTENT_IMPL" "$BOOL_RECIPE" "$COMPARE_RS" "$EMIT_COMPARE_RS" "$EMIT_BRANCH_RS" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
intent_impl = Path(sys.argv[2]).read_text(encoding="utf-8")
bool_recipe = Path(sys.argv[3]).read_text(encoding="utf-8")
compare_rs = Path(sys.argv[4]).read_text(encoding="utf-8")
emit_compare = Path(sys.argv[5]).read_text(encoding="utf-8")
emit_branch = Path(sys.argv[6]).read_text(encoding="utf-8")
task_order = Path(sys.argv[7]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderBoolRecipeCompareLoweringEmissionConsultationV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-BOOL-RECIPE-COMPARE-LOWERING-EMISSION-CONSULTATION-001", "bad token")
need(fixture.get("prerequisites") == [
    "MIRBUILDER-BOOL-RECIPE-COMPARE-LOWERING-OBSERVE-ONLY-PILOT-001",
    "MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-COND-RECIPE-RELATIONAL-ROW-BATCH-001",
], "bad prerequisites")

candidates = {row.get("name"): row for row in fixture.get("candidates") or []}
need(candidates["EmitMirCompareNow"].get("selected") is False, "Compare-only emission must not be selected")
need(candidates["EmitMirCompareAndBranchNow"].get("selected") is False, "Compare+Branch emission must not be selected")
selected = candidates["CloseIfLoopCompareRowBatchBeforeEmission"]
need(selected.get("selected") is True, "row batch closeout must be selected")
need(selected.get("selected_next_card") == "MIRBUILDER-PROGRAMJSON-IF-LOOP-COMPARE-ROW-BATCH-CLOSEOUT-001", "bad selected next")
need(candidates["SelectMutationBearingLoweringOwner"].get("selected") is False, "mutation-bearing owner selection must wait")

boundary = fixture.get("selected_boundary") or {}
need(boundary.get("name") == "CompareRowBatchCloseoutBeforeEmission", "bad boundary")
need(boundary.get("emission_owner_status") == "deferred", "emission owner must be deferred")
for action in [
    "MIR Compare emission",
    "MIR Branch emission",
    "BasicBlock mutation",
    "ValueId allocation",
    "route selection",
    "runtime route switch",
    "ProgramJSON runtime route authority",
    "runtime fallback",
]:
    need(action in boundary.get("forbidden_actions", []), f"missing forbidden action: {action}")
for requirement in [
    "explicit operand ValueId resolution boundary",
    "rhs bound materialization boundary",
    "compare dst ValueId allocation owner",
    "branch target BasicBlock ownership",
    "observe-only parity before runtime authority",
]:
    need(requirement in boundary.get("future_emission_owner_requirements", []), f"missing future requirement: {requirement}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "DeferBoolRecipeCompareMirEmissionSelectRowBatchCloseout", "bad decision")
need(decision.get("selected_next_card") == "MIRBUILDER-PROGRAMJSON-IF-LOOP-COMPARE-ROW-BATCH-CLOSEOUT-001", "bad decision next")

claims = fixture.get("claims") or {}
for key in [
    "lowering_emission_consultation",
    "compare_row_batch_closeout_next",
    "bool_recipe_compare_emission_deferred",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
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

need("MIR Compare/Branch emission" in intent_impl, "intent implementation must disclaim MIR emission")
need('"mir_cmp_emission" => 0' in intent_impl, "intent snapshot must keep mir_cmp_emission=0")
need('"branch_emission" => 0' in intent_impl, "intent snapshot must keep branch_emission=0")
need('"value_id_allocation" => 0' in intent_impl, "intent snapshot must keep value_id_allocation=0")
need("Non-responsibility:" in bool_recipe and "MIR Compare/Branch emission" in bool_recipe, "BoolRecipe must stay non-lowering")
need("build_comparison_op" in compare_rs and "next_value_id" in compare_rs, "Rust comparison owner must still own ValueId allocation")
need("MirInstruction::Compare" in emit_compare, "Rust MIR Compare emission owner missing")
need("MirInstruction::Branch" in emit_branch, "Rust MIR Branch emission owner missing")
need("MIRBUILDER-PROGRAMJSON-IF-LOOP-COMPARE-ROW-BATCH-CLOSEOUT-001" in task_order, "task-order missing closeout next")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-bool-recipe-compare-lowering-emission-consultation-guard-v0
token=MIRBUILDER-BOOL-RECIPE-COMPARE-LOWERING-EMISSION-CONSULTATION-001
decision=DeferBoolRecipeCompareMirEmissionSelectRowBatchCloseout
lowering_emission_consultation=1
compare_row_batch_closeout_next=1
bool_recipe_compare_emission_deferred=1
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
selected_next_card=MIRBUILDER-PROGRAMJSON-IF-LOOP-COMPARE-ROW-BATCH-CLOSEOUT-001
summary=ok
REPORT
