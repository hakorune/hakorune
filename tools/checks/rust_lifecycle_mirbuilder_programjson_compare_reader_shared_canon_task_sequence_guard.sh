#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-compare-reader-shared-canon-task-sequence-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-compare-reader-shared-canon-task-sequence-v0.json"
PREV_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_recipematcher_cond_recipe_observe_only_input_snapshot_gate.sh"
IF_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/if_stmt_handler.hako"
LOOP_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/loop_stmt_handler.hako"
BOOL_RECIPE="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/bool_recipe_box.hako"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$PREV_GATE" "$IF_HANDLER" "$LOOP_HANDLER" "$BOOL_RECIPE" "$TASK_ORDER"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GATE")"
if ! grep -q '^recipematcher_cond_recipe_observe_only_input_snapshot=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "cond_recipe observe-only input snapshot prerequisite is not green"
fi

python3 - "$FIXTURE" "$IF_HANDLER" "$LOOP_HANDLER" "$BOOL_RECIPE" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
if_handler = Path(sys.argv[2]).read_text(encoding="utf-8")
loop_handler = Path(sys.argv[3]).read_text(encoding="utf-8")
bool_recipe = Path(sys.argv[4]).read_text(encoding="utf-8")
task_order = Path(sys.argv[5]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderProgramJsonCompareReaderSharedCanonTaskSequenceV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-PROGRAMJSON-COMPARE-READER-SHARED-CANON-TASK-SEQUENCE-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-RECIPEMATCHER-COND-RECIPE-OBSERVE-ONLY-INPUT-SNAPSHOT-001", "bad prerequisite")

sequence = fixture.get("selected_sequence") or []
expected = [
    "MIRBUILDER-PROGRAMJSON-COMPARE-READER-SHARED-CANON-001",
    "MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-EQ-BEHAVIOR-PRESERVING-001",
    "MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-FIRST-NON-EQ-ROW-001",
    "MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-COND-RECIPE-BRIDGE-001",
    "MIRBUILDER-PROGRAMJSON-LOOP-COND-RECIPE-CONSTRUCTOR-CLEANUP-001",
    "MIRBUILDER-RUST-LOOP-CONDITION-SHAPE-EQ-NE-CANON-001",
    "MIRBUILDER-CONDSKELETON-IFCOND-CONSULTATION-001",
]
need([row.get("card") for row in sequence] == expected, "bad selected sequence")

decision = fixture.get("decision") or {}
need(decision.get("selected_next_card") == expected[0], "bad selected next card")

claims = fixture.get("claims") or {}
need(claims.get("compare_reader_task_sequence_selected") == 1, "selection claim missing")
for key in [
    "shared_compare_reader_implemented",
    "if_cond_recipe_attached",
    "if_compare_operator_expansion",
    "loop_nested_if_cond_recipe",
    "rust_loop_condition_shape_eq_ne",
    "condskeleton_ifcond",
    "recipe_matcher_input_authority",
    "bool_recipe_lowering",
    "mir_cmp_emission",
    "branch_emission",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need("If cond Compare op must be '=='" in if_handler, "If handler baseline asymmetry missing")
need("RecipeItemBox.if_item(" in if_handler, "If handler baseline RecipeItem call missing")
need(
    "loop_item.set(\"cond_recipe\", cond_recipe)" in loop_handler
    or "RecipeItemBox.loop_item_with_cond_recipe(cond_facts, cond_recipe, body_seq)" in loop_handler,
    "Loop handler cond_recipe producer missing",
)
need("cmp_code >= 1 && cmp_code <= 6" in bool_recipe, "BoolRecipe six-op vocabulary missing")
for needle in [
    "MIRBUILDER-PROGRAMJSON-COMPARE-READER-SHARED-CANON-TASK-SEQUENCE-001",
    expected[0],
    expected[1],
]:
    need(needle in task_order, f"task-order missing: {needle}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-compare-reader-shared-canon-task-sequence-guard-v0
token=MIRBUILDER-PROGRAMJSON-COMPARE-READER-SHARED-CANON-TASK-SEQUENCE-001
selected_next_card=MIRBUILDER-PROGRAMJSON-COMPARE-READER-SHARED-CANON-001
compare_reader_task_sequence_selected=1
shared_compare_reader_implemented=0
if_cond_recipe_attached=0
if_compare_operator_expansion=0
loop_nested_if_cond_recipe=0
rust_loop_condition_shape_eq_ne=0
recipe_matcher_input_authority=0
bool_recipe_lowering=0
route_selection=0
runtime_route_switch=0
source_selfhost_claim=0
summary=ok
REPORT
