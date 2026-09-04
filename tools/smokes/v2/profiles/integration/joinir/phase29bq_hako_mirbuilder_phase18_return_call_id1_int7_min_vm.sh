#!/bin/bash
# phase29bq_hako_mirbuilder_phase18_return_call_id1_int7_min_vm.sh
# Contract pin: reject the retired Stage1 Return(Call id(7)) writer before
# defs publication in the .hako mirbuilder route.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)" # tools/
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
source "$ROOT_DIR/smokes/v2/lib/stageb_helpers.sh"
require_env || exit 2

export HAKO_JOINIR_PLANNER_REQUIRED=1

FIXTURE="$NYASH_ROOT/apps/tests/phase29bq_hako_mirbuilder_phase18_return_call_id1_int7_min.hako"
ENTRY="$NYASH_ROOT/lang/src/mir/builder/compat/program_json_v0_entry.hako"

TMP_DIR="${PHASE29BQ_FAST_LOG_DIR:-/tmp}"
RUN_ID="phase29bq_hako_mirbuilder_phase18_return_call_id1_int7_min_${$}"
PJSON="$TMP_DIR/${RUN_ID}_program.json"
ROUTE_LOG="$TMP_DIR/${RUN_ID}_route.log"

rm -f "$PJSON" "$ROUTE_LOG"

stageb_emit_program_json_v0_fixture "$PJSON" "$FIXTURE"
set +e
HAKO_MIR_BUILDER_FUNCS=1 HAKO_PROGRAM_JSON_FILE="$PJSON" \
  "$NYASH_BIN" --backend vm "$ENTRY" >"$ROUTE_LOG" 2>&1
EXEC_RC=$?
set -e

if [ "$EXEC_RC" -eq 0 ]; then
  echo "[FAIL] hako_mirbuilder phase18_return_call_id1_int7 pin: retired Return(Call) unexpectedly succeeded" >&2
  echo "[FAIL] fixture=$FIXTURE" >&2
  echo "[FAIL] entry=$ENTRY" >&2
  echo "[FAIL] program_json=$PJSON" >&2
  echo "[FAIL] route_log=$ROUTE_LOG" >&2
  exit 1
fi

if ! rg -q '\[freeze:contract\]\[stage1/return-call-legacy-stopped\]' "$ROUTE_LOG"; then
  echo "[FAIL] hako_mirbuilder phase18_return_call_id1_int7 pin: missing typed Return(Call) stop" >&2
  echo "[FAIL] fixture=$FIXTURE" >&2
  echo "[FAIL] entry=$ENTRY" >&2
  echo "[FAIL] program_json=$PJSON" >&2
  echo "[FAIL] route_log=$ROUTE_LOG" >&2
  exit 1
fi

echo "[PASS] hako_mirbuilder phase18_return_call_id1_int7 pin: typed Stop PASS"
