#!/bin/bash
# S3 (C‑API pure): while-like loop with φ (i from 0..N) → rc=N（here N=5）
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/profiles/integration/core/phase2120/boundary_pure_helper.sh"
phase2120_boundary_pure_prepare "$ROOT" "s3_link_run_llvmcapi_pure_loop_count_canary_vm"

# JSON v1: blocks with phi at header, compare, body add, jump back, exit returns i
json='{"schema_version":"1.0","functions":[{"name":"main","blocks":[
 {"id":0,"instructions":[
   {"op":"const","dst":1,"value":{"type":"i64","value":0}},
   {"op":"const","dst":2,"value":{"type":"i64","value":5}},
   {"op":"const","dst":6,"value":{"type":"i64","value":1}},
   {"op":"jump","target":1}
 ]},
 {"id":1,"instructions":[
   {"op":"phi","dst":3,"values":[{"pred":0,"value":1},{"pred":2,"value":4}]},
   {"op":"compare","dst":5,"cmp":"Lt","lhs":3,"rhs":2},
   {"op":"branch","cond":5,"then":2,"else":3}
 ]},
 {"id":2,"instructions":[
   {"op":"binop","op_kind":"Add","dst":4,"lhs":3,"rhs":6},
   {"op":"jump","target":1}
 ]},
 {"id":3,"instructions":[
   {"op":"ret","value":3}
 ]}
]}]}'
export _MIR_JSON="$json"
phase2120_boundary_pure_run "$json" 5 "s3_exe_loop_phi_pure"
echo "[PASS] s3_link_run_llvmcapi_pure_loop_count_canary_vm"
exit 0
