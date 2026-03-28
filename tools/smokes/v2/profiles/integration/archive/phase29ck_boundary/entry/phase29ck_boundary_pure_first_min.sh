#!/bin/bash
# Phase 29ck boundary pure-first compile legacy lock
#
# Contract pin:
# 1) default `ny-llvmc` boundary object route tries the C-side pure subset first.
# 2) supported seed `ret_const_min_v1.mir.json` emits an object without falling through
#    to `ny-llvmc --driver harness`.
# 3) breaking `NYASH_NY_LLVM_COMPILER` must not break that supported seed.
# 4) this smoke is a legacy boundary lock after the daily owner moved to phase29x.

set -euo pipefail

source "$(dirname "$0")/../../../../../lib/test_runner.sh"
require_env || exit 2

if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
    test_skip "phase29ck_boundary_pure_first_min: LLVM backend not available"
    exit 0
fi

if ! command -v llc >/dev/null 2>&1 && ! command -v llc-18 >/dev/null 2>&1; then
    test_skip "phase29ck_boundary_pure_first_min: llc not found"
    exit 0
fi

FIXTURE="$NYASH_ROOT/apps/tests/mir_shape_guard/ret_const_min_v1.mir.json"
NY_LLVM_C="$NYASH_ROOT/target/release/ny-llvmc"
OUT_OBJ="${TMPDIR:-/tmp}/phase29ck_boundary_pure_first_min_$$.o"
BUILD_LOG="${TMPDIR:-/tmp}/phase29ck_boundary_pure_first_min_$$.log"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-90}"

cleanup() {
    rm -f "$OUT_OBJ" "$BUILD_LOG"
}
trap cleanup EXIT

require_smoke_path "phase29ck_boundary_pure_first_min" "fixture" "$FIXTURE" || exit 1
require_smoke_path "phase29ck_boundary_pure_first_min" "ny-llvmc" "$NY_LLVM_C" executable || exit 1
ensure_hako_llvmc_ffi_built "phase29ck_boundary_pure_first_min" || exit 1

if capture_boundary_compile_to_log \
    "$BUILD_LOG" \
    "$RUN_TIMEOUT_SECS" \
    env -u HAKO_BACKEND_COMPILE_RECIPE -u HAKO_BACKEND_COMPAT_REPLAY \
      NYASH_LLVM_ROUTE_TRACE=1 \
      NYASH_NY_LLVM_COMPILER=/__missing__/ny-llvmc \
      "$NY_LLVM_C" --in "$FIXTURE" --out "$OUT_OBJ"; then
    BUILD_RC=0
else
    BUILD_RC=$?
fi

if [ "$BUILD_RC" -eq 124 ]; then
    test_fail "phase29ck_boundary_pure_first_min: compile timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$BUILD_RC" -ne 0 ]; then
    echo "[INFO] compile output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "phase29ck_boundary_pure_first_min: boundary default still relied on harness fallback (rc=$BUILD_RC)"
    exit 1
fi

require_smoke_path "phase29ck_boundary_pure_first_min" "object" "$OUT_OBJ" || exit 1

if ! grep -Fq "[llvm-route/select] owner=boundary recipe=pure-first compat_replay=none" "$BUILD_LOG"; then
    echo "[INFO] compile output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "phase29ck_boundary_pure_first_min: boundary daily route did not advertise compat_replay=none"
    exit 1
fi

if grep -Fq "[llvm-route/replay] lane=harness" "$BUILD_LOG"; then
    echo "[INFO] compile output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "phase29ck_boundary_pure_first_min: boundary daily route still replayed harness"
    exit 1
fi

test_pass "phase29ck_boundary_pure_first_min: PASS (boundary default emits v1 ret-const seed with compat_replay=none and without ny-llvmc harness fallback)"
