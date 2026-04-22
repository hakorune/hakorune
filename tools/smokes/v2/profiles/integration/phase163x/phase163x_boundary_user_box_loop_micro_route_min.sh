#!/bin/bash
# phase-163x boundary pure-first smoke for UserBox loop micro route metadata
#
# Contract pin:
# 1) direct MIR emit publishes `userbox_loop_micro_seed_route` for the current
#    point-add and flag-toggle loop micro benchmarks.
# 2) boundary pure-first consumes that metadata through `exact_seed_backend_route`
#    without falling back to legacy raw-MIR UserBox loop seed matchers.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase163x_boundary_user_box_loop_micro_route_min"

if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
    test_skip "$SMOKE_NAME: LLVM backend not available"
    exit 0
fi

if ! command -v python3 >/dev/null 2>&1; then
    test_skip "$SMOKE_NAME: python3 not found"
    exit 0
fi

if ! python3 -c "import llvmlite" >/dev/null 2>&1; then
    test_skip "$SMOKE_NAME: llvmlite not found"
    exit 0
fi

HAKORUNE="$NYASH_ROOT/target/release/hakorune"
NY_LLVM_C="$NYASH_ROOT/target/release/ny-llvmc"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-90}"

CASES=(
    "point_add_micro|userbox_point_add_loop_micro_seed|Point|2000000|null|6|4|1|0|7|benchmarks/bench_kilo_micro_userbox_point_add.hako"
    "flag_toggle_micro|userbox_flag_toggle_loop_micro_seed|Flag|2000000|1000000|2|2|2|2|3|benchmarks/bench_kilo_micro_userbox_flag_toggle.hako"
)

cleanup() {
    rm -f "${TMPDIR:-/tmp}"/"${SMOKE_NAME}"_*_"$$".json
    rm -f "${TMPDIR:-/tmp}"/"${SMOKE_NAME}"_*_"$$".o
    rm -f "${TMPDIR:-/tmp}"/"${SMOKE_NAME}"_*_"$$".log
}
trap cleanup EXIT

require_smoke_path "$SMOKE_NAME" "hakorune" "$HAKORUNE" executable || exit 1
require_smoke_path "$SMOKE_NAME" "ny-llvmc" "$NY_LLVM_C" executable || exit 1
ensure_hako_llvmc_ffi_built "$SMOKE_NAME" || exit 1

