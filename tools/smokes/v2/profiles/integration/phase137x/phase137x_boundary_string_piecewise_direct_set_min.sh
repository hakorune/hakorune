#!/bin/bash
# phase-137x boundary pure-first smoke for direct-set piecewise slot store
#
# Contract:
# 1) boundary pure-first compiles `insert_hsi -> substring_hii -> array.set`.
# 2) the piecewise producer stays unpublished into
#    `nyash.string.kernel_slot_piecewise_subrange_hsiii -> nyash.array.kernel_slot_store_hi`.
# 3) lowering avoids eager `nyash.string.piecewise_subrange_hsiii` publish and `nyash.array.set_his`.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
    test_skip "phase137x_boundary_string_piecewise_direct_set_min: LLVM backend not available"
    exit 0
fi

if ! command -v llc >/dev/null 2>&1 && ! command -v llc-18 >/dev/null 2>&1; then
    test_skip "phase137x_boundary_string_piecewise_direct_set_min: llc not found"
    exit 0
fi

SMOKE_NAME="phase137x_boundary_string_piecewise_direct_set_min"
FIXTURE="$NYASH_ROOT/apps/tests/mir_shape_guard/string_piecewise_kernel_slot_store_min_v1.mir.json"
NY_LLVM_C="$NYASH_ROOT/target/release/ny-llvmc"
OUT_OBJ="${TMPDIR:-/tmp}/phase137x_boundary_string_piecewise_direct_set_min_$$.o"
BUILD_LOG="${TMPDIR:-/tmp}/phase137x_boundary_string_piecewise_direct_set_min_$$.log"
LL_DUMP="${TMPDIR:-/tmp}/phase137x_boundary_string_piecewise_direct_set_min_$$.ll"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-90}"

cleanup() {
    rm -f "$OUT_OBJ" "$BUILD_LOG" "$LL_DUMP"
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
      NYASH_LLVM_DUMP_IR="$LL_DUMP" \
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
    test_fail "$SMOKE_NAME: boundary pure-first compile failed (rc=$BUILD_RC)"
    exit 1
fi

require_smoke_path "$SMOKE_NAME" "object" "$OUT_OBJ" || exit 1
require_smoke_path "$SMOKE_NAME" "LLVM IR dump" "$LL_DUMP" || exit 1

for needle in \
    'call i64 @nyash.string.kernel_slot_piecewise_subrange_hsiii' \
    'call i64 @nyash.array.kernel_slot_store_hi'
do
    if ! grep -Fq "$needle" "$LL_DUMP"; then
        echo "[INFO] lowered IR:"
        tail -n 160 "$LL_DUMP" || true
        test_fail "$SMOKE_NAME: missing IR call: $needle"
        exit 1
    fi
done

if grep -Fq 'call i64 @nyash.string.piecewise_subrange_hsiii' "$LL_DUMP"; then
    echo "[INFO] lowered IR:"
    tail -n 160 "$LL_DUMP" || true
    test_fail "$SMOKE_NAME: eager piecewise publish survived the direct-set cut"
    exit 1
fi

if grep -Fq 'call i64 @nyash.array.set_his' "$LL_DUMP"; then
    echo "[INFO] lowered IR:"
    tail -n 160 "$LL_DUMP" || true
    test_fail "$SMOKE_NAME: lowered IR still calls nyash.array.set_his"
    exit 1
fi

if ! grep -Fq 'publication_boundary_deferred_piecewise' "$BUILD_LOG"; then
    echo "[INFO] route trace output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "$SMOKE_NAME: missing deferred piecewise route trace"
    exit 1
fi

test_pass "$SMOKE_NAME: PASS (piecewise direct-set now lowers through kernel_slot_piecewise_subrange_hsiii -> kernel_slot_store_hi)"
