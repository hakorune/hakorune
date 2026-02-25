#!/bin/bash
# phase29ae_regression_pack_vm.sh - JoinIR regression pack entrypoint (VM)

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

dump_adopt_env_on_failure() {
    # NOTE: Keep this list short and stable; these flags are a common source of "wrong hypothesis" failures.
    log_error "env(adopt): NYASH_OPERATOR_BOX_COMPARE_ADOPT=${NYASH_OPERATOR_BOX_COMPARE_ADOPT:-<unset>} NYASH_OPERATOR_BOX_ADD_ADOPT=${NYASH_OPERATOR_BOX_ADD_ADOPT:-<unset>} NYASH_VM_TOLERATE_VOID=${NYASH_VM_TOLERATE_VOID:-<unset>}"
}

run_filter() {
    local label="$1"
    local filter="$2"
    local args=("--profile" "integration" "--filter" "$filter")

    if [ "${PREFLIGHT_DONE:-0}" = "1" ]; then
        args+=("--skip-preflight")
    else
        PREFLIGHT_DONE=1
    fi

    log_info "phase29ae_regression_pack_vm: ${label} (${filter})"
    if ! "$NYASH_ROOT/tools/smokes/v2/run.sh" "${args[@]}"; then
        dump_adopt_env_on_failure
        log_error "phase29ae_regression_pack_vm: ${label} failed"
        return 1
    fi

    return 0
}

run_filter "pattern2" "phase29ab_pattern2_" || exit 1
run_filter "pattern2_realworld" "phase263_pattern2_" || exit 1
run_filter "pattern2_subset" "phase29ai_pattern2_break_plan_subset_ok_min_vm" || exit 1
run_filter "pattern2_release_adopt_vm" "phase29ao_pattern2_release_adopt_vm" || exit 1
run_filter "pattern3_ifphi_vm" "phase118_pattern3_if_sum_vm" || exit 1
run_filter "pattern3_release_adopt_vm" "phase29ao_pattern3_release_adopt_vm" || exit 1
run_filter "pattern4_continue_vm" "phase29ap_pattern4_continue_min_vm" || exit 1
run_filter "pattern1_strict_shadow_vm" "phase29ao_pattern1_strict_shadow_vm" || exit 1
run_filter "pattern1_subset_reject_extra_stmt_vm" "phase29ao_pattern1_subset_reject_extra_stmt_vm" || exit 1
run_filter "pattern1_stringutils_tolower_vm" "phase29ap_stringutils_tolower_vm" || exit 1
run_filter "pattern1_stringutils_join_vm" "phase29ap_stringutils_join_vm" || exit 1
run_filter "stdlib_string_pack_vm" "phase29aq_stdlib_pack_vm" || exit 1
run_filter "purity_gate_vm" "phase29as_purity_gate_vm" || exit 1
run_filter "string_is_integer_strict_shadow_vm" "phase29ar_string_is_integer_min_vm" || exit 1
run_filter "string_is_integer_release_adopt_vm" "phase29ar_string_is_integer_release_adopt_vm" || exit 1
run_filter "generic_loop_continue_strict_shadow_vm" "phase29ca_generic_loop_continue_strict_shadow_vm" || exit 1
run_filter "generic_loop_continue_release_adopt_vm" "phase29ca_generic_loop_continue_release_adopt_vm" || exit 1
run_filter "generic_loop_in_body_step_strict_shadow_vm" "phase29cb_generic_loop_in_body_step_strict_shadow_vm" || exit 1
run_filter "generic_loop_in_body_step_release_adopt_vm" "phase29cb_generic_loop_in_body_step_release_adopt_vm" || exit 1
run_filter "match_return_strict_shadow_vm" "phase29at_match_return_strict_shadow_vm" || exit 1
run_filter "match_return_release_adopt_vm" "phase29at_match_return_release_adopt_vm" || exit 1
run_filter "flowbox_tags_gate_vm" "phase29av_flowbox_tags_gate_vm" || exit 1
run_filter "flowbox_tag_coverage_gate_vm" "phase29aw_flowbox_tag_coverage_gate_vm" || exit 1
run_filter "pattern5_break_vm" "phase286_pattern5_break_vm" || exit 1
run_filter "pattern5_strict_shadow_vm" "phase29ao_pattern5_strict_shadow_vm" || exit 1
run_filter "pattern5_release_adopt_vm" "phase29ao_pattern5_release_adopt_vm" || exit 1
run_filter "pattern6_strict_shadow_vm" "phase29ao_pattern6_strict_shadow_vm" || exit 1
run_filter "pattern6_release_adopt_vm" "phase29ao_pattern6_release_adopt_vm" || exit 1
run_filter "pattern6" "phase29ab_pattern6_" || exit 1
run_filter "pattern6_nested_release_adopt_vm" "phase29ap_pattern6_nested_release_adopt_vm" || exit 1
run_filter "pattern6_nested_strict_shadow_vm" "phase29ap_pattern6_nested_strict_shadow_vm" || exit 1
run_filter "pattern7_strict_shadow_vm" "phase29ao_pattern7_strict_shadow_vm" || exit 1
run_filter "pattern7_release_adopt_vm" "phase29ao_pattern7_release_adopt_vm" || exit 1
run_filter "pattern7" "phase29ab_pattern7_" || exit 1

log_success "phase29ae_regression_pack_vm: all JoinIR regression filters passed"
exit 0
