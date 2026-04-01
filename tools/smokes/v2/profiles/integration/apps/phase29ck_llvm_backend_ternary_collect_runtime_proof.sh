#!/bin/bash
# Phase 29ck runtime proof for the former phase2111 ternary collect semantics.
# Root-first route:
# `.hako VM` -> LlvmBackendBox -> env.codegen C-API -> object -> exe

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null || (cd "$SCRIPT_DIR/../../../../../../.." && pwd))"

source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh"
source "$ROOT_DIR/tools/smokes/v2/lib/llvm_backend_runtime_proof_common.sh"
require_env || exit 2

TMP_MIR="$(mktemp --suffix .mir.json)"
cleanup() {
  rm -f "$TMP_MIR"
}
trap cleanup EXIT

cat >"$TMP_MIR" <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name":"main","blocks":[
      {"id":0,"instructions":[
        {"op":"const","dst":1,"value":{"type":"i64","value":3}},
        {"op":"const","dst":2,"value":{"type":"i64","value":5}},
        {"op":"compare","dst":3,"operation":"<","lhs":1,"rhs":2},
        {"op":"branch","cond":3,"then":1,"else":2}
      ]},
      {"id":1,"instructions":[
        {"op":"const","dst":4,"value":{"type":"i64","value":44}},
        {"op":"jump","target":3}
      ]},
      {"id":2,"instructions":[
        {"op":"const","dst":5,"value":{"type":"i64","value":40}},
        {"op":"jump","target":3}
      ]},
      {"id":3,"instructions":[
        {"op":"phi","dst":6,"incoming":[[4,1],[5,2]]},
        {"op":"ret","value":6}
      ]}
    ]}
  ]
}
JSON

llvm_backend_runtime_run_case "phase29ck_llvm_backend_ternary_collect_runtime_proof" "$TMP_MIR" 44
