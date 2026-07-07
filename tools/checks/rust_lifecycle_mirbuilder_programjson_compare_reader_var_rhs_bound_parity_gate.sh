#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-compare-reader-var-rhs-bound-parity-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-compare-reader-var-rhs-bound-parity-v0.json"
READER="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_compare_reader_box.hako"
BOOL_RECIPE="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/bool_recipe_box.hako"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
PREV_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_compare_reader_followon_selection_guard.sh"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$READER" "$BOOL_RECIPE" "$TASK_ORDER" "$PREV_GATE" "$HAKO_BIN"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GATE")"
if ! grep -q '^var_rhs_bound_selected=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "Var rhs bound selection prerequisite is not green"
fi

python3 - "$FIXTURE" "$READER" "$BOOL_RECIPE" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
reader = Path(sys.argv[2]).read_text(encoding="utf-8")
bool_recipe = Path(sys.argv[3]).read_text(encoding="utf-8")
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderProgramJsonCompareReaderVarRhsBoundParityV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-PROGRAMJSON-COMPARE-READER-VAR-RHS-BOUND-PARITY-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-PROGRAMJSON-COMPARE-READER-FOLLOWON-SELECTION-001", "bad prerequisite")
need(fixture.get("owner") == "ProgramJsonCompareReaderBox.read_var_int_compare", "bad owner")

contract = fixture.get("contract") or {}
need(contract.get("scope") == "Var op Var rhs bound only", "bad scope")
need(contract.get("operators") == ["<", "<=", ">", ">=", "==", "!="], "bad operator list")
need(contract.get("bound_kind_code") == 2, "bad bound kind")
need(contract.get("analysis_only") is True, "analysis_only must be true")
need(contract.get("producer_change") is False, "producer_change must be false")

rows = fixture.get("rows") or []
need(len(rows) == 6, "expected six Var rhs rows")
need([row["row_id"] for row in rows] == [
    "var_lt_var",
    "var_le_var",
    "var_gt_var",
    "var_ge_var",
    "var_eq_var",
    "var_ne_var",
], "bad row order")
for row in rows:
    need("bound_kind_code=2" in row.get("expected_reader_summary", ""), "reader row must expect SymbolRef")
    need("bound_symbol_id=2" in row.get("expected_reader_summary", ""), "reader row must expect symbol n")
    need("bound_kind=SymbolRef" in row.get("expected_recipe_summary", ""), "recipe row must expect SymbolRef")

claims = fixture.get("claims") or {}
for key in [
    "compare_reader_var_rhs_bound_parity",
    "var_rhs_bound_implemented",
    "var_rhs_bound_symbol_ref",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
need(claims.get("row_count") == 6, "bad row_count claim")
for key in [
    "producer_change",
    "length_bound_support",
    "reversed_var_var_context_aware",
    "bool_recipe_lowering_executed",
    "mir_cmp_emission",
    "branch_emission",
    "basic_block_mutation",
    "value_id_allocation",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

for needle in [
    "read_var_int_compare(program_json, compare_start): MapBox",
    '"bound_kind_code" => bound_kind',
    '"bound_symbol_id" => bound_symbol_id',
    'if me._token_eq(rhs_type, "Var") == 1',
    'return me._err_map("unknown_rhs_symbol")',
]:
    need(needle in reader, f"reader missing: {needle}")
need("symbol_ref(symbol_id)" in bool_recipe, "BoolRecipe SymbolRef boundary missing")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-compare-reader-var-rhs.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/compare_reader_var_rhs.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/compare_reader_var_rhs.exe"
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
    "using lang.compiler.mirbuilder.recipe.bool_recipe_box as BoolRecipeBox",
    "",
    "static box Main {",
    "  main() {",
]
expected_lines = []
for idx, row in enumerate(fixture["rows"]):
    fields = f"fields_{idx}"
    recipe = f"recipe_{idx}"
    compare_json = json.dumps(row["compare_json"])
    lines.append(f"    local {fields} = ProgramJsonCompareReaderBox.read_var_int_compare({compare_json}, 0)")
    lines.append(f"    print(\"reader:{row['row_id']}:\" + ProgramJsonCompareReaderBox.code_map_summary({fields}))")
    lines.append(f"    local {recipe} = BoolRecipeBox.from_numeric_compare_code_map({fields})")
    lines.append(f"    print(\"recipe:{row['row_id']}:\" + BoolRecipeBox.summary({recipe}))")
    expected_lines.append(f"reader:{row['row_id']}:{row['expected_reader_summary']}")
    expected_lines.append(f"recipe:{row['row_id']}:{row['expected_recipe_summary']}")
lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("\n".join(expected_lines) + "\n", encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$READER" >/dev/null
if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit Var rhs compare reader executable"
fi

chmod +x "$EXE"
"$EXE" >"$ACTUAL.raw"

python3 - "$EXPECTED" "$ACTUAL.raw" "$ACTUAL" <<'PY'
import sys
from pathlib import Path

expected = Path(sys.argv[1]).read_text(encoding="utf-8").splitlines()
raw = Path(sys.argv[2]).read_text(encoding="utf-8").splitlines()
actual = [line.strip() for line in raw if line.strip() and not line.startswith("Result:")]
Path(sys.argv[3]).write_text("\n".join(actual) + "\n", encoding="utf-8")
if actual != expected:
    print("[compare-reader/var-rhs] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-compare-reader-var-rhs-bound-parity-gate-v0
token=MIRBUILDER-PROGRAMJSON-COMPARE-READER-VAR-RHS-BOUND-PARITY-001
row_count=6
compare_reader_var_rhs_bound_parity=1
var_rhs_bound_implemented=1
var_rhs_bound_symbol_ref=1
producer_change=0
length_bound_support=0
reversed_var_var_context_aware=0
bool_recipe_lowering_executed=0
mir_cmp_emission=0
branch_emission=0
basic_block_mutation=0
value_id_allocation=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-COMPARE-READER-VAR-RHS-PRODUCER-ROW-SELECTION-001
summary=ok
REPORT
