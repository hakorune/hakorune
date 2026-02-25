#!/bin/bash
# Phase29z-S5l: vm-hako S5 parity smoke (array_set + const/ret minimal fixture)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
export VM_HAKO_PARITY_NAME="phase29z_vm_hako_s5_array_set_parity_vm"
export VM_HAKO_PARITY_OPCODE="array_set"
export VM_HAKO_PARITY_FIXTURE_REL="apps/tests/phase29z_vm_hako_s5_array_set_load_return_mir_v0.json"
export VM_HAKO_PARITY_EXPECTED_RC="42"
export VM_HAKO_PARITY_DENY_OPS="array_get,load,store,phi,typeop,weak_new,weak_load,ref_new,future_new,future_set,await,binop,compare,branch,jump,copy,unop,call,externcall,newbox,boxcall,nop,safepoint,keepalive,release_strong,debug,debug_log,select,barrier"
exec "$SCRIPT_DIR/lib/vm_hako_json_parity_common.sh" "$@"
