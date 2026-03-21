#!/bin/bash
# Phase 29ck boundary compat-keep compile canary
#
# Contract pin:
# 1) default `ny-llvmc` boundary object route stays boundary-first for an
#    unsupported seed.
# 2) unsupported `method_call_only_small.prebuilt.mir.json` may still replay the explicit
#    `--driver harness` keep lane, but that replay is still green.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env || exit 2

if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
    test_skip "phase29ck_boundary_compat_keep_min: LLVM backend not available"
    exit 0
fi

FIXTURE="$NYASH_ROOT/apps/tests/mir_shape_guard/method_call_only_small.prebuilt.mir.json"
NY_LLVM_C="$NYASH_ROOT/target/release/ny-llvmc"
OUT_OBJ="${TMPDIR:-/tmp}/phase29ck_boundary_compat_keep_min_$$.o"
BUILD_LOG="${TMPDIR:-/tmp}/phase29ck_boundary_compat_keep_min_$$.log"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-90}"

cleanup() {
    rm -f "$OUT_OBJ" "$BUILD_LOG"
}
trap cleanup EXIT

if [ ! -f "$FIXTURE" ]; then
    test_fail "phase29ck_boundary_compat_keep_min: fixture missing: $FIXTURE"
    exit 1
fi

if [ ! -x "$NY_LLVM_C" ]; then
    test_fail "phase29ck_boundary_compat_keep_min: ny-llvmc missing: $NY_LLVM_C"
    exit 1
fi

bash "$NYASH_ROOT/tools/build_hako_llvmc_ffi.sh" >/dev/null

set +e
BUILD_OUT=$(
  NYASH_NY_LLVM_COMPILER="$NY_LLVM_C" \
  timeout "$RUN_TIMEOUT_SECS" \
  "$NY_LLVM_C" --in "$FIXTURE" --out "$OUT_OBJ" 2>&1
)
BUILD_RC=$?
set -e

echo "$BUILD_OUT" >"$BUILD_LOG"

if [ "$BUILD_RC" -eq 124 ]; then
    test_fail "phase29ck_boundary_compat_keep_min: compile timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$BUILD_RC" -ne 0 ]; then
    echo "[INFO] compile output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "phase29ck_boundary_compat_keep_min: boundary default failed to replay compat keep (rc=$BUILD_RC)"
    exit 1
fi

if [ ! -f "$OUT_OBJ" ]; then
    test_fail "phase29ck_boundary_compat_keep_min: object missing: $OUT_OBJ"
    exit 1
fi

test_pass "phase29ck_boundary_compat_keep_min: PASS (boundary default still replays compat keep for method_call_only_small)"
