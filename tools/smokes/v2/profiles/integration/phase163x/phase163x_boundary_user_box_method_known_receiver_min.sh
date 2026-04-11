#!/bin/bash
# phase-163x boundary pure-first smoke for selected user-box method thin-entry metadata
#
# Contract pin:
# 1) MIR JSON fixtures carrying `user_box_method.known_receiver` selections stay
#    accepted by the product `ny-llvmc` boundary pure-first route.
# 2) the current native-driver/shim consumer slice stays narrow: selected
#    `Counter.step_chain`, `Counter.step`, and `Point.sum` rows are consumed
#    beneath canonical `Call` without compat replay while relying on the
#    already-landed scalar field selections, including the single-copy receiver
#    alias lane and the 1-hop recursive user-box delegate lane.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
    test_skip "phase163x_boundary_user_box_method_known_receiver_min: LLVM backend not available"
    exit 0
fi

if ! command -v python3 >/dev/null 2>&1; then
    test_skip "phase163x_boundary_user_box_method_known_receiver_min: python3 not found"
    exit 0
fi

if ! python3 -c "import llvmlite" >/dev/null 2>&1; then
    test_skip "phase163x_boundary_user_box_method_known_receiver_min: llvmlite not found"
    exit 0
fi

NY_LLVM_C="$NYASH_ROOT/target/release/ny-llvmc"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-90}"
FIXTURES=(
    "$NYASH_ROOT/apps/tests/mir_shape_guard/user_box_counter_step_local_i64_min.prebuilt.mir.json"
    "$NYASH_ROOT/apps/tests/mir_shape_guard/user_box_counter_step_copy_local_i64_min.prebuilt.mir.json"
    "$NYASH_ROOT/apps/tests/mir_shape_guard/user_box_counter_step_chain_local_i64_min.prebuilt.mir.json"
    "$NYASH_ROOT/apps/tests/mir_shape_guard/user_box_point_sum_local_i64_min.prebuilt.mir.json"
    "$NYASH_ROOT/apps/tests/mir_shape_guard/user_box_point_sum_copy_local_i64_min.prebuilt.mir.json"
)

cleanup() {
    rm -f "${TMPDIR:-/tmp}"/phase163x_boundary_user_box_method_known_receiver_min_*_"$$".o
    rm -f "${TMPDIR:-/tmp}"/phase163x_boundary_user_box_method_known_receiver_min_*_"$$".log
}
trap cleanup EXIT

require_smoke_path "phase163x_boundary_user_box_method_known_receiver_min" "ny-llvmc" "$NY_LLVM_C" executable || exit 1
ensure_hako_llvmc_ffi_built "phase163x_boundary_user_box_method_known_receiver_min" || exit 1

for FIXTURE in "${FIXTURES[@]}"; do
    BASENAME="$(basename "$FIXTURE" .prebuilt.mir.json)"
    OUT_OBJ="${TMPDIR:-/tmp}/phase163x_boundary_user_box_method_known_receiver_min_${BASENAME}_$$.o"
    BUILD_LOG="${TMPDIR:-/tmp}/phase163x_boundary_user_box_method_known_receiver_min_${BASENAME}_$$.log"

    require_smoke_path "phase163x_boundary_user_box_method_known_receiver_min" "fixture" "$FIXTURE" || exit 1

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
        test_fail "phase163x_boundary_user_box_method_known_receiver_min: compile timed out (>${RUN_TIMEOUT_SECS}s fixture=${BASENAME})"
        exit 1
    fi

    if [ "$BUILD_RC" -ne 0 ]; then
        echo "[INFO] compile output:"
        tail -n 120 "$BUILD_LOG" || true
        test_fail "phase163x_boundary_user_box_method_known_receiver_min: metadata-bearing known_receiver route failed (rc=$BUILD_RC fixture=${BASENAME})"
        exit 1
    fi

    require_smoke_path "phase163x_boundary_user_box_method_known_receiver_min" "object" "$OUT_OBJ" || exit 1

    if ! grep -Fq "[llvm-route/select] owner=boundary recipe=pure-first compat_replay=harness" "$BUILD_LOG"; then
        echo "[INFO] compile output:"
        tail -n 120 "$BUILD_LOG" || true
        test_fail "phase163x_boundary_user_box_method_known_receiver_min: boundary route did not advertise compat_replay=harness fixture=${BASENAME}"
        exit 1
    fi

    if grep -Fq "[llvm-route/replay]" "$BUILD_LOG"; then
        echo "[INFO] compile output:"
        tail -n 120 "$BUILD_LOG" || true
        test_fail "phase163x_boundary_user_box_method_known_receiver_min: boundary route unexpectedly replayed compat lane fixture=${BASENAME}"
        exit 1
    fi
done

test_pass "phase163x_boundary_user_box_method_known_receiver_min: PASS (selected Counter.step_chain, Counter.step and Point.sum known_receiver metadata stay green on boundary pure-first owner lane without compat replay, including the single-copy receiver alias lane and 1-hop recursive delegate lane)"
