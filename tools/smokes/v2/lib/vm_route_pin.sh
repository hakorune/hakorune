#!/bin/bash
# vm_route_pin.sh - shared helpers for vm route pin (NYASH_VM_HAKO_PREFER_STRICT_DEV=0)
#
# Contract:
# - Compiler/selfhost gate scripts that must stay on rust-vm lane should use these
#   helpers instead of duplicating raw env assignment at each callsite.

set -euo pipefail

export_vm_route_pin() {
  export NYASH_VM_HAKO_PREFER_STRICT_DEV=0
}

run_with_vm_route_pin() {
  env NYASH_VM_HAKO_PREFER_STRICT_DEV=0 "$@"
}

run_hermetic_vm_with_route_pin() {
  if ! declare -F run_hermetic_vm >/dev/null 2>&1; then
    echo "[vm-route-pin] ERROR: run_hermetic_vm is not defined" >&2
    return 2
  fi
  run_hermetic_vm env NYASH_VM_HAKO_PREFER_STRICT_DEV=0 "$@"
}
