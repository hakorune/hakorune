#!/bin/bash
# RC/GC alignment G-RC-5: GC mode semantics invariance gate (VM/LLVM)
#
# Contract pin:
# - "beginner" (rc+cycle) and "expert" (off) modes must keep exit semantics unchanged.
# - Per fixture, both modes and both backends must agree on expected exit.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/vm_route_pin.sh"
require_env || exit 2

CASES_FILE="$NYASH_ROOT/tools/checks/rc_gc_alignment_g5_mode_invariance_cases.txt"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-45}"

llvm_harness_enabled() {
    "$NYASH_BIN" --version 2>/dev/null | grep -q "features.*llvm"
}

run_step() {
    local cmd="$1"
    if ! bash "$NYASH_ROOT/$cmd"; then
        test_fail "rc_gc_alignment_g5_mode_invariance_vm_llvm: step failed: $cmd"
        exit 1
    fi
}

run_backend_case() {
    local backend="$1"
    local gc_mode="$2"
    local fixture="$3"
    local output
    local rc
    local -a cmd=("$NYASH_BIN" --backend "$backend" "$fixture")

    local -a env_prefix=()
    if [[ "$backend" = "vm" ]]; then
        env_prefix=(run_with_vm_route_pin env NYASH_DISABLE_PLUGINS=1 NYASH_GC_MODE="$gc_mode")
    else
        env_prefix=(env NYASH_LLVM_USE_HARNESS=1 NYASH_DISABLE_PLUGINS=1 NYASH_GC_MODE="$gc_mode")
    fi

    set +e
    output=$(
        "${env_prefix[@]}" timeout "$RUN_TIMEOUT_SECS" "${cmd[@]}" 2>&1
    )
    rc=$?
    set -e

    printf '%s' "$output"
    return "$rc"
}

if [ ! -f "$CASES_FILE" ]; then
    test_fail "rc_gc_alignment_g5_mode_invariance_vm_llvm: cases file missing: $CASES_FILE"
    exit 1
fi

run_step "tools/checks/rc_gc_alignment_g5_mode_invariance_guard.sh"

if ! llvm_harness_enabled; then
    test_skip "rc_gc_alignment_g5_mode_invariance_vm_llvm: hakorune built without --features llvm (harness lane unavailable)"
    exit 0
fi

