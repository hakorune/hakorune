#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-recipeitem-condition-slot-bool-recipe-bridge-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-recipeitem-condition-slot-bool-recipe-bridge-selection-v0.json"
RECIPE_ITEM="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/recipe_item_box.hako"
VERIFIER="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/recipe_verifier_box.hako"
SHAPE_CONTROL="$ROOT_DIR/lang/src/compiler/mirbuilder/mir_json_v0_shape_box_recipe_control.hako"
PUBLICATION_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_bool_recipe_compare_publication_parity_gate.sh"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$RECIPE_ITEM" "$VERIFIER" "$SHAPE_CONTROL" "$PUBLICATION_GATE" "$TASK_ORDER"

PUBLICATION_OUT="$(guard_cached_run "$TAG" bash "$PUBLICATION_GATE")"
if ! grep -q '^bool_recipe_compare_publication_parity=1$' <<<"$PUBLICATION_OUT"; then
  printf '%s\n' "$PUBLICATION_OUT" >&2
  guard_fail "$TAG" "BoolRecipe publication parity prerequisite is not green"
fi

python3 - "$FIXTURE" "$RECIPE_ITEM" "$VERIFIER" "$SHAPE_CONTROL" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
recipe_item = Path(sys.argv[2]).read_text(encoding="utf-8")
verifier = Path(sys.argv[3]).read_text(encoding="utf-8")
shape_control = Path(sys.argv[4]).read_text(encoding="utf-8")
task_order = Path(sys.argv[5]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderRecipeItemConditionSlotBoolRecipeBridgeSelectionV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-BRIDGE-SELECTION-001", "bad token")

candidates = {row.get("candidate"): row for row in fixture.get("candidate_bridges") or []}
need(candidates.get("OptionalCondRecipeSidecar", {}).get("selected") is True, "sidecar bridge must be selected")
need(candidates.get("ReplaceCondFactsWithBoolRecipe", {}).get("selected") is False, "replace bridge must not be selected")
need(candidates.get("LoweringReadsBoolRecipeNow", {}).get("selected") is False, "lowering bridge must not be selected")
need(candidates["OptionalCondRecipeSidecar"].get("selected_next_card") == "MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-SIDECAR-BRIDGE-001", "bad next card")

contract = fixture.get("selected_contract") or {}
need(contract.get("legacy_cond_facts_required") is True, "legacy cond_facts must remain required")
need(contract.get("cond_recipe_optional") is True, "cond_recipe must start optional")
need(contract.get("verifier_behavior_change") is False, "verifier behavior must not change in selection")
need(contract.get("lowering_behavior_change") is False, "lowering behavior must not change in selection")

claims = fixture.get("claims") or {}
for key in ["condition_slot_bridge_selection", "selected_optional_cond_recipe_sidecar"]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "recipe_item_attachment",
    "recipe_matcher_input_authority",
    "bool_recipe_lowering",
    "mir_cmp_emission",
    "branch_emission",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

for needle in [
    '"cond_facts" => me._map_or_empty(cond_facts)',
    "loop_item(cond_facts, body_item)",
    "if_item(cond_facts, then_item, else_item)",
]:
    need(needle in recipe_item, f"RecipeItem cond_facts contract missing: {needle}")
for needle in [
    'BoxHelpers.map_get(item, "cond_facts")',
]:
    need(needle in verifier, "RecipeVerifier still needs cond_facts")
need(
    'BoxHelpers.map_get(if_item, "cond_facts")' in shape_control
    and 'BoxHelpers.map_get(loop_item, "cond_facts")' in shape_control,
    "MIR shape control still needs cond_facts",
)

for needle in [
    "MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-BRIDGE-SELECTION-001",
    "MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-SIDECAR-BRIDGE-001",
]:
    need(needle in task_order, f"task-order missing: {needle}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-recipeitem-condition-slot-bool-recipe-bridge-selection-guard-v0
token=MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-BRIDGE-SELECTION-001
selected_bridge=OptionalCondRecipeSidecar
selected_next_card=MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-SIDECAR-BRIDGE-001
legacy_cond_facts_required=1
cond_recipe_optional=1
verifier_behavior_change=0
lowering_behavior_change=0
recipe_item_attachment=0
recipe_matcher_input_authority=0
bool_recipe_lowering=0
mir_cmp_emission=0
branch_emission=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
source_selfhost_claim=0
summary=ok
REPORT
