#!/bin/bash
# Match lowering via JSON v0 bridge (SSOT cf_common) → expect rc=2
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/program_match_$$.json"
cat > "$tmp_json" <<'JSON'
{
  "version": 0,
  "kind": "Program",
  "body": [
    { "type":"Local", "name":"s", "expr": {"type":"Str","value":"b"} },
    { "type":"Local", "name":"x", "expr": {"type":"Match","scrutinee":{"type":"Var","name":"s"},
        "arms":[
          {"label":"a","expr":{"type":"Int","value":1}},
          {"label":"b","expr":{"type":"Int","value":2}}
        ],
        "else":{"type":"Int","value":9}
    } },
    { "type":"Return", "expr": {"type":"Var","name":"x"} }
  ]
}
JSON

set +e
HAKO_VERIFY_PRIMARY=core verify_mir_rc "$tmp_json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_json" || true

if [ "$rc" -eq 2 ]; then
  echo "[PASS] mirbuilder_match_core_exec_canary_vm"
  exit 0
fi
echo "[FAIL] mirbuilder_match_core_exec_canary_vm (rc=$rc, expect 2)" >&2; exit 1

