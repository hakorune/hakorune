#!/bin/bash
# Phase 29ck boundary pure-first bool-phi branch legacy lock
#
# Contract pin:
# 1) default boundary compile accepts a narrow `compare -> phi(bool) -> branch` shape.
# 2) lowered IR must emit `phi i1` for the merged boolean.
# 3) the branch must consume that `phi i1` directly, without degrading through
#    `icmp ne i64 %r7, 0`.
# 4) this smoke is a legacy boundary lock after the daily owner moved to phase29x.

set -euo pipefail

source "$(dirname "$0")/../../../../../lib/test_runner.sh"
require_env || exit 2

if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
    test_skip "phase29ck_boundary_pure_bool_phi_branch_min: LLVM backend not available"
    exit 0
fi

if ! command -v llc >/dev/null 2>&1 && ! command -v llc-18 >/dev/null 2>&1; then
    test_skip "phase29ck_boundary_pure_bool_phi_branch_min: llc not found"
    exit 0
fi

FIXTURE="$NYASH_ROOT/apps/tests/mir_shape_guard/bool_phi_branch_min_v1.mir.json"
NY_LLVM_C="$NYASH_ROOT/target/release/ny-llvmc"
OUT_OBJ="${TMPDIR:-/tmp}/phase29ck_boundary_pure_bool_phi_branch_min_$$.o"
BUILD_LOG="${TMPDIR:-/tmp}/phase29ck_boundary_pure_bool_phi_branch_min_$$.log"
LL_DUMP="${TMPDIR:-/tmp}/phase29ck_boundary_pure_bool_phi_branch_min_$$.ll"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-90}"

cleanup() {
    rm -f "$OUT_OBJ" "$BUILD_LOG" "$LL_DUMP"
}
trap cleanup EXIT

require_smoke_path "phase29ck_boundary_pure_bool_phi_branch_min" "fixture" "$FIXTURE" || exit 1
require_smoke_path "phase29ck_boundary_pure_bool_phi_branch_min" "ny-llvmc" "$NY_LLVM_C" executable || exit 1
ensure_hako_llvmc_ffi_built "phase29ck_boundary_pure_bool_phi_branch_min" || exit 1

if capture_boundary_compile_to_log \
    "$BUILD_LOG" \
    "$RUN_TIMEOUT_SECS" \
    env \
      NYASH_LLVM_ROUTE_TRACE=1 \
      NYASH_NY_LLVM_COMPILER=/__missing__/ny-llvmc \
      NYASH_LLVM_DUMP_IR="$LL_DUMP" \
      "$NY_LLVM_C" --in "$FIXTURE" --out "$OUT_OBJ"; then
    BUILD_RC=0
else
    BUILD_RC=$?
fi

if [ "$BUILD_RC" -eq 124 ]; then
    test_fail "phase29ck_boundary_pure_bool_phi_branch_min: compile timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$BUILD_RC" -ne 0 ]; then
    echo "[INFO] compile output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "phase29ck_boundary_pure_bool_phi_branch_min: boundary default failed on bool phi branch shape (rc=$BUILD_RC)"
    exit 1
fi

require_smoke_path "phase29ck_boundary_pure_bool_phi_branch_min" "object" "$OUT_OBJ" || exit 1
require_smoke_path "phase29ck_boundary_pure_bool_phi_branch_min" "LLVM IR dump" "$LL_DUMP" || exit 1

if ! grep -Eq '%r7 = phi i1 \[ %r[0-9]+, %bb1 \], \[ %r6, %bb2 \]' "$LL_DUMP"; then
    echo "[INFO] lowered IR:"
    tail -n 120 "$LL_DUMP" || true
    test_fail "phase29ck_boundary_pure_bool_phi_branch_min: lowered IR did not emit phi i1 for merged bool"
    exit 1
fi

if ! grep -F 'br i1 %r7, label %bb4, label %bb5' "$LL_DUMP" >/dev/null 2>&1; then
    echo "[INFO] lowered IR:"
    tail -n 120 "$LL_DUMP" || true
    test_fail "phase29ck_boundary_pure_bool_phi_branch_min: branch did not consume phi i1 directly"
    exit 1
fi

if grep -F 'icmp ne i64 %r7, 0' "$LL_DUMP" >/dev/null 2>&1; then
    echo "[INFO] lowered IR:"
    tail -n 120 "$LL_DUMP" || true
    test_fail "phase29ck_boundary_pure_bool_phi_branch_min: branch still degraded phi bool through i64 compare"
    exit 1
fi

test_pass "phase29ck_boundary_pure_bool_phi_branch_min: PASS (boundary default emits phi i1 for merged bool branch)"
