#!/bin/bash
# Loop '!=' sentry (continue side) safe subset → rc=4（PRIMARY hv1 inline）
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

json='{"schema_version":"1.0","functions":[{"name":"main","blocks":[
  {"id":0,"instructions":[
    {"op":"const","dst":1,"value":{"type":"i64","value":1}},
    {"op":"const","dst":2,"value":{"type":"i64","value":2}},
    {"op":"compare","dst":3,"lhs":1,"rhs":2,"cmp":"Ne"},
    {"op":"branch","cond":3,"then":1,"else":2}
  ]},
  {"id":1,"instructions":[
    {"op":"const","dst":4,"value":{"type":"i64","value":4}},
    {"op":"ret","value":4}
  ]},
  {"id":2,"instructions":[
    {"op":"const","dst":5,"value":{"type":"i64","value":0}},
    {"op":"ret","value":5}
  ]}
]}]}'

tmp="/tmp/loop_ne_cont_$$.json"; printf '%s' "$json" > "$tmp"
set +e
rc=$(HAKO_PRIMARY_NO_FALLBACK=1 HAKO_VERIFY_PRIMARY=hakovm verify_v1_inline_file "$tmp" || true)
set -e
rm -f "$tmp"

if [[ "$rc" =~ ^-?[0-9]+$ ]] && [ "$rc" -eq 4 ]; then
  echo "[PASS] loop_ne_sentry_continue_rc_canary_vm"
  exit 0
fi
echo "[FAIL] loop_ne_sentry_continue_rc_canary_vm (rc=$rc, expect 4)" >&2
exit 1