while IFS= read -r row || [ -n "$row" ]; do
    [ -z "$row" ] && continue
    IFS='|' read -r case_id fixture_rel expected_exit focus <<<"$row"
    fixture="$NYASH_ROOT/$fixture_rel"
    if [ ! -f "$fixture" ]; then
        test_fail "rc_gc_alignment_g5_mode_invariance_vm_llvm: fixture missing: $fixture"
        exit 1
    fi

    set +e
    out_vm_rc="$(run_backend_case vm rc+cycle "$fixture")"
    rc_vm_rc=$?
    set -e
    if [ "$rc_vm_rc" -eq 124 ]; then
        test_fail "rc_gc_alignment_g5_mode_invariance_vm_llvm: VM rc+cycle timed out (>${RUN_TIMEOUT_SECS}s, case=$case_id)"
        exit 1
    fi

    set +e
    out_vm_off="$(run_backend_case vm off "$fixture")"
    rc_vm_off=$?
    set -e
    if [ "$rc_vm_off" -eq 124 ]; then
        test_fail "rc_gc_alignment_g5_mode_invariance_vm_llvm: VM off timed out (>${RUN_TIMEOUT_SECS}s, case=$case_id)"
        exit 1
    fi

    set +e
    out_llvm_rc="$(run_backend_case llvm rc+cycle "$fixture")"
    rc_llvm_rc=$?
    set -e
    if [ "$rc_llvm_rc" -eq 124 ]; then
        test_fail "rc_gc_alignment_g5_mode_invariance_vm_llvm: LLVM rc+cycle timed out (>${RUN_TIMEOUT_SECS}s, case=$case_id)"
        exit 1
    fi

    set +e
    out_llvm_off="$(run_backend_case llvm off "$fixture")"
    rc_llvm_off=$?
    set -e
    if [ "$rc_llvm_off" -eq 124 ]; then
        test_fail "rc_gc_alignment_g5_mode_invariance_vm_llvm: LLVM off timed out (>${RUN_TIMEOUT_SECS}s, case=$case_id)"
        exit 1
    fi

    if [ "$rc_vm_rc" -ne "$expected_exit" ]; then
        echo "[INFO] vm rc+cycle output (case=$case_id focus=$focus):"
        echo "$out_vm_rc" | tail -n 120 || true
        test_fail "rc_gc_alignment_g5_mode_invariance_vm_llvm: VM rc+cycle exit mismatch (case=$case_id expected=$expected_exit got=$rc_vm_rc)"
        exit 1
    fi
    if [ "$rc_vm_off" -ne "$expected_exit" ]; then
        echo "[INFO] vm off output (case=$case_id focus=$focus):"
        echo "$out_vm_off" | tail -n 120 || true
        test_fail "rc_gc_alignment_g5_mode_invariance_vm_llvm: VM off exit mismatch (case=$case_id expected=$expected_exit got=$rc_vm_off)"
        exit 1
    fi
    if [ "$rc_llvm_rc" -ne "$expected_exit" ]; then
        echo "[INFO] llvm rc+cycle output (case=$case_id focus=$focus):"
        echo "$out_llvm_rc" | tail -n 120 || true
        test_fail "rc_gc_alignment_g5_mode_invariance_vm_llvm: LLVM rc+cycle exit mismatch (case=$case_id expected=$expected_exit got=$rc_llvm_rc)"
        exit 1
    fi
    if [ "$rc_llvm_off" -ne "$expected_exit" ]; then
        echo "[INFO] llvm off output (case=$case_id focus=$focus):"
        echo "$out_llvm_off" | tail -n 120 || true
        test_fail "rc_gc_alignment_g5_mode_invariance_vm_llvm: LLVM off exit mismatch (case=$case_id expected=$expected_exit got=$rc_llvm_off)"
        exit 1
    fi

    if [ "$rc_vm_rc" -ne "$rc_vm_off" ]; then
        echo "[INFO] vm rc+cycle output (case=$case_id focus=$focus):"
        echo "$out_vm_rc" | tail -n 120 || true
        echo "[INFO] vm off output (case=$case_id focus=$focus):"
        echo "$out_vm_off" | tail -n 120 || true
        test_fail "rc_gc_alignment_g5_mode_invariance_vm_llvm: VM mode invariance mismatch (case=$case_id rc+cycle=$rc_vm_rc off=$rc_vm_off)"
        exit 1
    fi
    if [ "$rc_llvm_rc" -ne "$rc_llvm_off" ]; then
        echo "[INFO] llvm rc+cycle output (case=$case_id focus=$focus):"
        echo "$out_llvm_rc" | tail -n 120 || true
        echo "[INFO] llvm off output (case=$case_id focus=$focus):"
        echo "$out_llvm_off" | tail -n 120 || true
        test_fail "rc_gc_alignment_g5_mode_invariance_vm_llvm: LLVM mode invariance mismatch (case=$case_id rc+cycle=$rc_llvm_rc off=$rc_llvm_off)"
        exit 1
    fi

    if [ "$rc_vm_rc" -ne "$rc_llvm_rc" ]; then
        echo "[INFO] vm rc+cycle output (case=$case_id focus=$focus):"
        echo "$out_vm_rc" | tail -n 120 || true
        echo "[INFO] llvm rc+cycle output (case=$case_id focus=$focus):"
        echo "$out_llvm_rc" | tail -n 120 || true
        test_fail "rc_gc_alignment_g5_mode_invariance_vm_llvm: backend parity mismatch in rc+cycle mode (case=$case_id vm=$rc_vm_rc llvm=$rc_llvm_rc)"
        exit 1
    fi
    if [ "$rc_vm_off" -ne "$rc_llvm_off" ]; then
        echo "[INFO] vm off output (case=$case_id focus=$focus):"
        echo "$out_vm_off" | tail -n 120 || true
        echo "[INFO] llvm off output (case=$case_id focus=$focus):"
        echo "$out_llvm_off" | tail -n 120 || true
        test_fail "rc_gc_alignment_g5_mode_invariance_vm_llvm: backend parity mismatch in off mode (case=$case_id vm=$rc_vm_off llvm=$rc_llvm_off)"
        exit 1
    fi
done < <(grep -v '^[[:space:]]*#' "$CASES_FILE" | sed '/^[[:space:]]*$/d')

test_pass "rc_gc_alignment_g5_mode_invariance_vm_llvm: PASS (G-RC-5 GC mode semantics invariance locked)"
