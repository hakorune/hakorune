#!/bin/bash
# phase-137x boundary pure-first smoke for the current insert-mid direct-set route
#
# Contract:
# 1) the narrow synthetic direct-set probe still lowers through
#    `string_insert_mid_window` on the current direct-set path.
# 2) the lowered LLVM IR still calls `nyash.string.insert_hsi`.
# 3) this smoke pins the actual observed route on the current fixture set,
#    not a speculative `plan_window_match` reason.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
    test_skip "phase137x_boundary_string_insert_mid_direct_set_min: LLVM backend not available"
    exit 0
fi

if ! command -v llc >/dev/null 2>&1 && ! command -v llc-18 >/dev/null 2>&1; then
    test_skip "phase137x_boundary_string_insert_mid_direct_set_min: llc not found"
    exit 0
fi

SMOKE_NAME="phase137x_boundary_string_insert_mid_direct_set_min"
FIXTURE="$NYASH_ROOT/apps/tests/mir_shape_guard/string_insert_mid_direct_set_min_v1.mir.json"
NY_LLVM_C="$NYASH_ROOT/target/release/ny-llvmc"
OUT_OBJ="${TMPDIR:-/tmp}/phase137x_boundary_string_insert_mid_direct_set_min_$$.o"
BUILD_LOG="${TMPDIR:-/tmp}/phase137x_boundary_string_insert_mid_direct_set_min_$$.log"
LL_DUMP="${TMPDIR:-/tmp}/phase137x_boundary_string_insert_mid_direct_set_min_$$.ll"
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
    test_fail "$SMOKE_NAME: boundary direct-set compile failed (rc=$BUILD_RC)"
    exit 1
fi

require_smoke_path "$SMOKE_NAME" "object" "$OUT_OBJ" || exit 1
require_smoke_path "$SMOKE_NAME" "LLVM IR dump" "$LL_DUMP" || exit 1

if ! grep -Fq "stage=string_insert_mid_window result=hit" "$BUILD_LOG"; then
    echo "[INFO] route trace output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "$SMOKE_NAME: missing string_insert_mid_window route trace"
    exit 1
fi

if ! grep -Fq "reason=shape_match" "$BUILD_LOG"; then
    echo "[INFO] route trace output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "$SMOKE_NAME: direct-set route no longer matches the observed shape contract"
    exit 1
fi

if ! grep -Fq "direct_set=1" "$BUILD_LOG"; then
    echo "[INFO] route trace output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "$SMOKE_NAME: direct-set route trace did not record direct_set=1"
    exit 1
fi

if ! grep -Fq "nyash.string.insert_hsi" "$LL_DUMP"; then
    echo "[INFO] lowered IR:"
    tail -n 120 "$LL_DUMP" || true
    test_fail "$SMOKE_NAME: lowered IR did not call nyash.string.insert_hsi"
    exit 1
fi

test_pass "$SMOKE_NAME: PASS (synthetic direct-set probe still lowers through string_insert_mid_window and nyash.string.insert_hsi)"
