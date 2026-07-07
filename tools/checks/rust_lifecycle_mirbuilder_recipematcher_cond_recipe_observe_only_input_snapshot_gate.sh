#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-recipematcher-cond-recipe-observe-only-input-snapshot-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-recipematcher-cond-recipe-observe-only-input-snapshot-v0.json"
MATCHER="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipematcher_execution_boundary.hako"
SELECTION_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_recipematcher_cond_recipe_input_consume_boundary_selection_guard.sh"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$MATCHER" "$SELECTION_GATE" "$TASK_ORDER" "$HAKO_BIN"

SELECTION_OUT="$(guard_cached_run "$TAG" bash "$SELECTION_GATE")"
if ! grep -q '^selected_observe_only_cond_recipe_input_snapshot=1$' <<<"$SELECTION_OUT"; then
  printf '%s\n' "$SELECTION_OUT" >&2
  guard_fail "$TAG" "RecipeMatcher cond_recipe input boundary selection prerequisite is not green"
fi

python3 - "$FIXTURE" "$MATCHER" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
matcher = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderRecipeMatcherCondRecipeObserveOnlyInputSnapshotV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-RECIPEMATCHER-COND-RECIPE-OBSERVE-ONLY-INPUT-SNAPSHOT-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-RECIPEMATCHER-COND-RECIPE-INPUT-CONSUME-BOUNDARY-SELECTION-001", "bad prerequisite")
need(fixture.get("owner") == "ProgramJsonRecipeMatcherExecutionBoundaryBox.cond_recipe_input_snapshot", "bad owner")

contract = fixture.get("snapshot_contract") or {}
need(contract.get("observe_only") is True, "snapshot must be observe-only")
for key in [
    "recipe_matcher_input_authority",
    "recipe_matcher_executed",
    "full_recipe_matcher_execution",
    "route_selection",
    "lowering_behavior_change",
    "runtime_route_switch",
]:
    need(contract.get(key) is False, f"forbidden contract claim drift: {key}")

claims = fixture.get("claims") or {}
for key in [
    "recipematcher_cond_recipe_observe_only_input_snapshot",
    "cond_recipe_matcher_input_snapshot_readonly",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "recipe_matcher_input_authority",
    "recipe_matcher_executed",
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

for needle in [
    "cond_recipe_input_snapshot(recipe_item): MapBox",
    'BoxHelpers.map_get(recipe_item, "cond_recipe")',
    "BoolRecipeBox.is_valid_compare(cond_recipe)",
    '"recipe_matcher_executed" => 0',
    '"route_selection" => 0',
    '"mir_lowering" => 0',
    '"runtime_route_switch" => 0',
]:
    need(needle in matcher, f"matcher boundary missing: {needle}")
for needle in [
    "MIRBUILDER-RECIPEMATCHER-COND-RECIPE-OBSERVE-ONLY-INPUT-SNAPSHOT-001",
    "MIRBUILDER-PROGRAMJSON-COMPARE-READER-SHARED-CANON-TASK-SEQUENCE-001",
]:
    need(needle in task_order, f"task-order missing: {needle}")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-recipematcher-cond-recipe-snapshot.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/recipematcher_cond_recipe_snapshot.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/recipematcher_cond_recipe_snapshot.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

lines = [
    "using lang.compiler.mirbuilder.program_json_recipematcher_execution_boundary as ProgramJsonRecipeMatcherExecutionBoundaryBox",
    "using lang.compiler.mirbuilder.recipe.bool_recipe_box as BoolRecipeBox",
    "using lang.compiler.mirbuilder.recipe.recipe_item_box as RecipeItemBox",
    "",
    "static box Main {",
    "  main() {",
    "    local valid_recipe = BoolRecipeBox.from_numeric_compare_codes(1, 1, 2, 1, 3, 0)",
    "    local valid_loop = RecipeItemBox.loop_item_with_cond_recipe(%{}, valid_recipe, RecipeItemBox.seq([]))",
    "    local missing_loop = RecipeItemBox.loop_item(%{}, RecipeItemBox.seq([]))",
    "    print(\"valid_loop_cond_recipe_snapshot:\" + ProgramJsonRecipeMatcherExecutionBoundaryBox.cond_recipe_input_summary(ProgramJsonRecipeMatcherExecutionBoundaryBox.cond_recipe_input_snapshot(valid_loop)))",
    "    print(\"missing_loop_cond_recipe_snapshot:\" + ProgramJsonRecipeMatcherExecutionBoundaryBox.cond_recipe_input_summary(ProgramJsonRecipeMatcherExecutionBoundaryBox.cond_recipe_input_snapshot(missing_loop)))",
    "    return 0",
    "  }",
    "}",
    "",
]
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text(
    "\n".join(f"{row['row_id']}:{row['expected_summary']}" for row in fixture["rows"]) + "\n",
    encoding="utf-8",
)
PY

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit cond_recipe matcher input snapshot executable"
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
    print("[recipematcher/cond-recipe-observe-only-input-snapshot] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-recipematcher-cond-recipe-observe-only-input-snapshot-gate-v0
token=MIRBUILDER-RECIPEMATCHER-COND-RECIPE-OBSERVE-ONLY-INPUT-SNAPSHOT-001
owner=ProgramJsonRecipeMatcherExecutionBoundaryBox.cond_recipe_input_snapshot
row_count=2
recipematcher_cond_recipe_observe_only_input_snapshot=1
cond_recipe_matcher_input_snapshot_readonly=1
recipe_matcher_input_authority=0
recipe_matcher_executed=0
full_recipe_matcher_execution=0
bool_recipe_lowering=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
summary=ok
REPORT
