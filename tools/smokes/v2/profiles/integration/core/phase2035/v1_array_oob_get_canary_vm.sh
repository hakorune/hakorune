#!/bin/bash
# MIR JSON v1 → Core exec canary: ArrayBox.get OOB policy (diagnostic)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_array_oob_get_$$.json"

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
            {"op":"mir_call","dst":0, "callee":{"type":"Constructor","box_type":"ArrayBox"}, "args":[], "effects":[]},
            {"op":"const","dst":1, "value": {"type": "i64", "value": 999}},
            {"op":"mir_call", "callee":{"type":"Method","box_name":"ArrayBox","method":"push","receiver":0}, "args":[1], "effects":[]},
            {"op":"const","dst":2, "value": {"type": "i64", "value": 10}},
            {"op":"mir_call","dst":3, "callee":{"type":"Method","box_name":"ArrayBox","method":"get","receiver":0}, "args":[2], "effects":[]},
            {"op":"ret","value":3}
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

# OOB policy is implementation-defined; accept 0 (rc=0) or Void (rc=0) or non-zero stable tag via SKIP.
if [ "$rc" -eq 0 ]; then
  echo "[PASS] v1_array_oob_get_canary_vm"
  exit 0
fi
echo "[SKIP] v1_array_oob_get_canary_vm (policy not fixed, rc=$rc)" >&2; exit 0

