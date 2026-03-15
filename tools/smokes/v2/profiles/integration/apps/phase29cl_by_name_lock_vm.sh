#!/bin/bash
# Phase 29cl BYN-min1:
# lock `invoke_by_name_i64` owner set before further caller cutover.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

run_step() {
    local cmd="$1"
    if ! (cd "$NYASH_ROOT" && bash -lc "$cmd"); then
        test_fail "phase29cl_by_name_lock_vm: step failed: $cmd"
        exit 1
    fi
}

run_step "bash tools/checks/phase29cl_by_name_mainline_guard.sh"
run_step "bash tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_box_capi_link_min.sh"
run_step "SMOKES_FORCE_LLVM=1 bash tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh"

test_pass "phase29cl_by_name_lock_vm: PASS (BYN-min1 owner set locked and backend proof stays green)"
