#!/bin/bash
# Phase29z-S4d: vm-hako S4 parity smoke (barrier + const/ret minimal fixture)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
export VM_HAKO_PARITY_NAME="phase29z_vm_hako_s4_barrier_parity_vm"
export VM_HAKO_PARITY_OPCODE="barrier"
export VM_HAKO_PARITY_FIXTURE_REL="apps/tests/phase29z_vm_hako_s4_barrier_const_add_return_mir_v0.json"
export VM_HAKO_PARITY_DENY_OPS="binop,select,debug,debug_log"
exec "$SCRIPT_DIR/lib/vm_hako_json_parity_common.sh" "$@"
