#!/bin/bash
# Phase 29y lane gate (full profile)
#
# Contract pin:
# - Replay phase29y contracts in fixed order.
# - Full profile = quick profile + optional GC lane entry chain.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/gate_steps.sh"
require_env || exit 2

run_gate_step "phase29y_lane_gate_vm" "tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_quick_vm.sh"
run_gate_step "phase29y_lane_gate_vm" "tools/smokes/v2/profiles/integration/apps/phase29y_optional_gc_lane_entry_vm.sh"

test_pass "phase29y_lane_gate_vm: PASS (phase29y single-entry contracts locked)"
