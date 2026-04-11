#!/bin/bash
# phase-137x boundary pure-first smoke for plan-selected direct-kernel string substring
#
# Contract:
# 1) a direct `substring_concat3_hhhii` helper result with metadata `direct_kernel_entry`
#    concat-triplet proof compiles on boundary `pure-first` without shape-memory fallback.
# 2) the substring consumer reads the candidate plan proof and does not fall back to
#    `nyash.string.substring_hii`.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
    test_skip "phase137x_boundary_string_direct_kernel_plan_substring_min: LLVM backend not available"
    exit 0
fi

if ! command -v llc >/dev/null 2>&1 && ! command -v llc-18 >/dev/null 2>&1; then
    test_skip "phase137x_boundary_string_direct_kernel_plan_substring_min: llc not found"
    exit 0
fi

FIXTURE="$NYASH_ROOT/apps/tests/mir_shape_guard/string_direct_kernel_plan_substring_window_min_v1.mir.json"
NY_LLVM_C="$NYASH_ROOT/target/release/ny-llvmc"
OUT_OBJ="${TMPDIR:-/tmp}/phase137x_boundary_string_direct_kernel_plan_substring_min_$$.o"
BUILD_LOG="${TMPDIR:-/tmp}/phase137x_boundary_string_direct_kernel_plan_substring_min_$$.log"
LL_DUMP="${TMPDIR:-/tmp}/phase137x_boundary_string_direct_kernel_plan_substring_min_$$.ll"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-90}"

cleanup() {
    rm -f "$OUT_OBJ" "$BUILD_LOG" "$LL_DUMP"
}
trap cleanup EXIT

require_smoke_path "phase137x_boundary_string_direct_kernel_plan_substring_min" "fixture" "$FIXTURE" || exit 1
require_smoke_path "phase137x_boundary_string_direct_kernel_plan_substring_min" "ny-llvmc" "$NY_LLVM_C" executable || exit 1
ensure_hako_llvmc_ffi_built "phase137x_boundary_string_direct_kernel_plan_substring_min" || exit 1

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
    test_fail "phase137x_boundary_string_direct_kernel_plan_substring_min: compile timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$BUILD_RC" -ne 0 ]; then
    echo "[INFO] compile output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "phase137x_boundary_string_direct_kernel_plan_substring_min: boundary pure-first compile failed (rc=$BUILD_RC)"
    exit 1
fi

require_smoke_path "phase137x_boundary_string_direct_kernel_plan_substring_min" "object" "$OUT_OBJ" || exit 1
require_smoke_path "phase137x_boundary_string_direct_kernel_plan_substring_min" "LLVM IR dump" "$LL_DUMP" || exit 1

if ! grep -Fq "direct_kernel_plan_proof" "$BUILD_LOG"; then
    echo "[INFO] route trace output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "phase137x_boundary_string_direct_kernel_plan_substring_min: missing plan-proof direct-kernel substring route trace"
    exit 1
fi

if ! grep -Fq '%r14 = call i64 @nyash.string.substring_concat3_hhhii' "$LL_DUMP"; then
    echo "[INFO] lowered IR:"
    tail -n 120 "$LL_DUMP" || true
    test_fail "phase137x_boundary_string_direct_kernel_plan_substring_min: lowered IR missed substring_concat3_hhhii direct-kernel call"
    exit 1
fi

if grep -Fq 'call i64 @nyash.string.substring_hii(i64 %r11' "$LL_DUMP"; then
    echo "[INFO] lowered IR:"
    tail -n 120 "$LL_DUMP" || true
    test_fail "phase137x_boundary_string_direct_kernel_plan_substring_min: lowered IR fell back to nyash.string.substring_hii"
    exit 1
fi

if grep -Fq 'call i64 @nyash.string.insert_hsi' "$LL_DUMP"; then
    echo "[INFO] lowered IR:"
    tail -n 120 "$LL_DUMP" || true
    test_fail "phase137x_boundary_string_direct_kernel_plan_substring_min: lowered IR took insert_hsi instead of the plan-proof substring route"
    exit 1
fi

test_pass "phase137x_boundary_string_direct_kernel_plan_substring_min: PASS (boundary pure-first selects direct-kernel substring from metadata plan proof)"
