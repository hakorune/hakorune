#!/bin/bash
# phase-163x boundary compat-keep smoke for metadata-bearing sum JSON
#
# Contract pin:
# 1) a MIR JSON fixture carrying `thin_entry_selections` + sum placement metadata
#    stays accepted by the product `ny-llvmc` keep/replay route.
# 2) explicit `pure-first + compat_replay=harness` still advertises the replay
#    boundary instead of silently counting as a pure-first owner win.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
    test_skip "phase163x_boundary_sum_metadata_keep_min: LLVM backend not available"
    exit 0
fi

if ! command -v python3 >/dev/null 2>&1; then
    test_skip "phase163x_boundary_sum_metadata_keep_min: python3 not found"
    exit 0
fi

if ! python3 -c "import llvmlite" >/dev/null 2>&1; then
    test_skip "phase163x_boundary_sum_metadata_keep_min: llvmlite not found"
    exit 0
fi

FIXTURE="$NYASH_ROOT/apps/tests/mir_shape_guard/sum_option_project_local_i64_min.prebuilt.mir.json"
NY_LLVM_C="$NYASH_ROOT/target/release/ny-llvmc"
OUT_OBJ="${TMPDIR:-/tmp}/phase163x_boundary_sum_metadata_keep_min_$$.o"
BUILD_LOG="${TMPDIR:-/tmp}/phase163x_boundary_sum_metadata_keep_min_$$.log"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-90}"

cleanup() {
    rm -f "$OUT_OBJ" "$BUILD_LOG"
}
trap cleanup EXIT

require_smoke_path "phase163x_boundary_sum_metadata_keep_min" "fixture" "$FIXTURE" || exit 1
require_smoke_path "phase163x_boundary_sum_metadata_keep_min" "ny-llvmc" "$NY_LLVM_C" executable || exit 1
ensure_hako_llvmc_ffi_built "phase163x_boundary_sum_metadata_keep_min" || exit 1

if capture_boundary_compile_to_log \
    "$BUILD_LOG" \
    "$RUN_TIMEOUT_SECS" \
    env \
      HAKO_BACKEND_COMPILE_RECIPE="pure-first" \
      HAKO_BACKEND_COMPAT_REPLAY="harness" \
      NYASH_LLVM_ROUTE_TRACE=1 \
      NYASH_NY_LLVM_COMPILER="$NY_LLVM_C" \
      "$NY_LLVM_C" --in "$FIXTURE" --out "$OUT_OBJ"; then
    BUILD_RC=0
else
    BUILD_RC=$?
fi

if [ "$BUILD_RC" -eq 124 ]; then
    test_fail "phase163x_boundary_sum_metadata_keep_min: compile timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$BUILD_RC" -ne 0 ]; then
    echo "[INFO] compile output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "phase163x_boundary_sum_metadata_keep_min: metadata-bearing sum replay failed (rc=$BUILD_RC)"
    exit 1
fi

require_smoke_path "phase163x_boundary_sum_metadata_keep_min" "object" "$OUT_OBJ" || exit 1

if ! grep -Fq "[llvm-route/select] owner=boundary recipe=pure-first compat_replay=harness" "$BUILD_LOG"; then
    echo "[INFO] compile output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "phase163x_boundary_sum_metadata_keep_min: boundary route did not advertise compat_replay=harness"
    exit 1
fi

if ! grep -Fq "[llvm-route/replay] lane=harness reason=unsupported_pure_shape" "$BUILD_LOG"; then
    echo "[INFO] compile output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "phase163x_boundary_sum_metadata_keep_min: boundary route did not replay harness for metadata-bearing sum shape"
    exit 1
fi

test_pass "phase163x_boundary_sum_metadata_keep_min: PASS (metadata-bearing sum JSON stays green via boundary compat replay -> harness keep lane)"
