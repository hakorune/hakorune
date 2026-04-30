#!/bin/bash
# phase29bq_hako_mirbuilder_cleanup_try_min_vm.sh
# M7-min-1 pin: Stage-0 Program(JSON v0) fixture -> (.hako mirbuilder) MIR JSON v0 -> --mir-json-file execution
#
# Expected: stdout="11", RC=0

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)" # tools/
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
source "$ROOT_DIR/smokes/v2/lib/stageb_helpers.sh"
require_env || exit 2

export HAKO_JOINIR_PLANNER_REQUIRED=1

FIXTURE="$NYASH_ROOT/apps/tests/phase29bq_selfhost_cleanup_only_min.hako"
ENTRY="$NYASH_ROOT/lang/src/compiler/mirbuilder/emit_mir_json_v0_from_program_json_v0.hako"

TMP_DIR="${PHASE29BQ_FAST_LOG_DIR:-/tmp}"
RUN_ID="phase29bq_hako_mirbuilder_cleanup_try_min_${$}"
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
  echo "[FAIL] hako_mirbuilder cleanup_try pin: --mir-json-file failed (rc=$EXEC_RC)" >&2
  echo "[FAIL] expected rc=0" >&2
  echo "[FAIL] fixture=$FIXTURE" >&2
  echo "[FAIL] entry=$ENTRY" >&2
  echo "[FAIL] program_json=$PJSON" >&2
  echo "[FAIL] mir_json=$MJSON" >&2
  exit 1
fi

if [ "$OUT" != "11" ]; then
  echo "[FAIL] hako_mirbuilder cleanup_try pin: unexpected stdout (expected='11' got='$OUT')" >&2
  echo "[FAIL] fixture=$FIXTURE" >&2
  echo "[FAIL] entry=$ENTRY" >&2
  echo "[FAIL] program_json=$PJSON" >&2
  echo "[FAIL] mir_json=$MJSON" >&2
  exit 1
fi

echo "[PASS] hako_mirbuilder cleanup_try pin: PASS"
