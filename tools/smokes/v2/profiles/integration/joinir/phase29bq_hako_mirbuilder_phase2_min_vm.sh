#!/bin/bash
# phase29bq_hako_mirbuilder_phase2_min_vm.sh
# Phase-2 pin: Stage-0 Program(JSON v0) fixture -> (.hako mirbuilder) MIR JSON v0 -> --mir-json-file execution
#
# Expected:
# - phase2_print_min: stdout="0", RC=0
# - phase2_local_return_min: stdout="", RC=1
# - phase2_inc_return_min: stdout="", RC=1

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)" # tools/
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
source "$ROOT_DIR/smokes/v2/lib/stageb_helpers.sh"
require_env || exit 2

# Phase-29bq selfhost pins run in planner-required mode to exercise JoinIR plan boxes (no legacy fallback).
export HAKO_JOINIR_PLANNER_REQUIRED=1

ENTRY="$NYASH_ROOT/lang/src/compiler/mirbuilder/emit_mir_json_v0_from_program_json_v0.hako"

TMP_DIR="${PHASE29BQ_FAST_LOG_DIR:-/tmp}"
RUN_ID="phase29bq_hako_mirbuilder_phase2_min_${$}"

run_case() {
  local fixture="$1"
  local expected_stdout="$2"
  local expected_rc="$3"
  local case_id="$4"

  local pjson="$TMP_DIR/${RUN_ID}_${case_id}_program.json"
  local mjson="$TMP_DIR/${RUN_ID}_${case_id}_mir.json"
  rm -f "$pjson" "$mjson"

  # 1) Rust Stage-0: emit Program(JSON v0)
  stageb_emit_program_json_v0_fixture "$pjson" "$fixture"

  # 2) .hako mirbuilder (Phase-2): Program(JSON v0) → MIR JSON v0
  HAKO_PROGRAM_JSON_FILE="$pjson" "$NYASH_BIN" --backend vm "$ENTRY" >"$mjson"

  # 3) Execute MIR JSON v0 and compare stdout
  # Note: filter_noise uses a grep pipeline and may return non-zero for empty stdout under pipefail.
  set +e
  RAW_OUT="$("$NYASH_BIN" --mir-json-file "$mjson" 2>&1)"
  EXEC_RC=$?
  set -e
  OUT="$(printf "%s" "$RAW_OUT" | filter_noise || true)"

  if [ "$EXEC_RC" -ne "$expected_rc" ]; then
    echo "[FAIL] hako_mirbuilder phase2 pin: unexpected rc (expected=$expected_rc got=$EXEC_RC)" >&2
    echo "[FAIL] fixture=$fixture" >&2
    echo "[FAIL] entry=$ENTRY" >&2
    echo "[FAIL] program_json=$pjson" >&2
    echo "[FAIL] mir_json=$mjson" >&2
    exit 1
  fi

  if [ "$OUT" != "$expected_stdout" ]; then
    echo "[FAIL] hako_mirbuilder phase2 pin: unexpected stdout (expected='$expected_stdout' got='$OUT')" >&2
    echo "[FAIL] fixture=$fixture" >&2
    echo "[FAIL] entry=$ENTRY" >&2
    echo "[FAIL] program_json=$pjson" >&2
    echo "[FAIL] mir_json=$mjson" >&2
    exit 1
  fi
}

run_case "$NYASH_ROOT/apps/tests/phase29bq_hako_mirbuilder_phase2_print_min.hako" "0" 0 "print_min"
run_case "$NYASH_ROOT/apps/tests/phase29bq_hako_mirbuilder_phase2_local_return_min.hako" "" 1 "local_return_min"
run_case "$NYASH_ROOT/apps/tests/phase29bq_hako_mirbuilder_phase2_inc_return_min.hako" "" 1 "inc_return_min"

echo "[PASS] hako_mirbuilder phase2 pin: PASS"
