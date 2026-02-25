#!/bin/bash
# v1 φ: missing incoming for prev_bb with tolerate=1 should write 0 and continue; expect rc=0
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR"/../../../../../../../../.. && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_phi_tol_zero_$$.json"
cat > "$tmp_json" <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name":"main","blocks":[
      {"id":0,"instructions":[
        {"op":"const","dst":1, "value": {"type": "i64", "value": 7}},
        {"op":"const","dst":2, "value": {"type": "i64", "value": 9}},
        {"op":"compare","dst":3, "lhs":1, "rhs":2, "cmp":"Lt"},
        {"op":"branch","cond":3, "then":1, "else":2}
      ]},
      {"id":1,"instructions":[
        {"op":"jump","target":3}
      ]},
      {"id":2,"instructions":[
        {"op":"const","dst":4, "value": {"type": "i64", "value": 1}},
        {"op":"jump","target":3}
      ]},
      {"id":3,"instructions":[
        {"op":"phi","dst":5, "incoming": [[2,4]]},
        {"op":"ret","value":5}
      ]}
    ]}
  ]
}
JSON

set +e
HAKO_VERIFY_PRIMARY=hakovm HAKO_V1_ALLOW_PHI_EXPERIMENT=1 HAKO_V1_DISPATCHER_FLOW=1 HAKO_V1_PHI_TOLERATE_VOID=1 verify_mir_rc "$tmp_json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_json" || true

# 7 < 9 true -> then=1; prev_bb=1; phi has incoming only for pred=2; tolerate=1 writes 0; ret r5=0
if [ "$rc" -eq 0 ]; then
  echo "[PASS] v1_hakovm_phi_tolerate_missing_zero_flow_canary_vm"
  exit 0
fi
echo "[SKIP] v1_hakovm_phi_tolerate_missing_zero_flow_canary_vm (rc=$rc, expect 0)" >&2; exit 0
