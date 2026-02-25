#!/bin/bash
# hv1 inline: copy passthrough -> rc=7
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

json='{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[
  {"op":"const","dst":1, "value": {"type": "i64", "value": 7}},
  {"op":"copy","src":1,"dst":2},
  {"op":"ret","value":2}
]}]}]}'

tmp="/tmp/hv1_copy_$$.json"; printf '%s' "$json" > "$tmp"
set +e
rc=$(HAKO_PRIMARY_NO_FALLBACK=1 HAKO_VERIFY_PRIMARY=hakovm verify_v1_inline_file "$tmp" || true)
set -e
rm -f "$tmp"

if [[ "$rc" =~ ^-?[0-9]+$ ]] && [ "$rc" -eq 7 ]; then
  echo "[PASS] hv1_inline_copy_passthrough_rc7_canary_vm"
  exit 0
fi
echo "[FAIL] hv1_inline_copy_passthrough_rc7_canary_vm (rc=$rc, expect 7)" >&2
exit 1

