#!/bin/bash
# phase29av_flowbox_tags_gate_vm.sh - FlowBox schema tags gate (strict/non-strict)

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/output_validator.sh"
require_env || exit 2

RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}
FLOWBOX_PREFIX='[flowbox/'

assert_has_adopt_tag() {
    local label="$1"
    local output="$2"
    local kind="$3"

    if ! grep -qF "[flowbox/adopt box_kind=${kind}" <<<"$output"; then
        echo "[FAIL] ${label}: Missing flowbox adopt tag (${kind})"
        echo "$output" | tail -n 80 || true
        test_fail "${label}: flowbox adopt tag missing"
        exit 1
    fi
}

assert_no_flowbox_tags() {
    local label="$1"
    local output="$2"

    if grep -qF "$FLOWBOX_PREFIX" <<<"$output"; then
        echo "[FAIL] ${label}: Unexpected flowbox tag"
        echo "$output" | tail -n 80 || true
        test_fail "${label}: flowbox tag present"
        exit 1
    fi
}

run_if_phi_join_strict() {
    local input="$NYASH_ROOT/apps/tests/phase118_pattern3_if_sum_min.hako"
    local expected="12"

    set +e
    local output
    output=$(timeout "$RUN_TIMEOUT_SECS" env \
      NYASH_DISABLE_PLUGINS=1 \
      HAKO_JOINIR_STRICT=1 \
      "$NYASH_BIN" --backend vm "$input" 2>&1)
    local exit_code=$?
    set -e

    if [ "$exit_code" -eq 124 ]; then
        test_fail "phase29av_flowbox_tags_gate_vm: if_phi_join timed out"
        exit 1
    fi
    if [ "$exit_code" -ne 0 ]; then
        echo "[FAIL] if_phi_join exit code $exit_code"
        echo "$output" | tail -n 80 || true
        test_fail "phase29av_flowbox_tags_gate_vm: if_phi_join failed"
        exit 1
    fi
    assert_has_adopt_tag "if_phi_join" "$output" "Loop"
    if ! validate_numeric_output 1 "$expected" "$output"; then
        echo "$output" | tail -n 80 || true
        test_fail "phase29av_flowbox_tags_gate_vm: if_phi_join output mismatch"
        exit 1
    fi
}

run_match_return_strict() {
    local input="$NYASH_ROOT/apps/tests/phase29at_match_return_min.hako"

    set +e
    local output
    output=$(timeout "$RUN_TIMEOUT_SECS" env \
      NYASH_DISABLE_PLUGINS=1 \
      HAKO_JOINIR_STRICT=1 \
      "$NYASH_BIN" --backend vm "$input" 2>&1)
    local exit_code=$?
    set -e

    if [ "$exit_code" -eq 124 ]; then
        test_fail "phase29av_flowbox_tags_gate_vm: match_return strict timed out"
        exit 1
    fi
    if [ "$exit_code" -ne 20 ]; then
        echo "[FAIL] match_return strict exit code $exit_code"
        echo "$output" | tail -n 80 || true
        test_fail "phase29av_flowbox_tags_gate_vm: match_return strict failed"
        exit 1
    fi
    assert_has_adopt_tag "match_return_strict" "$output" "Seq"
}

run_match_return_release() {
    local input="$NYASH_ROOT/apps/tests/phase29at_match_return_min.hako"

    set +e
    local output
    output=$(timeout "$RUN_TIMEOUT_SECS" env \
      -u HAKO_JOINIR_STRICT \
      -u NYASH_JOINIR_STRICT \
      -u HAKO_JOINIR_DEBUG \
      -u NYASH_JOINIR_DEBUG \
      -u HAKO_JOINIR_DEV \
      -u NYASH_JOINIR_DEV \
      NYASH_DISABLE_PLUGINS=1 \
      "$NYASH_BIN" --backend vm "$input" 2>&1)
    local exit_code=$?
    set -e

    if [ "$exit_code" -eq 124 ]; then
        test_fail "phase29av_flowbox_tags_gate_vm: match_return release timed out"
        exit 1
    fi
    if [ "$exit_code" -ne 20 ]; then
        echo "[FAIL] match_return release exit code $exit_code"
        echo "$output" | tail -n 80 || true
        test_fail "phase29av_flowbox_tags_gate_vm: match_return release failed"
        exit 1
    fi
    assert_no_flowbox_tags "match_return_release" "$output"
}

run_if_phi_join_strict
run_match_return_strict
run_match_return_release

log_success "phase29av_flowbox_tags_gate_vm: PASS"
exit 0
