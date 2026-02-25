#!/bin/bash
# v1 φ: else-path selected; ensure correct incoming used and value returned.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR"/../../../../../../../../.. && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_phi_else_path_$$.json"
cat > "$tmp_json" <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name":"main","blocks":[
      {"id":0,"instructions":[
        {"op":"const","dst":1, "value": {"type": "i64", "value": 7}},
        {"op":"const","dst":2, "value": {"type": "i64", "value": 9}},
        {"op":"compare","dst":3, "lhs":1, "rhs":2, "cmp":"Gt"},
        {"op":"branch","cond":3, "then":1, "else":2}
      ]},
      {"id":1,"instructions":[
        {"op":"phi","dst":5, "incoming": [[1,0]]},
        {"op":"ret","value":5}
      ]},
      {"id":2,"instructions":[
        {"op":"phi","dst":6, "incoming": [[2,0]]},
        {"op":"ret","value":6}
      ]}
    ]}
  ]
}
JSON

set +e
HAKO_VERIFY_PRIMARY=hakovm HAKO_V1_DISPATCHER_FLOW=1 verify_mir_rc "$tmp_json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_json" || true

# 1 > 2 is false -> else=2; phi at block2 picks [2,0] -> 9
if [ "$rc" -eq 9 ]; then
  echo "[PASS] v1_hakovm_phi_else_path_flow_canary_vm"
  exit 0
fi
echo "[FAIL] v1_hakovm_phi_else_path_flow_canary_vm (rc=$rc, expect 9)" >&2; exit 1
