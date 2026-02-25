#!/bin/bash
# Hako extern provider minimal: env.mirbuilder.emit stub returns empty string
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_ext_emit_stub_$$.json"
cat > "$tmp_json" <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name":"main","blocks":[{"id":0,"instructions":[
      {"op":"const","dst":0, "value": {"type": {"kind":"handle","box_type":"StringBox"}, "value": "x"}},
      {"op":"mir_call","dst":1, "callee": {"type":"Extern","name":"env.mirbuilder.emit"}, "args": [0], "effects": [] },
      {"op":"const","dst":9, "value": {"type": "i64", "value": 0}},
      {"op":"ret","value":9}
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
  echo "[PASS] v1_extern_provider_emit_stub_canary_vm"
  exit 0
fi
echo "[FAIL] v1_extern_provider_emit_stub_canary_vm (rc=$rc, expect 0)" >&2; exit 1
