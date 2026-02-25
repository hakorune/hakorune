#!/bin/bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR"/../../../../../../../../.. && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

# per_recv=1: per-receiver length key. push on A then size on B → 0
tmp_json="/tmp/mir_v1_perrecv_per_$$.json"
cat > "$tmp_json" <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name":"main","blocks":[
      {"id":0,"instructions":[
        {"op":"mir_call", "dst": 1, "mir_call": {"callee": {"type":"Constructor","box_type":"ArrayBox"}, "args": [], "effects": []}},
        {"op":"const",   "dst": 2, "value": {"type": "i64", "value": 0}},
        {"op":"mir_call",          "mir_call": {"callee": {"type":"Method","box_name":"ArrayBox","method":"push","receiver":1}, "args": [2], "effects": []}},
        {"op":"mir_call", "dst": 4, "mir_call": {"callee": {"type":"Constructor","box_type":"ArrayBox"}, "args": [], "effects": []}},
        {"op":"mir_call", "dst": 5, "mir_call": {"callee": {"type":"Method","box_name":"ArrayBox","method":"size","receiver":4}, "args": [], "effects": []}},
        {"op":"ret","value":5}
      ]}
    ]}
  ]
}
JSON

set +e
HAKO_VERIFY_PRIMARY=hakovm HAKO_VM_MIRCALL_SIZESTATE=1 HAKO_VM_MIRCALL_SIZESTATE_PER_RECV=1 verify_mir_rc "$tmp_json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_json" || true

if [ "$rc" -eq 0 ]; then echo "[PASS] per_recv_per_canary_vm"; exit 0; fi
echo "[FAIL] per_recv_per_canary_vm (rc=$rc, want=0)" >&2; exit 1

