#!/bin/bash
# Hakovm v1 dispatcher: MapBox set(new key) → size increments (1→2)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR"/../../../../../../../../.. && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_map_set_size_$$.json"
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
        {"op":"const",   "dst": 4, "value": {"type": {"kind":"handle","box_type":"StringBox"}, "value": "k2"}},
        {"op":"const",   "dst": 5, "value": {"type": "i64", "value": 200}},
        {"op":"mir_call",          "mir_call": {"callee": {"type":"Method","box_name":"MapBox","method":"set","receiver":1}, "args": [4,5], "effects": []}},
        {"op":"mir_call", "dst": 6, "mir_call": {"callee": {"type":"Method","box_name":"MapBox","method":"size","receiver":1}, "args": [], "effects": []}},
        {"op":"ret","value":6}
      ]}
    ]}
  ]
}
JSON

# Expect rc=2 when stateful mir_call (Map.set) is ON and primary=hakovm
set +e
HAKO_VERIFY_PRIMARY=hakovm \
HAKO_VM_MIRCALL_SIZESTATE=1 HAKO_VM_MIRCALL_SIZESTATE_PER_RECV=1 \
verify_mir_rc "$tmp_json" >/dev/null 2>&1
rc=$?
set -e

rm -f "$tmp_json" || true

if [ "$rc" -eq 2 ]; then
  echo "[PASS] hv1_mircall_map_set_size_state_canary_vm"
  exit 0
fi
echo "[FAIL] hv1_mircall_map_set_size_state_canary_vm (rc=$rc, want=2)" >&2
exit 1

