#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-recipematcher-cond-recipe-input-consume-boundary-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-recipematcher-cond-recipe-input-consume-boundary-selection-v0.json"
MATCHER="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipematcher_execution_boundary.hako"
SHAPE_CONTROL="$ROOT_DIR/lang/src/compiler/mirbuilder/mir_json_v0_shape_box_recipe_control.hako"
VERIFIER_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_recipeverifier_cond_recipe_validate_only_consume_gate.sh"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$MATCHER" "$SHAPE_CONTROL" "$VERIFIER_GATE" "$TASK_ORDER"

VERIFIER_OUT="$(guard_cached_run "$TAG" bash "$VERIFIER_GATE")"
if ! grep -q '^recipeverifier_cond_recipe_validate_only_consume=1$' <<<"$VERIFIER_OUT"; then
  printf '%s\n' "$VERIFIER_OUT" >&2
  guard_fail "$TAG" "RecipeVerifier cond_recipe validate-only prerequisite is not green"
fi

python3 - "$FIXTURE" "$MATCHER" "$SHAPE_CONTROL" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
matcher = Path(sys.argv[2]).read_text(encoding="utf-8")
shape_control = Path(sys.argv[3]).read_text(encoding="utf-8")
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderRecipeMatcherCondRecipeInputConsumeBoundarySelectionV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-RECIPEMATCHER-COND-RECIPE-INPUT-CONSUME-BOUNDARY-SELECTION-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-RECIPEVERIFIER-COND-RECIPE-VALIDATE-ONLY-CONSUME-001", "bad prerequisite")

candidates = {row.get("candidate"): row for row in fixture.get("candidate_consumers") or []}
selected = candidates.get("ObserveOnlyCondRecipeMatcherInputSnapshot") or {}
need(selected.get("selected") is True, "observe-only snapshot consumer must be selected")
need(selected.get("selected_next_card") == "MIRBUILDER-RECIPEMATCHER-COND-RECIPE-OBSERVE-ONLY-INPUT-SNAPSHOT-001", "bad selected next")
for rejected in [
    "DirectRecipeMatcherCondRecipeAuthority",
    "ShapeControlCondRecipeConsumer",
    "RouteSelectionCondRecipeConsumer",
]:
    need(candidates.get(rejected, {}).get("selected") is False, f"{rejected} must be rejected")

contract = fixture.get("selected_contract") or {}
need(contract.get("consumer") == "ProgramJsonRecipeMatcherExecutionBoundaryBox", "bad consumer")
need(contract.get("mode") == "observe_only_input_snapshot", "bad mode")
need(contract.get("recipe_matcher_input_authority") is False, "matcher authority must not be claimed")
need(contract.get("full_recipe_matcher_execution") is False, "full matcher execution must not be claimed")
need(contract.get("route_selection_change") is False, "route selection must not change")
need(contract.get("lowering_behavior_change") is False, "lowering must not change")
need(contract.get("runtime_route_switch") is False, "runtime switch must not change")

claims = fixture.get("claims") or {}
for key in [
    "recipematcher_cond_recipe_input_consume_boundary_selection",
    "selected_observe_only_cond_recipe_input_snapshot",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "direct_recipematcher_cond_recipe_authority",
    "recipe_matcher_input_authority",
    "full_recipe_matcher_execution",
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

need("ProgramJsonRecipeMatcherExecutionBoundaryBox" in matcher, "RecipeMatcher boundary source missing")
need('"cond_recipe"' not in shape_control, "Shape control must not consume cond_recipe in selection card")
for needle in [
    "MIRBUILDER-RECIPEMATCHER-COND-RECIPE-INPUT-CONSUME-BOUNDARY-SELECTION-001",
    "MIRBUILDER-RECIPEMATCHER-COND-RECIPE-OBSERVE-ONLY-INPUT-SNAPSHOT-001",
]:
    need(needle in task_order, f"task-order missing: {needle}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-recipematcher-cond-recipe-input-consume-boundary-selection-guard-v0
token=MIRBUILDER-RECIPEMATCHER-COND-RECIPE-INPUT-CONSUME-BOUNDARY-SELECTION-001
selected_consumer=ObserveOnlyCondRecipeMatcherInputSnapshot
selected_next_card=MIRBUILDER-RECIPEMATCHER-COND-RECIPE-OBSERVE-ONLY-INPUT-SNAPSHOT-001
recipematcher_cond_recipe_input_consume_boundary_selection=1
selected_observe_only_cond_recipe_input_snapshot=1
direct_recipematcher_cond_recipe_authority=0
recipe_matcher_input_authority=0
full_recipe_matcher_execution=0
bool_recipe_lowering=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
summary=ok
REPORT
