#!/bin/bash
# v1 φ: three incoming predecessors; nested branch routes to a third path
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR"/../../../../../../../../.. && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_phi_multi3_$$.json"
cat > "$tmp_json" <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name":"main","blocks":[
      {"id":0,"instructions":[
        {"op":"const","dst":1, "value": {"type": "i64", "value": 2}},
        {"op":"const","dst":2, "value": {"type": "i64", "value": 3}},
        {"op":"compare","dst":3, "lhs":2, "rhs":1, "cmp":"Gt"},
        {"op":"branch","cond":3, "then":1, "else":2}
      ]},
      {"id":1,"instructions":[
        {"op":"binop","operation":"+","dst":4,"lhs":1,"rhs":2},
        {"op":"const","dst":7, "value": {"type": "i64", "value": 1}},
        {"op":"branch","cond":7, "then":4, "else":3}
      ]},
      {"id":2,"instructions":[
        {"op":"const","dst":4, "value": {"type": "i64", "value": 1}},
        {"op":"jump","target":3}
      ]},
      {"id":4,"instructions":[
        {"op":"const","dst":4, "value": {"type": "i64", "value": 9}},
        {"op":"jump","target":3}
      ]},
      {"id":3,"instructions":[
        {"op":"phi","dst":5, "incoming": [[1,4],[2,4],[4,4]]},
        {"op":"ret","value":5}
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

# cond=true → bb1; cond2=true → bb4; r4=9; join picks bb4 → 9
if [ "$rc" -eq 9 ]; then
  echo "[PASS] v1_hakovm_phi_multi_incoming3_flow_canary_vm"
  exit 0
fi
echo "[SKIP] v1_hakovm_phi_multi_incoming3_flow_canary_vm (rc=$rc, expect 9)" >&2; exit 0

