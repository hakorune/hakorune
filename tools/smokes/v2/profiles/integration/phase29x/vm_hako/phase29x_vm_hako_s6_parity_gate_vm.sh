#!/bin/bash
# Phase 29x X56: vm-hako S6 dual-run parity gate pack
#
# Contract pin:
# - X55 vocabulary guard + S5 success/reject parity probes are replayable via one command.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/vm_route_pin.sh"
require_env || exit 2

run_step() {
    local cmd="$1"
    if ! bash "$NYASH_ROOT/$cmd"; then
        test_fail "phase29x_vm_hako_s6_parity_gate_vm: step failed: $cmd"
        exit 1
    fi
}

run_step_with_route_pin() {
    local cmd="$1"
    if ! run_with_vm_route_pin bash "$NYASH_ROOT/$cmd"; then
        test_fail "phase29x_vm_hako_s6_parity_gate_vm: step failed (route pin): $cmd"
        exit 1
    fi
}

run_step "tools/smokes/v2/profiles/integration/phase29x/vm_hako/phase29x_vm_hako_s6_vocab_guard_vm.sh"
run_step_with_route_pin "tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_array_get_parity_vm.sh"
run_step_with_route_pin "tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_array_set_parity_vm.sh"
run_step_with_route_pin "tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_await_non_future_reject_vm.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_newclosure_probe_vm.sh"

test_pass "phase29x_vm_hako_s6_parity_gate_vm: PASS (X55+S5 contracts integrated)"
