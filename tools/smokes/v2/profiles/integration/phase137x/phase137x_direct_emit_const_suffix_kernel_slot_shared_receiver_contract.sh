#!/bin/bash
# phase-137x boundary pure-first smoke for the shared-receiver const-suffix kernel-slot bridge
#
# Contract:
# 1) the synthetic shared-receiver fixture still lowers on the boundary driver
#    without shape fallback.
# 2) the route trace records `defer_const_suffix_kernel_slot_shared`.
# 3) lowered LLVM IR uses
#    `nyash.string.kernel_slot_concat_hs` -> `nyash.array.kernel_slot_store_hi`
#    and the trailing substring lowers through `nyash.string.piecewise_subrange_hsiii`.
# 4) the widened exact-front bridge still avoids
#    `nyash.array.set_his`, `nyash.string.concat_hs`, `nyash.string.substring_hii`,
#    and `nyash.string.len_h`.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
    test_skip "phase137x_direct_emit_const_suffix_kernel_slot_shared_receiver_contract: LLVM backend not available"
    exit 0
fi

if ! command -v llc >/dev/null 2>&1 && ! command -v llc-18 >/dev/null 2>&1; then
    test_skip "phase137x_direct_emit_const_suffix_kernel_slot_shared_receiver_contract: llc not found"
    exit 0
fi

SMOKE_NAME="phase137x_direct_emit_const_suffix_kernel_slot_shared_receiver_contract"
FIXTURE="$NYASH_ROOT/apps/tests/mir_shape_guard/string_const_suffix_kernel_slot_shared_receiver_min_v1.mir.json"
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

if ! grep -Fq "result=defer_const_suffix_kernel_slot_shared" "$BUILD_LOG"; then
    echo "[INFO] route trace output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "$SMOKE_NAME: missing defer_const_suffix_kernel_slot_shared route trace"
    exit 1
fi

if ! grep -Fq "nyash.string.kernel_slot_concat_hs" "$OUT_LL"; then
    echo "[INFO] lowered IR:"
    tail -n 120 "$OUT_LL" || true
    test_fail "$SMOKE_NAME: lowered IR missed nyash.string.kernel_slot_concat_hs"
    exit 1
fi

if ! grep -Fq "nyash.array.kernel_slot_store_hi" "$OUT_LL"; then
    echo "[INFO] lowered IR:"
    tail -n 120 "$OUT_LL" || true
    test_fail "$SMOKE_NAME: lowered IR missed nyash.array.kernel_slot_store_hi"
    exit 1
fi

if ! grep -Fq "nyash.string.piecewise_subrange_hsiii" "$OUT_LL"; then
    echo "[INFO] lowered IR:"
    tail -n 160 "$OUT_LL" || true
    test_fail "$SMOKE_NAME: lowered IR missed nyash.string.piecewise_subrange_hsiii"
    exit 1
fi

if grep -Fq "call i64 @nyash.array.set_his" "$OUT_LL"; then
    echo "[INFO] lowered IR:"
    tail -n 160 "$OUT_LL" || true
    test_fail "$SMOKE_NAME: lowered IR still calls nyash.array.set_his"
    exit 1
fi

if grep -Fq "call i64 @nyash.string.concat_hs" "$OUT_LL"; then
    echo "[INFO] lowered IR:"
    tail -n 160 "$OUT_LL" || true
    test_fail "$SMOKE_NAME: lowered IR still materializes through nyash.string.concat_hs"
    exit 1
fi

if grep -Fq "call i64 @nyash.string.substring_hii" "$OUT_LL"; then
    echo "[INFO] lowered IR:"
    tail -n 160 "$OUT_LL" || true
    test_fail "$SMOKE_NAME: lowered IR still calls nyash.string.substring_hii"
    exit 1
fi

if grep -Fq "call i64 @nyash.string.len_h" "$OUT_LL"; then
    echo "[INFO] lowered IR:"
    tail -n 160 "$OUT_LL" || true
    test_fail "$SMOKE_NAME: lowered IR still calls nyash.string.len_h"
    exit 1
fi

test_pass "$SMOKE_NAME: PASS (shared-receiver const_suffix now lowers through kernel_slot_concat_hs -> kernel_slot_store_hi and trailing piecewise_subrange_hsiii)"
