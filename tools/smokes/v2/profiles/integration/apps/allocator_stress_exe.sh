#!/bin/bash
# allocator-stress EXE parity smoke
#
# Contract:
# - The unchanged app enters the canonical physical lifecycle EXE owner.
# - Pure-first compilation does not use compat replay.
# - Runtime output matches the VM correctness smoke, including reject
#   accounting after allocator saturation.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="allocator_stress_exe"
APP="$HAKO_ROOT/apps/allocator-stress/main.hako"
BUILD_TIMEOUT_SECS="${BUILD_TIMEOUT_SECS:-300}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-120}"
TMP_ROOT="${TMPDIR:-/tmp}/hakorune_allocator_stress_exe_$$"
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

resolve_lifecycle_runtime() {
  if [ -n "${HAKO_LIFECYCLE_NYRT:-}" ]; then
    printf '%s' "$HAKO_LIFECYCLE_NYRT"
  elif [ -n "${NYASH_EMIT_EXE_NYRT:-}" ] && [ -f "$NYASH_EMIT_EXE_NYRT/libnyash_lifecycle_kernel.a" ]; then
    printf '%s' "$NYASH_EMIT_EXE_NYRT"
  else
    printf '%s' "$HAKO_ROOT/target/lifecycle-kernel/release"
  fi
}

NYRT_DIR="$(resolve_lifecycle_runtime)"
if [ ! -f "$NYRT_DIR/libnyash_lifecycle_kernel.a" ]; then
  test_skip "$SMOKE_NAME: lifecycle runtime archive missing: $NYRT_DIR/libnyash_lifecycle_kernel.a"
  exit 0
fi

set +e
NYASH_DISABLE_PLUGINS=1 \
  NYASH_LLVM_ROUTE_TRACE=1 \
  HAKO_BACKEND_COMPILE_RECIPE=pure-first \
  HAKO_BACKEND_COMPAT_REPLAY=none \
  timeout "$BUILD_TIMEOUT_SECS" \
  "$NYASH_BIN" \
    --backend mir \
    --emit-exe "$EXE_OUT" \
    --emit-exe-nyrt "$NYRT_DIR" \
    "$APP" \
    >"$BUILD_LOG" 2>&1
build_rc=$?
set -e

if [ "$build_rc" -ne 0 ]; then
  echo "[INFO] build output tail:"
  tail -n 120 "$BUILD_LOG" || true
  test_fail "$SMOKE_NAME: EXE build failed"
  exit 1
fi

if ! grep -Fq 'stage=lifecycle-v4-measure result=ok' "$BUILD_LOG"; then
  echo "[INFO] build output tail:"
  tail -n 120 "$BUILD_LOG" || true
  test_fail "$SMOKE_NAME: lifecycle V4 route trace missing"
  exit 1
fi

if ! grep -Fq 'toolchain=llvm-c-api' "$BUILD_LOG"; then
  echo "[INFO] build output tail:"
  tail -n 120 "$BUILD_LOG" || true
  test_fail "$SMOKE_NAME: LLVM C API physical route missing"
  exit 1
fi

if ! grep -Fq "EXE written:" "$BUILD_LOG" || [ ! -x "$EXE_OUT" ]; then
  echo "[INFO] build output tail:"
  tail -n 120 "$BUILD_LOG" || true
  test_fail "$SMOKE_NAME: physical EXE artifact missing"
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
allocator-stress
small_allocs=11 frees=3 reused=3 peak=8 free=0
medium_allocs=6 frees=2 reused=2 peak=4 free=0
requested_bytes=454
outstanding=12
rejects=4
summary=ok
Result: 0
TXT
)

compare_outputs "$expected" "$output" "$SMOKE_NAME" || exit 1

test_pass "$SMOKE_NAME"
