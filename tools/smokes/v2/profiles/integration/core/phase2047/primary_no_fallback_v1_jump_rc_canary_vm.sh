#!/bin/bash
# PRIMARY no-fallback: hv1 inline executes v1 jump → rc=7
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

json='{"schema_version":"1.0","functions":[{"name":"main","blocks":[
  {"id":0,"instructions":[{"op":"jump","target":2}]},
  {"id":1,"instructions":[{"op":"ret","value":5}]},
  {"id":2,"instructions":[
    {"op":"const","dst":6,"value":{"type":"i64","value":7}},
    {"op":"ret","value":6}
  ]}
]}]}'

tmp="/tmp/primary_jump_$$.json"; printf '%s' "$json" > "$tmp"
set +e
rc=$(HAKO_PRIMARY_NO_FALLBACK=1 HAKO_VERIFY_PRIMARY=hakovm verify_v1_inline_file "$tmp" || true)
set -e
rm -f "$tmp"

if [[ "$rc" =~ ^-?[0-9]+$ ]] && [ "$rc" -eq 7 ]; then
  echo "[PASS] primary_no_fallback_v1_jump_rc_canary_vm"
  exit 0
fi
echo "[FAIL] primary_no_fallback_v1_jump_rc_canary_vm (rc=$rc, expect 7)" >&2
exit 1

