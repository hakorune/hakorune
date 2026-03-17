#!/bin/bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR"/../../../../../../../../.. && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

# value_state=1: Map.get returns stored value, Map.has reports presence
tmp_json="/tmp/mir_v1_map_value_state_$$.json"
cat > "$tmp_json" <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name":"main","blocks":[
      {"id":0,"instructions":[
        {"op":"mir_call", "dst": 1, "mir_call": {"callee": {"type":"Constructor","box_type":"MapBox"}, "args": [], "effects": []}},
        {"op":"const",   "dst": 2, "value": {"type": {"kind":"handle","box_type":"StringBox"}, "value": "k2"}},
        {"op":"const",   "dst": 3, "value": {"type": "i64", "value": 200}},
        {"op":"mir_call",          "mir_call": {"callee": {"type":"Method","box_name":"MapBox","method":"set","receiver":1}, "args": [2,3], "effects": []}},
        {"op":"mir_call", "dst": 4, "mir_call": {"callee": {"type":"Method","box_name":"MapBox","method":"get","receiver":1}, "args": [2], "effects": []}},
        {"op":"ret","value":4}
      ]}
    ]}
  ]
}
JSON

trap 'rm -f "$tmp_json" || true' EXIT

HAKO_VERIFY_PRIMARY=hakovm HAKO_ABI_ADAPTER=${HAKO_ABI_ADAPTER:-1} HAKO_VM_MIRCALL_SIZESTATE=1 HAKO_VM_MIRCALL_VALUESTATE=1 HAKO_VM_MIRCALL_SIZESTATE_PER_RECV=1 \
  run_verify_mir_rc_and_expect "$tmp_json" 200 "map_value_state_get_has_canary_vm" "map_value_state_get_has_canary_vm"
