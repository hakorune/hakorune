#!/bin/bash
# phase29y vm-hako capability gate (app-first runtime lane)
#
# Contract pin:
# - Replay vm-hako capability smokes in fixed order.
# - Mix of ported contracts and blocked pins must stay deterministic.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/gate_steps.sh"
require_env || exit 2

run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/vm_hako_caps/file/file_error_vm.sh"
run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/vm_hako_caps/select_emit/select_emit_block_vm.sh"
run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/vm_hako_caps/tls/tls_last_error_ported_vm.sh"
run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/vm_hako_caps/atomic/atomic_fence_ported_vm.sh"
run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/vm_hako_caps/compare/compare_ported_vm.sh"
run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/vm_hako_caps/misc/const_void_ported_vm.sh"
run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/vm_hako_caps/compare/compare_ge_ported_vm.sh"
run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/vm_hako_caps/args/boxcall_args_gt1_ported_vm.sh"
run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/vm_hako_caps/open_handle_phi/open_handle_phi_ported_vm.sh"
run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/vm_hako_caps/app1/app1_stack_overflow_after_open_ported_vm.sh"
run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/vm_hako_caps/app1/app1_summary_contract_ported_vm.sh"

test_pass "phase29y_vm_hako_caps_gate_vm: PASS (vm-hako capability matrix locked)"
