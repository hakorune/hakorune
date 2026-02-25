#!/bin/bash
# Phase 29x X51: Core C ABI delegation guard smoke
#
# Contract pin:
# - Core C ABI minimal symbols remain owned by canonical delegation files only.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if ! bash "$NYASH_ROOT/tools/checks/phase29x_core_cabi_delegation_guard.sh"; then
    test_fail "phase29x_core_cabi_delegation_guard_vm: guard failed"
    exit 1
fi

test_pass "phase29x_core_cabi_delegation_guard_vm: PASS (core cabi delegation ownership locked)"
