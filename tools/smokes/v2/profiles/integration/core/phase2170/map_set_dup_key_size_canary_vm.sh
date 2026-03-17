#!/bin/bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR"/../../../../../../../../.. && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_map_set_dup_key_size_$$.json"
cat > "$tmp_json" <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name":"main","blocks":[
      {"id":0,"instructions":[
        {"op":"mir_call", "dst": 1, "mir_call": {"callee": {"type":"Constructor","box_type":"MapBox"}, "args": [], "effects": []}},
        {"op":"const",   "dst": 2, "value": {"type": {"kind":"handle","box_type":"StringBox"}, "value": "k1"}},
        {"op":"const",   "dst": 3, "value": {"type": "i64", "value": 100}},
        {"op":"mir_call",          "mir_call": {"callee": {"type":"Method","box_name":"MapBox","method":"set","receiver":1}, "args": [2,3], "effects": []}},
        {"op":"const",   "dst": 5, "value": {"type": "i64", "value": 200}},
        {"op":"mir_call",          "mir_call": {"callee": {"type":"Method","box_name":"MapBox","method":"set","receiver":1}, "args": [2,5], "effects": []}},
        {"op":"const",   "dst": 6, "value": {"type": {"kind":"handle","box_type":"StringBox"}, "value": "k2"}},
        {"op":"const",   "dst": 7, "value": {"type": "i64", "value": 300}},
        {"op":"mir_call",          "mir_call": {"callee": {"type":"Method","box_name":"MapBox","method":"set","receiver":1}, "args": [6,7], "effects": []}},
        {"op":"mir_call", "dst": 8, "mir_call": {"callee": {"type":"Method","box_name":"MapBox","method":"size","receiver":1}, "args": [], "effects": []}},
        {"op":"ret","value":8}
      ]}
    ]}
  ]
}
JSON

trap 'rm -f "$tmp_json" || true' EXIT

# k1 (new) -> size=1, k1 (dup) -> size stays 1, k2 (new) -> size=2
HAKO_VERIFY_PRIMARY=hakovm HAKO_ABI_ADAPTER=${HAKO_ABI_ADAPTER:-1} HAKO_VM_MIRCALL_SIZESTATE=1 HAKO_VM_MIRCALL_SIZESTATE_PER_RECV=1 \
  run_verify_mir_rc_and_expect "$tmp_json" 2 "map_set_dup_key_size_canary_vm" "map_set_dup_key_size_canary_vm"
