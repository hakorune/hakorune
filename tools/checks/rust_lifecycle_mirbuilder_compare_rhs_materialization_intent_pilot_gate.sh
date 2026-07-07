#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-compare-rhs-materialization-intent-pilot-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-rhs-materialization-intent-pilot-v0.json"
IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/compare_rhs_materialization_intent_snapshot.hako"
SYMBOLIC_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/compare_lowering_symbolic_command_snapshot.hako"
SELECTION_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_rhs_materialization_owner_selection_guard.sh"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$IMPL" "$SYMBOLIC_IMPL" "$SELECTION_GATE" "$HAKO_BIN"

SELECTION_OUT="$(guard_cached_run "$TAG" bash "$SELECTION_GATE")"
if ! grep -q '^rhs_materialization_intent_pilot_selected=1$' <<<"$SELECTION_OUT"; then
  printf '%s\n' "$SELECTION_OUT" >&2
  guard_fail "$TAG" "RHS materialization intent selection prerequisite is not green"
fi

python3 - "$FIXTURE" "$IMPL" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
impl = Path(sys.argv[2]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderCompareRhsMaterializationIntentPilotV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-COMPARE-RHS-MATERIALIZATION-INTENT-PILOT-001", "bad token")
need(fixture.get("owner") == "CompareRhsMaterializationIntentSnapshotBox", "bad owner")
need(fixture.get("output_contract") == "CompareRhsMaterializationIntentSnapshotV1", "bad output contract")
need([row.get("row_id") for row in fixture.get("rows") or []] == ["command_literal_i64", "command_symbol_ref"], "row set drift")

claims = fixture.get("claims") or {}
for key in [
    "compare_rhs_materialization_intent_pilot",
    "rhs_materialization_intent_snapshot",
    "literal_i64_intent_row",
    "symbol_lookup_intent_row",
    "analysis_only",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "rhs_value_id_resolution",
    "rhs_runtime_materialization",
    "constant_mir_emission",
    "runtime_helper_emission",
    "mir_cmp_emission",
    "branch_emission",
    "basic_block_mutation",
    "value_id_allocation",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "programjson_var_rhs_full_dispatcher_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

for needle in [
    "build_intent(program_json): MapBox",
    "CompareLoweringSymbolicCommandSnapshotBox.build_command",
    "build_intent_from_command(command): MapBox",
    "\"rhs_materialization_intent_ready\" => 1",
    "\"literal_i64_required\" => literal_required",
    "\"symbol_lookup_required\" => symbol_required",
    "\"rhs_value_id_resolution\" => 0",
    "\"rhs_runtime_materialization\" => 0",
    "\"constant_mir_emission\" => 0",
    "\"runtime_helper_emission\" => 0",
    "\"mir_cmp_emission\" => 0",
    "\"value_id_allocation\" => 0",
    "intent_summary(intent)",
]:
    need(needle in impl, f"implementation missing token: {needle}")
for forbidden in [
    "MirInstruction",
    "emit_mir",
    "emit_compare",
    "emit_branch",
    "route_registry",
    "RecipeMatcherBox",
    "next_value_id",
]:
    need(forbidden not in impl, f"forbidden implementation token: {forbidden}")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-rhs-materialization-intent.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/rhs_materialization_intent.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/rhs_materialization_intent.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

lines = [
    "using lang.compiler.mirbuilder.compare_rhs_materialization_intent_snapshot as CompareRhsMaterializationIntentSnapshotBox",
    "",
    "static box Main {",
    "  main() {",
]
expected_lines = []
for idx, row in enumerate(fixture["rows"]):
    var = f"intent_{idx}"
    if row.get("input_kind") != "command_map":
        raise SystemExit(f"unknown input_kind for {row['row_id']}")
    command_entries = ", ".join(
        json.dumps(key) + " => " + str(value)
        for key, value in row["command"].items()
    )
    lines.append(f"    local command_{idx} = %{{{command_entries}}}")
    lines.append(f"    local {var} = CompareRhsMaterializationIntentSnapshotBox.build_intent_from_command(command_{idx})")
    lines.append(
        f"    print(\"rhs_intent:{row['row_id']}:\" + CompareRhsMaterializationIntentSnapshotBox.intent_summary({var}))"
    )
    expected_lines.append(f"rhs_intent:{row['row_id']}:{row['expected_intent_summary']}")

lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("\n".join(expected_lines) + "\n", encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$IMPL" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit RHS materialization intent executable"
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
    print("[compare-rhs/materialization-intent] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-compare-rhs-materialization-intent-pilot-gate-v0
token=MIRBUILDER-COMPARE-RHS-MATERIALIZATION-INTENT-PILOT-001
owner=CompareRhsMaterializationIntentSnapshotBox
intent_rows=2
compare_rhs_materialization_intent_pilot=1
rhs_materialization_intent_snapshot=1
literal_i64_intent_row=1
symbol_lookup_intent_row=1
analysis_only=1
rhs_value_id_resolution=0
rhs_runtime_materialization=0
constant_mir_emission=0
runtime_helper_emission=0
mir_cmp_emission=0
branch_emission=0
basic_block_mutation=0
value_id_allocation=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
programjson_var_rhs_full_dispatcher_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-COMPARE-RHS-MATERIALIZATION-INTENT-PARITY-001
summary=ok
REPORT
