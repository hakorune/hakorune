#!/bin/bash
# Phase 29x X50: NewClosure contract lock gate
#
# Decision (current phase):
# - Keep fail-fast contract for runtime execution lanes.
# - Do not add runtime NewClosure execution semantics in this lane.
#
# Contract pin:
# 1) MIR allowlist still accepts `new_closure` shape (compiler-side canonical contract).
# 2) Runtime probe keeps fail-fast behavior:
#    - vm route: unsupported op(new_closure) in loader
#    - hako-runner route: [vm-hako/unimplemented op=new_closure]

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

run_step() {
    local cmd="$1"
    if ! bash -lc "$cmd"; then
        test_fail "phase29x_vm_hako_newclosure_contract_vm: step failed: $cmd"
        exit 1
    fi
}

run_step "cd \"$NYASH_ROOT\" && cargo test -q mir_json_allowlist_accepts_new_closure -- --nocapture"
run_step "bash \"$NYASH_ROOT/tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_newclosure_probe_vm.sh\""

test_pass "phase29x_vm_hako_newclosure_contract_vm: PASS (newclosure fail-fast contract locked)"
