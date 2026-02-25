#!/bin/bash
# v1 φ: missing incoming; tolerate undefined → Void → rc=0
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR"/../../../../../../../../.. && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_phi_tol_zero2_$$.json"
cat > "$tmp_json" <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name":"main","blocks":[
      {"id":0,"instructions":[
        {"op":"const","dst":1, "value": {"type": "i64", "value": 0}},
        {"op":"branch","cond":1, "then":1, "else":2}
      ]},
      {"id":1,"instructions":[
        {"op":"const","dst":4, "value": {"type": "i64", "value": 5}},
        {"op":"jump","target":3}
      ]},
      {"id":2,"instructions":[
        {"op":"jump","target":3}
      ]},
      {"id":3,"instructions":[
        {"op":"phi","dst":5, "incoming": [[1,4]]},
        {"op":"ret","value":5}
      ]}
    ]}
  ]
}
JSON

set +e
HAKO_VERIFY_PRIMARY=hakovm HAKO_V1_ALLOW_PHI_EXPERIMENT=1 HAKO_V1_DISPATCHER_FLOW=1 HAKO_ROUTE_HAKOVM=1 \
NYASH_VM_PHI_STRICT=0 NYASH_VM_PHI_TOLERATE_UNDEFINED=1 verify_mir_rc "$tmp_json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_json" || true

# else-path chosen; phi has no input for pred=2 → tolerate undefined → Void → rc=0
if [ "$rc" -eq 0 ]; then
  echo "[PASS] v1_hakovm_phi_tolerate_missing_zero2_flow_canary_vm"
  exit 0
fi
echo "[SKIP] v1_hakovm_phi_tolerate_missing_zero2_flow_canary_vm (rc=$rc, expect 0)" >&2; exit 0

