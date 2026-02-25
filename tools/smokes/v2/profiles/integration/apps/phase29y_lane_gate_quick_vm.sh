#!/bin/bash
# Phase 29y lane gate (quick profile)
#
# Contract pin:
# - Keep day-to-day lane safety checks fast.
# - Exclude optional GC alignment chain (full profile only).

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/gate_steps.sh"
require_env || exit 2

run_gate_step "phase29y_lane_gate_quick_vm" "tools/checks/phase29y_lane_gate_guard.sh"
run_gate_step "phase29y_lane_gate_quick_vm" "tools/checks/phase29y_direct_v0_retirement_guard.sh"
run_gate_step "phase29y_lane_gate_quick_vm" "tools/smokes/v2/profiles/integration/apps/phase29y_ring1_gate_vm.sh"
run_gate_step "phase29y_lane_gate_quick_vm" "tools/smokes/v2/profiles/integration/apps/phase29y_hako_using_resolver_parity_vm.sh"
run_gate_step "phase29y_lane_gate_quick_vm" "tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_ported_vm.sh"
run_gate_step "phase29y_lane_gate_quick_vm" "tools/smokes/v2/profiles/integration/apps/mir_shape_guard_vm.sh"
run_gate_step "phase29y_lane_gate_quick_vm" "tools/smokes/v2/profiles/integration/apps/phase29y_direct_v0_bridge_guard_vm.sh"
run_gate_step "phase29y_lane_gate_quick_vm" "tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh"
run_gate_step "phase29y_lane_gate_quick_vm" "tools/smokes/v2/profiles/integration/apps/phase29y_core_contracts_vm.sh"

if [ "${PHASE29Y_DERUST_DONE_MATRIX_CHECK:-0}" = "1" ]; then
    run_gate_step "phase29y_lane_gate_quick_vm" "tools/smokes/v2/profiles/integration/apps/phase29x_derust_done_matrix_vm.sh"
fi

test_pass "phase29y_lane_gate_quick_vm: PASS (phase29y quick contracts locked)"
