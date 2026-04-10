#!/bin/bash
# phase-137x boundary pure-first smoke for plan-selected direct-kernel string len
#
# Contract:
# 1) a direct `substring_concat3_hhhii` helper result with metadata `direct_kernel_entry`
#    plan window compiles on boundary `pure-first` without harness fallback.
# 2) the length consumer reads the candidate plan window and does not fall back to
#    `nyash.string.len_h` / `nyash.string.substring_len_hii`.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
    test_skip "phase137x_boundary_string_direct_kernel_plan_len_min: LLVM backend not available"
    exit 0
fi

if ! command -v llc >/dev/null 2>&1 && ! command -v llc-18 >/dev/null 2>&1; then
    test_skip "phase137x_boundary_string_direct_kernel_plan_len_min: llc not found"
    exit 0
fi

FIXTURE="$NYASH_ROOT/apps/tests/mir_shape_guard/string_direct_kernel_plan_len_window_min_v1.mir.json"
NY_LLVM_C="$NYASH_ROOT/target/release/ny-llvmc"
OUT_OBJ="${TMPDIR:-/tmp}/phase137x_boundary_string_direct_kernel_plan_len_min_$$.o"
BUILD_LOG="${TMPDIR:-/tmp}/phase137x_boundary_string_direct_kernel_plan_len_min_$$.log"
LL_DUMP="${TMPDIR:-/tmp}/phase137x_boundary_string_direct_kernel_plan_len_min_$$.ll"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-90}"

cleanup() {
    rm -f "$OUT_OBJ" "$BUILD_LOG" "$LL_DUMP"
}
trap cleanup EXIT

require_smoke_path "phase137x_boundary_string_direct_kernel_plan_len_min" "fixture" "$FIXTURE" || exit 1
require_smoke_path "phase137x_boundary_string_direct_kernel_plan_len_min" "ny-llvmc" "$NY_LLVM_C" executable || exit 1
ensure_hako_llvmc_ffi_built "phase137x_boundary_string_direct_kernel_plan_len_min" || exit 1

if capture_boundary_compile_to_log \
    "$BUILD_LOG" \
    "$RUN_TIMEOUT_SECS" \
    env \
      NYASH_NY_LLVM_COMPILER=/__missing__/ny-llvmc \
      NYASH_LLVM_ROUTE_TRACE=1 \
      NYASH_LLVM_DUMP_IR="$LL_DUMP" \
      "$NY_LLVM_C" --in "$FIXTURE" --out "$OUT_OBJ"; then
    BUILD_RC=0
else
    BUILD_RC=$?
fi

if [ "$BUILD_RC" -eq 124 ]; then
    test_fail "phase137x_boundary_string_direct_kernel_plan_len_min: compile timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$BUILD_RC" -ne 0 ]; then
    echo "[INFO] compile output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "phase137x_boundary_string_direct_kernel_plan_len_min: boundary pure-first compile failed (rc=$BUILD_RC)"
    exit 1
fi

require_smoke_path "phase137x_boundary_string_direct_kernel_plan_len_min" "object" "$OUT_OBJ" || exit 1
require_smoke_path "phase137x_boundary_string_direct_kernel_plan_len_min" "LLVM IR dump" "$LL_DUMP" || exit 1

if ! grep -Fq "substring_len_direct_kernel_plan_window" "$BUILD_LOG"; then
    echo "[INFO] route trace output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "phase137x_boundary_string_direct_kernel_plan_len_min: missing plan-selected direct-kernel route trace"
    exit 1
fi

if grep -Fq 'nyash.string.len_h' "$LL_DUMP"; then
    echo "[INFO] lowered IR:"
    tail -n 120 "$LL_DUMP" || true
    test_fail "phase137x_boundary_string_direct_kernel_plan_len_min: lowered IR fell back to nyash.string.len_h"
    exit 1
fi

if grep -Fq 'nyash.string.substring_len_hii' "$LL_DUMP"; then
    echo "[INFO] lowered IR:"
    tail -n 120 "$LL_DUMP" || true
    test_fail "phase137x_boundary_string_direct_kernel_plan_len_min: lowered IR used substring_len_hii instead of plan window"
    exit 1
fi

test_pass "phase137x_boundary_string_direct_kernel_plan_len_min: PASS (boundary pure-first selects direct-kernel len from metadata plan window)"
