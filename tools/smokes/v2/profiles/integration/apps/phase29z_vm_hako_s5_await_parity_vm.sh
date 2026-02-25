#!/bin/bash
# Phase29z-S5i: vm-hako S5 parity smoke (await + future_new + const/ret minimal fixture)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
export VM_HAKO_PARITY_NAME="phase29z_vm_hako_s5_await_parity_vm"
export VM_HAKO_PARITY_OPCODE="await"
export VM_HAKO_PARITY_FIXTURE_REL="apps/tests/phase29z_vm_hako_s5_await_return_mir_v0.json"
export VM_HAKO_PARITY_EXPECTED_RC="42"
export VM_HAKO_PARITY_DENY_OPS="ref_new,weak_new,weak_load,future_set,load,store,phi,typeop,binop,compare,branch,jump,copy,unop,call,externcall,boxcall,nop,safepoint,keepalive,release_strong,debug,debug_log,select,barrier,newbox"
exec "$SCRIPT_DIR/lib/vm_hako_json_parity_common.sh" "$@"
