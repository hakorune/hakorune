#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-condskeleton-ifcond-consultation-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-condskeleton-ifcond-consultation-v0.json"
COND_PROFILE="$ROOT_DIR/src/mir/policies/cond_profile.rs"
BOOL_RECIPE="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/bool_recipe_box.hako"
RECIPE_ITEM="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/recipe_item_box.hako"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$COND_PROFILE" "$BOOL_RECIPE" "$RECIPE_ITEM" "$TASK_ORDER"

python3 - "$FIXTURE" "$COND_PROFILE" "$BOOL_RECIPE" "$RECIPE_ITEM" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
cond_profile = Path(sys.argv[2]).read_text(encoding="utf-8")
bool_recipe = Path(sys.argv[3]).read_text(encoding="utf-8")
recipe_item = Path(sys.argv[4]).read_text(encoding="utf-8")
task_order = Path(sys.argv[5]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderCondSkeletonIfCondConsultationV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-CONDSKELETON-IFCOND-CONSULTATION-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-RUST-LOOP-CONDITION-SHAPE-EQ-NE-CANON-001", "bad prerequisite")

inventory = fixture.get("inventory") or {}
need(inventory.get("rust_cond_skeleton_current") == "LoopCond only", "bad skeleton inventory")
need(inventory.get("hako_condition_recipe_surface") == "BoolRecipe::Compare sidecar on RecipeItem", "bad hako surface inventory")

candidates = {row.get("name"): row for row in fixture.get("candidates") or []}
need(candidates["AddCondSkeletonIfCondNow"].get("selected") is False, "IfCond-now must not be selected")
selected = candidates["DeferIfCondAndContinueBoolRecipePublication"]
need(selected.get("selected") is True, "defer candidate must be selected")
need(selected.get("selected_next_card") == "MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-PARITY-001", "bad selected next")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "DeferCondSkeletonIfCond", "bad decision")
need(decision.get("selected_next_card") == "MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-PARITY-001", "bad decision next")

claims = fixture.get("claims") or {}
for key in ["condskeleton_ifcond_deferred", "bool_recipe_publication_next"]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "condskeleton_ifcond_added",
    "rust_cond_profile_authority_expanded",
    "hako_consumer_change",
    "programjson_consumer_change",
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

need("pub enum CondSkeleton" in cond_profile, "CondSkeleton enum missing")
need("LoopCond" in cond_profile, "LoopCond missing")
need("IfCond" not in cond_profile, "IfCond must remain deferred")
need("static box BoolRecipeBox" in bool_recipe, "BoolRecipeBox missing")
need("cmp_code >= 1 && cmp_code <= 6" in bool_recipe, "BoolRecipe six-op vocabulary missing")
need("if_item_with_cond_recipe" in recipe_item, "If cond_recipe constructor missing")
need("loop_item_with_cond_recipe" in recipe_item, "Loop cond_recipe constructor missing")
need("MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-PARITY-001" in task_order, "task-order missing publication parity")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-condskeleton-ifcond-consultation-guard-v0
token=MIRBUILDER-CONDSKELETON-IFCOND-CONSULTATION-001
decision=DeferCondSkeletonIfCond
condskeleton_ifcond_added=0
condskeleton_ifcond_deferred=1
bool_recipe_publication_next=1
rust_cond_profile_authority_expanded=0
hako_consumer_change=0
programjson_consumer_change=0
recipe_matcher_input_authority=0
bool_recipe_lowering=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-PARITY-001
summary=ok
REPORT
