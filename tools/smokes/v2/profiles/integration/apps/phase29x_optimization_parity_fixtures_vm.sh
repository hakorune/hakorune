#!/bin/bash
# Phase 29x X64: optimization parity fixtures + reject fixture gate
#
# Contract pin:
# - Replay X63 optimization allowlist lock as precondition.
# - For parity fixtures, optimize ON/OFF must preserve rc + stdout.
# - For reject fixture, optimize ON/OFF must both fail (non-zero) with stable reason.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/vm_route_pin.sh"
require_env || exit 2

PARITY_FIXTURES_FILE="$NYASH_ROOT/tools/checks/phase29x_optimization_parity_fixtures.txt"
REJECT_FIXTURES_FILE="$NYASH_ROOT/tools/checks/phase29x_optimization_reject_fixtures.txt"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"

run_step() {
    local cmd="$1"
    if ! bash "$NYASH_ROOT/$cmd"; then
        test_fail "phase29x_optimization_parity_fixtures_vm: step failed: $cmd"
        exit 1
    fi
}

run_vm_case() {
    local fixture="$1"
    local mode="$2"
    local output
    local rc
    local -a cmd=("$NYASH_BIN" --backend vm "$fixture")
    if [ "$mode" = "off" ]; then
        cmd=("$NYASH_BIN" --backend vm --no-optimize "$fixture")
    fi

    set +e
    output=$(
        run_with_vm_route_pin env \
            NYASH_DISABLE_PLUGINS=1 \
            timeout "$RUN_TIMEOUT_SECS" \
            "${cmd[@]}" 2>&1
    )
    rc=$?
    set -e

    printf '%s' "$output"
    return "$rc"
}

normalize_output() {
    local raw="$1"
    printf '%s\n' "$raw" | filter_noise | sed '/^[[:space:]]*$/d' || true
}

load_case_lines() {
    local file="$1"
    grep -v '^[[:space:]]*#' "$file" | sed '/^[[:space:]]*$/d'
}

if [ ! -f "$PARITY_FIXTURES_FILE" ]; then
    test_fail "phase29x_optimization_parity_fixtures_vm: parity fixture inventory missing: $PARITY_FIXTURES_FILE"
    exit 1
fi
if [ ! -f "$REJECT_FIXTURES_FILE" ]; then
    test_fail "phase29x_optimization_parity_fixtures_vm: reject fixture inventory missing: $REJECT_FIXTURES_FILE"
    exit 1
fi

run_step "tools/checks/phase29x_optimization_parity_guard.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_optimization_allowlist_lock_vm.sh"

while IFS= read -r line || [ -n "$line" ]; do
    [ -z "$line" ] && continue
    IFS='|' read -r fixture_rel expected_rc expected_stdout <<<"$line"
    fixture="$NYASH_ROOT/$fixture_rel"
    if [ ! -f "$fixture" ]; then
        test_fail "phase29x_optimization_parity_fixtures_vm: parity fixture missing: $fixture"
        exit 1
    fi

    set +e
    out_on="$(run_vm_case "$fixture" on)"
    rc_on=$?
    set -e
    if [ "$rc_on" -eq 124 ]; then
        test_fail "phase29x_optimization_parity_fixtures_vm: parity fixture timed out (opt-on): $fixture_rel"
        exit 1
    fi

    set +e
    out_off="$(run_vm_case "$fixture" off)"
    rc_off=$?
    set -e
    if [ "$rc_off" -eq 124 ]; then
        test_fail "phase29x_optimization_parity_fixtures_vm: parity fixture timed out (no-opt): $fixture_rel"
        exit 1
    fi

    if [ "$rc_on" -ne "$expected_rc" ]; then
        echo "[INFO] parity fixture output (opt-on):"
        echo "$out_on" | tail -n 120 || true
        test_fail "phase29x_optimization_parity_fixtures_vm: rc mismatch (opt-on expected=$expected_rc got=$rc_on, fixture=$fixture_rel)"
        exit 1
    fi
    if [ "$rc_off" -ne "$expected_rc" ]; then
        echo "[INFO] parity fixture output (no-opt):"
        echo "$out_off" | tail -n 120 || true
        test_fail "phase29x_optimization_parity_fixtures_vm: rc mismatch (no-opt expected=$expected_rc got=$rc_off, fixture=$fixture_rel)"
        exit 1
    fi
    if [ "$rc_on" -ne "$rc_off" ]; then
        echo "[INFO] parity fixture output (opt-on):"
        echo "$out_on" | tail -n 120 || true
        echo "[INFO] parity fixture output (no-opt):"
        echo "$out_off" | tail -n 120 || true
        test_fail "phase29x_optimization_parity_fixtures_vm: rc parity mismatch (on=$rc_on off=$rc_off, fixture=$fixture_rel)"
        exit 1
    fi

    clean_on="$(normalize_output "$out_on")"
    clean_off="$(normalize_output "$out_off")"

    if [ "$clean_on" != "$expected_stdout" ]; then
        echo "[INFO] normalized output (opt-on):"
        echo "$clean_on"
        test_fail "phase29x_optimization_parity_fixtures_vm: stdout mismatch (opt-on expected='$expected_stdout', fixture=$fixture_rel)"
        exit 1
    fi
    if [ "$clean_off" != "$expected_stdout" ]; then
        echo "[INFO] normalized output (no-opt):"
        echo "$clean_off"
        test_fail "phase29x_optimization_parity_fixtures_vm: stdout mismatch (no-opt expected='$expected_stdout', fixture=$fixture_rel)"
        exit 1
    fi
    if [ "$clean_on" != "$clean_off" ]; then
        echo "[INFO] normalized output (opt-on):"
        echo "$clean_on"
        echo "[INFO] normalized output (no-opt):"
        echo "$clean_off"
        test_fail "phase29x_optimization_parity_fixtures_vm: stdout parity mismatch (fixture=$fixture_rel)"
        exit 1
    fi