for CASE in "${CASES[@]}"; do
    IFS='|' read -r KIND PROOF BOX_NAME OPS FLIP_AT FIELD_GET FIELD_SET CMP_LT CMP_EQ BINOP INPUT_REL <<<"$CASE"
    INPUT="$NYASH_ROOT/$INPUT_REL"
    BASENAME="$(basename "$INPUT" .hako)"
    MIR_JSON="${TMPDIR:-/tmp}/${SMOKE_NAME}_${BASENAME}_$$.json"
    EMIT_LOG="${TMPDIR:-/tmp}/${SMOKE_NAME}_${BASENAME}_emit_$$.log"
    OUT_OBJ="${TMPDIR:-/tmp}/${SMOKE_NAME}_${BASENAME}_$$.o"
    BUILD_LOG="${TMPDIR:-/tmp}/${SMOKE_NAME}_${BASENAME}_build_$$.log"

    require_smoke_path "$SMOKE_NAME" "input" "$INPUT" || exit 1

    if ! timeout "$RUN_TIMEOUT_SECS" "$HAKORUNE" --backend mir --emit-mir-json "$MIR_JSON" "$INPUT" >"$EMIT_LOG" 2>&1; then
        echo "[INFO] emit output:"
        tail -n 120 "$EMIT_LOG" || true
        test_fail "$SMOKE_NAME: direct MIR emit failed fixture=${BASENAME}"
        exit 1
    fi
    require_smoke_path "$SMOKE_NAME" "MIR JSON" "$MIR_JSON" || exit 1

    if ! jq -e \
        --arg kind "$KIND" \
        --arg proof "$PROOF" \
        --arg box_name "$BOX_NAME" \
        --argjson ops "$OPS" \
        --argjson flip_at "$FLIP_AT" \
        --argjson field_get "$FIELD_GET" \
        --argjson field_set "$FIELD_SET" \
        --argjson cmp_lt "$CMP_LT" \
        --argjson cmp_eq "$CMP_EQ" \
        --argjson binop "$BINOP" \
        '
        .functions[] | select(.name == "main") | .metadata as $m |
        ($m.userbox_loop_micro_seed_route.kind == $kind) and
        ($m.userbox_loop_micro_seed_route.box == $box_name) and
        ($m.userbox_loop_micro_seed_route.ops == $ops) and
        ($m.userbox_loop_micro_seed_route.flip_at == $flip_at) and
        ($m.userbox_loop_micro_seed_route.field_get_count == $field_get) and
        ($m.userbox_loop_micro_seed_route.field_set_count == $field_set) and
        ($m.userbox_loop_micro_seed_route.compare_lt_count == $cmp_lt) and
        ($m.userbox_loop_micro_seed_route.compare_eq_count == $cmp_eq) and
        ($m.userbox_loop_micro_seed_route.binop_count == $binop) and
        ($m.userbox_loop_micro_seed_route.proof == $proof) and
        ($m.userbox_loop_micro_seed_route.consumer_capability == "direct_userbox_loop_micro") and
        ($m.userbox_loop_micro_seed_route.publication_boundary == "none") and
        ($m.exact_seed_backend_route.tag == "userbox_loop_micro") and
        ($m.exact_seed_backend_route.source_route == "userbox_loop_micro_seed_route") and
        ($m.exact_seed_backend_route.proof == $proof)
        ' "$MIR_JSON" >/dev/null; then
        echo "[INFO] route metadata:"
        jq '.functions[] | select(.name == "main") | .metadata | {userbox_loop_micro_seed_route, exact_seed_backend_route}' "$MIR_JSON" || true
        test_fail "$SMOKE_NAME: route metadata mismatch fixture=${BASENAME}"
        exit 1
    fi

    if capture_boundary_compile_to_log \
        "$BUILD_LOG" \
        "$RUN_TIMEOUT_SECS" \
        env \
          HAKO_BACKEND_COMPILE_RECIPE="pure-first" \
          HAKO_BACKEND_COMPAT_REPLAY="harness" \
          NYASH_LLVM_ROUTE_TRACE=1 \
          NYASH_NY_LLVM_COMPILER="$NY_LLVM_C" \
          "$NY_LLVM_C" --in "$MIR_JSON" --out "$OUT_OBJ"; then
        BUILD_RC=0
    else
        BUILD_RC=$?
    fi

    if [ "$BUILD_RC" -eq 124 ]; then
        test_fail "$SMOKE_NAME: compile timed out (>${RUN_TIMEOUT_SECS}s fixture=${BASENAME})"
        exit 1
    fi

    if [ "$BUILD_RC" -ne 0 ]; then
        echo "[INFO] compile output:"
        tail -n 120 "$BUILD_LOG" || true
        test_fail "$SMOKE_NAME: boundary route compile failed (rc=$BUILD_RC fixture=${BASENAME})"
        exit 1
    fi

    require_smoke_path "$SMOKE_NAME" "object" "$OUT_OBJ" || exit 1

    if ! grep -Fq "[llvm-route/trace] stage=exact_seed_backend_route result=hit reason=mir_route_metadata extra=userbox_loop_micro" "$BUILD_LOG"; then
        echo "[INFO] compile output:"
        tail -n 120 "$BUILD_LOG" || true
        test_fail "$SMOKE_NAME: exact route trace missing fixture=${BASENAME}"
        exit 1
    fi

    if ! grep -Fq "[llvm-route/trace] stage=userbox_loop_micro result=emit reason=exact_match" "$BUILD_LOG"; then
        echo "[INFO] compile output:"
        tail -n 120 "$BUILD_LOG" || true
        test_fail "$SMOKE_NAME: userbox_loop_micro emit trace missing fixture=${BASENAME}"
        exit 1
    fi

    if grep -Fq "[llvm-route/replay]" "$BUILD_LOG"; then
        echo "[INFO] compile output:"
        tail -n 120 "$BUILD_LOG" || true
        test_fail "$SMOKE_NAME: boundary route unexpectedly replayed compat lane fixture=${BASENAME}"
        exit 1
    fi
done

test_pass "$SMOKE_NAME: PASS (point_add and flag_toggle loop micro benchmarks route through MIR-owned userbox_loop_micro metadata)"
