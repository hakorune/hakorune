#!/bin/bash
# MIR JSON v1 → Core exec canary: ArrayBox push→set→get → rc=updated value
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_array_get_set_update_$$.json"

cat > "$tmp_json" <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    { "name": "main", "blocks": [ { "id": 0, "instructions": [
      {"op":"mir_call","dst":0, "callee": {"type":"Constructor","box_type":"ArrayBox"}, "args": [], "effects": [] },
      {"op":"const","dst":1, "value": {"type": "i64", "value": 10}},
      {"op":"const","dst":2, "value": {"type": "i64", "value": 0}},
      {"op":"const","dst":3, "value": {"type": "i64", "value": 20}},
      {"op":"mir_call","callee": {"type":"Method","box_name":"ArrayBox","method":"push","receiver":0}, "args": [1], "effects": [] },
      {"op":"mir_call","callee": {"type":"Method","box_name":"ArrayBox","method":"set","receiver":0}, "args": [2,3], "effects": [] },
      {"op":"mir_call","dst":4, "callee": {"type":"Method","box_name":"ArrayBox","method":"get","receiver":0}, "args": [2], "effects": [] },
      {"op":"ret", "value": 4}
    ] } ] }
  ]
}
JSON

set +e
HAKO_VERIFY_PRIMARY=core verify_mir_rc "$tmp_json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_json" || true

if [ "$rc" -eq 20 ]; then
  echo "[PASS] v1_array_get_set_update_canary_vm"
  exit 0
fi
echo "[FAIL] v1_array_get_set_update_canary_vm (rc=$rc, expect 20)" >&2; exit 1

