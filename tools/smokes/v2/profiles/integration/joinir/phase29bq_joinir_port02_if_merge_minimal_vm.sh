#!/bin/bash
# phase29bq_joinir_port02_if_merge_minimal_vm.sh
# historical standalone probe.
# Current fast gate focuses later if-merge ports; this script stays as direct evidence.
# JIR-PORT-02 contract:
# - if/merge minimal shape is accepted in .hako route.
# - hako route must run via selfhost-first (no direct-emit/delegate fallback).
# - emitted main must include branch + merge(phi) + ret skeleton.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../.." && pwd)"
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase29bq_joinir_port02_if_merge_minimal_vm"
FIXTURE="${1:-$NYASH_ROOT/apps/tests/phase29bq_joinir_port02_if_merge_assign_return_var_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"

if ! [[ "$RUN_TIMEOUT_SECS" =~ ^[0-9]+$ ]]; then
  test_fail "$SMOKE_NAME: timeout must be integer: $RUN_TIMEOUT_SECS"
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

TMP_MIR="$(mktemp /tmp/phase29bq_port02_hako_mir.XXXXXX.json)"
HAKO_LOG="$(mktemp /tmp/phase29bq_port02_hako_log.XXXXXX.log)"
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

if ! grep -Fq "[OK] MIR JSON written (selfhost-first):" "$HAKO_LOG"; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: hako route did not use selfhost-first path"
  exit 1
fi
if grep -Fq "[OK] MIR JSON written (direct-emit):" "$HAKO_LOG"; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: direct-emit fallback detected"
  exit 1
fi
if grep -Fq "[OK] MIR JSON written (delegate:" "$HAKO_LOG"; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: delegate fallback detected"
  exit 1
fi

if ! jq -e '
  (.functions | map(select(.name=="main")) | length) == 1
' "$TMP_MIR" >/dev/null; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: main function missing in MIR"
  exit 1
fi

if ! jq -e '
  (.functions | map(select(.name=="main")) | .[0]) as $m
  | ($m.blocks | length) >= 4
  and (($m.blocks[].instructions[] | select(.op=="branch")) | length) >= 1
  and (($m.blocks[].instructions[] | select(.op=="phi")) | length) >= 1
  and (($m.blocks[].instructions[] | select(.op=="ret")) | length) >= 1
' "$TMP_MIR" >/dev/null; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: if/merge skeleton (branch+phi+ret) missing"
  exit 1
fi

if ! jq -e '
  (.functions | map(select(.name=="main")) | .[0]) as $m
  | [ $m.blocks[].instructions[] | select(.op=="phi") | ((.incoming // []) | length) ] | any(. >= 2)
' "$TMP_MIR" >/dev/null; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: merge phi incoming count is insufficient"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (selfhost if/merge minimal accepted)"
