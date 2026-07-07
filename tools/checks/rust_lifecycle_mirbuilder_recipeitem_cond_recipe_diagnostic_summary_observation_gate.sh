#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-recipeitem-cond-recipe-diagnostic-summary-observation-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-recipeitem-cond-recipe-diagnostic-summary-observation-v0.json"
EXPANDED_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-verifier-boundary-expanded-dto-coverage-parity-v0.json"
RECIPE_ITEM="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/recipe_item_box.hako"
VERIFIER="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/recipe_verifier_box.hako"
MATCHER="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipematcher_execution_boundary.hako"
SELECTION_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_recipeitem_cond_recipe_observation_boundary_selection_guard.sh"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$EXPANDED_FIXTURE" "$RECIPE_ITEM" "$VERIFIER" "$MATCHER" "$SELECTION_GATE" "$TASK_ORDER" "$HAKO_BIN"

SELECTION_OUT="$(guard_cached_run "$TAG" bash "$SELECTION_GATE")"
if ! grep -q '^selected_observer=RecipeItemDiagnosticSummaryObserver$' <<<"$SELECTION_OUT"; then
  printf '%s\n' "$SELECTION_OUT" >&2
  guard_fail "$TAG" "RecipeItem cond_recipe diagnostic observer selection prerequisite is not green"
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
need(fixture.get("kind") == "MirBuilderRecipeItemCondRecipeDiagnosticSummaryObservationV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-RECIPEITEM-COND-RECIPE-DIAGNOSTIC-SUMMARY-OBSERVATION-001", "bad token")
need(fixture.get("owner") == "RecipeItemBox.cond_recipe_summary", "bad owner")

contract = fixture.get("observation_contract") or {}
need(contract.get("observer") == "RecipeItemDiagnosticSummaryObserver", "bad observer")
need(contract.get("aot_required") is True, "AOT must be required")
need(contract.get("verifier_behavior_change") is False, "verifier behavior must not change")
need(contract.get("recipe_matcher_input_authority") is False, "matcher authority must not change")
need(contract.get("lowering_behavior_change") is False, "lowering behavior must not change")

claims = fixture.get("claims") or {}
for key in ["cond_recipe_diagnostic_summary_observation", "cond_recipe_deep_observation_implementation"]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
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

need("cond_recipe_summary(item)" in recipe_item, "RecipeItem cond_recipe diagnostic summary missing")
need('"cond_recipe"' not in matcher, "RecipeMatcher boundary must not read cond_recipe")
for needle in [
    "MIRBUILDER-RECIPEITEM-COND-RECIPE-DIAGNOSTIC-SUMMARY-OBSERVATION-001",
    "MIRBUILDER-RECIPEITEM-COND-RECIPE-CONSUME-BOUNDARY-SELECTION-001",
]:
    need(needle in task_order, f"task-order missing: {needle}")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-cond-recipe-summary.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/cond_recipe_summary.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/cond_recipe_summary.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$EXPANDED_FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
expanded = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
app = Path(sys.argv[3])
expected = Path(sys.argv[4])
expanded_by_id = {row["row_id"]: row["program_json"] for row in expanded.get("rows") or []}

lines = [
    "using selfhost.shared.common.box_helpers as BoxHelpers",
    "using lang.compiler.mirbuilder.program_json_v0_phase_state_box as ProgramJsonV0PhaseStateBox",
    "using lang.compiler.mirbuilder.recipe.recipe_item_box as RecipeItemBox",
    "",
    "static box Main {",
    "  main() {",
]
expected_lines = []
for idx, row in enumerate(fixture["rows"]):
    program = json.loads(json.dumps(expanded_by_id[row["source_program_row"]]))
    program["body"][1]["cond"] = row["loop_condition_patch"]
    program_json = json.dumps(program, separators=(",", ":"))
    out = f"out_{idx}"
    root = f"root_{idx}"
    items = f"items_{idx}"
    loop_item = f"loop_item_{idx}"
    lines.extend([
        f"    local {out} = ProgramJsonV0PhaseStateBox.parse({json.dumps(program_json)}, \"[test]\")",
        f"    local {root} = BoxHelpers.map_get({out}, \"recipe_root\")",
        f"    local {items} = BoxHelpers.map_get({root}, \"items\")",
        f"    local {loop_item} = BoxHelpers.array_get({items}, 1)",
        f"    print(\"{row['row_id']}:\" + RecipeItemBox.cond_recipe_summary({loop_item}))",
    ])
    expected_lines.append(f"{row['row_id']}:{row['expected_summary']}")
lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("\n".join(expected_lines) + "\n", encoding="utf-8")
PY

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit cond_recipe diagnostic observer executable"
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
    print("[recipeitem/cond-recipe-diagnostic-summary] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-recipeitem-cond-recipe-diagnostic-summary-observation-gate-v0
token=MIRBUILDER-RECIPEITEM-COND-RECIPE-DIAGNOSTIC-SUMMARY-OBSERVATION-001
owner=RecipeItemBox.cond_recipe_summary
row_count=1
cond_recipe_diagnostic_summary_observation=1
cond_recipe_deep_observation_implementation=1
verifier_cond_recipe_observer=0
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
