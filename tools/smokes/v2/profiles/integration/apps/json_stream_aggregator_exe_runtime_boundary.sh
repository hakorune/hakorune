#!/bin/bash
# JSON stream aggregator EXE runtime boundary probe
#
# Contract:
# - The app compiles through pure-first EXE without compat replay.
# - Runtime output matches the VM correctness smoke.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="json_stream_aggregator_exe_runtime_boundary"
APP="$HAKO_ROOT/apps/json-stream-aggregator/main.hako"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-120}"
TMP_ROOT="${TMPDIR:-/tmp}/hakorune_json_stream_aggregator_exe_boundary_$$"
EXE_OUT="${TMP_ROOT}.exe"
BUILD_LOG="${TMP_ROOT}.build.log"

cleanup() {
  rm -f "$EXE_OUT" "$BUILD_LOG" 2>/dev/null || true
}
trap cleanup EXIT

if [ ! -f "$APP" ]; then
  test_fail "$SMOKE_NAME: App not found: $APP"
  exit 2
fi

set +e
NYASH_DISABLE_PLUGINS=1 \
  HAKO_BACKEND_COMPILE_RECIPE=pure-first \
  HAKO_BACKEND_COMPAT_REPLAY=none \
  timeout "$RUN_TIMEOUT_SECS" \
    "$HAKO_ROOT/tools/selfhost/selfhost_build.sh" \
      --in "$APP" \
      --exe "$EXE_OUT" \
      >"$BUILD_LOG" 2>&1
build_rc=$?
set -e

if [ "$build_rc" -ne 0 ]; then
  echo "[INFO] build output tail:"
  tail -n 120 "$BUILD_LOG" || true
  test_fail "$SMOKE_NAME: EXE no longer reaches the pinned runtime boundary"
  exit 1
fi

if grep -Fq "compat_replay=harness" "$BUILD_LOG"; then
  echo "[INFO] build output tail:"
  tail -n 120 "$BUILD_LOG" || true
  test_fail "$SMOKE_NAME: compat replay was used"
  exit 1
fi

set +e
output=$(NYASH_DISABLE_PLUGINS=1 timeout "$RUN_TIMEOUT_SECS" "$EXE_OUT" 2>&1 | filter_noise)
run_rc=$?
set -e

if [ "$run_rc" -ne 0 ]; then
  echo "$output"
  test_fail "$SMOKE_NAME: EXE parity run failed"
  exit 1
fi

expected=$(cat << 'TXT'
json-stream-aggregator
events=5
users=3
ana_bytes=42 ok=2 fail=0
bob_bytes=27 ok=1 fail=1
cy_bytes=9 ok=1 fail=0
total_bytes=78
ok=4 fail=1
summary=ok
Result: 0
TXT
)

compare_outputs "$expected" "$output" "$SMOKE_NAME" || exit 1

test_pass "$SMOKE_NAME"
