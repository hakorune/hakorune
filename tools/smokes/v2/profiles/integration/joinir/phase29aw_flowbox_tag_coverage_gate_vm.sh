#!/bin/bash
# phase29aw_flowbox_tag_coverage_gate_vm.sh - FlowBox tag coverage gate (strict/non-strict)

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/output_validator.sh"
require_env || exit 2

RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}
FLOWBOX_PREFIX='[flowbox/'
export NYASH_ALLOW_USING_FILE=1

assert_flowbox_adopt_tag() {
    local label="$1"
    local output="$2"
    local kind="$3"
    local required_feature="$4"
    local via="$5"
    local pattern

    if [ -z "$required_feature" ]; then
        pattern="\\[flowbox/adopt box_kind=${kind} features=[^]]* via=${via}\\]"
    else
        pattern="\\[flowbox/adopt box_kind=${kind} features=[^]]*${required_feature}[^]]* via=${via}\\]"
    fi

    if ! grep -Eq "$pattern" <<<"$output"; then
        echo "[FAIL] ${label}: Missing flowbox adopt tag (box_kind=${kind} feature=${required_feature:-none} via=${via})"
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

run_scan_with_init_strict() {
    local input="$NYASH_ROOT/apps/tests/phase29ab_pattern6_scan_with_init_ok_min.hako"

    set +e
    local output
    output=$(timeout "$RUN_TIMEOUT_SECS" env \
      NYASH_DISABLE_PLUGINS=1 \
      HAKO_JOINIR_STRICT=1 \
      "$NYASH_BIN" --backend vm "$input" 2>&1)
    local exit_code=$?
    set -e

    if [ "$exit_code" -eq 124 ]; then
        test_fail "phase29aw_flowbox_tag_coverage_gate_vm: scan_with_init strict timed out"
        exit 1
    fi
    if [ "$exit_code" -ne 1 ]; then
        echo "[FAIL] scan_with_init strict exit code $exit_code"
        echo "$output" | tail -n 80 || true
        test_fail "phase29aw_flowbox_tag_coverage_gate_vm: scan_with_init strict failed"
        exit 1
    fi
    assert_flowbox_adopt_tag "scan_with_init_strict" "$output" "Loop" "return" "shadow"
}

run_scan_with_init_release() {
    local input="$NYASH_ROOT/apps/tests/phase29ab_pattern6_scan_with_init_ok_min.hako"

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
        test_fail "phase29aw_flowbox_tag_coverage_gate_vm: scan_with_init release timed out"
        exit 1
    fi
    if [ "$exit_code" -ne 1 ]; then
        echo "[FAIL] scan_with_init release exit code $exit_code"
        echo "$output" | tail -n 80 || true
        test_fail "phase29aw_flowbox_tag_coverage_gate_vm: scan_with_init release failed"
        exit 1
    fi
    assert_no_flowbox_tags "scan_with_init_release" "$output"
}

run_split_scan_strict() {
    local input="$NYASH_ROOT/apps/tests/phase29ab_pattern7_splitscan_ok_min.hako"

    set +e
    local output
    output=$(timeout "$RUN_TIMEOUT_SECS" env \
      NYASH_DISABLE_PLUGINS=1 \
      HAKO_JOINIR_STRICT=1 \
      "$NYASH_BIN" --backend vm "$input" 2>&1)
    local exit_code=$?
    set -e

    if [ "$exit_code" -eq 124 ]; then
        test_fail "phase29aw_flowbox_tag_coverage_gate_vm: split_scan strict timed out"
        exit 1
    fi
    if [ "$exit_code" -ne 1 ]; then
        echo "[FAIL] split_scan strict exit code $exit_code"
        echo "$output" | tail -n 80 || true
        test_fail "phase29aw_flowbox_tag_coverage_gate_vm: split_scan strict failed"
        exit 1
    fi
    assert_flowbox_adopt_tag "split_scan_strict" "$output" "Loop" "" "shadow"
}

run_split_scan_release() {
    local input="$NYASH_ROOT/apps/tests/phase29ab_pattern7_splitscan_ok_min.hako"

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
        test_fail "phase29aw_flowbox_tag_coverage_gate_vm: split_scan release timed out"
        exit 1
    fi
    if [ "$exit_code" -ne 3 ]; then
        echo "[FAIL] split_scan release exit code $exit_code"
        echo "$output" | tail -n 80 || true
        test_fail "phase29aw_flowbox_tag_coverage_gate_vm: split_scan release failed"
        exit 1
    fi
    assert_no_flowbox_tags "split_scan_release" "$output"
}

run_is_integer_strict() {
    local input="$NYASH_ROOT/apps/tests/phase29ar_string_is_integer_min.hako"

    set +e
    local output
    output=$(timeout "$RUN_TIMEOUT_SECS" env \
      NYASH_DISABLE_PLUGINS=1 \
      HAKO_JOINIR_STRICT=1 \
      "$NYASH_BIN" --backend vm "$input" 2>&1)
    local exit_code=$?
    set -e

    if [ "$exit_code" -eq 124 ]; then
        test_fail "phase29aw_flowbox_tag_coverage_gate_vm: is_integer strict timed out"
        exit 1
    fi
    if [ "$exit_code" -ne 1 ]; then
        echo "[FAIL] is_integer strict exit code $exit_code"
        echo "$output" | tail -n 80 || true
        test_fail "phase29aw_flowbox_tag_coverage_gate_vm: is_integer strict failed"
        exit 1
    fi
    if ! grep -qF "[vm-hako/unimplemented]" <<<"$output" \
        || ! grep -qF "newbox(StringUtils)" <<<"$output"; then
        echo "[FAIL] is_integer strict: missing fail-fast marker"
        echo "$output" | tail -n 80 || true
        test_fail "phase29aw_flowbox_tag_coverage_gate_vm: is_integer strict marker missing"
        exit 1
    fi
    assert_no_flowbox_tags "is_integer_strict_reject" "$output"
}

run_is_integer_release() {
    local input="$NYASH_ROOT/apps/tests/phase29ar_string_is_integer_min.hako"

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
        test_fail "phase29aw_flowbox_tag_coverage_gate_vm: is_integer release timed out"
        exit 1
    fi
    if [ "$exit_code" -ne 0 ]; then
        echo "[FAIL] is_integer release exit code $exit_code"
        echo "$output" | tail -n 80 || true
        test_fail "phase29aw_flowbox_tag_coverage_gate_vm: is_integer release failed"
        exit 1
    fi
    assert_no_flowbox_tags "is_integer_release" "$output"

    local output_clean
    output_clean=$(echo "$output" | filter_noise | grep -v '^\[plugins\]' | grep -v '^\[WARN\] \[plugin/init\]')
    local expected
    expected=$(cat << 'TXT'
1
0
TXT
)

    compare_outputs "$expected" "$output_clean" "phase29aw_flowbox_tag_coverage_gate_vm" || exit 1
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
        test_fail "phase29aw_flowbox_tag_coverage_gate_vm: match_return strict timed out"
        exit 1
    fi
    if [ "$exit_code" -ne 20 ]; then
        echo "[FAIL] match_return strict exit code $exit_code"
        echo "$output" | tail -n 80 || true
        test_fail "phase29aw_flowbox_tag_coverage_gate_vm: match_return strict failed"
        exit 1
    fi
    assert_flowbox_adopt_tag "match_return_strict" "$output" "Seq" "return" "shadow"
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
        test_fail "phase29aw_flowbox_tag_coverage_gate_vm: match_return release timed out"
        exit 1
    fi
    if [ "$exit_code" -ne 20 ]; then
        echo "[FAIL] match_return release exit code $exit_code"
        echo "$output" | tail -n 80 || true
        test_fail "phase29aw_flowbox_tag_coverage_gate_vm: match_return release failed"
        exit 1
    fi
    assert_no_flowbox_tags "match_return_release" "$output"
}

run_scan_with_init_strict
run_split_scan_strict
run_is_integer_strict
run_match_return_strict

run_scan_with_init_release
run_split_scan_release
run_is_integer_release
run_match_return_release

log_success "phase29aw_flowbox_tag_coverage_gate_vm: PASS"
exit 0
