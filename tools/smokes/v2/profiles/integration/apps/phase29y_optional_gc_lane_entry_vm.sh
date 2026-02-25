#!/bin/bash
# Phase 29y optional GC lane entry gate
#
# Contract pin:
# - Optional GC lane starts only after RC/GC alignment single-entry matrix is green.
# - Entry gate does not change semantics; it replays preconditions as fail-fast checks.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/gate_steps.sh"
require_env || exit 2

run_gate_step "phase29y_optional_gc_lane_entry_vm" "tools/checks/phase29y_optional_gc_lane_entry_guard.sh"
run_gate_step "phase29y_optional_gc_lane_entry_vm" "tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g2_fast_milestone_gate.sh"

test_pass "phase29y_optional_gc_lane_entry_vm: PASS (optional GC lane entry preconditions locked)"
