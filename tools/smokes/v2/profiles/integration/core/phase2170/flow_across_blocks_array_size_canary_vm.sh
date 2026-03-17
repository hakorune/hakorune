#!/bin/bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR"/../../../../../../../../.. && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_flow_across_blocks_array_$$.json"
cat > "$tmp_json" <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name":"main","blocks":[
      {"id":0,"instructions":[
        {"op":"mir_call", "dst": 1, "mir_call": {"callee": {"type":"Constructor","box_type":"ArrayBox"}, "args": [], "effects": []}},
        {"op":"const",   "dst": 2, "value": {"type": "i64", "value": 0}},
        {"op":"mir_call",          "mir_call": {"callee": {"type":"Method","box_name":"ArrayBox","method":"push","receiver":1}, "args": [2], "effects": []}},
        {"op":"mir_call",          "mir_call": {"callee": {"type":"Method","box_name":"ArrayBox","method":"push","receiver":1}, "args": [2], "effects": []}},
        {"op":"mir_call",          "mir_call": {"callee": {"type":"Method","box_name":"ArrayBox","method":"push","receiver":1}, "args": [2], "effects": []}},
        {"op":"const","dst":10, "value": {"type": "i64", "value": 1}},
        {"op":"const","dst":11, "value": {"type": "i64", "value": 1}},
        {"op":"compare","dst":12, "lhs":10, "rhs":11, "cmp":"Eq"},
        {"op":"branch","cond":12, "then":1, "else":2}
      ]},
      {"id":1,"instructions":[
        {"op":"mir_call", "dst": 3, "mir_call": {"callee": {"type":"Method","box_name":"ArrayBox","method":"size","receiver":1}, "args": [], "effects": []}},
        {"op":"ret","value":3}
      ]},
      {"id":2,"instructions":[
        {"op":"ret","value":2}
      ]}
    ]}
  ]
}
JSON

trap 'rm -f "$tmp_json" || true' EXIT

HAKO_VERIFY_PRIMARY=hakovm HAKO_V1_DISPATCHER_FLOW=1 HAKO_VM_MIRCALL_SIZESTATE=1 HAKO_VM_MIRCALL_SIZESTATE_PER_RECV=1 \
  run_verify_mir_rc_and_expect "$tmp_json" 3 "flow_across_blocks_array_size_canary_vm" "flow_across_blocks_array_size_canary_vm"
