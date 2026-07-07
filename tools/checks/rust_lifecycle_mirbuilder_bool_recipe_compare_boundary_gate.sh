#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-bool-recipe-compare-boundary-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-bool-recipe-compare-boundary-v0.json"
HAKO_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/bool_recipe_box.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" sha256sum
guard_require_files "$TAG" "$FIXTURE" "$HAKO_IMPL" "$HAKO_BIN"

export HAKO_BOOL_RECIPE_COMPARE_IMPL_HASH="$(sha256sum "$HAKO_IMPL" | awk '{ print $1 }')"

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderBoolRecipeCompareBoundaryV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-BOOL-RECIPE-COMPARE-BOUNDARY-DESIGN-001", "bad token")
need(fixture.get("owner") == "BoolRecipeBox", "bad owner")
need(fixture.get("input_contract") == "NumericCompareCanonSnapshotV1 semantic fields after symbol resolution", "bad input contract")
need(fixture.get("output_contract") == "BoolRecipeCompareV1", "bad output contract")

rows = fixture.get("rows") or []
need(len(rows) == 4, "row count must be 4")
need({row.get("row_id") for row in rows} == {
    "var_le_bound_var",
    "var_le_literal",
    "literal_ge_var",
    "snapshot_not_ok",
}, "row set drift")

claims = fixture.get("claims") or {}
for key in [
    "bool_recipe_compare_boundary",
    "numeric_compare_canon_fields_consumed_after_symbol_resolution",
    "bound_expr_shared",
    "analysis_only",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "raw_variable_name_map_authority",
    "recipe_item_attachment",
    "canonical_loop_facts_consume",
    "recipe_matcher_input_authority",
    "mir_cmp_emission",
    "branch_emission",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-bool-recipe-compare.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/bool_recipe_compare_boundary.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/bool_recipe_compare_boundary.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

expected.write_text("\n".join(row["expected_summary"] for row in fixture["rows"]) + "\n", encoding="utf-8")

calls = []
for idx, row in enumerate(fixture["rows"]):
    fields = row["input_codes"]
    var = f"fields_{idx}"
    calls.append(
        f"    local {var} = %{{"
        + f"\"ok\" => {int(fields['ok'])}, "
        + f"\"lhs_symbol_id\" => {int(fields['lhs_symbol_id'])}, "
        + f"\"cmp_code\" => {int(fields['cmp_code'])}, "
        + f"\"bound_kind_code\" => {int(fields['bound_kind_code'])}, "
        + f"\"bound_i64\" => {int(fields['bound_i64'])}, "
        + f"\"bound_symbol_id\" => {int(fields['bound_symbol_id'])}"
        + "}"
    )
    calls.append(
        f"    print(BoolRecipeBox.summary(BoolRecipeBox.from_numeric_compare_code_map({var})))"
    )

source = "\n".join([
    "using lang.compiler.mirbuilder.recipe.bool_recipe_box as BoolRecipeBox",
    "",
    "static box Main {",
    "  main() {",
    *calls,
    "    return 0",
    "  }",
    "}",
    "",
])
app.write_text(source, encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$HAKO_IMPL" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit BoolRecipe boundary executable"
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
    print("[bool-recipe-compare/boundary] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-bool-recipe-compare-boundary-gate-v0
token=MIRBUILDER-BOOL-RECIPE-COMPARE-BOUNDARY-DESIGN-001
owner=BoolRecipeBox
input_contract=NumericCompareCanonSnapshotV1
output_contract=BoolRecipeCompareV1
boundary_rows=4
bool_recipe_compare_boundary=1
numeric_compare_canon_fields_consumed_after_symbol_resolution=1
bound_expr_shared=1
analysis_only=1
raw_variable_name_map_authority=0
recipe_item_attachment=0
canonical_loop_facts_consume=0
recipe_matcher_input_authority=0
mir_cmp_emission=0
branch_emission=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-CANONICAL-LOOP-FACTS-NUMERIC-COMPARE-CANON-CONSUME-001
summary=ok
REPORT
