#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-recipeverifier-cond-recipe-validate-only-consume-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-recipeverifier-cond-recipe-validate-only-consume-v0.json"
VERIFIER="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/recipe_verifier_box.hako"
MATCHER="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipematcher_execution_boundary.hako"
SHAPE_CONTROL="$ROOT_DIR/lang/src/compiler/mirbuilder/mir_json_v0_shape_box_recipe_control.hako"
SELECTION_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_recipeitem_cond_recipe_consume_boundary_selection_guard.sh"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$VERIFIER" "$MATCHER" "$SHAPE_CONTROL" "$SELECTION_GATE" "$TASK_ORDER" "$HAKO_BIN"

SELECTION_OUT="$(guard_cached_run "$TAG" bash "$SELECTION_GATE")"
if ! grep -q '^selected_consumer=RecipeVerifierValidateOnlyConsumer$' <<<"$SELECTION_OUT"; then
  printf '%s\n' "$SELECTION_OUT" >&2
  guard_fail "$TAG" "RecipeVerifier validate-only selection prerequisite is not green"
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
need(fixture.get("kind") == "MirBuilderRecipeVerifierCondRecipeValidateOnlyConsumeV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-RECIPEVERIFIER-COND-RECIPE-VALIDATE-ONLY-CONSUME-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-RECIPEITEM-COND-RECIPE-CONSUME-BOUNDARY-SELECTION-001", "bad prerequisite")
need(fixture.get("owner") == "RecipeVerifierBox._verify_cond_recipe", "bad owner")

contract = fixture.get("implementation_contract") or {}
need(contract.get("consumer") == "RecipeVerifierBox", "bad consumer")
need(contract.get("allowed_effect") == "reject malformed cond_recipe only", "bad allowed effect")
need(contract.get("legacy_items_without_cond_recipe_still_valid") is True, "legacy item contract missing")
need(contract.get("valid_cond_recipe_does_not_change_port_sig") is True, "port sig contract missing")
need(contract.get("recipe_matcher_input_authority") is False, "matcher authority must not change")
need(contract.get("lowering_behavior_change") is False, "lowering must not change")
need(contract.get("route_selection_change") is False, "route selection must not change")

claims = fixture.get("claims") or {}
for key in [
    "recipeverifier_cond_recipe_validate_only_consume",
    "malformed_cond_recipe_rejected",
    "legacy_recipeitem_without_cond_recipe_still_valid",
    "valid_cond_recipe_port_sig_unchanged",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
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
    "using lang.compiler.mirbuilder.recipe.bool_recipe_box as BoolRecipeBox",
    "_verify_cond_recipe(item, tag): MapBox",
    'BoxHelpers.map_get(item, "cond_recipe")',
    "BoolRecipeBox.is_valid_compare(cond_recipe)",
    '"[recipe_verifier] invalid cond_recipe"',
]:
    need(needle in verifier, f"verifier missing: {needle}")
need('"cond_recipe"' not in matcher, "RecipeMatcher must not consume cond_recipe in validate-only card")
need('"cond_recipe"' not in shape_control, "Shape control must not consume cond_recipe in validate-only card")
for needle in [
    "MIRBUILDER-RECIPEVERIFIER-COND-RECIPE-VALIDATE-ONLY-CONSUME-001",
    "MIRBUILDER-RECIPEMATCHER-COND-RECIPE-INPUT-CONSUME-BOUNDARY-SELECTION-001",
]:
    need(needle in task_order, f"task-order missing: {needle}")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-recipeverifier-cond-recipe.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/recipeverifier_cond_recipe.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/recipeverifier_cond_recipe.exe"
EMIT_LOG="$TMP_DIR/emit.log"

cat >"$APP" <<'HAKO'
using selfhost.shared.common.box_helpers as BoxHelpers
using lang.compiler.mirbuilder.recipe.bool_recipe_box as BoolRecipeBox
using lang.compiler.mirbuilder.recipe.recipe_item_box as RecipeItemBox
using lang.compiler.mirbuilder.recipe.recipe_verifier_box as RecipeVerifierBox

