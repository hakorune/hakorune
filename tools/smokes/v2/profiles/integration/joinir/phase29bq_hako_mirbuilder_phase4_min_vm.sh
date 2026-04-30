#!/bin/bash
# phase29bq_hako_mirbuilder_phase4_min_vm.sh
# Phase-4 pin: --emit-program-json-v0 → (.hako mirbuilder) MIR JSON v0 → --mir-json-file execution
#
# Expected: stdout="7", RC=0

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)" # tools/
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
source "$ROOT_DIR/smokes/v2/lib/stageb_helpers.sh"
require_env || exit 2

# Phase-29bq selfhost pins run in planner-required mode to exercise JoinIR plan boxes (no legacy fallback).
export HAKO_JOINIR_PLANNER_REQUIRED=1

FIXTURE="$NYASH_ROOT/apps/tests/phase29bq_hako_mirbuilder_phase4_local_print_var_min.hako"
ENTRY="$NYASH_ROOT/lang/src/compiler/mirbuilder/emit_mir_json_v0_from_program_json_v0.hako"

TMP_DIR="${PHASE29BQ_FAST_LOG_DIR:-/tmp}"
RUN_ID="phase29bq_hako_mirbuilder_phase4_min_${$}"
PJSON="$TMP_DIR/${RUN_ID}_program.json"
MJSON="$TMP_DIR/${RUN_ID}_mir.json"

rm -f "$PJSON" "$MJSON"

# 1) Rust Stage-0: emit Program(JSON v0)
stageb_emit_program_json_v0_fixture "$PJSON" "$FIXTURE"

# 2) .hako mirbuilder (Phase-4): Program(JSON v0) → MIR JSON v0
HAKO_PROGRAM_JSON_FILE="$PJSON" "$NYASH_BIN" --backend vm "$ENTRY" >"$MJSON"

# 3) Execute MIR JSON v0 and compare stdout
# Note: filter_noise uses a grep pipeline and may return non-zero for empty stdout under pipefail.
set +e
RAW_OUT="$("$NYASH_BIN" --mir-json-file "$MJSON" 2>&1)"
EXEC_RC=$?
set -e
OUT="$(printf "%s" "$RAW_OUT" | filter_noise || true)"

if [ "$EXEC_RC" -ne 0 ]; then
  echo "[FAIL] hako_mirbuilder phase4 pin: --mir-json-file failed (rc=$EXEC_RC)" >&2
  echo "[FAIL] expected rc=0" >&2
  echo "[FAIL] fixture=$FIXTURE" >&2
  echo "[FAIL] entry=$ENTRY" >&2
  echo "[FAIL] program_json=$PJSON" >&2
  echo "[FAIL] mir_json=$MJSON" >&2
  exit 1
fi

if [ "$OUT" != "7" ]; then
  echo "[FAIL] hako_mirbuilder phase4 pin: unexpected stdout (expected='7' got='$OUT')" >&2
  echo "[FAIL] fixture=$FIXTURE" >&2
  echo "[FAIL] entry=$ENTRY" >&2
  echo "[FAIL] program_json=$PJSON" >&2
  echo "[FAIL] mir_json=$MJSON" >&2
  exit 1
fi

echo "[PASS] hako_mirbuilder phase4 pin: PASS"
