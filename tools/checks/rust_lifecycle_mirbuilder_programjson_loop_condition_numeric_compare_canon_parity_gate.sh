#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-loop-condition-numeric-compare-canon-parity-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-loop-condition-numeric-compare-canon-parity-v0.json"
HAKO_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_numeric_compare_canon_snapshot.hako"
SCANNER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_scanner_box.hako"
RUST_ORACLE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-rust-condition-numeric-compare-canon-authority-v0.json"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" sha256sum
guard_require_files "$TAG" "$FIXTURE" "$HAKO_IMPL" "$SCANNER_IMPL" "$RUST_ORACLE" "$HAKO_BIN"

export HAKO_NUMERIC_COMPARE_CANON_IMPL_HASH="$(
  sha256sum "$HAKO_IMPL" "$SCANNER_IMPL" | sha256sum | awk '{ print $1 }'
)"

python3 - "$FIXTURE" "$RUST_ORACLE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
oracle = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderProgramJsonLoopConditionNumericCompareCanonParityV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-PROGRAMJSON-LOOP-CONDITION-NUMERIC-COMPARE-CANON-PARITY-001", "bad token")
need(fixture.get("owner") == "ProgramJsonNumericCompareCanonSnapshotBox", "bad owner")
need((fixture.get("rust_oracle") or {}).get("shape") == "ConditionShape::VarCompareBound", "bad rust oracle shape")

rows = fixture.get("rows") or []
need(len(rows) == 4, "row count must be 4")
need({row.get("row_id") for row in rows} == {
    "var_le_bound_var",
    "var_le_literal",
    "literal_ge_var",
    "constant_compare_no_loop_var",
}, "row set drift")
summary = fixture.get("summary") or {}
need(summary.get("programjson_compare_to_numeric_compare_canon") == 1, "missing canon claim")
need(summary.get("raw_programjson_rewrite") == 0, "raw rewrite forbidden")
need(summary.get("canonical_loop_facts_consume") == 0, "CanonicalLoopFacts consume forbidden")
need(summary.get("bool_recipe_lowering") == 0, "BoolRecipe lowering forbidden")

claims = fixture.get("claims") or {}
for key in [
    "numeric_compare_canon_snapshot_v1",
    "programjson_compare_to_numeric_compare_canon",
    "rust_oracle_parity_for_numeric_compare_canon",
    "bound_expr_shared",
    "analysis_only",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "canonical_loop_facts_consume",
    "recipe_matcher_input_authority",
    "bool_recipe_lowering",
    "mir_cmp_emission",
    "branch_emission",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

oracle_rows = {row.get("row_id"): row for row in oracle.get("rows") or []}
for row_id in ["var_le_bound_var", "var_le_literal", "literal_ge_var", "constant_compare_no_loop_var"]:
    need(row_id in oracle_rows, f"rust oracle missing {row_id}")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-numeric-compare-canon.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/numeric_compare_canon_parity.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/numeric_compare_canon_parity.exe"
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
for row in fixture["rows"]:
    compare_json = json.dumps(row["program_json_compare"], separators=(",", ":"))
    calls.append(
        "    print(ProgramJsonNumericCompareCanonSnapshotBox.build_summary("
        f"{json.dumps(compare_json)}))"
    )

source = "\n".join([
    "using lang.compiler.mirbuilder.program_json_numeric_compare_canon_snapshot as ProgramJsonNumericCompareCanonSnapshotBox",
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
  guard_fail "$TAG" "failed to emit parity executable"
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
    print("[numeric-compare-canon/parity] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-loop-condition-numeric-compare-canon-parity-gate-v0
token=MIRBUILDER-PROGRAMJSON-LOOP-CONDITION-NUMERIC-COMPARE-CANON-PARITY-001
owner=ProgramJsonNumericCompareCanonSnapshotBox
snapshot=NumericCompareCanonSnapshotV1
parity_rows=4
programjson_compare_to_numeric_compare_canon=1
rust_oracle_parity_for_numeric_compare_canon=1
analysis_only=1
raw_programjson_rewrite=0
canonical_loop_facts_consume=0
recipe_matcher_input_authority=0
bool_recipe_lowering=0
mir_cmp_emission=0
branch_emission=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-BOOL-RECIPE-COMPARE-BOUNDARY-DESIGN-001
summary=ok
REPORT
