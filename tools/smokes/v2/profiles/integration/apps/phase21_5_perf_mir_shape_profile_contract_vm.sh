#!/bin/bash
# phase21_5_perf_mir_shape_profile_contract_vm.sh
#
# Contract pin:
# - MIR_SHAPE_PROFILE=1 emits PROFILE line.
# - PROFILE read_ms/scan_ms/total_ms are numeric and non-negative.
# - total_ms must be >= read_ms and >= scan_ms.
# - scan_ms must not exceed PERF_MIR_SHAPE_SCAN_MAX_MS (default 50ms).

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/perf_vm_env.sh"
require_env || exit 2

SMOKE_NAME="phase21_5_perf_mir_shape_profile_contract_vm"
APP="$NYASH_ROOT/apps/tools/mir_shape_guard/main.hako"
EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
EMIT_TIMEOUT_SECS="${EMIT_TIMEOUT_SECS:-30}"
SOURCE_HAKO="$NYASH_ROOT/benchmarks/bench_method_call_only_small.hako"
SCAN_MAX_MS="${PERF_MIR_SHAPE_SCAN_MAX_MS:-50}"

if [ ! -f "$APP" ]; then
  test_fail "$SMOKE_NAME: app not found: $APP"
  exit 2
fi
if [ ! -x "$EMIT_ROUTE" ]; then
  test_fail "$SMOKE_NAME: emit route helper missing/executable: $EMIT_ROUTE"
  exit 2
fi
if [ ! -f "$SOURCE_HAKO" ]; then
  test_fail "$SMOKE_NAME: source fixture missing: $SOURCE_HAKO"
  exit 2
fi
if ! [[ "$SCAN_MAX_MS" =~ ^[0-9]+$ ]]; then
  test_fail "$SMOKE_NAME: PERF_MIR_SHAPE_SCAN_MAX_MS must be uint (got: $SCAN_MAX_MS)"
  exit 2
fi

TMP_MIR="$(mktemp /tmp/phase21_5_mir_shape_profile.XXXXXX.json)"
TMP_LOG="$(mktemp /tmp/phase21_5_mir_shape_profile_emit.XXXXXX.log)"
cleanup() {
  rm -f "$TMP_MIR" "$TMP_LOG" >/dev/null 2>&1 || true
}
trap cleanup EXIT

set +e
"$EMIT_ROUTE" --route hako-helper --timeout-secs "$EMIT_TIMEOUT_SECS" --out "$TMP_MIR" --input "$SOURCE_HAKO" >"$TMP_LOG" 2>&1
emit_rc=$?
set -e
if [ "$emit_rc" -ne 0 ]; then
  test_fail "$SMOKE_NAME: emit helper failed rc=$emit_rc"
  tail -n 40 "$TMP_LOG" || true
  exit 1
fi

set +e
OUT=$(MIR_SHAPE_INPUT="$TMP_MIR" MIR_SHAPE_STRICT=1 MIR_SHAPE_PROFILE=1 \
      perf_vm_lane_run run_nyash_vm "$APP" 2>&1)
rc=$?
set -e

if [ "$rc" -ne 0 ]; then
  test_fail "$SMOKE_NAME: expected rc=0, got rc=$rc"
  echo "$OUT" | tail -n 20
  exit 1
fi

summary_line=$(printf '%s\n' "$OUT" | grep '^SUMMARY ' || true)
profile_line=$(printf '%s\n' "$OUT" | grep '^PROFILE ' || true)
if [ -z "$summary_line" ]; then
  test_fail "$SMOKE_NAME: SUMMARY line missing"
  exit 1
fi
if [ -z "$profile_line" ]; then
  test_fail "$SMOKE_NAME: PROFILE line missing"
  exit 1
fi

read_ms=$(printf '%s\n' "$profile_line" | sed -n 's/.*read_ms=\([0-9][0-9]*\).*/\1/p')
scan_ms=$(printf '%s\n' "$profile_line" | sed -n 's/.*scan_ms=\([0-9][0-9]*\).*/\1/p')
total_ms=$(printf '%s\n' "$profile_line" | sed -n 's/.*total_ms=\([0-9][0-9]*\).*/\1/p')
if [ -z "$read_ms" ] || [ -z "$scan_ms" ] || [ -z "$total_ms" ]; then
  test_fail "$SMOKE_NAME: failed to parse PROFILE metrics"
  echo "$profile_line"
  exit 1
fi

if [ "$total_ms" -lt "$read_ms" ]; then
  test_fail "$SMOKE_NAME: total_ms < read_ms (${total_ms} < ${read_ms})"
  exit 1
fi
if [ "$total_ms" -lt "$scan_ms" ]; then
  test_fail "$SMOKE_NAME: total_ms < scan_ms (${total_ms} < ${scan_ms})"
  exit 1
fi
if [ "$scan_ms" -gt "$SCAN_MAX_MS" ]; then
  test_fail "$SMOKE_NAME: scan_ms exceeded budget (${scan_ms} > ${SCAN_MAX_MS})"
  echo "$profile_line"
  exit 1
fi

test_pass "$SMOKE_NAME"
