#!/bin/bash
# Phase 29ck boundary pure-first RuntimeDataBox.get -> indexOf -> branch
# with same-slot const-suffix store in the then-block.
#
# Contract pin:
# 1) default `ny-llvmc` boundary object route accepts a narrow
#    array-string `get -> indexOf("line") -> compare -> branch` seed even when
#    the original fetched string is still consumed in the then-block.
# 2) accepted lowering emits `nyash.array.string_indexof_hih` for the observer
#    and routes the then-block same-slot suffix store through
#    `nyash.array.string_suffix_store_his` without reloading a stable string.
# 3) the supported seed emits an object without falling through to
#    `ny-llvmc --driver harness`.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env || exit 2

if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
    test_skip "phase29ck_boundary_pure_array_string_indexof_branch_live_after_get_min: LLVM backend not available"
    exit 0
fi

if ! command -v llc >/dev/null 2>&1 && ! command -v llc-18 >/dev/null 2>&1; then
    test_skip "phase29ck_boundary_pure_array_string_indexof_branch_live_after_get_min: llc not found"
    exit 0
fi

FIXTURE="$NYASH_ROOT/apps/tests/mir_shape_guard/array_string_indexof_branch_live_after_get_min_v1.mir.json"
NY_LLVM_C="$NYASH_ROOT/target/release/ny-llvmc"
OUT_OBJ="${TMPDIR:-/tmp}/phase29ck_boundary_pure_array_string_indexof_branch_live_after_get_min_$$.o"
BUILD_LOG="${TMPDIR:-/tmp}/phase29ck_boundary_pure_array_string_indexof_branch_live_after_get_min_$$.log"
LL_DUMP="${TMPDIR:-/tmp}/phase29ck_boundary_pure_array_string_indexof_branch_live_after_get_min_$$.ll"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-90}"

cleanup() {
    rm -f "$OUT_OBJ" "$BUILD_LOG" "$LL_DUMP"
}
trap cleanup EXIT

if [ ! -f "$FIXTURE" ]; then
    test_fail "phase29ck_boundary_pure_array_string_indexof_branch_live_after_get_min: fixture missing: $FIXTURE"
    exit 1
fi

if [ ! -x "$NY_LLVM_C" ]; then
    test_fail "phase29ck_boundary_pure_array_string_indexof_branch_live_after_get_min: ny-llvmc missing: $NY_LLVM_C"
    exit 1
fi

bash "$NYASH_ROOT/tools/build_hako_llvmc_ffi.sh" >/dev/null

set +e
BUILD_OUT=$(
  NYASH_NY_LLVM_COMPILER=/__missing__/ny-llvmc \
  NYASH_LLVM_ROUTE_TRACE=1 \
  NYASH_LLVM_DUMP_IR="$LL_DUMP" \
  timeout "$RUN_TIMEOUT_SECS" \
  "$NY_LLVM_C" --in "$FIXTURE" --out "$OUT_OBJ" 2>&1
)
BUILD_RC=$?
set -e

echo "$BUILD_OUT" >"$BUILD_LOG"

if [ "$BUILD_RC" -eq 124 ]; then
    test_fail "phase29ck_boundary_pure_array_string_indexof_branch_live_after_get_min: compile timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$BUILD_RC" -ne 0 ]; then
    echo "[INFO] compile output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "phase29ck_boundary_pure_array_string_indexof_branch_live_after_get_min: boundary default still relied on harness fallback (rc=$BUILD_RC)"
    exit 1
fi

if [ ! -f "$OUT_OBJ" ]; then
    test_fail "phase29ck_boundary_pure_array_string_indexof_branch_live_after_get_min: object missing: $OUT_OBJ"
    exit 1
fi

if ! grep -F 'stage=array_string_indexof_branch_window result=hit' "$BUILD_LOG" >/dev/null 2>&1; then
    echo "[INFO] route trace output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "phase29ck_boundary_pure_array_string_indexof_branch_live_after_get_min: exact branch recipe was not accepted"
    exit 1
fi

if [ ! -f "$LL_DUMP" ]; then
    test_fail "phase29ck_boundary_pure_array_string_indexof_branch_live_after_get_min: LLVM IR dump missing: $LL_DUMP"
    exit 1
fi

if ! grep -F 'nyash.array.string_indexof_hih' "$LL_DUMP" >/dev/null 2>&1; then
    echo "[INFO] lowered IR:"
    tail -n 120 "$LL_DUMP" || true
    test_fail "phase29ck_boundary_pure_array_string_indexof_branch_live_after_get_min: lowered IR did not call nyash.array.string_indexof_hih"
    exit 1
fi

if grep -E 'call .*nyash\.array\.slot_load_hi' "$LL_DUMP" >/dev/null 2>&1; then
    echo "[INFO] lowered IR:"
    tail -n 120 "$LL_DUMP" || true
    test_fail "phase29ck_boundary_pure_array_string_indexof_branch_live_after_get_min: lowered IR still reloads nyash.array.slot_load_hi for same-slot const-suffix store"
    exit 1
fi

if ! grep -E 'call .*nyash\.array\.string_suffix_store_his' "$LL_DUMP" >/dev/null 2>&1; then
    echo "[INFO] lowered IR:"
    tail -n 120 "$LL_DUMP" || true
    test_fail "phase29ck_boundary_pure_array_string_indexof_branch_live_after_get_min: lowered IR did not use same-slot const-suffix store"
    exit 1
fi

if grep -E 'call .*nyash\.array\.kernel_slot_concat_his' "$LL_DUMP" >/dev/null 2>&1; then
    echo "[INFO] lowered IR:"
    tail -n 120 "$LL_DUMP" || true
    test_fail "phase29ck_boundary_pure_array_string_indexof_branch_live_after_get_min: lowered IR still uses by-index kernel const-suffix concat"
    exit 1
fi

if grep -E 'call .*nyash\.array\.kernel_slot_store_hi' "$LL_DUMP" >/dev/null 2>&1; then
    echo "[INFO] lowered IR:"
    tail -n 120 "$LL_DUMP" || true
    test_fail "phase29ck_boundary_pure_array_string_indexof_branch_live_after_get_min: lowered IR still stores through a kernel text slot"
    exit 1
fi

test_pass "phase29ck_boundary_pure_array_string_indexof_branch_live_after_get_min: PASS (boundary default keeps same-slot const-suffix store in text lane without ny-llvmc harness fallback)"
