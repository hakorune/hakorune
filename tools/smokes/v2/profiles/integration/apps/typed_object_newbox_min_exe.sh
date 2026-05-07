#!/bin/bash
# typed-object-newbox-min EXE smoke
#
# Contract pin:
# - MIR emits typed_object_plans for an i64-only user box.
# - pure-first EXE lowers newbox + field_set + field_get through typed object
#   runtime helpers.
# - No compat replay is used as proof.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="typed_object_newbox_min_exe"
APP="$HAKO_ROOT/apps/typed-object-newbox-min/main.hako"
NY_LLVM_C="$HAKO_ROOT/target/release/ny-llvmc"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-120}"
TMP_ROOT="${TMPDIR:-/tmp}/hakorune_typed_object_newbox_min_$$"
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

if [ ! -x "$NY_LLVM_C" ]; then
  test_skip "$SMOKE_NAME: ny-llvmc missing: $NY_LLVM_C"
  exit 0
fi

set +e
NYASH_DISABLE_PLUGINS=1 \
  timeout "$RUN_TIMEOUT_SECS" \
  "$HAKO_ROOT/tools/selfhost/selfhost_build.sh" \
    --in "$APP" \
    --mir "$MIR_OUT" \
    >"$BUILD_LOG" 2>&1
mir_rc=$?
set -e

if [ "$mir_rc" -ne 0 ]; then
  echo "[INFO] MIR output tail:"
  tail -n 120 "$BUILD_LOG" || true
  test_fail "$SMOKE_NAME: MIR emit failed rc=$mir_rc"
  exit 1
fi

if ! grep -Fq '"typed_object_plans"' "$MIR_OUT"; then
  cat "$MIR_OUT" >&2
  test_fail "$SMOKE_NAME: MIR missing typed_object_plans"
  exit 1
fi

if ! grep -Fq '"box_name": "Pair"' "$MIR_OUT"; then
  cat "$MIR_OUT" >&2
  test_fail "$SMOKE_NAME: Pair typed object plan missing"
  exit 1
fi

if ! grep -Fq '"storage": "i64"' "$MIR_OUT"; then
  cat "$MIR_OUT" >&2
  test_fail "$SMOKE_NAME: i64 storage plan missing"
  exit 1
fi

set +e
NYASH_DISABLE_PLUGINS=1 \
  NYASH_LLVM_ROUTE_TRACE=1 \
  HAKO_BACKEND_COMPILE_RECIPE=pure-first \
  HAKO_BACKEND_COMPAT_REPLAY=none \
  timeout "$RUN_TIMEOUT_SECS" \
    "$NY_LLVM_C" \
      --in "$MIR_OUT" \
      --emit exe \
      --nyrt "$HAKO_ROOT/target/release" \
      --out "$EXE_OUT" \
      >>"$BUILD_LOG" 2>&1
build_rc=$?
set -e

if [ "$build_rc" -ne 0 ]; then
  echo "[INFO] EXE build output tail:"
  tail -n 160 "$BUILD_LOG" || true
  test_fail "$SMOKE_NAME: EXE build failed rc=$build_rc"
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

if [ "$exe_rc" -ne 30 ]; then
  echo "[INFO] EXE stdout/stderr:"
  cat "$RUN_LOG" || true
  test_fail "$SMOKE_NAME: expected exit code 30, got $exe_rc"
  exit 1
fi

test_pass "$SMOKE_NAME"
