#!/bin/bash
# phase-163x boundary pure-first smoke for metadata-bearing sum JSON
#
# Contract pin:
# 1) MIR JSON fixtures carrying `thin_entry_selections` + sum placement metadata
#    stay accepted by the product `ny-llvmc` boundary pure-first route.
# 2) explicit `pure-first + compat_replay=harness` no longer replays harness for
#    the selected local-sum proving slice.

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

NY_LLVM_C="$NYASH_ROOT/target/release/ny-llvmc"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-90}"
FIXTURES=(
    "$NYASH_ROOT/apps/tests/mir_shape_guard/sum_option_project_local_i64_min.prebuilt.mir.json"
    "$NYASH_ROOT/apps/tests/mir_shape_guard/sum_result_ok_local_i64_min.prebuilt.mir.json"
    "$NYASH_ROOT/apps/tests/mir_shape_guard/sum_result_ok_project_copy_local_i64_min.prebuilt.mir.json"
    "$NYASH_ROOT/apps/tests/mir_shape_guard/sum_result_ok_project_copy_local_f64_min.prebuilt.mir.json"
    "$NYASH_ROOT/apps/tests/mir_shape_guard/sum_result_ok_project_copy_local_handle_min.prebuilt.mir.json"
    "$NYASH_ROOT/apps/tests/mir_shape_guard/sum_result_ok_project_local_f64_min.prebuilt.mir.json"
    "$NYASH_ROOT/apps/tests/mir_shape_guard/sum_result_ok_project_local_handle_min.prebuilt.mir.json"
    "$NYASH_ROOT/apps/tests/mir_shape_guard/sum_result_ok_tag_only_local_min.prebuilt.mir.json"
    "$NYASH_ROOT/apps/tests/mir_shape_guard/sum_result_ok_tag_local_f64_min.prebuilt.mir.json"
    "$NYASH_ROOT/apps/tests/mir_shape_guard/sum_result_ok_tag_local_handle_min.prebuilt.mir.json"
    "$NYASH_ROOT/apps/tests/mir_shape_guard/sum_result_ok_tag_local_i64_min.prebuilt.mir.json"
    "$NYASH_ROOT/apps/tests/mir_shape_guard/sum_result_ok_tag_copy_local_i64_min.prebuilt.mir.json"
)

cleanup() {
    rm -f "${TMPDIR:-/tmp}"/phase163x_boundary_sum_metadata_keep_min_*_"$$".o
    rm -f "${TMPDIR:-/tmp}"/phase163x_boundary_sum_metadata_keep_min_*_"$$".log
}
trap cleanup EXIT

require_smoke_path "phase163x_boundary_sum_metadata_keep_min" "ny-llvmc" "$NY_LLVM_C" executable || exit 1
ensure_hako_llvmc_ffi_built "phase163x_boundary_sum_metadata_keep_min" || exit 1

for FIXTURE in "${FIXTURES[@]}"; do
    BASENAME="$(basename "$FIXTURE" .prebuilt.mir.json)"
    OUT_OBJ="${TMPDIR:-/tmp}/phase163x_boundary_sum_metadata_keep_min_${BASENAME}_$$.o"
    BUILD_LOG="${TMPDIR:-/tmp}/phase163x_boundary_sum_metadata_keep_min_${BASENAME}_$$.log"

    require_smoke_path "phase163x_boundary_sum_metadata_keep_min" "fixture" "$FIXTURE" || exit 1

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
        test_fail "phase163x_boundary_sum_metadata_keep_min: compile timed out (>${RUN_TIMEOUT_SECS}s) fixture=${BASENAME}"
        exit 1
    fi

    if [ "$BUILD_RC" -ne 0 ]; then
        echo "[INFO] compile output:"
        tail -n 120 "$BUILD_LOG" || true
        test_fail "phase163x_boundary_sum_metadata_keep_min: metadata-bearing sum replay failed (rc=$BUILD_RC fixture=${BASENAME})"
        exit 1
    fi

    require_smoke_path "phase163x_boundary_sum_metadata_keep_min" "object" "$OUT_OBJ" || exit 1

    if ! grep -Fq "[llvm-route/select] owner=boundary recipe=pure-first compat_replay=harness" "$BUILD_LOG"; then
        echo "[INFO] compile output:"
        tail -n 120 "$BUILD_LOG" || true
        test_fail "phase163x_boundary_sum_metadata_keep_min: boundary route did not advertise compat_replay=harness fixture=${BASENAME}"
        exit 1
    fi

    if grep -Fq "[llvm-route/replay]" "$BUILD_LOG"; then
        echo "[INFO] compile output:"
        tail -n 120 "$BUILD_LOG" || true
        test_fail "phase163x_boundary_sum_metadata_keep_min: boundary route unexpectedly replayed compat lane for metadata-bearing sum shape fixture=${BASENAME}"
        exit 1
    fi

    case "$BASENAME" in
        sum_result_ok_tag_only_local_min|sum_result_ok_tag_local_f64_min|sum_result_ok_tag_local_handle_min|sum_result_ok_tag_local_i64_min|sum_result_ok_tag_copy_local_i64_min)
            if ! grep -Fq "stage=exact_seed_backend_route result=hit reason=mir_route_metadata extra=sum_variant_tag_local" "$BUILD_LOG"; then
                echo "[INFO] compile output:"
                tail -n 120 "$BUILD_LOG" || true
                test_fail "phase163x_boundary_sum_metadata_keep_min: sum variant_tag fixture did not hit exact route metadata fixture=${BASENAME}"
                exit 1
            fi
            if ! grep -Fq "stage=sum_variant_tag_local result=emit reason=exact_match" "$BUILD_LOG"; then
                echo "[INFO] compile output:"
                tail -n 120 "$BUILD_LOG" || true
                test_fail "phase163x_boundary_sum_metadata_keep_min: sum variant_tag fixture did not emit from route metadata fixture=${BASENAME}"
                exit 1
            fi
            ;;
        sum_option_project_local_i64_min|sum_result_ok_local_i64_min|sum_result_ok_project_copy_local_i64_min|sum_result_ok_project_copy_local_f64_min|sum_result_ok_project_copy_local_handle_min|sum_result_ok_project_local_f64_min|sum_result_ok_project_local_handle_min)
            if ! grep -Fq "stage=exact_seed_backend_route result=hit reason=mir_route_metadata extra=sum_variant_project_local" "$BUILD_LOG"; then
                echo "[INFO] compile output:"
                tail -n 120 "$BUILD_LOG" || true
                test_fail "phase163x_boundary_sum_metadata_keep_min: sum variant_project fixture did not hit exact route metadata fixture=${BASENAME}"
                exit 1
            fi
            if ! grep -Fq "stage=sum_variant_project_local result=emit reason=exact_match" "$BUILD_LOG"; then
                echo "[INFO] compile output:"
                tail -n 120 "$BUILD_LOG" || true
                test_fail "phase163x_boundary_sum_metadata_keep_min: sum variant_project fixture did not emit from route metadata fixture=${BASENAME}"
                exit 1
            fi
            ;;
    esac
done

test_pass "phase163x_boundary_sum_metadata_keep_min: PASS (metadata-bearing sum JSON fixtures stay green on boundary pure-first owner lane, including direct and copied variant_project, tag-only, tag-f64, tag-handle, direct variant_tag, and copied variant_tag aliases)"
