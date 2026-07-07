#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-loop-cond-recipe-constructor-cleanup-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-loop-cond-recipe-constructor-cleanup-v0.json"
LOOP_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/loop_stmt_handler.hako"
PREV_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_loop_nested_if_cond_recipe_bridge_gate.sh"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$LOOP_HANDLER" "$PREV_GATE" "$TASK_ORDER" "$HAKO_BIN"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GATE")"
if ! grep -q '^loop_nested_if_cond_recipe=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "Loop nested If cond_recipe prerequisite is not green"
fi

python3 - "$FIXTURE" "$LOOP_HANDLER" "$TASK_ORDER" <<'PY'
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
need(fixture.get("kind") == "MirBuilderProgramJsonLoopCondRecipeConstructorCleanupV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-PROGRAMJSON-LOOP-COND-RECIPE-CONSTRUCTOR-CLEANUP-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-COND-RECIPE-BRIDGE-001", "bad prerequisite")

contract = fixture.get("cleanup_contract") or {}
need(contract.get("manual_set_removed") is True, "manual set must be removed")
need(contract.get("constructor") == "RecipeItemBox.loop_item_with_cond_recipe", "bad constructor")
need(contract.get("behavior_preserved") is True, "behavior must be preserved")
need(contract.get("new_operator") is False, "must not add operator")

claims = fixture.get("claims") or {}
for key in ["loop_item_with_cond_recipe_constructor_used", "manual_cond_recipe_set_removed", "behavior_preserved"]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "new_loop_condition_operator",
    "rust_loop_condition_shape_eq_ne",
    "condskeleton_ifcond",
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

need("RecipeItemBox.loop_item_with_cond_recipe(cond_facts, cond_recipe, body_seq)" in impl, "constructor not used")
need('loop_item.set("cond_recipe", cond_recipe)' not in impl, "manual cond_recipe set remains")
need("RecipeItemBox.loop_item(cond_facts, body_seq)" in impl, "legacy fallback missing")
for needle in [
    "MIRBUILDER-PROGRAMJSON-LOOP-COND-RECIPE-CONSTRUCTOR-CLEANUP-001",
    "MIRBUILDER-RUST-LOOP-CONDITION-SHAPE-EQ-NE-CANON-001",
]:
    need(needle in task_order, f"task-order missing: {needle}")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-loop-cond-constructor-cleanup.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/loop_cond_constructor_cleanup.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/loop_cond_constructor_cleanup.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

lines = [
    "using selfhost.shared.common.string_helpers as StringHelpers",
    "using selfhost.shared.common.box_helpers as BoxHelpers",
    "using lang.compiler.mirbuilder.program_json_v0_phase_state_box as ProgramJsonV0PhaseStateBox",
    "using lang.compiler.mirbuilder.recipe.recipe_item_box as RecipeItemBox",
    "",
    "static box Main {",
    "  main() {",
]
expected_lines = []
for idx, row in enumerate(fixture["rows"]):
    out = f"out_{idx}"
    root = f"root_{idx}"
    items = f"items_{idx}"
    loop_item = f"loop_item_{idx}"
    lines.extend([
        f"    local {out} = ProgramJsonV0PhaseStateBox.parse({json.dumps(row['program_json'])}, \"[test]\")",
        f"    local {root} = BoxHelpers.map_get({out}, \"recipe_root\")",
        f"    local {items} = BoxHelpers.map_get({root}, \"items\")",
        f"    local {loop_item} = BoxHelpers.array_get({items}, 1)",
        f"    print(\"{row['row_id']}:err=\" + StringHelpers.int_to_str(BoxHelpers.map_get({out}, \"err\"))",
        f"      + \";loop_cond_recipe_present=\" + StringHelpers.int_to_str(RecipeItemBox.cond_recipe_present({loop_item}))",
        f"      + \";\" + RecipeItemBox.cond_recipe_summary({loop_item}))",
    ])
    expected_lines.append(row["expected_summary"])
lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("\n".join(expected_lines) + "\n", encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$LOOP_HANDLER" >/dev/null
if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit Loop cond constructor cleanup executable"
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
    print("[loop-cond/constructor-cleanup] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-loop-cond-recipe-constructor-cleanup-gate-v0
token=MIRBUILDER-PROGRAMJSON-LOOP-COND-RECIPE-CONSTRUCTOR-CLEANUP-001
owner=LoopStmtHandler top-level Loop RecipeItem construction
row_count=1
loop_item_with_cond_recipe_constructor_used=1
manual_cond_recipe_set_removed=1
behavior_preserved=1
new_loop_condition_operator=0
rust_loop_condition_shape_eq_ne=0
recipe_matcher_input_authority=0
bool_recipe_lowering=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
summary=ok
REPORT
