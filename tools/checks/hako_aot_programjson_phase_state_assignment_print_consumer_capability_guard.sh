#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="hako-aot-programjson-phase-state-assignment-print-consumer-capability-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/hako-aot-programjson-phase-state-assignment-print-consumer-capability-v0.json"
CONSUMER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_phase_state_consumer_box.hako"
ASSIGN_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/assignment_stmt_handler.hako"
PRINT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/print_stmt_handler.hako"
SEQ_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_seq_recipe_dto_parity_gate.sh"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$FIXTURE" "$CONSUMER_IMPL" "$ASSIGN_IMPL" "$PRINT_IMPL" "$SEQ_GATE" "$HAKO_BIN"

TMP_DIR="$(mktemp -d /tmp/hakorune-phase-state-assignment-print.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/phase_state_assignment_print_probe.hako"
MIR_JSON="$TMP_DIR/phase_state_assignment_print_probe.mir.json"
EXE="$TMP_DIR/phase_state_assignment_print_probe.exe"
EXPECTED="$TMP_DIR/expected.txt"
BUILD_LOG="$TMP_DIR/build.log"
RUN_OUT="$TMP_DIR/run.out"
RUN_FILTERED="$TMP_DIR/run.filtered"
RUN_ERR="$TMP_DIR/run.err"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

if fixture.get("kind") != "HakoAotProgramJsonPhaseStateAssignmentPrintConsumerCapabilityV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "HAKO-AOT-PROGRAMJSON-PHASE-STATE-ASSIGNMENT-PRINT-CONSUMER-CAPABILITY-001":
    raise SystemExit("bad fixture token")

rule = fixture.get("contract_rule") or {}
for key in [
    "scanner_string_tokens_preserved_as_raw_stringbox",
    "dynamic_token_comparison_uses_same_token",
    "assignment_rhs_kind_preserved_through_phase_state",
    "top_level_assignment_runtime_green",
    "top_level_print_runtime_green",
    "seq_recipe_dto_assignment_print_rows_unclaimed",
]:
    if rule.get(key) is not True:
        raise SystemExit(f"bad contract field: {key}")

claims = fixture.get("claims") or {}
for key, value in claims.items():
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

rows = fixture.get("rows") or []
if len(rows) < 4:
    raise SystemExit("assignment/print consumer capability requires at least 4 rows")

calls = []
expected_rows = []
for row in rows:
    row_id = row["row_id"]
    program_json = json.dumps(row["program_json"], separators=(",", ":"), ensure_ascii=False)
    exp = row["expected"]
    if exp.get("err") != 0:
        raise SystemExit(f"row must be runtime-green: {row_id}")
    shape = exp["shape_kind"]
    assign_kind = exp.get("assign_rhs_kind") or ""
    print_kind = exp.get("print_kind") or ""
    return_kind = exp.get("return_kind") or ""
    calls.append(
        "    {\n"
        "      local out = ProgramJsonV0PhaseStateBox.parse("
        + json.dumps(program_json)
        + ', "[guard]")\n'
        "      print("
        + json.dumps(f"row:{row_id}:")
        + ' + "err=" + BoxHelpers.map_get(out, "err")'
        + ' + ";shape_ok=" + BoxHelpers.same_token(BoxHelpers.map_get(out, "shape_kind"), '
        + json.dumps(shape)
        + ")"
        + ' + ";assign_kind_ok=" + BoxHelpers.same_token(BoxHelpers.map_get(out, "assign_rhs_kind"), '
        + json.dumps(assign_kind)
        + ")"
        + ' + ";print_kind_ok=" + BoxHelpers.same_token(BoxHelpers.map_get(out, "print_kind"), '
        + json.dumps(print_kind)
        + ")"
        + ' + ";return_kind_ok=" + BoxHelpers.same_token(BoxHelpers.map_get(out, "return_kind"), '
        + json.dumps(return_kind)
        + ")"
        + ")\n"
        "    }"
    )
    assign_ok = 1 if assign_kind else 0
    print_ok = 1 if print_kind else 0
    expected_rows.append(
        f"row:{row_id}:err=0;shape_ok=1;assign_kind_ok={assign_ok};print_kind_ok={print_ok};return_kind_ok=1"
    )

