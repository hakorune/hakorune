#!/bin/bash
# typed-object-untyped-field-min EXE smoke
#
# Contract pin:
# - source enters the selected physical lifecycle V4 owner directly.
# - generic MIR JSON intentionally rejects lifecycle Invoke (a
#   handle-typed field forces the lifecycle lane); the designed rejection
#   is pinned as a negative, not used as an artifact caller.
# - pure-first EXE lowers inferred i64 + handle field storage through the
#   same runtime slot-object ABI (typed_object_plans serialization is
#   covered by mir_json_emit unit tests).
# - No compat replay is used as proof.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="typed_object_untyped_field_min_exe"
APP="$HAKO_ROOT/apps/typed-object-untyped-field-min/main.hako"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-120}"
TMP_ROOT="${TMPDIR:-/tmp}/hakorune_typed_object_untyped_field_min_$$"
MIR_OUT="${TMP_ROOT}.mir.json"
EXE_OUT="${TMP_ROOT}.exe"
BUILD_LOG="${TMP_ROOT}.build.log"
RUN_LOG="${TMP_ROOT}.run.log"

cleanup() {
  rm -f "$MIR_OUT" "$EXE_OUT" "$BUILD_LOG" "$RUN_LOG" 2>/dev/null || true
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

# Negative contract pin: this app carries a handle-typed field, so its MIR
# holds lifecycle Invoke rows. Generic MIR JSON deliberately keeps
# rejecting Invoke (accepted MIRBUILDER-INVOKE-LIFECYCLE-JSON-TERMINATOR-
# D0); a pass here would mean the harness lane silently admitted the
# lifecycle shape.
set +e
NYASH_DISABLE_PLUGINS=1 \
  timeout "$RUN_TIMEOUT_SECS" \
  "$NYASH_BIN" \
    --emit-mir-json "$MIR_OUT" \
    "$APP" \
    >"$BUILD_LOG" 2>&1
mir_rc=$?
set -e

if [ "$mir_rc" -eq 0 ]; then
  test_fail "$SMOKE_NAME: generic MIR JSON emitted a lifecycle Invoke program"
  exit 1
fi

if ! grep -Fq "unsupported terminator Invoke" "$BUILD_LOG"; then
  echo "[INFO] MIR emit output tail:"
  tail -n 120 "$BUILD_LOG" || true
  test_fail "$SMOKE_NAME: MIR emit failed off the designed Invoke boundary"
  exit 1
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
    >>"$BUILD_LOG" 2>&1
build_rc=$?
set -e

if [ "$build_rc" -ne 0 ]; then
  echo "[INFO] EXE build output tail:"
  tail -n 160 "$BUILD_LOG" || true
  test_fail "$SMOKE_NAME: EXE build failed rc=$build_rc"
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

if ! grep -Fq "EXE written:" "$BUILD_LOG" || [ ! -x "$EXE_OUT" ]; then
  echo "[INFO] EXE build output tail:"
  tail -n 160 "$BUILD_LOG" || true
  test_fail "$SMOKE_NAME: physical EXE artifact missing"
  exit 1
fi

if grep -Fq "unsupported pure shape" "$BUILD_LOG"; then
  echo "[INFO] EXE build output tail:"
  tail -n 160 "$BUILD_LOG" || true
  test_fail "$SMOKE_NAME: pure-first reported unsupported shape"
  exit 1
fi

if grep -Fq "compat_replay=harness" "$BUILD_LOG"; then
  echo "[INFO] EXE build output tail:"
  tail -n 160 "$BUILD_LOG" || true
  test_fail "$SMOKE_NAME: compat replay was used"
  exit 1
fi

set +e
NYASH_DISABLE_PLUGINS=1 "$EXE_OUT" >"$RUN_LOG" 2>&1
exe_rc=$?
set -e

if [ "$exe_rc" -ne 7 ]; then
  echo "[INFO] EXE stdout/stderr:"
  cat "$RUN_LOG" || true
  test_fail "$SMOKE_NAME: expected exit code 7, got $exe_rc"
  exit 1
fi

test_pass "$SMOKE_NAME"
