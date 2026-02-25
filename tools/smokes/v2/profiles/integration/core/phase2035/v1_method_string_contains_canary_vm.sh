#!/bin/bash
# MIR JSON v1 → Core exec canary: StringBox.contains(search)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_string_contains_$$.json"

cat > "$tmp_json" <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {
      "name": "main",
      "blocks": [
        {
          "id": 0,
          "instructions": [
            {"op":"const","dst":0, "value": {"type": {"kind":"handle","box_type":"StringBox"}, "value": "hello world"}},
            {"op":"const","dst":1, "value": {"type": {"kind":"handle","box_type":"StringBox"}, "value": "world"}},
            {"op":"mir_call","dst":2, "callee": {"type":"Method","box_name":"StringBox","method":"contains","receiver":0}, "args": [1], "effects": [] },
            {"op":"ret", "value": 2}
          ]
        }
      ]
    }
  ]
}
JSON

set +e
HAKO_VERIFY_PRIMARY=core verify_mir_rc "$tmp_json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_json" || true

if [ "$rc" -eq 1 ]; then
  echo "[PASS] v1_method_string_contains_canary_vm"
  exit 0
fi
echo "[FAIL] v1_method_string_contains_canary_vm (rc=$rc, expect 1)" >&2; exit 1

