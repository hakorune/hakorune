#!/bin/bash
# Phase 29x X28: verifier gate single-entry smoke
#
# Contract pin:
# 1) MirVerifier callsite is owned by common_util/verifier_gate.rs only.
# 2) vm / vm-fallback / vm-hako routes must call enforce_vm_verify_gate_or_exit().
# 3) direct NYASH_VM_VERIFY_MIR checks are not duplicated in route mode files.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if ! bash "$NYASH_ROOT/tools/checks/vm_verifier_gate_guard.sh"; then
    test_fail "phase29x_vm_verifier_gate_single_entry_vm: verifier gate guard failed"
    exit 1
fi

test_pass "phase29x_vm_verifier_gate_single_entry_vm: PASS (single-entry verifier ownership)"
