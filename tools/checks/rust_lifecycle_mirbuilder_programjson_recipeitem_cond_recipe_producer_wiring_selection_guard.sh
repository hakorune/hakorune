#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipeitem-cond-recipe-producer-wiring-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipeitem-cond-recipe-producer-wiring-selection-v0.json"
LOOP_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/loop_stmt_handler.hako"
IF_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/if_stmt_handler.hako"
RECIPE_ITEM="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/recipe_item_box.hako"
BOOL_PUBLICATION="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_bool_recipe_compare_publication.hako"
SHAPE_CONTROL="$ROOT_DIR/lang/src/compiler/mirbuilder/mir_json_v0_shape_box_recipe_control.hako"
SIDECAR_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_recipeitem_condition_slot_bool_recipe_sidecar_bridge_gate.sh"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$LOOP_HANDLER" "$IF_HANDLER" "$RECIPE_ITEM" "$BOOL_PUBLICATION" "$SHAPE_CONTROL" "$SIDECAR_GATE" "$TASK_ORDER"

SIDECAR_OUT="$(guard_cached_run "$TAG" bash "$SIDECAR_GATE")"
if ! grep -q '^recipeitem_cond_recipe_sidecar_bridge=1$' <<<"$SIDECAR_OUT"; then
  printf '%s\n' "$SIDECAR_OUT" >&2
  guard_fail "$TAG" "RecipeItem cond_recipe sidecar prerequisite is not green"
fi

python3 - "$FIXTURE" "$LOOP_HANDLER" "$IF_HANDLER" "$RECIPE_ITEM" "$BOOL_PUBLICATION" "$SHAPE_CONTROL" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
loop_handler = Path(sys.argv[2]).read_text(encoding="utf-8")
if_handler = Path(sys.argv[3]).read_text(encoding="utf-8")
recipe_item = Path(sys.argv[4]).read_text(encoding="utf-8")
bool_publication = Path(sys.argv[5]).read_text(encoding="utf-8")
shape_control = Path(sys.argv[6]).read_text(encoding="utf-8")
task_order = Path(sys.argv[7]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderProgramJsonRecipeItemCondRecipeProducerWiringSelectionV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-PROGRAMJSON-RECIPEITEM-COND-RECIPE-PRODUCER-WIRING-SELECTION-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-SIDECAR-BRIDGE-001", "bad prerequisite")

candidates = {row.get("candidate"): row for row in fixture.get("candidate_producers") or []}
need(candidates.get("LoopStmtHandlerLoopConditionProducer", {}).get("selected") is True, "LoopStmtHandler producer must be selected")
need(candidates["LoopStmtHandlerLoopConditionProducer"].get("selected_next_card") == "MIRBUILDER-PROGRAMJSON-LOOP-STMT-COND-RECIPE-SIDECAR-WIRING-001", "bad next card")
for rejected in ["IfStmtHandlerConditionProducer", "RecipeBodiesPostHocDecoration", "MirShapeControlConsumer"]:
    need(candidates.get(rejected, {}).get("selected") is False, f"{rejected} must not be selected")

contract = fixture.get("selected_contract") or {}
need(contract.get("producer") == "LoopStmtHandler", "bad producer")
need(contract.get("target_constructor") == "RecipeItemBox.loop_item_with_cond_recipe", "bad target constructor")
need(contract.get("source_publication") == "ProgramJsonBoolRecipeComparePublicationBox", "bad source publication")
need(contract.get("legacy_cond_facts_required") is True, "cond_facts must remain required")
need(contract.get("cond_recipe_optional") is True, "cond_recipe must remain optional")
need(contract.get("lowering_behavior_change") is False, "lowering behavior must not change in selection")
need(contract.get("verifier_behavior_change") is False, "verifier behavior must not change in selection")
need(contract.get("route_selection_change") is False, "route selection must not change in selection")

claims = fixture.get("claims") or {}
for key in ["cond_recipe_producer_wiring_selection", "selected_loop_stmt_handler_cond_recipe_producer"]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "recipe_item_attachment_implementation",
    "if_stmt_cond_recipe_wiring",
    "posthoc_recipeitem_decoration",
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

for needle in [
    "RecipeItemBox.loop_item(cond_facts, body_seq)",
    "cond_facts",
    "body_seq",
]:
    need(needle in loop_handler, f"LoopStmtHandler missing existing producer seam: {needle}")
for needle in [
    "RecipeItemBox.if_item(",
    "cond_facts",
]:
    need(needle in if_handler, f"IfStmtHandler missing candidate seam: {needle}")
for needle in [
    "loop_item_with_cond_recipe(cond_facts, cond_recipe, body_item)",
    "cond_recipe_summary(item)",
]:
    need(needle in recipe_item, f"RecipeItem sidecar API missing: {needle}")
for needle in [
    "static box ProgramJsonBoolRecipeComparePublicationBox",
    "build_publication(program_json): MapBox",
    '"bool_recipe" => recipe',
]:
    need(needle in bool_publication, f"BoolRecipe publication missing: {needle}")
for needle in [
    'BoxHelpers.map_get(loop_item, "cond_facts")',
    "RecipeItemBox.kind_of",
]:
    need(needle in shape_control, f"consumer still expects legacy shape: {needle}")
for needle in [
    "MIRBUILDER-PROGRAMJSON-RECIPEITEM-COND-RECIPE-PRODUCER-WIRING-SELECTION-001",
    "MIRBUILDER-PROGRAMJSON-LOOP-STMT-COND-RECIPE-SIDECAR-WIRING-001",
]:
    need(needle in task_order, f"task-order missing: {needle}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipeitem-cond-recipe-producer-wiring-selection-guard-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEITEM-COND-RECIPE-PRODUCER-WIRING-SELECTION-001
selected_producer=LoopStmtHandlerLoopConditionProducer
selected_next_card=MIRBUILDER-PROGRAMJSON-LOOP-STMT-COND-RECIPE-SIDECAR-WIRING-001
cond_recipe_producer_wiring_selection=1
selected_loop_stmt_handler_cond_recipe_producer=1
recipe_item_attachment_implementation=0
if_stmt_cond_recipe_wiring=0
posthoc_recipeitem_decoration=0
legacy_cond_facts_required=1
cond_recipe_optional=1
lowering_behavior_change=0
verifier_behavior_change=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
summary=ok
REPORT
