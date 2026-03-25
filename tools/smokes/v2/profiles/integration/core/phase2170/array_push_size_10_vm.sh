#!/bin/bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR"/../../../../../../../../.. && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_array_push_size_10_$$.json"
cat > "$tmp_json" <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name":"main","blocks":[
      {"id":0,"instructions":[
        {"op":"mir_call", "dst": 1, "mir_call": {"callee": {"type":"Constructor","box_type":"ArrayBox"}, "args": [], "effects": []}},
        {"op":"const",   "dst": 2, "value": {"type": "i64", "value": 1}},
        {"op":"mir_call",          "mir_call": {"callee": {"type":"Method","box_name":"ArrayBox","method":"push","receiver":1}, "args": [2], "effects": []}},
        {"op":"mir_call",          "mir_call": {"callee": {"type":"Method","box_name":"ArrayBox","method":"push","receiver":1}, "args": [2], "effects": []}},
        {"op":"mir_call",          "mir_call": {"callee": {"type":"Method","box_name":"ArrayBox","method":"push","receiver":1}, "args": [2], "effects": []}},
        {"op":"mir_call",          "mir_call": {"callee": {"type":"Method","box_name":"ArrayBox","method":"push","receiver":1}, "args": [2], "effects": []}},
        {"op":"mir_call",          "mir_call": {"callee": {"type":"Method","box_name":"ArrayBox","method":"push","receiver":1}, "args": [2], "effects": []}},
        {"op":"mir_call",          "mir_call": {"callee": {"type":"Method","box_name":"ArrayBox","method":"push","receiver":1}, "args": [2], "effects": []}},
        {"op":"mir_call",          "mir_call": {"callee": {"type":"Method","box_name":"ArrayBox","method":"push","receiver":1}, "args": [2], "effects": []}},
        {"op":"mir_call",          "mir_call": {"callee": {"type":"Method","box_name":"ArrayBox","method":"push","receiver":1}, "args": [2], "effects": []}},
        {"op":"mir_call",          "mir_call": {"callee": {"type":"Method","box_name":"ArrayBox","method":"push","receiver":1}, "args": [2], "effects": []}},
        {"op":"mir_call",          "mir_call": {"callee": {"type":"Method","box_name":"ArrayBox","method":"push","receiver":1}, "args": [2], "effects": []}},
        {"op":"mir_call", "dst": 3, "mir_call": {"callee": {"type":"Method","box_name":"ArrayBox","method":"size","receiver":1}, "args": [], "effects": []}},
        {"op":"ret","value":3}
      ]}
    ]}
  ]
}
JSON

trap 'rm -f "$tmp_json" || true' EXIT

run_verify_mir_canary_and_expect_rc \
  run_verify_mir_via_hakovm_size_state_per_recv \
  "$tmp_json" \
  10 \
  "array_push_size_10_vm" \
  "array_push_size_10_vm"
