#!/bin/bash
# PRIMARY hv1-inline: phi selects by pred id when incoming regs differ -> rc=99 (then path)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

json='{"schema_version":"1.0","functions":[{"name":"main","blocks":[
 {"id":0,"instructions":[
   {"op":"const","dst":1,"value":{"type":"i64","value":1}},
   {"op":"const","dst":2,"value":{"type":"i64","value":0}},
   {"op":"compare","dst":3,"lhs":1,"rhs":2,"cmp":"Gt"},
   {"op":"branch","cond":3,"then":1,"else":2}
 ]},
 {"id":1,"instructions":[
   {"op":"const","dst":6,"value":{"type":"i64","value":99}},
   {"op":"jump","target":3}
 ]},
 {"id":2,"instructions":[
   {"op":"const","dst":4,"value":{"type":"i64","value":40}},
   {"op":"jump","target":3}
 ]},
 {"id":3,"instructions":[
   {"op":"phi","dst":5,"incoming":[[6,1],[4,2]]},
   {"op":"ret","value":5}
 ]}
]}]}'

tmp="/tmp/phi2_sel_pred_$$.json"; printf '%s' "$json" > "$tmp"
set +e
rc=$(HAKO_PRIMARY_NO_FALLBACK=1 HAKO_VERIFY_PRIMARY=hakovm verify_v1_inline_file "$tmp" || true)
set -e
rm -f "$tmp"

if [[ "$rc" =~ ^-?[0-9]+$ ]] && [ "$rc" -eq 99 ]; then
  echo "[PASS] flow_phi2_select_by_pred_rc99_primary_canary_vm"
  exit 0
fi
echo "[FAIL] flow_phi2_select_by_pred_rc99_primary_canary_vm (rc=$rc, expect 99)" >&2
exit 1

