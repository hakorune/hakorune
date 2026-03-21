#!/bin/bash
# Phase 29x X36: de-rust done matrix replay gate
#
# Contract pin:
# - Replay X32/X33/X34/X35 evidence scripts in fixed order.
# - Keep de-rust done judgement independent from lane quick/full gates.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/gate_steps.sh"
require_env || exit 2

run_gate_step "phase29x_derust_done_matrix_vm" "tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_derust_route_dualrun_vm.sh"
run_gate_step "phase29x_derust_done_matrix_vm" "tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_derust_verifier_vm.sh"
run_gate_step "phase29x_derust_done_matrix_vm" "tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_derust_safety_vm.sh"
run_gate_step "phase29x_derust_done_matrix_vm" "tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_derust_strict_default_route_vm.sh"

test_pass "phase29x_derust_done_matrix_vm: PASS (X32/X33/X34/X35 replay locked)"
