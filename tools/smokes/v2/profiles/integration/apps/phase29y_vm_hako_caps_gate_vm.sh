#!/bin/bash
# phase29y vm-hako capability gate (app-first runtime lane)
#
# Contract pin:
# - Replay vm-hako capability smokes in fixed order.
# - Mix of ported contracts and blocked pins must stay deterministic.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/gate_steps.sh"
require_env || exit 2

run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/apps/vm_hako_caps_filebox_newbox_vm.sh"
run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/apps/vm_hako_caps_args_vm.sh"
run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/apps/vm_hako_caps_file_error_vm.sh"
run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/apps/vm_hako_caps_select_emit_block_vm.sh"
run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/apps/vm_hako_caps_env_get_ported_vm.sh"
run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/apps/vm_hako_caps_compare_ported_vm.sh"
run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/apps/vm_hako_caps_file_read_ported_vm.sh"
run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/apps/vm_hako_caps_file_close_ported_vm.sh"
run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/apps/vm_hako_caps_const_void_ported_vm.sh"
run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/apps/vm_hako_caps_compare_ge_ported_vm.sh"
run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/apps/vm_hako_caps_boxcall_args_gt1_ported_vm.sh"
run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/apps/vm_hako_caps_open_handle_phi_ported_vm.sh"
run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/apps/vm_hako_caps_app1_stack_overflow_after_open_ported_vm.sh"
run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/apps/vm_hako_caps_app1_summary_contract_ported_vm.sh"

test_pass "phase29y_vm_hako_caps_gate_vm: PASS (vm-hako capability matrix locked)"
