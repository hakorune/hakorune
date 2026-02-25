#!/bin/bash
# v1 φ: missing incoming (strict). Expect rc=1 (error) and treat as PASS.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR"/../../../../../../../../.. && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_phi_missing_strict_$$.json"
cat > "$tmp_json" <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name":"main","blocks":[
      {"id":0,"instructions":[
        {"op":"const","dst":1, "value": {"type": "i64", "value": 7}},
        {"op":"const","dst":2, "value": {"type": "i64", "value": 9}},
        {"op":"compare","dst":3, "lhs":2, "rhs":1, "cmp":"Lt"},
        {"op":"branch","cond":3, "then":1, "else":2}
      ]},
      {"id":1,"instructions":[
        {"op":"phi","dst":5, "incoming": [[2,0]]},
        {"op":"ret","value":5}
      ]},
      {"id":2,"instructions":[
        {"op":"phi","dst":6, "incoming": [[1,1]]},
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

if [ "$rc" -eq 1 ]; then
  echo "[PASS] v1_hakovm_phi_missing_strict_flow_canary_vm"
  exit 0
fi
echo "[FAIL] v1_hakovm_phi_missing_strict_flow_canary_vm (rc=$rc, expect 1)" >&2; exit 1

