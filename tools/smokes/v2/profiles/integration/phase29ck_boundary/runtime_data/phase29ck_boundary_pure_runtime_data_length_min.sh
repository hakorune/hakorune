#!/bin/bash
# Phase 29ck boundary pure-first RuntimeDataBox.length canary
#
# Contract pin:
# 1) default `ny-llvmc` boundary object route accepts a narrow
#    `RuntimeDataBox.length()` v1 seed when the receiver is a StringBox.
# 2) the supported seed emits an object without falling through to
#    `ny-llvmc --driver harness`.
# 3) breaking `NYASH_NY_LLVM_COMPILER` must not break that supported seed.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env || exit 2

if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
    test_skip "phase29ck_boundary_pure_runtime_data_length_min: LLVM backend not available"
    exit 0
fi

if ! command -v llc >/dev/null 2>&1 && ! command -v llc-18 >/dev/null 2>&1; then
    test_skip "phase29ck_boundary_pure_runtime_data_length_min: llc not found"
    exit 0
fi

FIXTURE="$NYASH_ROOT/apps/tests/mir_shape_guard/runtime_data_string_length_ascii_min_v1.mir.json"
NY_LLVM_C="$NYASH_ROOT/target/release/ny-llvmc"
OUT_OBJ="${TMPDIR:-/tmp}/phase29ck_boundary_pure_runtime_data_length_min_$$.o"
BUILD_LOG="${TMPDIR:-/tmp}/phase29ck_boundary_pure_runtime_data_length_min_$$.log"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-90}"

cleanup() {
    rm -f "$OUT_OBJ" "$BUILD_LOG"
}
trap cleanup EXIT

if [ ! -f "$FIXTURE" ]; then
    test_fail "phase29ck_boundary_pure_runtime_data_length_min: fixture missing: $FIXTURE"
    exit 1
fi

if [ ! -x "$NY_LLVM_C" ]; then
    test_fail "phase29ck_boundary_pure_runtime_data_length_min: ny-llvmc missing: $NY_LLVM_C"
    exit 1
fi

bash "$NYASH_ROOT/tools/build_hako_llvmc_ffi.sh" >/dev/null

set +e
BUILD_OUT=$(
  NYASH_NY_LLVM_COMPILER=/__missing__/ny-llvmc \
  timeout "$RUN_TIMEOUT_SECS" \
  "$NY_LLVM_C" --in "$FIXTURE" --out "$OUT_OBJ" 2>&1
)
BUILD_RC=$?
set -e

echo "$BUILD_OUT" >"$BUILD_LOG"

if [ "$BUILD_RC" -eq 124 ]; then
    test_fail "phase29ck_boundary_pure_runtime_data_length_min: compile timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$BUILD_RC" -ne 0 ]; then
    echo "[INFO] compile output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "phase29ck_boundary_pure_runtime_data_length_min: boundary default still relied on harness fallback (rc=$BUILD_RC)"
    exit 1
fi

if [ ! -f "$OUT_OBJ" ]; then
    test_fail "phase29ck_boundary_pure_runtime_data_length_min: object missing: $OUT_OBJ"
    exit 1
fi

test_pass "phase29ck_boundary_pure_runtime_data_length_min: PASS (boundary default emits RuntimeDataBox.length(StringBox) seed without ny-llvmc harness fallback)"
