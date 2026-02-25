#!/bin/bash
# PRIMARY hv1 inline: TypeOp cast/as pass-through -> rc=42
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

json='{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[
  {"op":"const","dst":1, "value": {"type": "i64", "value": 42}},
  {"op":"typeop","operation":"cast","src":1,"target_type":"string","dst":2},
  {"op":"ret","value":2}
]}]}]}'

tmp="/tmp/typeop_cast_$$.json"; printf '%s' "$json" > "$tmp"
set +e
rc=$(HAKO_PRIMARY_NO_FALLBACK=1 HAKO_VERIFY_PRIMARY=hakovm verify_v1_inline_file "$tmp" || true)
set -e
rm -f "$tmp"

if [[ "$rc" =~ ^-?[0-9]+$ ]] && [ "$rc" -eq 42 ]; then
  echo "[PASS] typeop_cast_passthrough_primary_canary_vm"
  exit 0
fi
echo "[FAIL] typeop_cast_passthrough_primary_canary_vm (rc=$rc, expect 42)" >&2
exit 1

