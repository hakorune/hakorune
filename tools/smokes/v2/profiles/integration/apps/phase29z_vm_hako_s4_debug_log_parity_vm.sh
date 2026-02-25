#!/bin/bash
# Phase29z-S4b: vm-hako S4 parity smoke (debug_log + const/ret minimal fixture)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
export VM_HAKO_PARITY_NAME="phase29z_vm_hako_s4_debug_log_parity_vm"
export VM_HAKO_PARITY_OPCODE="debug_log"
export VM_HAKO_PARITY_FIXTURE_REL="apps/tests/phase29z_vm_hako_s4_debug_log_const_add_return_mir_v0.json"
export VM_HAKO_PARITY_DENY_OPS="binop,select,barrier,debug"
exec "$SCRIPT_DIR/lib/vm_hako_json_parity_common.sh" "$@"
