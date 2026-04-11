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

run_gate_step "phase29y_vm_hako_caps_gate_vm" "tools/smokes/v2/profiles/integration/vm_hako_caps/select_emit/select_emit_block_vm.sh"

test_pass "phase29y_vm_hako_caps_gate_vm: PASS (vm-hako capability matrix locked)"
