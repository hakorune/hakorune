#!/bin/bash
# phase29ae_pattern6_scan_with_init_pack_vm.sh - scan_with_init supplemental regression pack (VM)
#
# Contract:
# - strict lane fixtures only
# - ok subsets exit with RC=1
# - contract fixtures are validated via joinir freeze tag helper

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if ! command -v timeout >/dev/null 2>&1; then
    log_error "phase29ae_pattern6_scan_with_init_pack_vm: 'timeout' command not found"
    exit 2
fi

RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

run_expect_rc1() {
    local label="$1"
    local fixture="$2"

    set +e
    local output
    output=$(NYASH_DISABLE_PLUGINS=1 run_joinir_vm_strict "$fixture")
    local exit_code=$?
    set -e

    if [ "$exit_code" -eq 124 ]; then
        test_fail "phase29ae_pattern6_scan_with_init_pack_vm: ${label} timed out"
        exit 1
    fi

    if [ "$exit_code" -ne 1 ]; then
        echo "[FAIL] ${label}: expected exit 1, got $exit_code"
        echo "$output" | tail -n 60 || true
        test_fail "phase29ae_pattern6_scan_with_init_pack_vm: ${label} unexpected rc"
        exit 1
    fi

    return 0
}

run_contract_freeze_case() {
    local label="$1"
    local fixture="$2"

    set +e
    local output
    output=$(NYASH_DISABLE_PLUGINS=1 run_joinir_vm_strict "$fixture")
    local exit_code=$?
    set -e

    if [ "$exit_code" -eq 124 ]; then
        test_fail "phase29ae_pattern6_scan_with_init_pack_vm: ${label} timed out"
        exit 1
    fi

    if ! expect_joinir_contract_freeze "phase29ae_pattern6_scan_with_init_pack_vm:${label}" "$output" "$exit_code" "[joinir/phase29ab/scan_with_init/contract]"; then
        echo "$output" | tail -n 80 || true
        test_fail "phase29ae_pattern6_scan_with_init_pack_vm: ${label} contract mismatch"
        exit 1
    fi

    return 0
}

run_expect_rc1 \
  "scan_with_init_ok" \
  "$NYASH_ROOT/apps/tests/phase29ab_pattern6_scan_with_init_ok_min.hako"

run_expect_rc1 \
  "matchscan_ok" \
  "$NYASH_ROOT/apps/tests/phase29ab_pattern6_matchscan_ok_min.hako"

run_expect_rc1 \
  "reverse_ok" \
  "$NYASH_ROOT/apps/tests/phase29ab_pattern6_reverse_ok_min.hako"

run_contract_freeze_case \
  "scan_with_init_contract" \
  "$NYASH_ROOT/apps/tests/phase29ab_pattern6_scan_with_init_contract_min.hako"

run_contract_freeze_case \
  "matchscan_contract" \
  "$NYASH_ROOT/apps/tests/phase29ab_pattern6_matchscan_contract_min.hako"

run_contract_freeze_case \
  "reverse_contract" \
  "$NYASH_ROOT/apps/tests/phase29ab_pattern6_reverse_contract_min.hako"

log_success "phase29ae_pattern6_scan_with_init_pack_vm: PASS"
exit 0
