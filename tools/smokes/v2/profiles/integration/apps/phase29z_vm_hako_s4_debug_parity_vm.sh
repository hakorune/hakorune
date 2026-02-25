#!/bin/bash
# Phase29z-S4a: vm-hako S4 parity smoke (debug + const/ret minimal fixture)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
export VM_HAKO_PARITY_NAME="phase29z_vm_hako_s4_debug_parity_vm"
export VM_HAKO_PARITY_OPCODE="debug"
export VM_HAKO_PARITY_FIXTURE_REL="apps/tests/phase29z_vm_hako_s4_debug_const_add_return_mir_v0.json"
export VM_HAKO_PARITY_DENY_OPS="binop,select,barrier,debug_log"
exec "$SCRIPT_DIR/lib/vm_hako_json_parity_common.sh" "$@"
