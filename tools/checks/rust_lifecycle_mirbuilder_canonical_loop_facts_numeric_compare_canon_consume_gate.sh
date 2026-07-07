#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-canonical-loop-facts-numeric-compare-canon-consume-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-canonical-loop-facts-numeric-compare-canon-consume-v0.json"
EXPANDED_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-verifier-boundary-expanded-dto-coverage-parity-v0.json"
NUMERIC_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_numeric_compare_canon_snapshot.hako"
FACTS_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_canonical_loop_facts_input_snapshot.hako"
SCANNER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_scanner_box.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" sha256sum
guard_require_files "$TAG" "$FIXTURE" "$EXPANDED_FIXTURE" "$NUMERIC_IMPL" "$FACTS_IMPL" "$SCANNER_IMPL" "$HAKO_BIN"

export HAKO_CANONICAL_LOOP_FACTS_NUMERIC_COMPARE_CONSUME_IMPL_HASH="$(
  sha256sum "$NUMERIC_IMPL" "$FACTS_IMPL" "$SCANNER_IMPL" | sha256sum | awk '{ print $1 }'
)"

python3 - "$FIXTURE" "$NUMERIC_IMPL" "$FACTS_IMPL" <<'PY'
import json
import os
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
numeric_text = Path(sys.argv[2]).read_text(encoding="utf-8")
facts_text = Path(sys.argv[3]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderCanonicalLoopFactsNumericCompareCanonConsumeV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-CANONICAL-LOOP-FACTS-NUMERIC-COMPARE-CANON-CONSUME-001", "bad token")
need((fixture.get("owners") or {}).get("numeric_compare") == "ProgramJsonNumericCompareCanonSnapshotBox", "bad numeric owner")
need((fixture.get("owners") or {}).get("canonical_loop_facts") == "ProgramJsonCanonicalLoopFactsInputSnapshotBox", "bad facts owner")

need({row.get("row_id") for row in fixture.get("code_map_rows") or []} == {
    "var_le_bound_var",
    "var_le_literal",
    "literal_ge_var",
}, "code-map row set drift")
need([row.get("row_id") for row in fixture.get("verified_snapshot_rows") or []] == ["var_le_literal"], "verified row drift")

claims = fixture.get("claims") or {}
for key in [
    "canonical_loop_facts_numeric_compare_canon_consume",
    "numeric_compare_canon_consumed",
    "bool_recipe_compare_ready_fields",
    "analysis_only",
    "raw_compare_reader_replaced_for_covered_rows",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "global_symbol_table_authority",
    "legacy_loop_var_code_removed",
    "recipe_item_attachment",
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

for needle in [
    "build_code_map(compare_json): MapBox",
    "code_map_summary(fields)",
    "\"legacy_loop_var_code\"",
    "\"bound_symbol_id\"",
]:
    need(needle in numeric_text, f"numeric implementation missing {needle}")
for needle in [
    "ProgramJsonNumericCompareCanonSnapshotBox.build_code_map",
    "numeric_compare_consume_summary(snapshot)",
    "\"numeric_compare_canon_consumed\" => 1",
    "\"bool_recipe_compare_ready\"",
    "\"lhs_symbol_id\"",
    "\"update_target_symbol_id\"",
]:
    need(needle in facts_text, f"facts implementation missing {needle}")
for forbidden in [
    "_read_var_lt_int",
    "_read_var_le_int",
    "_read_int_ge_var",
]:
    need(forbidden not in facts_text, f"raw/per-spelling reader remains: {forbidden}")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-canonical-loop-facts-numeric-consume.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/canonical_loop_facts_numeric_compare_consume.hako"
EXPECTED_CODE="$TMP_DIR/expected_code.txt"
EXPECTED_SNAPSHOT="$TMP_DIR/expected_snapshot.txt"
ACTUAL_CODE="$TMP_DIR/actual_code.txt"
ACTUAL_SNAPSHOT="$TMP_DIR/actual_snapshot.txt"
EXE="$TMP_DIR/canonical_loop_facts_numeric_compare_consume.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$EXPANDED_FIXTURE" "$APP" "$EXPECTED_CODE" "$EXPECTED_SNAPSHOT" <<'PY'
import json
import os
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
expanded = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
app = Path(sys.argv[3])
expected_code = Path(sys.argv[4])
expected_snapshot = Path(sys.argv[5])

expanded_by_id = {row["row_id"]: row["program_json"] for row in expanded.get("rows") or []}

lines = [
    "using lang.compiler.mirbuilder.program_json_numeric_compare_canon_snapshot as ProgramJsonNumericCompareCanonSnapshotBox",
    "using lang.compiler.mirbuilder.program_json_canonical_loop_facts_input_snapshot as ProgramJsonCanonicalLoopFactsInputSnapshotBox",
    "",
    "static box Main {",
    "  main() {",
    "    local cache_hash = " + json.dumps(os.environ.get("HAKO_CANONICAL_LOOP_FACTS_NUMERIC_COMPARE_CONSUME_IMPL_HASH", "")),
    "    if cache_hash == \"__never__\" { print(cache_hash) }",
]

expected_code_lines = []
for idx, row in enumerate(fixture["code_map_rows"]):
    compare_json = json.dumps(row["program_json_compare"], separators=(",", ":"))
    var = f"code_{idx}"
    lines.append(
        f"    local {var} = ProgramJsonNumericCompareCanonSnapshotBox.build_code_map({json.dumps(compare_json)})"
    )
    lines.append(
        f"    print(\"code:{row['row_id']}:\" + ProgramJsonNumericCompareCanonSnapshotBox.code_map_summary({var}))"
    )
    expected_code_lines.append(f"code:{row['row_id']}:{row['expected_code_map_summary']}")

expected_snapshot_lines = []
for idx, row in enumerate(fixture["verified_snapshot_rows"]):
    program = json.loads(json.dumps(expanded_by_id[row["source_program_row"]]))
    program["body"][1]["cond"] = row["loop_condition_patch"]
    program_json = json.dumps(program, separators=(",", ":"))
    var = f"snap_{idx}"
    lines.append(
        f"    local {var} = ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_snapshot({json.dumps(program_json)})"
    )
    lines.append(
        f"    print(\"consume:{row['row_id']}:\" + ProgramJsonCanonicalLoopFactsInputSnapshotBox.numeric_compare_consume_summary({var}))"
    )
    lines.append(
        f"    print(\"legacy:{row['row_id']}:\" + ProgramJsonCanonicalLoopFactsInputSnapshotBox.snapshot_summary({var}))"
    )
    expected_snapshot_lines.append(f"consume:{row['row_id']}:{row['expected_consume_summary']}")
    expected_snapshot_lines.append("legacy:" + row["row_id"] + ":")

lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
expected_code.write_text("\n".join(expected_code_lines) + "\n", encoding="utf-8")
expected_snapshot.write_text("\n".join(expected_snapshot_lines) + "\n", encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$NUMERIC_IMPL" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$FACTS_IMPL" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit consume executable"
fi

chmod +x "$EXE"
"$EXE" >"$TMP_DIR/run.raw"

python3 - "$FIXTURE" "$EXPECTED_CODE" "$EXPECTED_SNAPSHOT" "$TMP_DIR/run.raw" "$ACTUAL_CODE" "$ACTUAL_SNAPSHOT" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
expected_code = Path(sys.argv[2]).read_text(encoding="utf-8").splitlines()
expected_snapshot = Path(sys.argv[3]).read_text(encoding="utf-8").splitlines()
raw = Path(sys.argv[4]).read_text(encoding="utf-8").splitlines()
actual_code_path = Path(sys.argv[5])
actual_snapshot_path = Path(sys.argv[6])

actual = [line.strip() for line in raw if line.strip() and not line.startswith("Result:")]
actual_code = [line for line in actual if line.startswith("code:")]
actual_snapshot = [line for line in actual if line.startswith("consume:") or line.startswith("legacy:")]
actual_code_path.write_text("\n".join(actual_code) + "\n", encoding="utf-8")
actual_snapshot_path.write_text("\n".join(actual_snapshot) + "\n", encoding="utf-8")

if actual_code != expected_code:
    print("[canonical-loop-facts/numeric-consume] code-map mismatch")
    print("expected:", expected_code)
    print("actual:", actual_code)
    raise SystemExit(1)

consume_expected = [line for line in expected_snapshot if line.startswith("consume:")]
consume_actual = [line for line in actual_snapshot if line.startswith("consume:")]
if consume_actual != consume_expected:
    print("[canonical-loop-facts/numeric-consume] consume mismatch")
    print("expected:", consume_expected)
    print("actual:", consume_actual)
    raise SystemExit(1)

legacy_actual = [line for line in actual_snapshot if line.startswith("legacy:")]
if len(legacy_actual) != len(fixture["verified_snapshot_rows"]):
    raise SystemExit("missing legacy output")
for line, row in zip(legacy_actual, fixture["verified_snapshot_rows"]):
    for token in row["expected_legacy_summary_contains"]:
        if token not in line:
            raise SystemExit(f"legacy output missing token {token!r}: {line}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-canonical-loop-facts-numeric-compare-canon-consume-gate-v0
token=MIRBUILDER-CANONICAL-LOOP-FACTS-NUMERIC-COMPARE-CANON-CONSUME-001
numeric_compare_code_map_rows=3
verified_snapshot_consume_rows=1
canonical_loop_facts_numeric_compare_canon_consume=1
numeric_compare_canon_consumed=1
bool_recipe_compare_ready_fields=1
analysis_only=1
raw_compare_reader_replaced_for_covered_rows=1
legacy_loop_var_code_removed=0
global_symbol_table_authority=0
recipe_item_attachment=0
recipe_matcher_input_authority=0
bool_recipe_lowering=0
mir_cmp_emission=0
branch_emission=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-PARITY-001
summary=ok
REPORT
