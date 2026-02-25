#!/bin/bash
# phase29bq_phase5_joinir_integration_gate_vm.sh
#
# Contract pin (Phase 5: JoinIR integration):
# - PHI/exit invariant lock fixture remains green.
# - PHI join-cursor selfhost fixture remains green.
# - JoinIR expression parity seed remains green.
#
# Optional:
# - PHASE29BQ_PHASE5_WITH_29BP=1 adds planner-required dev gate replay.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/gate_steps.sh"
source "$(dirname "$0")/../../../lib/vm_route_pin.sh"
require_env || exit 2
export_vm_route_pin

run_gate_cmd() {
  local gate_name="$1"
  shift
  if ! "$@"; then
    test_fail "${gate_name}: step failed: $*"
    exit 1
  fi
}

run_gate_step "phase29bq_phase5_joinir_integration_gate_vm" \
  "tools/smokes/v2/profiles/integration/joinir/phase29bq_joinir_port04_phi_exit_invariant_lock_vm.sh"

run_gate_cmd "phase29bq_phase5_joinir_integration_gate_vm" \
  bash "$NYASH_ROOT/tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh" \
  --only \
  selfhost_phi_missing_step_join_cursor_min

run_gate_step "phase29bq_phase5_joinir_integration_gate_vm" \
  "tools/smokes/v2/profiles/integration/joinir/phase29bq_joinir_port07_expr_parity_seed_vm.sh"

if [ "${PHASE29BQ_PHASE5_WITH_29BP:-0}" = "1" ]; then
  run_gate_step "phase29bq_phase5_joinir_integration_gate_vm" \
    "tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh"
fi

test_pass "phase29bq_phase5_joinir_integration_gate_vm: PASS (phase5 joinir integration locked)"
