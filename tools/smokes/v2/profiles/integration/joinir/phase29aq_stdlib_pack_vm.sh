#!/bin/bash
# phase29aq_stdlib_pack_vm.sh - Phase 29aq stdlib subset pack (VM)
# Contract:
# - Do NOT call run.sh inside this script (avoid nested per-test timeout).
# - Runs joinir case scripts in order:
#   - phase29aq_string_index_of_min_vm.sh
#   - phase29aq_string_last_index_of_min_vm.sh
#   - phase29aq_string_index_of_string_min_vm.sh
#   - phase29aq_string_parse_integer_min_vm.sh
#   - phase29aq_string_parse_integer_sign_min_vm.sh
#   - phase29aq_string_parse_integer_ws_min_vm.sh
#   - phase29aq_string_parse_integer_leading_zero_min_vm.sh
#   - phase29aq_string_split_min_vm.sh
#   - phase29aq_string_split_char_min_vm.sh
#   - phase29aq_string_split_string_min_vm.sh
#   - phase29aq_string_trim_start_min_vm.sh
#   - phase29aq_string_trim_end_min_vm.sh
#   - phase29aq_string_contains_min_vm.sh
#   - phase29aq_string_starts_with_min_vm.sh
#   - phase29aq_string_ends_with_min_vm.sh
#   - phase29aq_string_trim_min_vm.sh
#   - phase29aq_string_to_upper_min_vm.sh

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if ! command -v timeout >/dev/null 2>&1; then
    log_error "phase29aq_stdlib_pack_vm: 'timeout' command not found"
    exit 2
fi

run_case_script() {
    local label="$1"
    local script_basename="$2"

    local script_path="$NYASH_ROOT/tools/smokes/v2/profiles/integration/joinir/$script_basename"
    if [ ! -f "$script_path" ]; then
        log_error "phase29aq_stdlib_pack_vm: missing script: $script_path"
        return 1
    fi

    log_info "phase29aq_stdlib_pack_vm: ${label} (${script_basename})"
    if ! timeout "${SMOKES_DEFAULT_TIMEOUT:-120}" bash "$script_path"; then
        log_error "phase29aq_stdlib_pack_vm: ${label} failed"
        return 1
    fi

    return 0
}

run_case_script "string_index_of_vm" "phase29aq_string_index_of_min_vm.sh" || exit 1
run_case_script "string_last_index_of_vm" "phase29aq_string_last_index_of_min_vm.sh" || exit 1
run_case_script "string_index_of_string_vm" "phase29aq_string_index_of_string_min_vm.sh" || exit 1
run_case_script "string_parse_integer_vm" "phase29aq_string_parse_integer_min_vm.sh" || exit 1
run_case_script "string_parse_integer_sign_vm" "phase29aq_string_parse_integer_sign_min_vm.sh" || exit 1
run_case_script "string_parse_integer_ws_vm" "phase29aq_string_parse_integer_ws_min_vm.sh" || exit 1
run_case_script "string_parse_integer_leading_zero_vm" "phase29aq_string_parse_integer_leading_zero_min_vm.sh" || exit 1
run_case_script "string_split_vm" "phase29aq_string_split_min_vm.sh" || exit 1
run_case_script "string_split_char_vm" "phase29aq_string_split_char_min_vm.sh" || exit 1
run_case_script "string_split_string_vm" "phase29aq_string_split_string_min_vm.sh" || exit 1
run_case_script "string_trim_start_vm" "phase29aq_string_trim_start_min_vm.sh" || exit 1
run_case_script "string_trim_end_vm" "phase29aq_string_trim_end_min_vm.sh" || exit 1
run_case_script "string_contains_vm" "phase29aq_string_contains_min_vm.sh" || exit 1
run_case_script "string_starts_with_vm" "phase29aq_string_starts_with_min_vm.sh" || exit 1
run_case_script "string_ends_with_vm" "phase29aq_string_ends_with_min_vm.sh" || exit 1
run_case_script "string_trim_vm" "phase29aq_string_trim_min_vm.sh" || exit 1
run_case_script "string_to_upper_vm" "phase29aq_string_to_upper_min_vm.sh" || exit 1

log_success "phase29aq_stdlib_pack_vm: all stdlib subset filters passed"
exit 0
