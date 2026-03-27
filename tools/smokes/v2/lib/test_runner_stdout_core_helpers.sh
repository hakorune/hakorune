#!/bin/bash
# test_runner_stdout_core_helpers.sh - split from test_runner.sh
run_verify_canary_and_expect_rc() {
    local runner_fn="$1"
    local prog_json_path="$2"
    local expected_rc="$3"
    local fail_label="$4"
    local pass_label="$5"
    local runner_arg="${6:-0}"

    set +e
    "$runner_fn" "$prog_json_path" "$runner_arg"
    local rc=$?
    set -e

    if [ "$rc" -ne "$expected_rc" ]; then
        echo "[FAIL] ${fail_label} rc=$rc (expected $expected_rc)" >&2
        return 1
    fi

    echo "[PASS] ${pass_label}"
    return 0
}

apply_verify_mir_route_env() {
    local primary_hakovm="${1:-0}"
    local size_state="${2:-0}"
    local size_state_per_recv="${3:-0}"
    local dispatcher_flow="${4:-0}"
    local abi_adapter="${5:-0}"
    local value_state="${6:-0}"

    if [ "$primary_hakovm" = "1" ]; then
        export HAKO_VERIFY_PRIMARY=hakovm
    fi
    if [ "$size_state" = "1" ]; then
        export HAKO_VM_MIRCALL_SIZESTATE=1
    fi
    if [ "$size_state_per_recv" = "1" ]; then
        export HAKO_VM_MIRCALL_SIZESTATE_PER_RECV=1
    else
        export HAKO_VM_MIRCALL_SIZESTATE_PER_RECV=0
    fi
    if [ "$dispatcher_flow" = "1" ]; then
        export HAKO_V1_DISPATCHER_FLOW=1
    fi
    if [ "$abi_adapter" = "1" ]; then
        export HAKO_ABI_ADAPTER="${HAKO_ABI_ADAPTER:-1}"
    fi
    if [ "$value_state" = "1" ]; then
        export HAKO_VM_MIRCALL_VALUESTATE=1
    fi
}

run_verify_mir_rc_with_env() {
    local json_path="$1"
    local primary_hakovm="${2:-0}"
    local size_state="${3:-0}"
    local size_state_per_recv="${4:-0}"
    local dispatcher_flow="${5:-0}"
    local abi_adapter="${6:-0}"
    local value_state="${7:-0}"

    (
        apply_verify_mir_route_env \
            "$primary_hakovm" \
            "$size_state" \
            "$size_state_per_recv" \
            "$dispatcher_flow" \
            "$abi_adapter" \
            "$value_state"
        verify_mir_rc "$json_path" >/dev/null 2>&1
    )
}

run_verify_mir_via_hakovm_size_state_per_recv() {
    local json_path="$1"
    local _unused="${2:-0}"

    run_verify_mir_rc_with_env "$json_path" 1 1 1 0 0 0
}

run_verify_mir_via_hakovm_size_state_global() {
    local json_path="$1"
    local _unused="${2:-0}"

    run_verify_mir_rc_with_env "$json_path" 1 1 0 0 0 0
}

run_verify_mir_via_hakovm_size_state_flow() {
    local json_path="$1"
    local _unused="${2:-0}"

    run_verify_mir_rc_with_env "$json_path" 1 1 1 1 0 0
}

run_verify_mir_via_hakovm_map_size_state() {
    local json_path="$1"
    local _unused="${2:-0}"

    run_verify_mir_rc_with_env "$json_path" 1 1 1 0 1 0
}

run_verify_mir_via_hakovm_map_value_state() {
    local json_path="$1"
    local _unused="${2:-0}"

    run_verify_mir_rc_with_env "$json_path" 1 1 1 0 1 1
}

run_verify_mir_canary_and_expect_rc() {
    local runner_fn="$1"
    local json_path="$2"
    local expected_rc="$3"
    local fail_label="$4"
    local pass_label="$5"
    local runner_arg="${6:-0}"

    set +e
    "$runner_fn" "$json_path" "$runner_arg"
    local rc=$?
    set -e

    if [ "$rc" -ne "$expected_rc" ]; then
        echo "[FAIL] ${fail_label} (rc=$rc, want=$expected_rc)" >&2
        return 1
    fi

    echo "[PASS] ${pass_label}"
    return 0
}

