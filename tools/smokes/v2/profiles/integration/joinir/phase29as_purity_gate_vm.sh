#!/bin/bash
# phase29as_purity_gate_vm.sh - CorePlan purity gate (strict/dev fallback visibility)

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/output_validator.sh"
require_env || exit 2

RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}
FALLBACK_TAG='[plan/fallback:'
export NYASH_ALLOW_USING_FILE=1

assert_no_fallback() {
    local label="$1"
    local output="$2"

    # Avoid `echo ... | grep -q` under `set -o pipefail` (SIGPIPE false negatives).
    if grep -qF "$FALLBACK_TAG" <<<"$output"; then
        echo "[FAIL] ${label}: Unexpected fallback tag"
        echo "$output" | tail -n 80 || true
        test_fail "${label}: fallback tag present"
        exit 1
    fi
}

assert_flowbox_adopt() {
    local label="$1"
    local output="$2"
    local feature="$3"

    # Avoid `echo ... | grep -q` under `set -o pipefail` (SIGPIPE false negatives).
    if ! grep -qF "[flowbox/adopt box_kind=Loop" <<<"$output" \
        || ! grep -qF "via=shadow" <<<"$output"; then
        echo "[FAIL] ${label}: Missing FlowBox tag (box_kind=Loop via=shadow)"
        echo "$output" | tail -n 80 || true
        test_fail "${label}: flowbox tag missing"
        exit 1
    fi

    if [ -n "$feature" ] && ! grep -qF "features=${feature}" <<<"$output"; then
        echo "[FAIL] ${label}: Missing FlowBox feature ${feature}"
        echo "$output" | tail -n 80 || true
        test_fail "${label}: flowbox feature missing"
        exit 1
    fi
}

run_if_phi_join() {
    # still-live legacy fixture key for the if_phi_join purity gate lane
    local input="$NYASH_ROOT/apps/tests/phase118_pattern3_if_sum_min.hako"
    local expected="12"

    set +e
    local output
    output=$(NYASH_DISABLE_PLUGINS=1 run_joinir_vm_strict "$input")
    local exit_code=$?
    set -e

    if [ "$exit_code" -eq 124 ]; then
        test_fail "phase29as_purity_gate_vm: if_phi_join timed out"
        exit 1
    fi
    if [ "$exit_code" -ne 0 ]; then
        echo "[FAIL] if_phi_join exit code $exit_code"
        echo "$output" | tail -n 80 || true
        test_fail "phase29as_purity_gate_vm: if_phi_join failed"
        exit 1
    fi
    assert_flowbox_adopt "if_phi_join" "$output" ""
    if ! validate_numeric_output 1 "$expected" "$output"; then
        echo "$output" | tail -n 80 || true
        test_fail "phase29as_purity_gate_vm: if_phi_join output mismatch"
        exit 1
    fi
    assert_no_fallback "if_phi_join" "$output"
}

run_scan_with_init() {
    local input="$NYASH_ROOT/apps/tests/phase29ab_pattern6_scan_with_init_ok_min.hako"

    set +e
    local output
    output=$(NYASH_DISABLE_PLUGINS=1 run_joinir_vm_strict "$input")
    local exit_code=$?
    set -e

    if [ "$exit_code" -eq 124 ]; then
        test_fail "phase29as_purity_gate_vm: scan_with_init timed out"
        exit 1
    fi
    if [ "$exit_code" -ne 1 ]; then
        echo "[FAIL] scan_with_init exit code $exit_code"
        echo "$output" | tail -n 80 || true
        test_fail "phase29as_purity_gate_vm: scan_with_init failed"
        exit 1
    fi
    assert_flowbox_adopt "scan_with_init" "$output" "return"
    assert_no_fallback "scan_with_init" "$output"
}

run_is_integer_strict_reject() {
    local input="$NYASH_ROOT/apps/tests/phase29ar_string_is_integer_min.hako"

    set +e
    local output
    output=$(NYASH_DISABLE_PLUGINS=1 run_joinir_vm_strict "$input")
    local exit_code=$?
    set -e

    if [ "$exit_code" -eq 124 ]; then
        test_fail "phase29as_purity_gate_vm: is_integer strict reject timed out"
        exit 1
    fi
    if [ "$exit_code" -ne 1 ]; then
        echo "[FAIL] is_integer strict reject exit code $exit_code"
        echo "$output" | tail -n 80 || true
        test_fail "phase29as_purity_gate_vm: is_integer strict reject failed"
        exit 1
    fi

    if ! grep -qF "[vm-hako/unimplemented]" <<<"$output" \
        || ! grep -qF "newbox(StringUtils)" <<<"$output"; then
        echo "[FAIL] is_integer strict reject: missing fail-fast marker"
        echo "$output" | tail -n 80 || true
        test_fail "phase29as_purity_gate_vm: is_integer strict reject marker missing"
        exit 1
    fi

    if grep -qF "[flowbox/" <<<"$output"; then
        echo "[FAIL] is_integer strict reject: unexpected flowbox tag"
        echo "$output" | tail -n 80 || true
        test_fail "phase29as_purity_gate_vm: is_integer strict reject had flowbox tag"
        exit 1
    fi

    assert_no_fallback "is_integer_strict_reject" "$output"
}

run_if_phi_join
run_scan_with_init
run_is_integer_strict_reject

log_success "phase29as_purity_gate_vm: PASS"
exit 0
