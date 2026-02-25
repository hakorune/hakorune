#!/bin/bash
# Phase 29x X27: compat bypass fail-fast guard smoke
#
# Contract pin:
# 1) Direct callsites of `execute_vm_fallback_interpreter` are owned by route_orchestrator only.
# 2) vm_fallback entry must enforce explicit fallback guard.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if ! bash "$NYASH_ROOT/tools/checks/vm_route_bypass_guard.sh"; then
    test_fail "phase29x_vm_route_compat_bypass_guard_vm: bypass guard check failed"
    exit 1
fi

test_pass "phase29x_vm_route_compat_bypass_guard_vm: PASS (fallback callsite ownership + guard hook)"
