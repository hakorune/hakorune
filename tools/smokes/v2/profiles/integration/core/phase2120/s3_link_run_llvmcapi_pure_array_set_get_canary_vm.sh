#!/bin/bash
# S3 (C‑API pure): array set→get → rc=7（historical pure-lowering / pureフラグON）
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/profiles/integration/core/phase2120/boundary_pure_helper.sh"
phase2120_boundary_pure_prepare "$ROOT" "s3_link_run_llvmcapi_pure_array_set_get_canary_vm"

# Historical note:
# - filename is legacy
# - current payload proves `ArrayBox.set -> get`
# - keep the script name stable for compat pack continuity
# JSON v1 with explicit box_name/method/receiver so generic lowering picks it up
json='{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[
  {"op":"const","dst":2,"value":{"type":"i64","value":0}},
  {"op":"const","dst":3,"value":{"type":"i64","value":7}},
  {"op":"mir_call","dst":1,"mir_call":{"callee":{"type":"Constructor","box_name":"ArrayBox"},"args":[],"effects":[]}},
  {"op":"mir_call","mir_call":{"callee":{"type":"Method","box_name":"ArrayBox","method":"set","receiver":1},"args":[2,3],"effects":[]}},
  {"op":"mir_call","dst":4,"mir_call":{"callee":{"type":"Method","box_name":"ArrayBox","method":"get","receiver":1},"args":[2],"effects":[]}},
  {"op":"ret","value":4}
]}]}]}'
export _MIR_JSON="$json"
phase2120_boundary_pure_require_kernel_symbol "$ROOT" "nyash.array.slot_store_hii" "s3_link_run_llvmcapi_pure_array_set_get_canary_vm"
phase2120_boundary_pure_require_kernel_symbol "$ROOT" "nyash.array.slot_load_hi" "s3_link_run_llvmcapi_pure_array_set_get_canary_vm"
phase2120_boundary_pure_run "$json" 7 "s3_exe_array_set_get_pure"
echo "[PASS] s3_link_run_llvmcapi_pure_array_set_get_canary_vm"
exit 0
