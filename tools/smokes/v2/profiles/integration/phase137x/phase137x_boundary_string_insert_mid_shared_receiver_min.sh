#!/bin/bash
# phase-137x boundary pure-first smoke for insert-mid shared-receiver widening
#
# Contract:
# 1) deferred `insert_hsi` can feed both `ArrayBox.set(...)` and a trailing
#    `substring(...)` without eager `insert_hsi` publication.
# 2) the store lowers through
#    `nyash.string.kernel_slot_insert_hsi -> nyash.array.kernel_slot_store_hi`.
# 3) the trailing substring reuses the deferred piecewise producer through
#    `nyash.string.piecewise_subrange_hsiii`.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
    test_skip "phase137x_boundary_string_insert_mid_shared_receiver_min: LLVM backend not available"
    exit 0
fi

if ! command -v llc >/dev/null 2>&1 && ! command -v llc-18 >/dev/null 2>&1; then
    test_skip "phase137x_boundary_string_insert_mid_shared_receiver_min: llc not found"
    exit 0
fi

SMOKE_NAME="phase137x_boundary_string_insert_mid_shared_receiver_min"
FIXTURE="$NYASH_ROOT/apps/tests/mir_shape_guard/string_insert_mid_kernel_slot_shared_receiver_min_v1.mir.json"
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
    test_fail "$SMOKE_NAME: boundary shared-receiver compile failed (rc=$BUILD_RC)"
    exit 1
fi

require_smoke_path "$SMOKE_NAME" "object" "$OUT_OBJ" || exit 1
require_smoke_path "$SMOKE_NAME" "LLVM IR dump" "$OUT_LL" || exit 1

if ! grep -Fq "publication_boundary_deferred_insert_mid_kernel_slot_shared" "$BUILD_LOG"; then
    echo "[INFO] route trace output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "$SMOKE_NAME: missing shared insert-mid route trace"
    exit 1
fi

if ! grep -Fq "shared_receiver=1" "$BUILD_LOG"; then
    echo "[INFO] route trace output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "$SMOKE_NAME: shared insert-mid route trace did not record shared_receiver=1"
    exit 1
fi

for needle in \
    "nyash.string.kernel_slot_insert_hsi" \
    "nyash.array.kernel_slot_store_hi" \
    "nyash.string.piecewise_subrange_hsiii"
do
    if ! grep -Fq "$needle" "$OUT_LL"; then
        echo "[INFO] lowered IR:"
        tail -n 160 "$OUT_LL" || true
        test_fail "$SMOKE_NAME: lowered IR missed $needle"
        exit 1
    fi
done

if grep -Fq "call i64 @nyash.string.insert_hsi" "$OUT_LL"; then
    echo "[INFO] lowered IR:"
    tail -n 160 "$OUT_LL" || true
    test_fail "$SMOKE_NAME: lowered IR still calls nyash.string.insert_hsi"
    exit 1
fi

if grep -Fq "call i64 @nyash.array.set_his" "$OUT_LL"; then
    echo "[INFO] lowered IR:"
    tail -n 160 "$OUT_LL" || true
    test_fail "$SMOKE_NAME: lowered IR still calls nyash.array.set_his"
    exit 1
fi

test_pass "$SMOKE_NAME: PASS (insert-mid shared receiver lowers through kernel_slot_insert_hsi -> kernel_slot_store_hi and piecewise_subrange_hsiii)"
