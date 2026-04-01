#!/bin/bash
# Phase 29ck runtime proof for the former phase2111 map set->size semantics.
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
        {"op":"newbox","dst":3,"type":"MapBox"},
        {"op":"const","dst":1,"value":{"type":"i64","value":1}},
        {"op":"const","dst":2,"value":{"type":"i64","value":1}},
        {"op":"mir_call","dst":4,"mir_call":{"callee":{"type":"Method","box_name":"MapBox","name":"set","receiver":3},"args":[1,2],"effects":[],"flags":{}}},
        {"op":"mir_call","dst":5,"mir_call":{"callee":{"type":"Method","box_name":"MapBox","name":"size","receiver":3},"args":[],"effects":[],"flags":{}}},
        {"op":"ret","value":5}
      ]}
    ]}
  ]
}
JSON

llvm_backend_runtime_run_case "phase29ck_llvm_backend_map_set_size_runtime_proof" "$TMP_MIR" 1