source = "\n".join([
    "using selfhost.shared.common.box_helpers as BoxHelpers",
    "using lang.compiler.mirbuilder.program_json_v0_phase_state_box as ProgramJsonV0PhaseStateBox",
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
expected.write_text("\n".join(expected_rows) + "\n", encoding="utf-8")
PY

python3 - "$CONSUMER_IMPL" "$ASSIGN_IMPL" "$PRINT_IMPL" <<'PY'
import sys
from pathlib import Path

consumer = Path(sys.argv[1]).read_text(encoding="utf-8")
assign = Path(sys.argv[2]).read_text(encoding="utf-8")
print_impl = Path(sys.argv[3]).read_text(encoding="utf-8")

required_assign = [
    'local expr_type = BoxHelpers.map_get(expr_info, "type")',
    'BoxHelpers.same_token(op, "+")',
    'BoxHelpers.same_token(lhs_name, assign_name)',
    'local rhs_type = BoxHelpers.map_get(rhs_info, "type")',
]
required_print = [
    'local expr_type = BoxHelpers.map_get(expr_info, "type")',
    'BoxHelpers.same_token(expr_type, "Call")',
    'BoxHelpers.same_token(call_name, "env.console.log")',
    'BoxHelpers.same_token(op, "+")',
    'local rhs_type = BoxHelpers.map_get(rhs_info, "type")',
]
for needle in required_assign:
    if needle not in assign:
        raise SystemExit(f"missing assignment token contract: {needle}")
for needle in required_print:
    if needle not in print_impl:
        raise SystemExit(f"missing print token contract: {needle}")
for forbidden in [
    'local expr_type = "" + BoxHelpers.map_get(expr_info, "type")',
    'local op = "" + BoxHelpers.map_get(op_out, "value")',
    'op != "+"',
    'call_name != "env.console.log"',
]:
    if forbidden in assign or forbidden in print_impl:
        raise SystemExit(f"forbidden dynamic string comparison remains: {forbidden}")
if 'BoxHelpers.map_get(result, "assign_rhs_kind")' not in consumer:
    raise SystemExit("assign_rhs_kind must be preserved as raw token through PhaseState")
PY

bash "$HAKO_BIN" --backend mir --verify "$CONSUMER_IMPL" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$ASSIGN_IMPL" >/dev/null
bash "$HAKO_BIN" --backend mir --verify "$PRINT_IMPL" >/dev/null

if ! timeout 90s bash "$HAKO_BIN" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >"$BUILD_LOG" 2>&1; then
  tail -n 160 "$BUILD_LOG" || true
  guard_fail "$TAG" "failed to emit MIR JSON for assignment/print consumer probe"
fi

python3 - "$MIR_JSON" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
main = next((fn for fn in data.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("main function missing")
routes = (main.get("metadata") or {}).get("global_call_routes") or []
phase_routes = [row for row in routes if row.get("symbol") == "ProgramJsonV0PhaseStateBox.parse/2"]
if len(phase_routes) < 4:
    raise SystemExit("main does not call PhaseState parse once per row")
for row in phase_routes:
    if row.get("tier") != "DirectAbi" or row.get("return_shape") != "map_handle":
        raise SystemExit(f"PhaseState parse route is not DirectAbi/map_handle: {row}")
PY

if ! timeout --kill-after=2s 120s bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$BUILD_LOG" 2>&1; then
  tail -n 160 "$BUILD_LOG" >&2 || true
  guard_fail "$TAG" "emit-exe probe failed or timed out"
fi
if ! "$EXE" >"$RUN_OUT" 2>"$RUN_ERR"; then
  cat "$RUN_ERR" >&2 || true
  guard_fail "$TAG" "executable failed at runtime"
fi
grep -v '^Result: 0$' "$RUN_OUT" >"$RUN_FILTERED" || true
if ! diff -u "$EXPECTED" "$RUN_FILTERED" >/dev/null; then
  echo "[${TAG}] expected:" >&2
  cat "$EXPECTED" >&2
  echo "[${TAG}] actual:" >&2
  cat "$RUN_FILTERED" >&2
  guard_fail "$TAG" "runtime parity mismatch"
fi

bash "$SEQ_GATE" >/dev/null

cat <<'REPORT'
output_contract=hako-aot-programjson-phase-state-assignment-print-consumer-capability-guard-v0
token=HAKO-AOT-PROGRAMJSON-PHASE-STATE-ASSIGNMENT-PRINT-CONSUMER-CAPABILITY-001
owner=ProgramJsonPhaseStateAssignmentPrintConsumerCapabilityV1
top_level_assignment_runtime_green=1
top_level_print_runtime_green=1
assignment_rhs_kind_preserved_through_phase_state=1
seq_recipe_dto_assignment_print_rows_green=0
runtime_route_switch=0
full_recipe_matcher_execution=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
source_selfhost_claim=0
summary=ok
REPORT
