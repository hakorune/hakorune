#!/bin/bash
# Phase 29x X29: safety gate single-entry smoke
#
# Contract pin:
# 1) vm / vm-fallback source safety uses common_util/safety_gate only.
# 2) vm / vm-fallback / vm-hako lifecycle safety hook is mandatory.
# 3) lifecycle fail-fast reason vocabulary is owned by safety_gate.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if ! bash "$NYASH_ROOT/tools/checks/vm_safety_gate_guard.sh"; then
    test_fail "phase29x_vm_safety_gate_single_entry_vm: safety gate guard failed"
    exit 1
fi

test_pass "phase29x_vm_safety_gate_single_entry_vm: PASS (single-entry safety ownership)"
