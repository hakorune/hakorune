#!/bin/bash
# Phase29z-S5a: vm-hako S5 parity smoke (load + const/ret minimal fixture)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
export VM_HAKO_PARITY_NAME="phase29z_vm_hako_s5_load_parity_vm"
export VM_HAKO_PARITY_OPCODE="load"
export VM_HAKO_PARITY_FIXTURE_REL="apps/tests/phase29z_vm_hako_s5_load_const_ptr_return_mir_v0.json"
export VM_HAKO_PARITY_EXPECTED_RC="0"
export VM_HAKO_PARITY_DENY_OPS="store,binop,compare,branch,jump,copy,unop,call,externcall,newbox,boxcall,nop,safepoint,keepalive,release_strong,debug,debug_log,select,barrier"
exec "$SCRIPT_DIR/lib/vm_hako_json_parity_common.sh" "$@"
