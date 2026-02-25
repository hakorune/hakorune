#!/bin/bash
# Parity: MirCallV1Handler used by Mini‑VM and Hakovm dispatcher produce same rc
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_parity_mircall_$$.json"
cat > "$tmp_json" <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name":"main","blocks":[{"id":0,"instructions":[
      {"op":"mir_call","dst":0, "callee":{"type":"Constructor","box_type":"ArrayBox"}, "args":[], "effects":[]},
      {"op":"const","dst":1, "value": {"type": "i64", "value": 10}},
      {"op":"const","dst":2, "value": {"type": "i64", "value": 20}},
      {"op":"mir_call", "callee":{"type":"Method","box_name":"ArrayBox","method":"push","receiver":0}, "args":[1], "effects":[]},
      {"op":"mir_call", "callee":{"type":"Method","box_name":"ArrayBox","method":"push","receiver":0}, "args":[2], "effects":[]},
      {"op":"mir_call","dst":3, "callee":{"type":"Method","box_name":"ArrayBox","method":"size","receiver":0}, "args":[], "effects":[]},
      {"op":"ret","value":3}
    ]}]}
  ]
}
JSON

set +e
# Mini‑VM path
rc_min=0; HAKO_VERIFY_PRIMARY=hakovm HAKO_V1_DISPATCHER_INTERNAL=0 HAKO_V1_DISPATCHER_FLOW=0 verify_mir_rc "$tmp_json" >/dev/null 2>&1; rc_min=$?
# Hakovm internal flow path
rc_hv1=0; HAKO_VERIFY_PRIMARY=hakovm HAKO_V1_DISPATCHER_FLOW=1 verify_mir_rc "$tmp_json" >/dev/null 2>&1; rc_hv1=$?
set -e
rm -f "$tmp_json" || true

if [ "$rc_min" -eq "$rc_hv1" ]; then
  echo "[PASS] v1_mircall_parity_hakovm_minivm_vm (rc=$rc_hv1)"
  exit 0
fi
echo "[FAIL] v1_mircall_parity_hakovm_minivm_vm (mini=$rc_min, hv1=$rc_hv1)" >&2; exit 1

