#!/bin/bash
# PRIMARY hv1 inline: TypeOp check bool on 2 -> rc=0
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

json='{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[
  {"op":"const","dst":1, "value": {"type": "i64", "value": 2}},
  {"op":"typeop","operation":"check","src":1,"target_type":"bool","dst":2},
  {"op":"ret","value":2}
]}]}]}'

tmp="/tmp/typeop_bool_false_$$.json"; printf '%s' "$json" > "$tmp"
set +e
rc=$(HAKO_PRIMARY_NO_FALLBACK=1 HAKO_VERIFY_PRIMARY=hakovm verify_v1_inline_file "$tmp" || true)
set -e
rm -f "$tmp"

if [[ "$rc" =~ ^-?[0-9]+$ ]] && [ "$rc" -eq 0 ]; then
  echo "[PASS] typeop_check_bool_false_primary_canary_vm"
  exit 0
fi
echo "[FAIL] typeop_check_bool_false_primary_canary_vm (rc=$rc, expect 0)" >&2
exit 1