run_verify_mir_rc_and_expect() {
    local json_path="$1"
    local expected_rc="$2"
    local fail_label="$3"
    local pass_label="$4"

    set +e
    verify_mir_rc "$json_path" >/dev/null 2>&1
    local rc=$?
    set -e

    if [ "$rc" -ne "$expected_rc" ]; then
        echo "[FAIL] ${fail_label} (rc=$rc, want=$expected_rc)" >&2
        return 1
    fi

    echo "[PASS] ${pass_label}"
    return 0
}

run_hako_primary_no_fallback_canary_and_expect_rc() {
    local prog_json_path="$1"
    local expected_rc="$2"
    local fail_label="$3"
    local pass_label="$4"
    local prefer_mirbuilder="${5:-0}"

    run_verify_canary_and_expect_rc \
        run_verify_program_via_hako_primary_no_fallback_to_core \
        "$prog_json_path" \
        "$expected_rc" \
        "$fail_label" \
        "$pass_label" \
        "$prefer_mirbuilder"
}

run_preferred_mirbuilder_canary_and_expect_rc() {
    local prog_json_path="$1"
    local expected_rc="$2"
    local fail_label="$3"
    local pass_label="$4"
    local builder_only="${5:-0}"

    run_verify_canary_and_expect_rc \
        run_verify_program_via_preferred_mirbuilder_to_core \
        "$prog_json_path" \
        "$expected_rc" \
        "$fail_label" \
        "$pass_label" \
        "$builder_only"
}

capture_runner_stdout_to_file() {
    local runner_fn="$1"
    local builder_module="$2"
    local prog_json="$3"
    local runner_arg3="$4"
    local runner_arg4="$5"
    local tmp_stdout="$6"
    local rc=0

    set +e
    "$runner_fn" "$builder_module" "$prog_json" "$runner_arg3" "$runner_arg4" 2>/dev/null | tee "$tmp_stdout" >/dev/null
    rc=${PIPESTATUS[0]}
    set -e
    return "$rc"
}

select_registry_builder_module_runner() {
    local use_preinclude="${1:-0}"

    if [ "$use_preinclude" = "1" ]; then
        printf '%s' "run_program_json_via_registry_builder_module_vm_with_preinclude"
        return 0
    fi

    printf '%s' "run_program_json_via_registry_builder_module_vm"
}

run_builder_module_vm_to_stdout_file() {
    local builder_module="$1"
    local prog_json="$2"
    local tmp_stdout="$3"

    capture_runner_stdout_to_file \
        run_program_json_via_builder_module_vm \
        "$builder_module" \
        "$prog_json" \
        "" \
        "" \
        "$tmp_stdout"
    return $?
}

run_builder_module_tag_to_stdout_file() {
    local builder_module="$1"
    local prog_json="$2"
    local _registry_only="${3:-}"
    local _use_preinclude="${4:-0}"
    local tmp_stdout="$5"

    run_builder_module_vm_to_stdout_file "$builder_module" "$prog_json" "$tmp_stdout"
}

run_registry_builder_module_vm_to_stdout_file() {
    local builder_module="$1"
    local prog_json="$2"
    local registry_only="$3"
    local use_preinclude="${4:-0}"
    local tmp_stdout="$5"
    local runner_fn=""

    runner_fn="$(select_registry_builder_module_runner "$use_preinclude")"
    capture_runner_stdout_to_file \
        "$runner_fn" \
        "$builder_module" \
        "$prog_json" \
        "$registry_only" \
        "" \
        "$tmp_stdout"
    return $?
}

run_registry_builder_tag_to_stdout_file() {
    local builder_module="$1"
    local prog_json="$2"
    local registry_only="$3"
    local use_preinclude="${4:-0}"
    local tmp_stdout="$5"

    run_registry_builder_module_vm_to_stdout_file \
        "$builder_module" \
        "$prog_json" \
        "$registry_only" \
        "$use_preinclude" \
        "$tmp_stdout"
}
