#!/bin/bash
# Phase 29x X46: cache lane integration gate
#
# Contract pin:
# - X42-X45 cache contracts are replayable via one command.
# - daily/milestone entry can include this gate and observe miss->hit behavior.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if ! can_run_llvm; then
    test_skip "phase29x_cache_lane_gate: LLVM backend not available"
    exit 0
fi

run_step() {
    local cmd="$1"
    if ! bash "$NYASH_ROOT/$cmd"; then
        test_fail "phase29x_cache_lane_gate: step failed: $cmd"
        exit 1
    fi
}

run_step "tools/smokes/v2/profiles/integration/apps/phase29x_cache_key_determinism_vm.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_l1_mir_cache_vm.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_l2_object_cache_vm.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_l3_link_cache_vm.sh"

test_pass "phase29x_cache_lane_gate: PASS (X42-X45 cache contracts integrated)"
