#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-recipeitem-condition-slot-bool-recipe-sidecar-bridge-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-recipeitem-condition-slot-bool-recipe-sidecar-bridge-v0.json"
RECIPE_ITEM="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/recipe_item_box.hako"
BOOL_RECIPE="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/bool_recipe_box.hako"
SELECTION_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_recipeitem_condition_slot_bool_recipe_bridge_selection_guard.sh"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" sha256sum
guard_require_files "$TAG" "$FIXTURE" "$RECIPE_ITEM" "$BOOL_RECIPE" "$SELECTION_GATE" "$TASK_ORDER" "$HAKO_BIN"

SELECTION_OUT="$(guard_cached_run "$TAG" bash "$SELECTION_GATE")"
if ! grep -q '^selected_bridge=OptionalCondRecipeSidecar$' <<<"$SELECTION_OUT"; then
  printf '%s\n' "$SELECTION_OUT" >&2
  guard_fail "$TAG" "RecipeItem condition-slot selection prerequisite is not green"
fi

python3 - "$FIXTURE" "$RECIPE_ITEM" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
impl = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderRecipeItemConditionSlotBoolRecipeSidecarBridgeV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-SIDECAR-BRIDGE-001", "bad token")
need(fixture.get("owner") == "RecipeItemBox", "bad owner")
need(fixture.get("prerequisite") == "MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-BRIDGE-SELECTION-001", "bad prerequisite")

contract = fixture.get("sidecar_contract") or {}
need(contract.get("legacy_cond_facts_required") is True, "legacy cond_facts must remain required")
need(contract.get("cond_recipe_optional") is True, "cond_recipe must be optional")
need(contract.get("cond_recipe_validated_by") == "BoolRecipeBox.is_valid_compare", "bad cond_recipe validator")
need(contract.get("legacy_constructors_behavior_change") is False, "legacy constructors must not change")
need(contract.get("verifier_behavior_change") is False, "verifier behavior must not change")
need(contract.get("lowering_behavior_change") is False, "lowering behavior must not change")

need([row.get("row_id") for row in fixture.get("rows") or []] == [
    "loop_with_cond_recipe_sidecar",
    "if_with_cond_recipe_sidecar",
    "legacy_loop_without_cond_recipe",
], "row set drift")

claims = fixture.get("claims") or {}
for key in [
    "recipeitem_cond_recipe_sidecar_bridge",
    "optional_cond_recipe_sidecar",
    "legacy_cond_facts_required",
    "recipe_item_attachment",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "verifier_behavior_change",
    "lowering_behavior_change",
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
    "if_item_with_cond_recipe(cond_facts, cond_recipe, then_item, else_item)",
    "loop_item_with_cond_recipe(cond_facts, cond_recipe, body_item)",
    'item.set("cond_recipe", me._cond_recipe_or_empty(cond_recipe))',
    "cond_recipe_present(item)",
    "cond_recipe_summary(item)",
    "BoolRecipeBox.is_valid_compare",
    "BoolRecipeBox.summary",
    '"cond_facts" => me._map_or_empty(cond_facts)',
]:
    need(needle in impl, f"implementation missing token: {needle}")

for forbidden in [
    "RecipeMatcherBox",
    "emit_mir",
    "route_registry",
    "PlanLowerer",
]:
    need(forbidden not in impl, f"forbidden implementation token: {forbidden}")

for needle in [
    "MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-SIDECAR-BRIDGE-001",
    "MIRBUILDER-PROGRAMJSON-RECIPEITEM-COND-RECIPE-PRODUCER-WIRING-SELECTION-001",
]:
    need(needle in task_order, f"task-order missing: {needle}")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-recipeitem-sidecar.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/recipeitem_sidecar.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/recipeitem_sidecar.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import os
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

lines = [
    "using lang.compiler.mirbuilder.recipe.recipe_item_box as RecipeItemBox",
    "using lang.compiler.mirbuilder.recipe.bool_recipe_box as BoolRecipeBox",
    "",
    "static box Main {",
    "  main() {",
    "    local cache_hash = " + json.dumps(os.environ.get("HAKO_RECIPEITEM_SIDECAR_HASH", "")),
    "    if cache_hash == \"__never__\" { print(cache_hash) }",
    "    local cond_facts = %{\"cond_kind\" => \"VarLeInt\", \"cond_var_name\" => \"i\", \"cond_rhs_int\" => 3}",
    "    local cond_recipe = BoolRecipeBox.from_numeric_compare_codes(1, 1, 2, 1, 3, 0)",
    "    local empty_seq = RecipeItemBox.seq([])",
    "    local loop_item = RecipeItemBox.loop_item_with_cond_recipe(cond_facts, cond_recipe, empty_seq)",
    "    local loop_kind = \"Other\"",
    "    if RecipeItemBox.kind_is(loop_item, \"Loop\") == 1 { loop_kind = \"Loop\" }",
    "    print(\"loop_with_cond_recipe_sidecar:\" + loop_kind + \":\" + RecipeItemBox.cond_recipe_summary(loop_item))",
    "    local if_item = RecipeItemBox.if_item_with_cond_recipe(cond_facts, cond_recipe, empty_seq, empty_seq)",
    "    local if_kind = \"Other\"",
    "    if RecipeItemBox.kind_is(if_item, \"If\") == 1 { if_kind = \"If\" }",
    "    print(\"if_with_cond_recipe_sidecar:\" + if_kind + \":\" + RecipeItemBox.cond_recipe_summary(if_item))",
    "    local legacy_loop = RecipeItemBox.loop_item(cond_facts, empty_seq)",
    "    local legacy_kind = \"Other\"",
    "    if RecipeItemBox.kind_is(legacy_loop, \"Loop\") == 1 { legacy_kind = \"Loop\" }",
    "    print(\"legacy_loop_without_cond_recipe:\" + legacy_kind + \":\" + RecipeItemBox.cond_recipe_summary(legacy_loop))",
    "    return 0",
    "  }",
    "}",
    "",
]

expected_lines = [
    f"{row['row_id']}:{row['expected_item_kind']}:{row['expected_summary']}"
    for row in fixture["rows"]
]

app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("\n".join(expected_lines) + "\n", encoding="utf-8")
PY

export HAKO_RECIPEITEM_SIDECAR_HASH="$(sha256sum "$RECIPE_ITEM" "$BOOL_RECIPE" | sha256sum | awk '{ print $1 }')"

bash "$HAKO_BIN" --backend mir --verify "$RECIPE_ITEM" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit RecipeItem sidecar executable"
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
    print("[recipeitem/cond-recipe-sidecar] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-recipeitem-condition-slot-bool-recipe-sidecar-bridge-gate-v0
token=MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-SIDECAR-BRIDGE-001
owner=RecipeItemBox
row_count=3
recipeitem_cond_recipe_sidecar_bridge=1
optional_cond_recipe_sidecar=1
legacy_cond_facts_required=1
recipe_item_attachment=1
verifier_behavior_change=0
lowering_behavior_change=0
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
