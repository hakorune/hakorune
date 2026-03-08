#!/bin/bash
# split_scan_regression_pack_vm.sh - split_scan supplemental regression pack (VM)
# current semantic wrapper; canonical entry for split_scan_regression_pack_vm
#
# Contract:
# - strict lane fixtures only
# - near-miss subset exits with RC=1
# - contract fixture is validated via joinir freeze tag helper

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

LEGACY_STEM="${LEGACY_STEM_OVERRIDE:-}"
SEMANTIC_STEM="split_scan_regression_pack_vm"
LABEL_PREFIX="$SEMANTIC_STEM"
if [ -n "$LEGACY_STEM" ]; then
    LABEL_PREFIX="${SEMANTIC_STEM} (legacy stem ${LEGACY_STEM})"
fi

if ! command -v timeout >/dev/null 2>&1; then
    log_error "${LABEL_PREFIX}: 'timeout' command not found"
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
        test_fail "${LABEL_PREFIX}: ${label} timed out"
        exit 1
    fi

    if [ "$exit_code" -ne 1 ]; then
        echo "[FAIL] ${label}: expected exit 1, got $exit_code"
        echo "$output" | tail -n 60 || true
        test_fail "${LABEL_PREFIX}: ${label} unexpected rc"
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
        test_fail "${LABEL_PREFIX}: ${label} timed out"
        exit 1
    fi

    if ! expect_joinir_contract_freeze "${LABEL_PREFIX}:${label}" "$output" "$exit_code" "[joinir/phase29ab/split_scan/contract]"; then
        echo "$output" | tail -n 80 || true
        test_fail "${LABEL_PREFIX}: ${label} contract mismatch"
        exit 1
    fi

    return 0
}

run_expect_rc1 \
  "split_scan_nearmiss_ok" \
  "$NYASH_ROOT/apps/tests/phase29ab_pattern7_splitscan_nearmiss_ok_min.hako"

run_contract_freeze_case \
  "split_scan_contract" \
  "$NYASH_ROOT/apps/tests/phase29ab_pattern7_splitscan_contract_min.hako"

log_success "${LABEL_PREFIX}: PASS"
exit 0
