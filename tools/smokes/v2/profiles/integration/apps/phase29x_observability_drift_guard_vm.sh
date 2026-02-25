#!/bin/bash
# Phase 29x X61: observability drift guard gate
#
# Contract pin:
# - Replay X60 RC phase2 queue gate as precondition.
# - Replay observability 5-category smokes under one single-entry gate.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

run_step() {
    local cmd="$1"
    if ! bash "$NYASH_ROOT/$cmd"; then
        test_fail "phase29x_observability_drift_guard_vm: step failed: $cmd"
        exit 1
    fi
}

run_step "tools/checks/phase29x_observability_drift_guard.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_rc_phase2_queue_lock_vm.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_observability_temps_vm.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_observability_heap_fields_vm.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_observability_singletons_vm.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_observability_summary_vm.sh"

test_pass "phase29x_observability_drift_guard_vm: PASS (X61 observability drift locked)"
