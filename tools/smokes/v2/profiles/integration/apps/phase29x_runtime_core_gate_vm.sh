#!/bin/bash
# Phase 29x X62: runtime core integrated gate
#
# Contract pin:
# - Replay runtime core hardening contracts in fixed order:
#   X59 ABI -> X60 RC -> X61 observability.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

run_step() {
    local cmd="$1"
    if ! bash "$NYASH_ROOT/$cmd"; then
        test_fail "phase29x_runtime_core_gate_vm: step failed: $cmd"
        exit 1
    fi
}

run_step "tools/checks/phase29x_runtime_core_gate_guard.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_abi_borrowed_owned_conformance_vm.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_rc_phase2_queue_lock_vm.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_observability_drift_guard_vm.sh"

test_pass "phase29x_runtime_core_gate_vm: PASS (X62 runtime core integrated gate locked)"
