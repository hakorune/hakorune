#!/bin/bash
# Phase 29x X57: NewClosure runtime lane decision refresh gate
#
# Contract pin:
# - Runtime boundary stays fail-fast for new_closure at X57.
# - X56 parity gate stays green before NewClosure decision is confirmed.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

run_step() {
    local cmd="$1"
    if ! bash "$NYASH_ROOT/$cmd"; then
        test_fail "phase29x_vm_hako_newclosure_decision_refresh_vm: step failed: $cmd"
        exit 1
    fi
}

run_step "tools/checks/phase29x_vm_hako_newclosure_decision_guard.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_s6_parity_gate_vm.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_newclosure_contract_vm.sh"

test_pass "phase29x_vm_hako_newclosure_decision_refresh_vm: PASS (X57 NewClosure runtime decision locked)"
