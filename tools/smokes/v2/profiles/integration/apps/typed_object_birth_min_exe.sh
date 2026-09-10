#!/bin/bash
# typed-object-birth-min EXE smoke
#
# Contract pin:
# - source enters the selected physical lifecycle V4 owner directly.
# - generic MIR JSON intentionally rejects lifecycle Invoke and is not an
#   artifact caller for this cohort.
# - birth lowers through same-module uniform ABI and consumes TypedObjectPlan.
# - No compat replay is used as proof.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="typed_object_birth_min_exe"
APP="$HAKO_ROOT/apps/typed-object-birth-min/main.hako"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-120}"
TMP_ROOT="${TMPDIR:-/tmp}/hakorune_typed_object_birth_min_$$"
EXE_OUT="${TMP_ROOT}.exe"
BUILD_LOG="${TMP_ROOT}.build.log"
RUN_LOG="${TMP_ROOT}.run.log"

cleanup() {
  rm -f "$EXE_OUT" "$BUILD_LOG" "$RUN_LOG" 2>/dev/null || true
}
trap cleanup EXIT

if [ ! -f "$APP" ]; then
  test_fail "$SMOKE_NAME: app missing: $APP"
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
  timeout "$RUN_TIMEOUT_SECS" \
  "$NYASH_BIN" \
    --backend mir \
    --emit-exe "$EXE_OUT" \
    --emit-exe-nyrt "$NYRT_DIR" \
    "$APP" \
    >"$BUILD_LOG" 2>&1
build_rc=$?
set -e

if [ "$build_rc" -ne 0 ]; then
  echo "[INFO] EXE output tail:"
  tail -n 120 "$BUILD_LOG" || true
  test_fail "$SMOKE_NAME: physical EXE emit failed rc=$build_rc"
  exit 1
fi

if ! grep -Fq 'stage=lifecycle-v4-measure result=ok' "$BUILD_LOG"; then
  echo "[INFO] EXE build output tail:"
  tail -n 160 "$BUILD_LOG" || true
  test_fail "$SMOKE_NAME: lifecycle V4 route trace missing"
  exit 1
fi

if ! grep -Fq 'toolchain=llvm-c-api' "$BUILD_LOG"; then
  echo "[INFO] EXE build output tail:"
  tail -n 160 "$BUILD_LOG" || true
  test_fail "$SMOKE_NAME: LLVM C API physical route missing"
  exit 1
fi

if grep -Fq "unsupported pure shape" "$BUILD_LOG" || grep -Fq "compat_replay=harness" "$BUILD_LOG"; then
  echo "[INFO] EXE build output tail:"
  tail -n 160 "$BUILD_LOG" || true
  test_fail "$SMOKE_NAME: fallback or unsupported pure shape was used"
  exit 1
fi

if ! grep -Fq "EXE written:" "$BUILD_LOG" || [ ! -x "$EXE_OUT" ]; then
  echo "[INFO] EXE build output tail:"
  tail -n 160 "$BUILD_LOG" || true
  test_fail "$SMOKE_NAME: physical EXE artifact missing"
  exit 1
fi

set +e
NYASH_DISABLE_PLUGINS=1 "$EXE_OUT" >"$RUN_LOG" 2>&1
exe_rc=$?
set -e

if [ "$exe_rc" -ne 30 ]; then
  echo "[INFO] EXE stdout/stderr:"
  cat "$RUN_LOG" || true
  test_fail "$SMOKE_NAME: expected exit code 30, got $exe_rc"
  exit 1
fi

test_pass "$SMOKE_NAME"
