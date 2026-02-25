#!/bin/bash
# Phase 29x X31: thin-rust gate pack
#
# Goal:
# - Lock X24-X30 contracts with one executable gate pack.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

run_guard() {
    local cmd="$1"
    if ! bash "$NYASH_ROOT/$cmd"; then
        test_fail "phase29x_thin_rust_gate_vm: guard failed: $cmd"
        exit 1
    fi
}

run_smoke() {
    local cmd="$1"
    if ! bash "$NYASH_ROOT/$cmd"; then
        test_fail "phase29x_thin_rust_gate_vm: smoke failed: $cmd"
        exit 1
    fi
}

run_guard "tools/checks/vm_route_bypass_guard.sh"
run_guard "tools/checks/vm_verifier_gate_guard.sh"
run_guard "tools/checks/vm_safety_gate_guard.sh"
run_guard "tools/checks/nyrt_core_cabi_surface_guard.sh"

run_smoke "tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_observability_vm.sh"
run_smoke "tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_strict_dev_priority_vm.sh"
run_smoke "tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh"
run_smoke "tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_compat_bypass_guard_vm.sh"
run_smoke "tools/smokes/v2/profiles/integration/apps/phase29x_vm_verifier_gate_single_entry_vm.sh"
run_smoke "tools/smokes/v2/profiles/integration/apps/phase29x_vm_safety_gate_single_entry_vm.sh"
run_smoke "tools/smokes/v2/profiles/integration/apps/phase29x_core_cabi_surface_guard_vm.sh"

test_pass "phase29x_thin_rust_gate_vm: PASS (x24-x30 contracts locked)"
