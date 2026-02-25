#!/bin/bash
# Phase29z-S5d: vm-hako S5 parity smoke (typeop + const/ret minimal fixture)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
export VM_HAKO_PARITY_NAME="phase29z_vm_hako_s5_typeop_parity_vm"
export VM_HAKO_PARITY_OPCODE="typeop"
export VM_HAKO_PARITY_FIXTURE_REL="apps/tests/phase29z_vm_hako_s5_typeop_cast_return_mir_v0.json"
export VM_HAKO_PARITY_EXPECTED_RC="42"
export VM_HAKO_PARITY_DENY_OPS="load,store,phi,binop,compare,branch,jump,copy,unop,call,externcall,newbox,boxcall,nop,safepoint,keepalive,release_strong,debug,debug_log,select,barrier"
exec "$SCRIPT_DIR/lib/vm_hako_json_parity_common.sh" "$@"
