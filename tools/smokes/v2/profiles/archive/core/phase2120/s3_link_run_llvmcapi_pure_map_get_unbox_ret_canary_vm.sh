#!/bin/bash
# S3 (C-API pure/TM): map set->get->ret (auto-unbox) -> rc=9
#
# Historical pure-lowering canary:
# - keep proving the pure C-API map.get auto-unbox route
# - do not depend on the retired hostbridge caller lane
# - current caller path uses `ny-llvmc --driver boundary`
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/profiles/integration/core/phase2120/boundary_pure_helper.sh"
phase2120_boundary_pure_prepare "$ROOT" "s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm"
phase2120_boundary_pure_require_kernel_symbol "$ROOT" "nyash.map.slot_store_hhh" "s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm"
phase2120_boundary_pure_require_kernel_symbol "$ROOT" "nyash.map.slot_load_hh" "s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm"

json='{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[
  {"op":"const","dst":2,"value":{"type":"i64","value":5}},
  {"op":"const","dst":3,"value":{"type":"i64","value":9}},
  {"op":"mir_call","dst":1,"mir_call":{"callee":{"type":"Constructor","box_name":"MapBox"},"args":[],"effects":[]}},
  {"op":"mir_call","mir_call":{"callee":{"type":"Method","box_name":"MapBox","method":"set","receiver":1},"args":[2,3],"effects":[]}},
  {"op":"mir_call","dst":4,"mir_call":{"callee":{"type":"Method","box_name":"MapBox","method":"get","receiver":1},"args":[2],"effects":[]}},
  {"op":"ret","value":4}
]}]}]}'
export _MIR_JSON="$json"

phase2120_boundary_pure_run "$json" 9 "s3_exe_map_get_unbox_pure"
echo "[PASS] s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm"
exit 0
