#!/bin/bash
# Phase29z-S4-clean-1: bool const whitespace parity smoke on vm-hako JSON route

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
export VM_HAKO_PARITY_NAME="phase29z_vm_hako_s4_bool_const_ws_parity_vm"
export VM_HAKO_PARITY_OPCODE="select"
export VM_HAKO_PARITY_FIXTURE_REL="apps/tests/phase29z_vm_hako_s4_bool_const_ws_select_return_mir_v0.json"
export VM_HAKO_PARITY_DENY_OPS="binop,barrier,debug,debug_log"
exec "$SCRIPT_DIR/lib/vm_hako_json_parity_common.sh" "$@"
