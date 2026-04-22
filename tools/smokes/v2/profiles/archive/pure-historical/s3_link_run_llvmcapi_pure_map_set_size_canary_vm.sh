#!/bin/bash
# S3 (C‑API pure): map set→size → rc=1（pureフラグON）
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/phase2120_boundary_pure_helper.sh"
phase2120_boundary_pure_prepare "$ROOT" "s3_link_run_llvmcapi_pure_map_set_size_canary_vm"
phase2120_boundary_pure_require_kernel_symbol "$ROOT" "nyash.map.entry_count_h" "s3_link_run_llvmcapi_pure_map_set_size_canary_vm"
phase2120_boundary_pure_require_kernel_symbol "$ROOT" "nyash.map.slot_store_hhh" "s3_link_run_llvmcapi_pure_map_set_size_canary_vm"

json='{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"mir_call","dst":3,"mir_call":{"callee":{"type":"Constructor","box_type":"MapBox"},"args":[],"effects":[]}},{"op":"const","dst":1,"value":{"type":"i64","value":1}},{"op":"const","dst":2,"value":{"type":"i64","value":1}},{"op":"mir_call","dst":4,"mir_call":{"callee":{"type":"Method","box_name":"MapBox","method":"set","receiver":3},"args":[1,2],"effects":[]}},{"op":"mir_call","dst":5,"mir_call":{"callee":{"type":"Method","box_name":"MapBox","method":"size","receiver":3},"args":[],"effects":[]}},{"op":"ret","value":5}]}]}]}'
export _MIR_JSON="$json"
phase2120_boundary_pure_run "$json" 1 "s3_exe_map_capi_pure"
echo "[PASS] s3_link_run_llvmcapi_pure_map_set_size_canary_vm"
exit 0
