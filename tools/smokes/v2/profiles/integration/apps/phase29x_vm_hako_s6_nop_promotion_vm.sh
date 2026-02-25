#!/bin/bash
# Phase 29x X58: vm-hako S6 first vocabulary promotion gate (nop)
#
# Contract pin:
# - Promote exactly one vocabulary (`nop`) in S6 allowlist/subset-check.
# - Replay X56 parity precondition before running nop parity fixture.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/vm_route_pin.sh"
require_env || exit 2

run_step() {
    local cmd="$1"
    if ! bash "$NYASH_ROOT/$cmd"; then
        test_fail "phase29x_vm_hako_s6_nop_promotion_vm: step failed: $cmd"
        exit 1
    fi
}

run_step_with_route_pin() {
    local cmd="$1"
    if ! run_with_vm_route_pin bash "$NYASH_ROOT/$cmd"; then
        test_fail "phase29x_vm_hako_s6_nop_promotion_vm: step failed (route pin): $cmd"
        exit 1
    fi
}

run_step "tools/checks/phase29x_vm_hako_s6_nop_promotion_guard.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_s6_parity_gate_vm.sh"
run_step_with_route_pin "tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s3_nop_parity_vm.sh"

test_pass "phase29x_vm_hako_s6_nop_promotion_vm: PASS (X58 nop promotion locked)"
