#!/bin/bash
# Phase 29x X48: route pin inventory guard smoke
#
# Contract pin:
# - `NYASH_VM_HAKO_PREFER_STRICT_DEV` hard-pin assignments stay in allowlisted gate/smoke callsites only.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if ! bash "$NYASH_ROOT/tools/checks/phase29x_vm_route_pin_guard.sh"; then
    test_fail "phase29x_vm_route_pin_guard_vm: guard failed"
    exit 1
fi

test_pass "phase29x_vm_route_pin_guard_vm: PASS (route pin inventory locked)"
