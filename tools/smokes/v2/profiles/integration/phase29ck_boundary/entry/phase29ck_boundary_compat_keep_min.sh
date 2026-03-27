#!/bin/bash
# Phase 29ck boundary compat-keep compile canary
#
# Contract pin:
# 1) explicit `pure-first + compat_replay=harness` stays the supported keep lane
#    for the current unsupported seed.
# 2) unsupported `method_call_only_small.prebuilt.mir.json` may still replay the
#    explicit `--driver harness` keep lane, but that replay is no longer implicit.

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

require_smoke_path "phase29ck_boundary_compat_keep_min" "fixture" "$FIXTURE" || exit 1
require_smoke_path "phase29ck_boundary_compat_keep_min" "ny-llvmc" "$NY_LLVM_C" executable || exit 1
ensure_hako_llvmc_ffi_built "phase29ck_boundary_compat_keep_min" || exit 1

if capture_boundary_compile_to_log \
    "$BUILD_LOG" \
    "$RUN_TIMEOUT_SECS" \
    env \
      HAKO_BACKEND_COMPILE_RECIPE="pure-first" \
      HAKO_BACKEND_COMPAT_REPLAY="harness" \
      NYASH_LLVM_ROUTE_TRACE=1 \
      NYASH_NY_LLVM_COMPILER="$NY_LLVM_C" \
      "$NY_LLVM_C" --in "$FIXTURE" --out "$OUT_OBJ"; then
    BUILD_RC=0
else
    BUILD_RC=$?
fi

if [ "$BUILD_RC" -eq 124 ]; then
    test_fail "phase29ck_boundary_compat_keep_min: compile timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$BUILD_RC" -ne 0 ]; then
    echo "[INFO] compile output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "phase29ck_boundary_compat_keep_min: explicit compat replay failed (rc=$BUILD_RC)"
    exit 1
fi

require_smoke_path "phase29ck_boundary_compat_keep_min" "object" "$OUT_OBJ" || exit 1

if ! grep -Fq "[llvm-route/select] owner=boundary recipe=pure-first compat_replay=harness" "$BUILD_LOG"; then
    echo "[INFO] compile output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "phase29ck_boundary_compat_keep_min: explicit keep route did not advertise compat_replay=harness"
    exit 1
fi

if ! grep -Fq "[llvm-route/replay] lane=harness reason=unsupported_pure_shape" "$BUILD_LOG"; then
    echo "[INFO] compile output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "phase29ck_boundary_compat_keep_min: explicit keep route did not replay harness as expected"
    exit 1
fi

test_pass "phase29ck_boundary_compat_keep_min: PASS (explicit compat replay keeps method_call_only_small green via harness keep lane)"
