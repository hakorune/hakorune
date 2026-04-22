#!/bin/bash
# phase-137x source-only fixture smoke.
#
# Contract:
# 1) the len window is selected from MIR metadata.
# 2) the source-only mode publishes the get source without a slot load.
# 3) the direct set lowers to the array insert-mid store helper.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
    test_skip "phase137x_boundary_array_string_len_insert_mid_source_only_min: LLVM backend not available"
    exit 0
fi

if ! command -v llc >/dev/null 2>&1 && ! command -v llc-18 >/dev/null 2>&1; then
    test_skip "phase137x_boundary_array_string_len_insert_mid_source_only_min: llc not found"
    exit 0
fi

SMOKE_NAME="phase137x_boundary_array_string_len_insert_mid_source_only_min"
FIXTURE="$NYASH_ROOT/apps/tests/mir_shape_guard/array_string_len_insert_mid_source_only_min_v1.mir.json"
NY_LLVM_C="$NYASH_ROOT/target/release/ny-llvmc"
OUT_OBJ="${TMPDIR:-/tmp}/${SMOKE_NAME}_$$.o"
OUT_LL="${TMPDIR:-/tmp}/${SMOKE_NAME}_$$.ll"
BUILD_LOG="${TMPDIR:-/tmp}/${SMOKE_NAME}_$$.log"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-90}"

cleanup() {
    rm -f "$OUT_OBJ" "$OUT_LL" "$BUILD_LOG"
}
trap cleanup EXIT

require_smoke_path "$SMOKE_NAME" "fixture" "$FIXTURE" || exit 1
require_smoke_path "$SMOKE_NAME" "ny-llvmc" "$NY_LLVM_C" executable || exit 1
ensure_hako_llvmc_ffi_built "$SMOKE_NAME" || exit 1

if capture_boundary_compile_to_log \
    "$BUILD_LOG" \
    "$RUN_TIMEOUT_SECS" \
    env \
      NYASH_NY_LLVM_COMPILER=/__missing__/ny-llvmc \
      NYASH_LLVM_ROUTE_TRACE=1 \
      NYASH_LLVM_DUMP_IR="$OUT_LL" \
      "$NY_LLVM_C" --in "$FIXTURE" --out "$OUT_OBJ"; then
    BUILD_RC=0
else
    BUILD_RC=$?
fi

if [ "$BUILD_RC" -eq 124 ]; then
    test_fail "$SMOKE_NAME: compile timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$BUILD_RC" -ne 0 ]; then
    echo "[INFO] compile output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "$SMOKE_NAME: source-only insert-mid compile failed (rc=$BUILD_RC)"
    exit 1
fi

require_smoke_path "$SMOKE_NAME" "object" "$OUT_OBJ" || exit 1
require_smoke_path "$SMOKE_NAME" "LLVM IR dump" "$OUT_LL" || exit 1

if ! grep -Fq "stage=array_string_len_window result=hit reason=mir_route_metadata" "$BUILD_LOG"; then
    echo "[INFO] route trace output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "$SMOKE_NAME: len window did not use MIR metadata"
    exit 1
fi

if ! grep -Fq "source_only_insert_mid=1" "$BUILD_LOG"; then
    echo "[INFO] route trace output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "$SMOKE_NAME: source-only metadata route was not selected"
    exit 1
fi

if ! grep -Fq "keep_get_live=0" "$BUILD_LOG"; then
    echo "[INFO] route trace output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "$SMOKE_NAME: source-only metadata route unexpectedly kept get live"
    exit 1
fi

if ! grep -Fq "proof=array_get_len_source_only_direct_set" "$BUILD_LOG"; then
    echo "[INFO] route trace output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "$SMOKE_NAME: source-only proof tag missing"
    exit 1
fi

for needle in \
    "call i64 @nyash.array.string_len_hi" \
    "call i64 @nyash.array.string_insert_mid_store_hisii"
do
    if ! grep -Fq "$needle" "$OUT_LL"; then
        echo "[INFO] lowered IR:"
        tail -n 160 "$OUT_LL" || true
        test_fail "$SMOKE_NAME: lowered IR missed $needle"
        exit 1
    fi
done

for forbidden in \
    "call i64 @nyash.array.slot_load_hi" \
    "call i64 @nyash.string.substring_hii" \
    "call i64 @nyash.string.concat3_hhh" \
    "call i64 @nyash.array.set_his" \
    "call i64 @nyash.string.kernel_slot_insert_hsi" \
    "call i64 @\"nyash.string.kernel_slot_insert_hsi\""
do
    if grep -Fq "$forbidden" "$OUT_LL"; then
        echo "[INFO] lowered IR:"
        tail -n 160 "$OUT_LL" || true
        test_fail "$SMOKE_NAME: lowered IR still has forbidden call: $forbidden"
        exit 1
    fi
done

test_pass "$SMOKE_NAME: PASS (source-only len window uses MIR metadata route)"
