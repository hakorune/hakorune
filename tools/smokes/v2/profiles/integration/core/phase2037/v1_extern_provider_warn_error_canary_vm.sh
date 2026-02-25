#!/bin/bash
# Hako extern provider minimal: warn/error (print equivalent)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_ext_warn_error_$$.json"
cat > "$tmp_json" <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name":"main","blocks":[{"id":0,"instructions":[
      {"op":"const","dst":0, "value": {"type": {"kind":"handle","box_type":"StringBox"}, "value": "hello"}},
      {"op":"mir_call", "callee": {"type":"Extern","name":"env.console.warn"}, "args": [0], "effects": [] },
      {"op":"mir_call", "callee": {"type":"Extern","name":"env.console.error"}, "args": [0], "effects": [] },
      {"op":"ret","value":0}
    ]}]}
  ]
}
JSON

set +e
HAKO_VERIFY_PRIMARY=hakovm HAKO_V1_DISPATCHER_FLOW=1 HAKO_V1_EXTERN_PROVIDER=1 verify_mir_rc "$tmp_json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_json" || true

if [ "$rc" -eq 0 ]; then
  echo "[PASS] v1_extern_provider_warn_error_canary_vm"
  exit 0
fi
echo "[FAIL] v1_extern_provider_warn_error_canary_vm (rc=$rc, expect 0)" >&2; exit 1

