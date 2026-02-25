#!/bin/bash
# vm_hako_caps_common.sh - shared helpers for vm-hako capability smokes

set -euo pipefail

vm_hako_caps_timeout_profile() {
    local run_timeout_secs="$1"
    shift
    timeout "$run_timeout_secs" env \
        NYASH_JOINIR_DEV="${VM_HAKO_CAPS_NYASH_JOINIR_DEV:-0}" \
        HAKO_JOINIR_STRICT="${VM_HAKO_CAPS_HAKO_JOINIR_STRICT:-0}" \
        NYASH_USE_NY_COMPILER="${VM_HAKO_CAPS_NYASH_USE_NY_COMPILER:-0}" \
        NYASH_VM_HAKO_PREFER_STRICT_DEV="${VM_HAKO_CAPS_VM_HAKO_PREFER_STRICT_DEV:-0}" \
        NYASH_VM_USE_FALLBACK="${VM_HAKO_CAPS_VM_USE_FALLBACK:-0}" \
        "$@"
}

vm_hako_caps_capture_output() {
    local run_timeout_secs="$1"
    shift

    local output_file
    output_file="$(mktemp /tmp/vm_hako_caps_capture.XXXXXX.log)"
    local exit_code
    set +e
    vm_hako_caps_timeout_profile "$run_timeout_secs" "$@" >"$output_file" 2>&1
    exit_code=$?
    set -e

    VM_HAKO_CAPS_CAPTURE_EXIT_CODE="$exit_code"
    VM_HAKO_CAPS_CAPTURE_OUTPUT="$(cat "$output_file")"
    rm -f "$output_file"
    return 0
}

vm_hako_caps_require_fixture() {
    local smoke_name="$1"
    local input="$2"
    if [ ! -f "$input" ]; then
        test_fail "$smoke_name: fixture missing: $input"
        return 1
    fi
    return 0
}

vm_hako_caps_emit_mir_or_fail() {
    local smoke_name="$1"
    local run_timeout_secs="$2"
    local tmp_mir="$3"
    local input="$4"
    shift 4

    local mir_out
    local mir_rc
    set +e
    # Keep optimizer ON for emit preflight:
    # `--no-optimize` can produce dominance-invalid MIR on APP-1 and trip
    # direct-verify before vm-hako capability checks run.
    mir_out=$(vm_hako_caps_timeout_profile "$run_timeout_secs" \
            env \
            NYASH_MIR_UNIFIED_CALL=0 \
            NYASH_JSON_SCHEMA_V1=0 \
            "$NYASH_BIN" --emit-mir-json "$tmp_mir" "$input" "$@" 2>&1)
    mir_rc=$?
    set -e

    if [ "$mir_rc" -eq 124 ]; then
        test_fail "$smoke_name: emit-mir timed out (>${run_timeout_secs}s)"
        return 1
    fi
    if [ "$mir_rc" -ne 0 ]; then
        echo "$mir_out" | tail -n 80 || true
        test_fail "$smoke_name: emit-mir failed (rc=$mir_rc)"
        return 1
    fi

    VM_HAKO_CAPS_MIR_OUT="$mir_out"
    return 0
}

vm_hako_caps_assert_mir_jq() {
    local smoke_name="$1"
    local tmp_mir="$2"
    local jq_expr="$3"
    local reason="$4"
    if ! jq -e "$jq_expr" "$tmp_mir" >/dev/null 2>&1; then
        test_fail "$smoke_name: $reason"
        return 1
    fi
    return 0
}

vm_hako_caps_run_vm_hako_or_fail_timeout() {
    local smoke_name="$1"
    local run_timeout_secs="$2"
    local input="$3"
    shift 3

    vm_hako_caps_capture_output "$run_timeout_secs" \
        "$NYASH_BIN" --backend vm-hako "$input" "$@"
    local exit_code="$VM_HAKO_CAPS_CAPTURE_EXIT_CODE"
    local output="$VM_HAKO_CAPS_CAPTURE_OUTPUT"

    if [ "$exit_code" -eq 124 ]; then
        test_fail "$smoke_name: timed out (>${run_timeout_secs}s)"
        return 1
    fi

    VM_HAKO_CAPS_OUTPUT="$output"
    VM_HAKO_CAPS_EXIT_CODE="$exit_code"
    return 0
}

vm_hako_caps_run_vm_hako_with_fixture_or_fail_timeout() {
    local smoke_name="$1"
    local run_timeout_secs="$2"
    local input="$3"
    local fixture="$4"
    shift 4

    vm_hako_caps_run_vm_hako_with_fixture \
        "$smoke_name" \
        "$run_timeout_secs" \
        "$input" \
        "$fixture" \
        "$@" || return 1

    if [ "$VM_HAKO_CAPS_EXIT_CODE" -eq 124 ]; then
        test_fail "$smoke_name: timed out (>${run_timeout_secs}s)"
        return 1
    fi
    return 0
}

vm_hako_caps_run_vm_hako_with_fixture() {
    local smoke_name="$1"
    local run_timeout_secs="$2"
    local input="$3"
    local fixture="$4"
    shift 4

    vm_hako_caps_capture_output "$run_timeout_secs" \
        env \
        GATE_LOG_FILE="$fixture" \
        "$NYASH_BIN" --backend vm-hako "$input" "$@"
    local exit_code="$VM_HAKO_CAPS_CAPTURE_EXIT_CODE"
    local output="$VM_HAKO_CAPS_CAPTURE_OUTPUT"

    VM_HAKO_CAPS_OUTPUT="$output"
    VM_HAKO_CAPS_OUTPUT_CLEAN=$(printf '%s\n' "$output" | filter_noise)
    VM_HAKO_CAPS_EXIT_CODE="$exit_code"
    return 0
}

vm_hako_caps_assert_no_unimplemented() {
    local smoke_name="$1"
    local output="$2"
    if echo "$output" | rg -q "^\[vm-hako/unimplemented\]"; then
        echo "$output" | tail -n 80 || true
        test_fail "$smoke_name: unexpected vm-hako unimplemented tag"
        return 1
    fi
    return 0
}

vm_hako_caps_assert_no_contract() {
    local smoke_name="$1"
    local output="$2"
    if echo "$output" | rg -q "^\[vm-hako/contract"; then
        echo "$output" | tail -n 80 || true
        test_fail "$smoke_name: unexpected vm-hako contract tag"
        return 1
    fi
    return 0
}

vm_hako_caps_assert_rc_marker() {
    local smoke_name="$1"
    local output="$2"
    local expected_rc="$3"
    if ! echo "$output" | rg -q "^RC: ${expected_rc}$"; then
        echo "$output" | tail -n 80 || true
        test_fail "$smoke_name: missing RC: ${expected_rc} execution marker"
        return 1
    fi
    return 0
}
