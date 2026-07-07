#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-compare-reader-shared-canon-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-compare-reader-shared-canon-v0.json"
OWNER="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_compare_reader_box.hako"
PREV_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_compare_reader_shared_canon_task_sequence_guard.sh"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$OWNER" "$PREV_GATE" "$TASK_ORDER" "$HAKO_BIN"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GATE")"
if ! grep -q '^compare_reader_task_sequence_selected=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "shared compare reader task sequence prerequisite is not green"
fi

python3 - "$FIXTURE" "$OWNER" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
owner = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderProgramJsonCompareReaderSharedCanonV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-PROGRAMJSON-COMPARE-READER-SHARED-CANON-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-PROGRAMJSON-COMPARE-READER-SHARED-CANON-TASK-SEQUENCE-001", "bad prerequisite")
need(fixture.get("owner") == "ProgramJsonCompareReaderBox.read_var_int_compare", "bad owner")

contract = fixture.get("contract") or {}
need(contract.get("scope") == "Var op Int only", "bad scope")
need(contract.get("operators") == ["<", "<=", ">", ">=", "==", "!="], "bad operator list")
need(contract.get("analysis_only") is True, "analysis_only must be true")
need(contract.get("consumer_change") is False, "consumer_change must be false")

rows = fixture.get("rows") or []
need(len(rows) == 6, "expected six operator rows")
need([row["row_id"] for row in rows] == [
    "var_lt_int",
    "var_le_int",
    "var_gt_int",
    "var_ge_int",
    "var_eq_int",
    "var_ne_int",
], "bad row order")

claims = fixture.get("claims") or {}
for key in [
    "programjson_shared_compare_reader",
    "compare_reader_var_op_int",
    "cmp_code_6_vocab_present",
    "analysis_only_compare_view",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "consumer_change",
    "if_cond_recipe_attached",
    "if_compare_operator_expansion",
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
    "read_var_int_compare(program_json, compare_start): MapBox",
    "ProgramJsonCompareReaderCodeMapV1",
    '"analysis_only" => 1',
    '"bound_kind_code" => bound_kind',
    '"constant_compare" => 0',
    "if me._token_eq(op, \"<\") == 1 { return 1 }",
    "if me._token_eq(op, \"<=\") == 1 { return 2 }",
    "if me._token_eq(op, \">\") == 1 { return 3 }",
    "if me._token_eq(op, \">=\") == 1 { return 4 }",
    "if me._token_eq(op, \"==\") == 1 { return 5 }",
    "if me._token_eq(op, \"!=\") == 1 { return 6 }",
]:
    need(needle in owner, f"owner missing: {needle}")
for forbidden in [
    "if_item_with_cond_recipe(",
    "loop_item_with_cond_recipe(",
    "RecipeMatcherBox",
    "PlanLowerer",
]:
    need(forbidden not in owner, f"owner has forbidden downstream dependency: {forbidden}")
for needle in [
    "MIRBUILDER-PROGRAMJSON-COMPARE-READER-SHARED-CANON-001",
    "MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-EQ-BEHAVIOR-PRESERVING-001",
]:
    need(needle in task_order, f"task-order missing: {needle}")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-compare-reader-shared-canon.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/compare_reader_shared_canon.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/compare_reader_shared_canon.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

lines = [
    "using lang.compiler.mirbuilder.program_json_compare_reader_box as ProgramJsonCompareReaderBox",
    "",
    "static box Main {",
    "  main() {",
]
for row in fixture["rows"]:
    row_id = row["row_id"]
    compare_json = json.dumps(row["compare_json"])
    lines.extend([
        f"    local fields_{row_id} = ProgramJsonCompareReaderBox.read_var_int_compare({compare_json}, 0)",
        f"    print(\"{row_id}:\" + ProgramJsonCompareReaderBox.code_map_summary(fields_{row_id}))",
    ])
lines.extend([
    "    return 0",
    "  }",
    "}",
    "",
])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text(
    "\n".join(f"{row['row_id']}:{row['expected_summary']}" for row in fixture["rows"]) + "\n",
    encoding="utf-8",
)
PY

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit shared compare reader executable"
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
    print("[programjson/compare-reader-shared-canon] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-compare-reader-shared-canon-gate-v0
token=MIRBUILDER-PROGRAMJSON-COMPARE-READER-SHARED-CANON-001
owner=ProgramJsonCompareReaderBox.read_var_int_compare
row_count=6
programjson_shared_compare_reader=1
compare_reader_var_op_int=1
cmp_code_6_vocab_present=1
analysis_only_compare_view=1
consumer_change=0
if_cond_recipe_attached=0
if_compare_operator_expansion=0
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
