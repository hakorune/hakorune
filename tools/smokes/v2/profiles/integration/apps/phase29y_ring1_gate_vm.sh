#!/bin/bash
# Phase 29y ring1 pre-gate (single-entry replay)
#
# Contract pin:
# - Replay ring1 boundary checks before the full phase29y lane gate.
# - Fixed order: scope guard -> array provider guard/smoke -> map provider guard/smoke -> path provider guard/smoke -> console provider guard/smoke.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/gate_steps.sh"
require_env || exit 2

run_gate_step "phase29y_ring1_gate_vm" "tools/checks/ring1_core_scope_guard.sh"
run_gate_step "phase29y_ring1_gate_vm" "tools/checks/ring1_array_provider_guard.sh"
run_gate_step "phase29y_ring1_gate_vm" "tools/smokes/v2/profiles/integration/apps/ring1_array_provider_vm.sh"
run_gate_step "phase29y_ring1_gate_vm" "tools/checks/ring1_map_provider_guard.sh"
run_gate_step "phase29y_ring1_gate_vm" "tools/smokes/v2/profiles/integration/apps/ring1_map_provider_vm.sh"
run_gate_step "phase29y_ring1_gate_vm" "tools/checks/ring1_path_provider_guard.sh"
run_gate_step "phase29y_ring1_gate_vm" "tools/smokes/v2/profiles/integration/apps/ring1_path_provider_vm.sh"
run_gate_step "phase29y_ring1_gate_vm" "tools/checks/ring1_console_provider_guard.sh"
run_gate_step "phase29y_ring1_gate_vm" "tools/smokes/v2/profiles/integration/apps/ring1_console_provider_vm.sh"

test_pass "phase29y_ring1_gate_vm: PASS (ring1 boundary contracts locked)"
