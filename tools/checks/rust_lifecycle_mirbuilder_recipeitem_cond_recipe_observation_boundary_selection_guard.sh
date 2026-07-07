#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-recipeitem-cond-recipe-observation-boundary-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-recipeitem-cond-recipe-observation-boundary-selection-v0.json"
RECIPE_ITEM="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/recipe_item_box.hako"
VERIFIER="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/recipe_verifier_box.hako"
MATCHER="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipematcher_execution_boundary.hako"
WIRING_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_loop_stmt_cond_recipe_sidecar_wiring_gate.sh"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$RECIPE_ITEM" "$VERIFIER" "$MATCHER" "$WIRING_GATE" "$TASK_ORDER"

WIRING_OUT="$(guard_cached_run "$TAG" bash "$WIRING_GATE")"
if ! grep -q '^loop_stmt_cond_recipe_sidecar_wiring=1$' <<<"$WIRING_OUT"; then
  printf '%s\n' "$WIRING_OUT" >&2
  guard_fail "$TAG" "LoopStmtHandler cond_recipe sidecar wiring prerequisite is not green"
fi

python3 - "$FIXTURE" "$RECIPE_ITEM" "$VERIFIER" "$MATCHER" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
recipe_item = Path(sys.argv[2]).read_text(encoding="utf-8")
verifier = Path(sys.argv[3]).read_text(encoding="utf-8")
matcher = Path(sys.argv[4]).read_text(encoding="utf-8")
task_order = Path(sys.argv[5]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderRecipeItemCondRecipeObservationBoundarySelectionV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-RECIPEITEM-COND-RECIPE-OBSERVATION-BOUNDARY-SELECTION-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-PROGRAMJSON-LOOP-STMT-COND-RECIPE-SIDECAR-WIRING-001", "bad prerequisite")

candidates = {row.get("candidate"): row for row in fixture.get("candidate_observers") or []}
need(candidates.get("RecipeItemDiagnosticSummaryObserver", {}).get("selected") is True, "diagnostic observer must be selected")
need(candidates["RecipeItemDiagnosticSummaryObserver"].get("selected_next_card") == "MIRBUILDER-RECIPEITEM-COND-RECIPE-DIAGNOSTIC-SUMMARY-OBSERVATION-001", "bad next card")
for rejected in ["RecipeVerifierObserver", "RecipeMatcherInputObserver", "LoweringObserver"]:
    need(candidates.get(rejected, {}).get("selected") is False, f"{rejected} must not be selected")

contract = fixture.get("selected_contract") or {}
need(contract.get("observer") == "RecipeItemDiagnosticSummaryObserver", "bad observer")
need(contract.get("verifier_behavior_change") is False, "verifier behavior must not change")
need(contract.get("recipe_matcher_input_authority") is False, "matcher authority must not change")
need(contract.get("lowering_behavior_change") is False, "lowering behavior must not change")

claims = fixture.get("claims") or {}
for key in ["cond_recipe_observation_boundary_selection", "selected_diagnostic_summary_observer"]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "cond_recipe_deep_observation_implementation",
    "verifier_cond_recipe_observer",
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

need("cond_recipe_summary(item)" in recipe_item, "RecipeItem diagnostic summary observer missing")
need("RecipeVerifierBox" in verifier, "RecipeVerifier source missing")
need("ProgramJsonRecipeMatcherExecutionBoundaryBox" in matcher, "RecipeMatcher boundary source missing")
for needle in [
    "MIRBUILDER-RECIPEITEM-COND-RECIPE-OBSERVATION-BOUNDARY-SELECTION-001",
    "MIRBUILDER-RECIPEITEM-COND-RECIPE-DIAGNOSTIC-SUMMARY-OBSERVATION-001",
]:
    need(needle in task_order, f"task-order missing: {needle}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-recipeitem-cond-recipe-observation-boundary-selection-guard-v0
token=MIRBUILDER-RECIPEITEM-COND-RECIPE-OBSERVATION-BOUNDARY-SELECTION-001
selected_observer=RecipeItemDiagnosticSummaryObserver
selected_next_card=MIRBUILDER-RECIPEITEM-COND-RECIPE-DIAGNOSTIC-SUMMARY-OBSERVATION-001
cond_recipe_observation_boundary_selection=1
selected_diagnostic_summary_observer=1
cond_recipe_deep_observation_implementation=0
verifier_cond_recipe_observer=0
recipe_matcher_input_authority=0
bool_recipe_lowering=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
summary=ok
REPORT
