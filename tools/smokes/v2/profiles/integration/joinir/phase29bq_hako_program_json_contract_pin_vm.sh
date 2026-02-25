#!/bin/bash
# phase29bq_hako_program_json_contract_pin_vm.sh
# Program(JSON v0) contract pin for .hako mirbuilder route.
#
# Contract targets:
# - Print node shape
# - Expr(Call env.console.log(...)) shape
# - If node shape (R4 minimal)
# - Loop node shape (R5 minimal)

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)" # tools/
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
require_env || exit 2

export HAKO_JOINIR_PLANNER_REQUIRED=1

ENTRY="$NYASH_ROOT/lang/src/compiler/mirbuilder/emit_mir_json_v0_from_program_json_v0.hako"
FIXTURE_EXPR_CALL="$NYASH_ROOT/apps/tests/phase29bq_hako_mirbuilder_phase4_local_print_var_min.hako"
FIXTURE_IF="$NYASH_ROOT/apps/tests/phase29bq_hako_mirbuilder_phase10_local_if_return_min.hako"
FIXTURE_LOOP="$NYASH_ROOT/apps/tests/phase29bq_hako_mirbuilder_phase11_local_loop_return_var_min.hako"

TMP_DIR="${PHASE29BQ_FAST_LOG_DIR:-/tmp}"
RUN_ID="phase29bq_hako_program_json_contract_pin_${$}"

emit_and_run_case() {
  local case_name="$1"
  local pjson="$2"
  local expected_stdout="$3"
  local expected_rc="$4"

  local mjson="$TMP_DIR/${RUN_ID}_${case_name}_mir.json"
  rm -f "$mjson"

  set +e
  HAKO_PROGRAM_JSON_FILE="$pjson" "$NYASH_BIN" --backend vm "$ENTRY" >"$mjson"
  local emit_rc=$?
  set -e
  if [ "$emit_rc" -ne 0 ]; then
    echo "[FAIL] $case_name: emit route failed (rc=$emit_rc)" >&2
    echo "[FAIL] program_json=$pjson" >&2
    echo "[FAIL] mir_json=$mjson" >&2
    exit 1
  fi

  set +e
  local raw_out
  raw_out="$("$NYASH_BIN" --mir-json-file "$mjson" 2>&1)"
  local exec_rc=$?
  set -e
  local out
  out="$(printf "%s" "$raw_out" | filter_noise || true)"

  if [ "$exec_rc" -ne "$expected_rc" ]; then
    echo "[FAIL] $case_name: unexpected rc (expected=$expected_rc got=$exec_rc)" >&2
    echo "[FAIL] program_json=$pjson" >&2
    echo "[FAIL] mir_json=$mjson" >&2
    exit 1
  fi

  if [ "$out" != "$expected_stdout" ]; then
    echo "[FAIL] $case_name: unexpected stdout (expected='$expected_stdout' got='$out')" >&2
    echo "[FAIL] program_json=$pjson" >&2
    echo "[FAIL] mir_json=$mjson" >&2
    exit 1
  fi
}

# 1) Print node shape (hand-crafted Program JSON v0)
PJSON_PRINT="$TMP_DIR/${RUN_ID}_print_node.json"
cat >"$PJSON_PRINT" <<'JSON'
{"version":0,"kind":"Program","body":[{"name":"x","expr":{"value":7,"type":"Int"},"type":"Local"},{"expr":{"name":"x","type":"Var"},"type":"Print"},{"expr":{"value":0,"type":"Int"},"type":"Return"}]}
JSON
if ! rg -n '"type":"Print"' "$PJSON_PRINT" >/dev/null; then
  echo "[FAIL] print_node: Program JSON contract missing Print node" >&2
  exit 1
fi
emit_and_run_case "print_node" "$PJSON_PRINT" "7" 0

# 2) Expr(Call env.console.log(...)) shape from Stage-0 emit
PJSON_EXPR_CALL="$TMP_DIR/${RUN_ID}_expr_call_node.json"
"$NYASH_BIN" --emit-program-json-v0 "$PJSON_EXPR_CALL" "$FIXTURE_EXPR_CALL" >/dev/null
if ! rg -n '"type":"Expr"' "$PJSON_EXPR_CALL" >/dev/null; then
  echo "[FAIL] expr_call_node: Program JSON contract missing Expr node" >&2
  exit 1
fi
if ! rg -n '"name":"env\.console\.log"' "$PJSON_EXPR_CALL" >/dev/null; then
  echo "[FAIL] expr_call_node: Program JSON contract missing env.console.log call" >&2
  exit 1
fi
emit_and_run_case "expr_call_node" "$PJSON_EXPR_CALL" "7" 0

# 3) If node shape from Stage-0 emit
PJSON_IF="$TMP_DIR/${RUN_ID}_if_node.json"
"$NYASH_BIN" --emit-program-json-v0 "$PJSON_IF" "$FIXTURE_IF" >/dev/null
if ! rg -n '"type":"If"' "$PJSON_IF" >/dev/null; then
  echo "[FAIL] if_node: Program JSON contract missing If node" >&2
  exit 1
fi
emit_and_run_case "if_node" "$PJSON_IF" "" 0

# 4) Loop node shape from Stage-0 emit
PJSON_LOOP="$TMP_DIR/${RUN_ID}_loop_node.json"
"$NYASH_BIN" --emit-program-json-v0 "$PJSON_LOOP" "$FIXTURE_LOOP" >/dev/null
if ! rg -n '"type":"Loop"' "$PJSON_LOOP" >/dev/null; then
  echo "[FAIL] loop_node: Program JSON contract missing Loop node" >&2
  exit 1
fi
if ! rg -n '"op":"<"' "$PJSON_LOOP" >/dev/null; then
  echo "[FAIL] loop_node: Program JSON contract missing Loop Compare '<' op" >&2
  exit 1
fi
emit_and_run_case "loop_node" "$PJSON_LOOP" "" 3

echo "[PASS] hako_program_json_contract_pin: PASS"