done < <(load_case_lines "$PARITY_FIXTURES_FILE")

while IFS= read -r line || [ -n "$line" ]; do
    [ -z "$line" ] && continue
    IFS='|' read -r fixture_rel expected_error <<<"$line"
    fixture="$NYASH_ROOT/$fixture_rel"
    if [ ! -f "$fixture" ]; then
        test_fail "phase29x_optimization_parity_fixtures_vm: reject fixture missing: $fixture"
        exit 1
    fi

    set +e
    out_on="$(run_vm_case "$fixture" on)"
    rc_on=$?
    set -e
    if [ "$rc_on" -eq 124 ]; then
        test_fail "phase29x_optimization_parity_fixtures_vm: reject fixture timed out (opt-on): $fixture_rel"
        exit 1
    fi

    set +e
    out_off="$(run_vm_case "$fixture" off)"
    rc_off=$?
    set -e
    if [ "$rc_off" -eq 124 ]; then
        test_fail "phase29x_optimization_parity_fixtures_vm: reject fixture timed out (no-opt): $fixture_rel"
        exit 1
    fi

    if [ "$rc_on" -eq 0 ]; then
        echo "[INFO] reject fixture output (opt-on):"
        echo "$out_on" | tail -n 120 || true
        test_fail "phase29x_optimization_parity_fixtures_vm: reject fixture expected non-zero (opt-on, fixture=$fixture_rel)"
        exit 1
    fi
    if [ "$rc_off" -eq 0 ]; then
        echo "[INFO] reject fixture output (no-opt):"
        echo "$out_off" | tail -n 120 || true
        test_fail "phase29x_optimization_parity_fixtures_vm: reject fixture expected non-zero (no-opt, fixture=$fixture_rel)"
        exit 1
    fi
    if [ "$rc_on" -ne "$rc_off" ]; then
        echo "[INFO] reject fixture output (opt-on):"
        echo "$out_on" | tail -n 120 || true
        echo "[INFO] reject fixture output (no-opt):"
        echo "$out_off" | tail -n 120 || true
        test_fail "phase29x_optimization_parity_fixtures_vm: reject rc parity mismatch (on=$rc_on off=$rc_off, fixture=$fixture_rel)"
        exit 1
    fi

    clean_on="$(normalize_output "$out_on")"
    clean_off="$(normalize_output "$out_off")"

    if ! printf '%s\n' "$clean_on" | rg -qF "$expected_error"; then
        echo "[INFO] normalized reject output (opt-on):"
        echo "$clean_on"
        test_fail "phase29x_optimization_parity_fixtures_vm: reject message missing (opt-on expected='$expected_error', fixture=$fixture_rel)"
        exit 1
    fi
    if ! printf '%s\n' "$clean_off" | rg -qF "$expected_error"; then
        echo "[INFO] normalized reject output (no-opt):"
        echo "$clean_off"
        test_fail "phase29x_optimization_parity_fixtures_vm: reject message missing (no-opt expected='$expected_error', fixture=$fixture_rel)"
        exit 1
    fi
    if [ "$clean_on" != "$clean_off" ]; then
        echo "[INFO] normalized reject output (opt-on):"
        echo "$clean_on"
        echo "[INFO] normalized reject output (no-opt):"
        echo "$clean_off"
        test_fail "phase29x_optimization_parity_fixtures_vm: reject output parity mismatch (fixture=$fixture_rel)"
        exit 1
    fi
done < <(load_case_lines "$REJECT_FIXTURES_FILE")

test_pass "phase29x_optimization_parity_fixtures_vm: PASS (X64 optimization pre/post parity + reject fixture locked)"
