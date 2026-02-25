#!/bin/bash
# Logical nested: ( (1<2) && (3<2) ) || (4<5 ) → rc=1 (PRIMARY hv1 inline)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

json='{"schema_version":"1.0","functions":[{"name":"main","blocks":[
  {"id":0,"instructions":[
    {"op":"const","dst":1,"value":{"type":"i64","value":1}},
    {"op":"const","dst":2,"value":{"type":"i64","value":2}},
    {"op":"compare","dst":3,"lhs":1,"rhs":2,"cmp":"Lt"},
    {"op":"const","dst":4,"value":{"type":"i64","value":3}},
    {"op":"const","dst":5,"value":{"type":"i64","value":2}},
    {"op":"compare","dst":6,"lhs":4,"rhs":5,"cmp":"Lt"},
    {"op":"const","dst":7,"value":{"type":"i64","value":4}},
    {"op":"const","dst":8,"value":{"type":"i64","value":5}},
    {"op":"compare","dst":9,"lhs":7,"rhs":8,"cmp":"Lt"},
    {"op":"branch","cond":9,"then":10,"else":11}
  ]},
  {"id":10,"instructions":[{"op":"const","dst":20,"value":{"type":"i64","value":1}},{"op":"ret","value":20}]},
  {"id":11,"instructions":[{"op":"branch","cond":3,"then":12,"else":13}]},
  {"id":12,"instructions":[{"op":"branch","cond":6,"then":14,"else":13}]},
  {"id":14,"instructions":[{"op":"const","dst":21,"value":{"type":"i64","value":1}},{"op":"ret","value":21}]},
  {"id":13,"instructions":[{"op":"const","dst":22,"value":{"type":"i64","value":0}},{"op":"ret","value":22}]}
]}]}'

tmp="/tmp/logical_nested_$$.json"; printf '%s' "$json" > "$tmp"
set +e
rc=$(HAKO_PRIMARY_NO_FALLBACK=1 HAKO_VERIFY_PRIMARY=hakovm verify_v1_inline_file "$tmp" || true)
set -e
rm -f "$tmp"

if [[ "$rc" =~ ^-?[0-9]+$ ]] && [ "$rc" -eq 1 ]; then
  echo "[PASS] logical_nested_or_and_primary_canary_vm"
  exit 0
fi
echo "[FAIL] logical_nested_or_and_primary_canary_vm (rc=$rc, expect 1)" >&2
exit 1

