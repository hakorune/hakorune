#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-if-cond-recipe-first-non-eq-row-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-if-cond-recipe-first-non-eq-row-v0.json"
IF_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/if_stmt_handler.hako"
PREV_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_if_cond_recipe_eq_behavior_preserving_gate.sh"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$IF_HANDLER" "$PREV_GATE" "$TASK_ORDER" "$HAKO_BIN"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GATE")"
if ! grep -q '^if_eq_behavior_preserved=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "If Eq behavior-preserving prerequisite is not green"
fi

python3 - "$FIXTURE" "$IF_HANDLER" "$TASK_ORDER" <<'PY'
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
need(fixture.get("kind") == "MirBuilderProgramJsonIfCondRecipeFirstNonEqRowV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-FIRST-NON-EQ-ROW-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-EQ-BEHAVIOR-PRESERVING-001", "bad prerequisite")

scope = fixture.get("row_scope") or {}
need(scope.get("operator") == "!=", "first non-Eq operator must be !=")
need(scope.get("other_non_eq_operators_claimed") is False, "other non-Eq ops must remain unclaimed")

claims = fixture.get("claims") or {}
for key in [
    "if_cond_recipe_ne_row",
    "if_first_non_eq_row",
    "shared_compare_reader_used",
    "legacy_cond_facts_var_ne_int",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "if_accepts_all_6_compare_operators",
    "if_lt_le_gt_ge_rows",
    "loop_nested_if_cond_recipe",
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

for needle in [
    "BoxHelpers.same_token(op, \"!=\")",
    'if cmp_code == 6 { return "VarNeInt" }',
    "ProgramJsonCompareReaderBox.read_var_int_compare(program_json, cond_start)",
    "BoolRecipeBox.from_numeric_compare_code_map(cond_reader)",
]:
    need(needle in impl, f"IfStmtHandler missing token: {needle}")
for forbidden in ["PlanLowerer", "route_registry", "emit_mir"]:
    need(forbidden not in impl, f"forbidden implementation token: {forbidden}")
for needle in [
    "MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-FIRST-NON-EQ-ROW-001",
    "MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-COND-RECIPE-BRIDGE-001",
]:
    need(needle in task_order, f"task-order missing: {needle}")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-if-cond-recipe-ne.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/if_cond_recipe_ne.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/if_cond_recipe_ne.exe"
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
    if_item = f"if_item_{idx}"
    facts = f"facts_{idx}"
    lines.extend([
        f"    local {out} = ProgramJsonV0PhaseStateBox.parse({json.dumps(row['program_json'])}, \"[test]\")",
        f"    local {root} = BoxHelpers.map_get({out}, \"recipe_root\")",
        f"    local {items} = BoxHelpers.map_get({root}, \"items\")",
        f"    local {if_item} = BoxHelpers.array_get({items}, 1)",
        f"    local {facts} = BoxHelpers.map_get({if_item}, \"cond_facts\")",
        f"    print(\"{row['row_id']}:err=\" + StringHelpers.int_to_str(BoxHelpers.map_get({out}, \"err\"))",
        f"      + \";cond_kind_ne=\" + StringHelpers.int_to_str(BoxHelpers.same_token(BoxHelpers.map_get({facts}, \"cond_kind\"), \"VarNeInt\"))",
        f"      + \";cond_rhs_int=\" + StringHelpers.int_to_str(BoxHelpers.map_get({facts}, \"cond_rhs_int\"))",
        f"      + \";cond_recipe_present=\" + StringHelpers.int_to_str(RecipeItemBox.cond_recipe_present({if_item}))",
        f"      + \";\" + RecipeItemBox.cond_recipe_summary({if_item}))",
    ])
    expected_lines.append(row["expected_summary"])
lines.extend([
    "    return 0",
    "  }",
    "}",
    "",
])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("\n".join(expected_lines) + "\n", encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$IF_HANDLER" >/dev/null
if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit If cond_recipe first non-Eq executable"
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
    print("[if-stmt/cond-recipe-first-non-eq-row] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-if-cond-recipe-first-non-eq-row-gate-v0
token=MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-FIRST-NON-EQ-ROW-001
owner=IfStmtHandler
row_count=1
if_cond_recipe_ne_row=1
if_first_non_eq_row=1
shared_compare_reader_used=1
legacy_cond_facts_var_ne_int=1
if_accepts_all_6_compare_operators=0
if_lt_le_gt_ge_rows=0
loop_nested_if_cond_recipe=0
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
