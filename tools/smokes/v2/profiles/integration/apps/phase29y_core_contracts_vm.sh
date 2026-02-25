#!/bin/bash
# Phase 29y core contracts gate (single-entry replay)
#
# Contract pin:
# - Replay phase29y core contracts in fixed order:
#   mirbuilder delegate-forbidden -> ABI -> RC insertion entry -> observability summary.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/gate_steps.sh"
require_env || exit 2

run_gate_step "phase29y_core_contracts_vm" "tools/checks/phase29y_core_contracts_guard.sh"
run_gate_step "phase29y_core_contracts_vm" "tools/smokes/v2/profiles/integration/apps/phase29y_mirbuilder_delegate_forbidden_vm.sh"
run_gate_step "phase29y_core_contracts_vm" "tools/smokes/v2/profiles/integration/apps/phase29y_handle_abi_borrowed_owned_vm.sh"
run_gate_step "phase29y_core_contracts_vm" "tools/smokes/v2/profiles/integration/apps/phase29y_rc_insertion_entry_vm.sh"
run_gate_step "phase29y_core_contracts_vm" "tools/smokes/v2/profiles/integration/apps/phase29y_observability_summary_vm.sh"

test_pass "phase29y_core_contracts_vm: PASS (phase29y core contracts locked)"