static box Main {
  main() {
    local valid_recipe = BoolRecipeBox.from_numeric_compare_codes(1, 1, 2, 1, 3, 0)
    local bad_recipe = BoolRecipeBox.unsupported(77)

    local legacy_loop = RecipeItemBox.loop_item(%{}, RecipeItemBox.seq([]))
    local legacy_res = RecipeVerifierBox.verify(legacy_loop, "[test]")
    local legacy_sig = BoxHelpers.map_get(legacy_res, "port_sig")
    print("legacy_loop_without_cond_recipe_still_valid:err=" + ("" + BoxHelpers.map_get(legacy_res, "err")) + ";def_count=" + ("" + BoxHelpers.map_get(legacy_sig, "def_count")) + ";update_count=" + ("" + BoxHelpers.map_get(legacy_sig, "update_count")))

    local valid_loop = RecipeItemBox.loop_item_with_cond_recipe(%{}, valid_recipe, RecipeItemBox.seq([]))
    local valid_res = RecipeVerifierBox.verify(valid_loop, "[test]")
    local valid_sig = BoxHelpers.map_get(valid_res, "port_sig")
    print("loop_with_valid_cond_recipe_is_verified:err=" + ("" + BoxHelpers.map_get(valid_res, "err")) + ";def_count=" + ("" + BoxHelpers.map_get(valid_sig, "def_count")) + ";update_count=" + ("" + BoxHelpers.map_get(valid_sig, "update_count")))

    local bad_loop = RecipeItemBox.loop_item(%{}, RecipeItemBox.seq([]))
    bad_loop.set("cond_recipe", bad_recipe)
    local bad_loop_res = RecipeVerifierBox.verify(bad_loop, "[test]")
    local bad_loop_sig = BoxHelpers.map_get(bad_loop_res, "port_sig")
    print("loop_with_malformed_cond_recipe_rejected:err=" + ("" + BoxHelpers.map_get(bad_loop_res, "err")) + ";err_line_match=" + ("" + BoxHelpers.same_token(BoxHelpers.map_get(bad_loop_res, "err_line"), "[test][recipe_verifier] invalid cond_recipe")) + ";def_count=" + ("" + BoxHelpers.map_get(bad_loop_sig, "def_count")) + ";update_count=" + ("" + BoxHelpers.map_get(bad_loop_sig, "update_count")))

    local bad_if = RecipeItemBox.if_item(%{}, RecipeItemBox.seq([]), RecipeItemBox.seq([]))
    bad_if.set("cond_recipe", bad_recipe)
    local bad_if_res = RecipeVerifierBox.verify(bad_if, "[test]")
    local bad_if_sig = BoxHelpers.map_get(bad_if_res, "port_sig")
    print("if_with_malformed_cond_recipe_rejected:err=" + ("" + BoxHelpers.map_get(bad_if_res, "err")) + ";err_line_match=" + ("" + BoxHelpers.same_token(BoxHelpers.map_get(bad_if_res, "err_line"), "[test][recipe_verifier] invalid cond_recipe")) + ";def_count=" + ("" + BoxHelpers.map_get(bad_if_sig, "def_count")) + ";update_count=" + ("" + BoxHelpers.map_get(bad_if_sig, "update_count")))
    return 0
  }
}
HAKO

cat >"$EXPECTED" <<'TEXT'
legacy_loop_without_cond_recipe_still_valid:err=0;def_count=0;update_count=0
loop_with_valid_cond_recipe_is_verified:err=0;def_count=0;update_count=0
loop_with_malformed_cond_recipe_rejected:err=1;err_line_match=1;def_count=0;update_count=0
if_with_malformed_cond_recipe_rejected:err=1;err_line_match=1;def_count=0;update_count=0
TEXT

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit RecipeVerifier cond_recipe executable"
fi

chmod +x "$EXE"
"$EXE" >"$ACTUAL.raw"

python3 - "$EXPECTED" "$ACTUAL.raw" "$ACTUAL" <<'PY'
import sys
from pathlib import Path

expected = Path(sys.argv[1]).read_text(encoding="utf-8").splitlines()
raw = Path(sys.argv[2]).read_text(encoding="utf-8").splitlines()
actual_path = Path(sys.argv[3])
actual = [line.strip() for line in raw if line.strip() and not line.startswith("Result:")]
actual_path.write_text("\n".join(actual) + "\n", encoding="utf-8")
if actual != expected:
    print("[recipeverifier/cond-recipe-validate-only] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-recipeverifier-cond-recipe-validate-only-consume-gate-v0
token=MIRBUILDER-RECIPEVERIFIER-COND-RECIPE-VALIDATE-ONLY-CONSUME-001
owner=RecipeVerifierBox._verify_cond_recipe
row_count=4
recipeverifier_cond_recipe_validate_only_consume=1
malformed_cond_recipe_rejected=1
legacy_recipeitem_without_cond_recipe_still_valid=1
valid_cond_recipe_port_sig_unchanged=1
recipe_matcher_input_authority=0
bool_recipe_lowering=0
mir_cmp_emission=0
branch_emission=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
summary=ok
REPORT
