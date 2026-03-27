#!/bin/bash
# Phase 29x: backend-owner cutover compare smoke
#
# Contract pin:
# 1) explicit `.hako ll emitter` compare lane stays opt-in only.
# 2) this smoke is archived from the active suite and now runs from
#    `phase29x-derust-archive.txt`.
# 3) narrow min-v0 fixtures (`ret const`, `bool phi/branch`, `concat3 extern`)
#    emit objects through the compare owner candidate.
# 4) compare lane emits stable ownership evidence before object handoff.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env || exit 2

if ! command -v llc >/dev/null 2>&1 && ! command -v llc-18 >/dev/null 2>&1; then
    test_skip "phase29x_backend_owner_hako_ll_compare_min: llc not found"
    exit 0
fi

SMOKE_NAME="phase29x_backend_owner_hako_ll_compare_min"
HAKORUNE_BIN="$NYASH_ROOT/target/release/hakorune"
APP="$NYASH_ROOT/apps/tests/phase29x_backend_owner_hako_ll_compare_min.hako"
FIXTURE_RET_CONST="$NYASH_ROOT/apps/tests/mir_shape_guard/ret_const_min_v1.mir.json"
FIXTURE_BOOL_PHI="$NYASH_ROOT/apps/tests/mir_shape_guard/bool_phi_branch_min_v1.mir.json"
FIXTURE_CONCAT3="$NYASH_ROOT/apps/tests/mir_shape_guard/string_concat3_extern_min_v1.mir.json"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-90}"

require_smoke_path "$SMOKE_NAME" "hakorune" "$HAKORUNE_BIN" executable || exit 1
require_smoke_path "$SMOKE_NAME" "compare app" "$APP" || exit 1
require_smoke_path "$SMOKE_NAME" "ret const fixture" "$FIXTURE_RET_CONST" || exit 1
require_smoke_path "$SMOKE_NAME" "bool phi fixture" "$FIXTURE_BOOL_PHI" || exit 1
require_smoke_path "$SMOKE_NAME" "concat3 fixture" "$FIXTURE_CONCAT3" || exit 1

run_case() {
    local case_name="$1"
    local fixture="$2"
    local out_obj="${TMPDIR:-/tmp}/${SMOKE_NAME}_${case_name}_$$.o"
    local out=""
    local rc=0
    local raw_log

    raw_log="$(mktemp "/tmp/${SMOKE_NAME}.${case_name}.XXXXXX.log")"

    rm -f "$out_obj"
    set +e
    timeout "$RUN_TIMEOUT_SECS" env -i \
        PATH="$PATH" \
        HOME="${HOME:-/tmp}" \
        TMPDIR="${TMPDIR:-/tmp}" \
        "$HAKORUNE_BIN" "$APP" -- "$fixture" "$out_obj" >"$raw_log" 2>&1
    rc=$?
    set -e
    out="$(cat "$raw_log")"

    if [ "$rc" -eq 124 ]; then
        echo "[INFO] compare output (case=$case_name):"
        echo "$out" | head -n 120 || true
        test_fail "$SMOKE_NAME: compare lane timed out (case=$case_name >${RUN_TIMEOUT_SECS}s)"
        rm -f "$out_obj" "$raw_log"
        exit 1
    fi

    if [ "$rc" -ne 0 ]; then
        echo "[INFO] compare output (case=$case_name):"
        echo "$out" | head -n 120 || true
        test_fail "$SMOKE_NAME: compare lane failed (case=$case_name rc=$rc)"
        rm -f "$out_obj" "$raw_log"
        exit 1
    fi

    require_smoke_path "$SMOKE_NAME" "object ($case_name)" "$out_obj" || exit 1

    if ! echo "$out" | grep -Fq "[hako-ll/compare] chosen_owner=hako-ll-min-v0 accepted=min-v0 first_blocker=none"; then
        echo "[INFO] compare output (case=$case_name):"
        echo "$out" | head -n 120 || true
        test_fail "$SMOKE_NAME: missing compare ownership tag (case=$case_name)"
        rm -f "$out_obj" "$raw_log"
        exit 1
    fi

    if ! echo "$out" | grep -Fq "[backend-owner-compare] object=$out_obj"; then
        echo "[INFO] compare output (case=$case_name):"
        echo "$out" | head -n 120 || true
        test_fail "$SMOKE_NAME: missing object handoff tag (case=$case_name)"
        rm -f "$out_obj" "$raw_log"
        exit 1
    fi

    rm -f "$out_obj" "$raw_log"
}

run_case "ret_const" "$FIXTURE_RET_CONST"
run_case "bool_phi_branch" "$FIXTURE_BOOL_PHI"
run_case "concat3_extern" "$FIXTURE_CONCAT3"

test_pass "$SMOKE_NAME: PASS (explicit .hako ll emitter compare lane emits min-v0 narrow fixtures)"
