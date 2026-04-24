#!/bin/bash
# S3 (C‑API pure): map set→has -> rc=1（historical pure-lowering / pureフラグON）
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/phase2120_boundary_pure_helper.sh"
phase2120_boundary_pure_prepare "$ROOT" "s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm"
phase2120_boundary_pure_require_kernel_symbol "$ROOT" "nyash.map.slot_store_hhh" "s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm"
phase2120_boundary_pure_require_kernel_symbol "$ROOT" "nyash.map.probe_hh" "s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm"

# Historical note:
# - filename is legacy
# - current payload locks `MapBox.set -> has`
# - `MapBox.get -> ret` has its own dedicated canary
# GEN2: map set/has → has returns 1 → rc=1
json_has='{"schema_version":"1.0","functions":[{"name":"main","metadata":{"generic_method_routes":[{"route_id":"generic_method.set","block":0,"instruction_index":3,"box_name":"MapBox","method":"set","receiver_origin_box":"MapBox","key_route":"i64_const","arity":2,"receiver_value":1,"key_value":2,"result_value":null,"emit_kind":"set","route_kind":"map_store_any","helper_symbol":"nyash.map.slot_store_hhh","proof":"set_surface_policy","core_method":{"op":"MapSet","proof":"core_method_contract_manifest","lowering_tier":"cold_fallback"},"return_shape":null,"value_demand":"write_any","publication_policy":null,"effects":["mutate.slot"]}]},"blocks":[{"id":0,"instructions":[
  {"op":"const","dst":2,"value":{"type":"i64","value":5}},
  {"op":"const","dst":3,"value":{"type":"i64","value":9}},
  {"op":"mir_call","dst":1,"mir_call":{"callee":{"type":"Constructor","box_name":"MapBox"},"args":[],"effects":[]}},
  {"op":"mir_call","mir_call":{"callee":{"type":"Method","box_name":"MapBox","method":"set","receiver":1},"args":[2,3],"effects":[]}},
  {"op":"mir_call","dst":4,"mir_call":{"callee":{"type":"Method","box_name":"MapBox","method":"has","receiver":1},"args":[2],"effects":[]}},
  {"op":"ret","value":4}
]}]}]}'
phase2120_boundary_pure_run "$json_has" 1 "s3_exe_map_case_pure"

echo "[PASS] s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm"
exit 0
