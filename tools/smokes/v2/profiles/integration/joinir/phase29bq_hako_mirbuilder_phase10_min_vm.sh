#!/bin/bash
# phase29bq_hako_mirbuilder_phase10_min_vm.sh
# Phase-10 pin: --emit-program-json-v0 -> (.hako mirbuilder) MIR JSON v0 -> --mir-json-file execution
#
# Expected: stdout="", RC=0

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)" # tools/
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
source "$ROOT_DIR/smokes/v2/lib/stageb_helpers.sh"
require_env || exit 2

export HAKO_JOINIR_PLANNER_REQUIRED=1

FIXTURE="$NYASH_ROOT/apps/tests/phase29bq_hako_mirbuilder_phase10_local_if_return_min.hako"
ENTRY="$NYASH_ROOT/lang/src/compiler/mirbuilder/emit_mir_json_v0_from_program_json_v0.hako"

TMP_DIR="${PHASE29BQ_FAST_LOG_DIR:-/tmp}"
RUN_ID="phase29bq_hako_mirbuilder_phase10_min_${$}"
PJSON="$TMP_DIR/${RUN_ID}_program.json"
MJSON="$TMP_DIR/${RUN_ID}_mir.json"

rm -f "$PJSON" "$MJSON"

stageb_emit_program_json_v0_fixture "$PJSON" "$FIXTURE"
HAKO_PROGRAM_JSON_FILE="$PJSON" "$NYASH_BIN" --backend vm "$ENTRY" >"$MJSON"

set +e
RAW_OUT="$("$NYASH_BIN" --mir-json-file "$MJSON" 2>&1)"
EXEC_RC=$?
set -e
OUT="$(printf "%s" "$RAW_OUT" | filter_noise || true)"

if [ "$EXEC_RC" -ne 0 ]; then
  echo "[FAIL] hako_mirbuilder phase10 pin: --mir-json-file failed (rc=$EXEC_RC)" >&2
  echo "[FAIL] expected rc=0" >&2
  echo "[FAIL] fixture=$FIXTURE" >&2
  echo "[FAIL] entry=$ENTRY" >&2
  echo "[FAIL] program_json=$PJSON" >&2
  echo "[FAIL] mir_json=$MJSON" >&2
  exit 1
fi

if [ -n "$OUT" ]; then
  echo "[FAIL] hako_mirbuilder phase10 pin: unexpected stdout (expected='' got='$OUT')" >&2
  echo "[FAIL] fixture=$FIXTURE" >&2
  echo "[FAIL] entry=$ENTRY" >&2
  echo "[FAIL] program_json=$PJSON" >&2
  echo "[FAIL] mir_json=$MJSON" >&2
  exit 1
fi

echo "[PASS] hako_mirbuilder phase10 pin: PASS"
