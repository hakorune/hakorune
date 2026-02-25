#!/bin/bash
# If (<=) int-int — PRIMARY hv1 inline rc=1
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

json='{"schema_version":"1.0","functions":[{"name":"main","blocks":[
  {"id":0,"instructions":[
    {"op":"const","dst":1,"value":{"type":"i64","value":3}},
    {"op":"const","dst":2,"value":{"type":"i64","value":3}},
    {"op":"compare","dst":3,"lhs":1,"rhs":2,"cmp":"Le"},
    {"op":"branch","cond":3,"then":1,"else":2}
  ]},
  {"id":1,"instructions":[
    {"op":"const","dst":10,"value":{"type":"i64","value":1}},
    {"op":"ret","value":10}
  ]},
  {"id":2,"instructions":[
    {"op":"const","dst":11,"value":{"type":"i64","value":0}},
    {"op":"ret","value":11}
  ]}
]}]}'

tmp="/tmp/if_le_ii_$$.json"; printf '%s' "$json" > "$tmp"
set +e
rc=$(HAKO_PRIMARY_NO_FALLBACK=1 HAKO_VERIFY_PRIMARY=hakovm verify_v1_inline_file "$tmp" || true)
set -e
rm -f "$tmp"

if [[ "$rc" =~ ^-?[0-9]+$ ]] && [ "$rc" -eq 1 ]; then
  echo "[PASS] if_le_intint_primary_canary_vm"
  exit 0
fi
echo "[FAIL] if_le_intint_primary_canary_vm (rc=$rc, expect 1)" >&2
exit 1
