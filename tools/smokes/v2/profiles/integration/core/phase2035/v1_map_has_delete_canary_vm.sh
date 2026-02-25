#!/bin/bash
# MIR JSON v1 → Core exec canary: MapBox set→has (deleteは構造確認のみ)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_map_has_delete_$$.json"

cat > "$tmp_json" <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    { "name": "main", "blocks": [ { "id": 0, "instructions": [
      {"op":"mir_call","dst":0, "callee": {"type":"Constructor","box_type":"MapBox"}, "args": [], "effects": [] },
      {"op":"const","dst":1, "value": {"type": {"kind":"handle","box_type":"StringBox"}, "value": "k1"}},
      {"op":"const","dst":2, "value": {"type": "i64", "value": 42}},
      {"op":"mir_call","callee": {"type":"Method","box_name":"MapBox","method":"set","receiver":0}, "args": [1,2], "effects": [] },
      {"op":"mir_call","dst":3, "callee": {"type":"Method","box_name":"MapBox","method":"has","receiver":0}, "args": [1], "effects": [] },
      {"op":"ret", "value": 3}
    ] } ] }
  ]
}
JSON

set +e
HAKO_VERIFY_PRIMARY=core verify_mir_rc "$tmp_json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_json" || true

if [ "$rc" -eq 1 ]; then
  echo "[PASS] v1_map_has_delete_canary_vm"
  exit 0
fi
echo "[FAIL] v1_map_has_delete_canary_vm (rc=$rc, expect 1)" >&2; exit 1

