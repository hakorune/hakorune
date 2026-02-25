#!/bin/bash
# Phase 29x X38: LLVM-only daily gate
#
# Goal:
# - Make LLVM+C ABI lane the default daily/milestone entry for Phase 29x.
# - Integrate cache lane replay (X42-X45) into the same daily/milestone contract.
# - Keep runtime route/selfhost heavy gates as opt-in compatibility checks.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

run_step() {
    local cmd="$1"
    if ! bash "$NYASH_ROOT/$cmd"; then
        test_fail "phase29x_llvm_only_daily_gate: step failed: $cmd"
        exit 1
    fi
}

run_step "tools/checks/abi_lane_guard.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_llvm_cabi_link_min.sh"
run_step "tools/checks/phase29x_cache_gate_integration_guard.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_cache_lane_gate_vm.sh"

test_pass "phase29x_llvm_only_daily_gate: PASS (llvm-only + cache lane default daily gate)"
