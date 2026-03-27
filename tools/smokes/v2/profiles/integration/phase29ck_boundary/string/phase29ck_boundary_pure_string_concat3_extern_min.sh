#!/bin/bash
# Phase 29ck boundary pure-first concat3 extern canary
#
# Contract pin:
# 1) default `ny-llvmc` boundary object route accepts a narrow
#    `Extern nyash.string.concat3_hhh` v1 seed.
# 2) the supported seed emits an object without falling through to
#    `ny-llvmc --driver harness`.
# 3) lowered IR must keep `nyash.string.concat3_hhh` instead of degrading to
#    chained `nyash.string.concat_hh`.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env || exit 2

if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
    test_skip "phase29ck_boundary_pure_string_concat3_extern_min: LLVM backend not available"
    exit 0
fi

if ! command -v llc >/dev/null 2>&1 && ! command -v llc-18 >/dev/null 2>&1; then
    test_skip "phase29ck_boundary_pure_string_concat3_extern_min: llc not found"
    exit 0
fi

FIXTURE="$NYASH_ROOT/apps/tests/mir_shape_guard/string_concat3_extern_min_v1.mir.json"
NY_LLVM_C="$NYASH_ROOT/target/release/ny-llvmc"
OUT_OBJ="${TMPDIR:-/tmp}/phase29ck_boundary_pure_string_concat3_extern_min_$$.o"
BUILD_LOG="${TMPDIR:-/tmp}/phase29ck_boundary_pure_string_concat3_extern_min_$$.log"
LL_DUMP="${TMPDIR:-/tmp}/phase29ck_boundary_pure_string_concat3_extern_min_$$.ll"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-90}"

cleanup() {
    rm -f "$OUT_OBJ" "$BUILD_LOG" "$LL_DUMP"
}
trap cleanup EXIT

require_smoke_path "phase29ck_boundary_pure_string_concat3_extern_min" "fixture" "$FIXTURE" || exit 1
require_smoke_path "phase29ck_boundary_pure_string_concat3_extern_min" "ny-llvmc" "$NY_LLVM_C" executable || exit 1
ensure_hako_llvmc_ffi_built "phase29ck_boundary_pure_string_concat3_extern_min" || exit 1

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
    test_fail "phase29ck_boundary_pure_string_concat3_extern_min: compile timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$BUILD_RC" -ne 0 ]; then
    echo "[INFO] compile output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "phase29ck_boundary_pure_string_concat3_extern_min: boundary default still relied on harness fallback (rc=$BUILD_RC)"
    exit 1
fi

require_smoke_path "phase29ck_boundary_pure_string_concat3_extern_min" "object" "$OUT_OBJ" || exit 1

if ! grep -F 'mname=nyash.string.concat3_hhh' "$BUILD_LOG" >/dev/null 2>&1; then
    echo "[INFO] route trace output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "phase29ck_boundary_pure_string_concat3_extern_min: concat3 extern route was not observed"
    exit 1
fi

if [ ! -f "$LL_DUMP" ]; then
    test_fail "phase29ck_boundary_pure_string_concat3_extern_min: LLVM IR dump missing: $LL_DUMP"
    exit 1
fi

if ! grep -F 'nyash.string.concat3_hhh' "$LL_DUMP" >/dev/null 2>&1; then
    echo "[INFO] lowered IR:"
    tail -n 120 "$LL_DUMP" || true
    test_fail "phase29ck_boundary_pure_string_concat3_extern_min: lowered IR did not call nyash.string.concat3_hhh"
    exit 1
fi

if grep -F 'call i64 @"nyash.string.concat_hh"' "$LL_DUMP" >/dev/null 2>&1; then
    echo "[INFO] lowered IR:"
    tail -n 120 "$LL_DUMP" || true
    test_fail "phase29ck_boundary_pure_string_concat3_extern_min: lowered IR degraded concat3 extern to concat_hh"
    exit 1
fi

test_pass "phase29ck_boundary_pure_string_concat3_extern_min: PASS (boundary default emits concat3 extern seed without ny-llvmc harness fallback)"
