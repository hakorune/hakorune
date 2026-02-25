#!/bin/bash
# Phase 29x X30: thin-rust Core C ABI minimal surface guard smoke
#
# Contract pin:
# 1) Core C ABI symbols for route/verifier/safety/lifecycle are present in header/shim/doc.
# 2) Surface sync is checked by one guard script.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if ! bash "$NYASH_ROOT/tools/checks/nyrt_core_cabi_surface_guard.sh"; then
    test_fail "phase29x_core_cabi_surface_guard_vm: core cabi surface guard failed"
    exit 1
fi

test_pass "phase29x_core_cabi_surface_guard_vm: PASS (core cabi surface sync)"
