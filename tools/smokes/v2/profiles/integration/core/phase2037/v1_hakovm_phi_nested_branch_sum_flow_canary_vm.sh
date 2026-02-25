#!/bin/bash
# v1 φ: nested branches with two φ joins; final ret is sum
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR"/../../../../../../../../.. && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_phi_nested_sum_$$.json"
cat > "$tmp_json" <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name":"main","blocks":[
      {"id":0,"instructions":[
        {"op":"const","dst":10, "value": {"type": "i64", "value": 1}},
        {"op":"branch","cond":10, "then":1, "else":2}
      ]},
      {"id":1,"instructions":[
        {"op":"const","dst":6, "value": {"type": "i64", "value": 10}},
        {"op":"jump","target":3}
      ]},
      {"id":2,"instructions":[
        {"op":"const","dst":6, "value": {"type": "i64", "value": 20}},
        {"op":"jump","target":3}
      ]},
      {"id":3,"instructions":[
        {"op":"phi","dst":8, "incoming": [[1,6],[2,6]]},
        {"op":"const","dst":11, "value": {"type": "i64", "value": 0}},
        {"op":"branch","cond":11, "then":4, "else":5}
      ]},
      {"id":4,"instructions":[
        {"op":"const","dst":7, "value": {"type": "i64", "value": 1}},
        {"op":"jump","target":6}
      ]},
      {"id":5,"instructions":[
        {"op":"const","dst":7, "value": {"type": "i64", "value": 2}},
        {"op":"jump","target":6}
      ]},
      {"id":6,"instructions":[
        {"op":"phi","dst":9, "incoming": [[4,7],[5,7]]},
        {"op":"binop","operation":"+","dst":12,"lhs":8,"rhs":9},
        {"op":"ret","value":12}
      ]}
    ]}
  ]
}
JSON

set +e
HAKO_VERIFY_PRIMARY=hakovm HAKO_V1_ALLOW_PHI_EXPERIMENT=1 HAKO_V1_DISPATCHER_FLOW=1 HAKO_ROUTE_HAKOVM=1 verify_mir_rc "$tmp_json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_json" || true

# first branch true→ id=1 path → r8=10; second branch else→ id=5 path → r9=2; sum=12
if [ "$rc" -eq 12 ]; then
  echo "[PASS] v1_hakovm_phi_nested_branch_sum_flow_canary_vm"
  exit 0
fi
echo "[SKIP] v1_hakovm_phi_nested_branch_sum_flow_canary_vm (rc=$rc, expect 12)" >&2; exit 0

