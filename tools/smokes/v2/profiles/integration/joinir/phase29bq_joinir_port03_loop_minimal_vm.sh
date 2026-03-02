#!/bin/bash
# phase29bq_joinir_port03_loop_minimal_vm.sh
# JIR-PORT-03 contract:
# - loop minimal shape is accepted in .hako route.
# - hako route must run via selfhost-first (no direct-emit/delegate fallback).
# - emitted main must include loop skeleton (phi + compare + branch + binop + ret).
# - executing emitted MIR must return expected rc.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../.." && pwd)"
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
source "$ROOT_DIR/smokes/v2/lib/joinir_port_helpers.sh"
require_env || exit 2

SMOKE_NAME="phase29bq_joinir_port03_loop_minimal_vm"
FIXTURE="${1:-$NYASH_ROOT/apps/tests/phase29bq_joinir_port03_loop_local_return_var_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
EXPECTED_RC="${EXPECTED_RC:-3}"

if ! [[ "$RUN_TIMEOUT_SECS" =~ ^[0-9]+$ ]]; then
  test_fail "$SMOKE_NAME: timeout must be integer: $RUN_TIMEOUT_SECS"
  exit 2
fi
if ! [[ "$EXPECTED_RC" =~ ^-?[0-9]+$ ]]; then
  test_fail "$SMOKE_NAME: expected rc must be integer: $EXPECTED_RC"
  exit 2
fi

if [[ "$FIXTURE" != /* ]]; then
  FIXTURE="$NYASH_ROOT/$FIXTURE"
fi

EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
if [ ! -f "$FIXTURE" ]; then
  test_fail "$SMOKE_NAME: fixture missing: $FIXTURE"
  exit 2
fi
if [ ! -x "$EMIT_ROUTE" ]; then
  test_fail "$SMOKE_NAME: emit route helper missing/executable: $EMIT_ROUTE"
  exit 2
fi

TMP_MIR="$(mktemp /tmp/phase29bq_port03_hako_mir.XXXXXX.json)"
HAKO_LOG="$(mktemp /tmp/phase29bq_port03_hako_log.XXXXXX.log)"
KEEP_DEBUG_ARTIFACTS=0

cleanup() {
  if [ "$KEEP_DEBUG_ARTIFACTS" -eq 1 ]; then
    return 0
  fi
  rm -f "$TMP_MIR" "$HAKO_LOG"
}
trap cleanup EXIT

retain_debug_artifacts() {
  KEEP_DEBUG_ARTIFACTS=1
  echo "[INFO] hako_log=$HAKO_LOG"
  echo "[INFO] hako_mir=$TMP_MIR"
}

TIMEOUT_MS=$((RUN_TIMEOUT_SECS * 1000))

set +e
timeout "$RUN_TIMEOUT_SECS" env \
  NYASH_DISABLE_PLUGINS=1 \
  HAKO_JOINIR_STRICT=1 \
  HAKO_JOINIR_PLANNER_REQUIRED=1 \
  NYASH_JOINIR_STRICT=1 \
  NYASH_JOINIR_DEV=1 \
  NYASH_NY_COMPILER_TIMEOUT_MS="$TIMEOUT_MS" \
  "$EMIT_ROUTE" --route hako-mainline --timeout-secs 0 --out "$TMP_MIR" --input "$FIXTURE" >"$HAKO_LOG" 2>&1
rc_hako=$?
set -e

if [ "$rc_hako" -eq 124 ]; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: timeout"
  exit 1
fi
if [ "$rc_hako" -ne 0 ]; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: hako route failed (rc=$rc_hako)"
  exit 1
fi
if [ ! -s "$TMP_MIR" ]; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: emitted MIR missing"
  exit 1
fi

if ! joinir_port_check_selfhost_first_log "$SMOKE_NAME" "$HAKO_LOG"; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: selfhost-first route contract violated"
  exit 1
fi

if ! joinir_port_require_main_ops "$SMOKE_NAME" "$TMP_MIR" phi compare branch binop ret; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: loop skeleton (phi+compare+branch+binop+ret) missing"
  exit 1
fi

set +e
"$NYASH_BIN" --mir-json-file "$TMP_MIR" >/dev/null 2>&1
rc_exec=$?
set -e
if [ "$rc_exec" -ne "$EXPECTED_RC" ]; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: emitted MIR exec rc mismatch (expected=$EXPECTED_RC actual=$rc_exec)"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (selfhost loop minimal accepted)"
