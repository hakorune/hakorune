#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-recipeitem-cond-recipe-consume-boundary-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-recipeitem-cond-recipe-consume-boundary-selection-v0.json"
VERIFIER="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/recipe_verifier_box.hako"
MATCHER="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipematcher_execution_boundary.hako"
SHAPE_CONTROL="$ROOT_DIR/lang/src/compiler/mirbuilder/mir_json_v0_shape_box_recipe_control.hako"
DIAG_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_recipeitem_cond_recipe_diagnostic_summary_observation_gate.sh"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$VERIFIER" "$MATCHER" "$SHAPE_CONTROL" "$DIAG_GATE" "$TASK_ORDER"

DIAG_OUT="$(guard_cached_run "$TAG" bash "$DIAG_GATE")"
if ! grep -q '^cond_recipe_diagnostic_summary_observation=1$' <<<"$DIAG_OUT"; then
  printf '%s\n' "$DIAG_OUT" >&2
  guard_fail "$TAG" "cond_recipe diagnostic summary prerequisite is not green"
fi

python3 - "$FIXTURE" "$VERIFIER" "$MATCHER" "$SHAPE_CONTROL" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
verifier = Path(sys.argv[2]).read_text(encoding="utf-8")
matcher = Path(sys.argv[3]).read_text(encoding="utf-8")
shape_control = Path(sys.argv[4]).read_text(encoding="utf-8")
task_order = Path(sys.argv[5]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderRecipeItemCondRecipeConsumeBoundarySelectionV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-RECIPEITEM-COND-RECIPE-CONSUME-BOUNDARY-SELECTION-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-RECIPEITEM-COND-RECIPE-DIAGNOSTIC-SUMMARY-OBSERVATION-001", "bad prerequisite")

candidates = {row.get("candidate"): row for row in fixture.get("candidate_consumers") or []}
need(candidates.get("RecipeVerifierValidateOnlyConsumer", {}).get("selected") is True, "RecipeVerifier validate-only must be selected")
need(candidates["RecipeVerifierValidateOnlyConsumer"].get("selected_next_card") == "MIRBUILDER-RECIPEVERIFIER-COND-RECIPE-VALIDATE-ONLY-CONSUME-001", "bad next card")
for rejected in ["RecipeMatcherInputConsumer", "MirShapeControlConsumer", "LoweringConsumer"]:
    need(candidates.get(rejected, {}).get("selected") is False, f"{rejected} must not be selected")

contract = fixture.get("selected_contract") or {}
need(contract.get("consumer") == "RecipeVerifierBox", "bad consumer")
need(contract.get("allowed_effect") == "reject malformed cond_recipe only", "bad allowed effect")
need(contract.get("port_sig_behavior_change") is False, "port sig must not change")
need(contract.get("recipe_matcher_input_authority") is False, "matcher authority must not change")
need(contract.get("lowering_behavior_change") is False, "lowering must not change")
need(contract.get("route_selection_change") is False, "route selection must not change")

claims = fixture.get("claims") or {}
for key in ["cond_recipe_consume_boundary_selection", "selected_recipeverifier_validate_only_consumer"]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "recipeverifier_cond_recipe_consume_implementation",
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

need("RecipeVerifierBox" in verifier, "RecipeVerifier source missing")
need("ProgramJsonRecipeMatcherExecutionBoundaryBox" in matcher, "RecipeMatcher boundary source missing")
need('"cond_recipe"' not in shape_control, "Shape control must not consume cond_recipe in selection card")
for needle in [
    "MIRBUILDER-RECIPEITEM-COND-RECIPE-CONSUME-BOUNDARY-SELECTION-001",
    "MIRBUILDER-RECIPEVERIFIER-COND-RECIPE-VALIDATE-ONLY-CONSUME-001",
]:
    need(needle in task_order, f"task-order missing: {needle}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-recipeitem-cond-recipe-consume-boundary-selection-guard-v0
token=MIRBUILDER-RECIPEITEM-COND-RECIPE-CONSUME-BOUNDARY-SELECTION-001
selected_consumer=RecipeVerifierValidateOnlyConsumer
selected_next_card=MIRBUILDER-RECIPEVERIFIER-COND-RECIPE-VALIDATE-ONLY-CONSUME-001
cond_recipe_consume_boundary_selection=1
selected_recipeverifier_validate_only_consumer=1
recipeverifier_cond_recipe_consume_implementation=0
recipe_matcher_input_authority=0
bool_recipe_lowering=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
summary=ok
REPORT
