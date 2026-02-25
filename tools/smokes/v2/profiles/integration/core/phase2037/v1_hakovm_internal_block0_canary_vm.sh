#!/bin/bash
# Hakovm v1 internal dispatcher (block0 only): const→ret == 13
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/mir_v1_hakovm_internal_block0_$$.json"
cat > "$tmp_json" <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name":"main","blocks":[{"id":0,"instructions":[
      {"op":"const","dst":1, "value": {"type": "i64", "value": 13}},
      {"op":"ret","value":1}
    ]}]}
  ]
}
JSON

set +e
HAKO_VERIFY_PRIMARY=hakovm HAKO_V1_DISPATCHER_INTERNAL=1 verify_mir_rc "$tmp_json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_json" || true

if [ "$rc" -eq 13 ]; then
  echo "[PASS] v1_hakovm_internal_block0_canary_vm"
  exit 0
fi
echo "[FAIL] v1_hakovm_internal_block0_canary_vm (rc=$rc, expect 13)" >&2; exit 1

