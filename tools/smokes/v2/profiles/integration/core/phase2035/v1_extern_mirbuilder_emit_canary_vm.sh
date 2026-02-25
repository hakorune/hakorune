#!/bin/bash
# MIR JSON v1 → Core exec canary: extern env.mirbuilder.emit (direct)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_extern_emit_$$.json"

# Minimal Program(JSON v0) payload as a JSON string literal
prog_v0_raw='{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Int","value":13}}]}'
prog_v0_quoted=$(printf '%s' "$prog_v0_raw" | jq -Rs .)

cat > "$tmp_json" <<JSON
{
  "schema_version": "1.0",
  "functions": [
    {
      "name": "main",
      "blocks": [
        {
          "id": 0,
          "instructions": [
            {"op":"const","dst":0, "value": {"type": {"kind":"handle","box_type":"StringBox"}, "value": ${prog_v0_quoted}}},
            {"op":"mir_call","dst":1, "callee": {"type":"Extern","name":"env.mirbuilder.emit"}, "args": [0], "effects": [] },
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

# env.mirbuilder.emit returns a String → rc=0 expected
if [ "$rc" -eq 0 ]; then
  echo "[PASS] v1_extern_mirbuilder_emit_canary_vm"
  exit 0
fi
echo "[FAIL] v1_extern_mirbuilder_emit_canary_vm (rc=$rc, expect 0)" >&2; exit 1
