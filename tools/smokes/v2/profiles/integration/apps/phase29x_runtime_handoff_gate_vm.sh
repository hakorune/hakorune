#!/bin/bash
# Phase 29x X52: runtime handoff integration gate
#
# Contract pin:
# - X48-X51 contracts are replayable via one command.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

run_step() {
    local cmd="$1"
    if ! bash "$NYASH_ROOT/$cmd"; then
        test_fail "phase29x_runtime_handoff_gate_vm: step failed: $cmd"
        exit 1
    fi
}

run_step "tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_pin_guard_vm.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_strict_dev_replay_vm.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_newclosure_contract_vm.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_core_cabi_delegation_guard_vm.sh"

test_pass "phase29x_runtime_handoff_gate_vm: PASS (X48-X51 contracts integrated)"
