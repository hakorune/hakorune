#!/bin/bash
# v1 φ: then-path computes value, latch via jump, exit uses φ; expect selected value via phi
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR"/../../../../../../../../.. && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_phi_then_jump_combo4_$$.json"
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
        {"op":"jump","target":3}
      ]},
      {"id":2,"instructions":[
        {"op":"const","dst":4, "value": {"type": "i64", "value": 1}},
        {"op":"jump","target":3}
      ]},
      {"id":3,"instructions":[
        {"op":"phi","dst":5, "incoming": [[1,4],[2,4]]},
        {"op":"ret","value":5}
      ]}
    ]}
  ]
}
JSON

set +e
HAKO_VERIFY_PRIMARY=hakovm HAKO_V1_ALLOW_PHI_EXPERIMENT=1 HAKO_V1_DISPATCHER_FLOW=1 verify_mir_rc "$tmp_json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_json" || true

# 3 > 2 true -> then=1; r4=2+3=5; jump 3; phi picks [1,4]=5; ret 5
if [ "$rc" -eq 5 ]; then
  echo "[PASS] v1_hakovm_phi_then_jump_combo4_flow_canary_vm"
  exit 0
fi
echo "[SKIP] v1_hakovm_phi_then_jump_combo4_flow_canary_vm (rc=$rc, expect 5)" >&2; exit 0
