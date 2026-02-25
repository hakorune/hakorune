#!/bin/bash
# Hakovm v1 dispatcher: ArrayBox push → size state increments (1→2→3)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR"/../../../../../../../../.. && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_array_push_size_$$.json"
cat > "$tmp_json" <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name":"main","blocks":[
      {"id":0,"instructions":[
        {"op":"mir_call", "dst": 1, "mir_call": {"callee": {"type":"Constructor","box_type":"ArrayBox"}, "args": [], "effects": []}},
        {"op":"const",   "dst": 2, "value": {"type": "i64", "value": 111}},
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

# Expect rc=3 when stateful mir_call is ON and primary=hakovm
set +e
HAKO_VERIFY_PRIMARY=hakovm \
HAKO_VM_MIRCALL_SIZESTATE=1 HAKO_VM_MIRCALL_SIZESTATE_PER_RECV=1 \
verify_mir_rc "$tmp_json" >/dev/null 2>&1
rc=$?
set -e

rm -f "$tmp_json" || true

if [ "$rc" -eq 3 ]; then
  echo "[PASS] hv1_mircall_array_push_size_state_canary_vm"
  exit 0
fi
echo "[FAIL] hv1_mircall_array_push_size_state_canary_vm (rc=$rc, want=3)" >&2
exit 1

