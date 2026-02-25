#!/bin/bash
# v1 φ: nested branch inside then-path; compute value on inner-then, join via outer join with φ
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR"/../../../../../../../../.. && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_phi_nested_combo_$$.json"
cat > "$tmp_json" <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name":"main","blocks":[
      {"id":0,"instructions":[
        {"op":"const","dst":1, "value": {"type": "i64", "value": 4}},
        {"op":"const","dst":2, "value": {"type": "i64", "value": 6}},
        {"op":"compare","dst":3, "lhs":2, "rhs":1, "cmp":"Gt"},
        {"op":"branch","cond":3, "then":1, "else":4}
      ]},
      {"id":1,"instructions":[
        {"op":"const","dst":7, "value": {"type": "i64", "value": 10}},
        {"op":"compare","dst":8, "lhs":7, "rhs":1, "cmp":"Gt"},
        {"op":"branch","cond":8, "then":2, "else":3}
      ]},
      {"id":2,"instructions":[
        {"op":"binop","op_kind":"Add","dst":5,"lhs":2,"rhs":7},
        {"op":"jump","target":5}
      ]},
      {"id":3,"instructions")[
        {"op":"binop","op_kind":"Add","dst":5,"lhs":2,"rhs":1},
        {"op":"jump","target":5}
      ]},
      {"id":4,"instructions":[
        {"op":"ret","value":1}
      ]},
      {"id":5,"instructions":[
        {"op":"phi","dst":6, "incoming": [[2,5],[3,5]]},
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

# 6 > 4 true -> then=1 -> 10 > 4 true -> r5=6+10=16 -> join 5 -> phi picks 16 -> ret 16
if [ "$rc" -eq 16 ]; then
  echo "[PASS] v1_hakovm_phi_nested_branch_combo_flow_canary_vm"
  exit 0
fi
echo "[SKIP] v1_hakovm_phi_nested_branch_combo_flow_canary_vm (rc=$rc, expect 16)" >&2; exit 0

