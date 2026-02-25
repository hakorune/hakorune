#!/bin/bash
# MIR JSON v1 → Core exec canary: Extern env.get(key)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_extern_env_get_$$.json"
export NYASH_TEST_ENV_GET="xyz"

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
            {"op":"const","dst":0, "value": {"type": {"kind":"handle","box_type":"StringBox"}, "value": "NYASH_TEST_ENV_GET"}},
            {"op":"mir_call","dst":1, "callee": {"type":"Extern","name":"env.get"}, "args": [0], "effects": [] },
            {"op":"ret", "value": 1}
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

if [ "$rc" -eq 0 ]; then
  echo "[PASS] v1_extern_env_get_canary_vm"
  exit 0
fi
echo "[FAIL] v1_extern_env_get_canary_vm (rc=$rc, expect 0)" >&2; exit 1
