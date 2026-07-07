#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-bool-recipe-compare-lowering-boundary-consultation-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-bool-recipe-compare-lowering-boundary-consultation-v0.json"
BOOL_RECIPE="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/bool_recipe_box.hako"
PUBLICATION="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_bool_recipe_compare_publication.hako"
COMPARE_RS="$ROOT_DIR/src/mir/builder/ops/comparison.rs"
EMIT_COMPARE_RS="$ROOT_DIR/src/mir/builder/emission/compare.rs"
EMIT_BRANCH_RS="$ROOT_DIR/src/mir/builder/emission/branch.rs"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$BOOL_RECIPE" "$PUBLICATION" "$COMPARE_RS" "$EMIT_COMPARE_RS" "$EMIT_BRANCH_RS" "$TASK_ORDER"

python3 - "$FIXTURE" "$BOOL_RECIPE" "$PUBLICATION" "$COMPARE_RS" "$EMIT_COMPARE_RS" "$EMIT_BRANCH_RS" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
bool_recipe = Path(sys.argv[2]).read_text(encoding="utf-8")
publication = Path(sys.argv[3]).read_text(encoding="utf-8")
compare_rs = Path(sys.argv[4]).read_text(encoding="utf-8")
emit_compare = Path(sys.argv[5]).read_text(encoding="utf-8")
emit_branch = Path(sys.argv[6]).read_text(encoding="utf-8")
task_order = Path(sys.argv[7]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderBoolRecipeCompareLoweringBoundaryConsultationV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-BOOL-RECIPE-COMPARE-LOWERING-BOUNDARY-CONSULTATION-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-PROGRAMJSON-IF-LOOP-COMPARE-READER-PARITY-FLOOR-001", "bad prerequisite")

candidates = {row.get("name"): row for row in fixture.get("candidates") or []}
need(candidates["LowerBoolRecipeCompareToMirNow"].get("selected") is False, "MIR lowering must not be selected now")
selected = candidates["SelectObserveOnlyLoweringIntentPilot"]
need(selected.get("selected") is True, "observe-only intent pilot must be selected")
need(selected.get("selected_next_card") == "MIRBUILDER-BOOL-RECIPE-COMPARE-LOWERING-OBSERVE-ONLY-PILOT-001", "bad selected next")
need(candidates["ExpandIfLoopOperatorsBeforeLoweringIntent"].get("selected") is False, "operator expansion must wait for intent boundary")

boundary = fixture.get("selected_boundary") or {}
need(boundary.get("name") == "BoolRecipeCompareLoweringIntentSnapshotV1", "bad selected boundary")
for action in [
    "MIR Compare emission",
    "MIR Branch emission",
    "BasicBlock mutation",
    "ValueId allocation",
    "route selection",
    "runtime route switch",
    "runtime fallback",
]:
    need(action in boundary.get("forbidden_actions", []), f"missing forbidden action: {action}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectBoolRecipeCompareObserveOnlyLoweringIntentPilot", "bad decision")
need(decision.get("selected_next_card") == "MIRBUILDER-BOOL-RECIPE-COMPARE-LOWERING-OBSERVE-ONLY-PILOT-001", "bad decision next")

claims = fixture.get("claims") or {}
for key in [
    "lowering_boundary_consultation",
    "observe_only_lowering_intent_next",
    "bool_recipe_compare_lowering_intent_selected",
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

need("Non-responsibility:" in bool_recipe, "BoolRecipe non-responsibility missing")
need("MIR Compare/Branch emission" in bool_recipe, "BoolRecipe must disclaim MIR emission")
need('"analysis_only" => 1' in bool_recipe, "BoolRecipe analysis_only missing")
need('"lowering_executed" => 0' in bool_recipe, "BoolRecipe lowering_executed=0 missing")
need("lowering_executed\" => 0" in publication, "publication must remain non-lowering")
need("build_comparison_op" in compare_rs, "Rust comparison owner missing")
need("emission::compare::emit_to" in compare_rs, "Rust comparison emission handoff missing")
need("MirInstruction::Compare" in emit_compare, "MIR Compare emission owner missing")
need("MirInstruction::Branch" in emit_branch, "MIR Branch emission owner missing")
need("MIRBUILDER-BOOL-RECIPE-COMPARE-LOWERING-OBSERVE-ONLY-PILOT-001" in task_order, "task-order missing observe-only pilot")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-bool-recipe-compare-lowering-boundary-consultation-guard-v0
token=MIRBUILDER-BOOL-RECIPE-COMPARE-LOWERING-BOUNDARY-CONSULTATION-001
decision=SelectBoolRecipeCompareObserveOnlyLoweringIntentPilot
lowering_boundary_consultation=1
observe_only_lowering_intent_next=1
bool_recipe_compare_lowering_intent_selected=1
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
selected_next_card=MIRBUILDER-BOOL-RECIPE-COMPARE-LOWERING-OBSERVE-ONLY-PILOT-001
summary=ok
REPORT
