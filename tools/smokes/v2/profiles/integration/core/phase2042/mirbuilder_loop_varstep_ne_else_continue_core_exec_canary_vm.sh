#!/usr/bin/env bash
# Loop(sum with continue) — step Var(ST=2), Var/Var compare: If(i != M) then add, else Continue → expect 0+2=2
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/program_loop_varstep_ne_else_continue_$$.json"
cat > "$tmp_json" <<'JSON'
{
  "version": 0,
  "kind": "Program",
  "body": [
    { "type":"Local", "name":"N",  "expr": {"type":"Int","value":6} },
    { "type":"Local", "name":"M",  "expr": {"type":"Int","value":4} },
    { "type":"Local", "name":"i",  "expr": {"type":"Int","value":0} },
    { "type":"Local", "name":"s",  "expr": {"type":"Int","value":0} },
    { "type":"Loop",
      "cond": {"type":"Compare","op":"<","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Var","name":"N"}},
      "body": [
        { "type":"If", "cond": {"type":"Compare","op":"!=","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Var","name":"M"}},
          "then": [ { "type":"Local", "name":"s", "expr": {"type":"Binary","op":"+","lhs":{"type":"Var","name":"s"},"rhs":{"type":"Var","name":"i"}} } ],
          "else": [ { "type":"Continue" } ]
        },
        { "type":"Local", "name":"i", "expr": {"type":"Binary","op":"+","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":2}} }
      ]
    },
    { "type":"Return", "expr": {"type":"Var","name":"s"} }
  ]
}
JSON

set +e
HAKO_VERIFY_PRIMARY=core verify_program_via_builder_to_core "$tmp_json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_json" || true

if [ "$rc" -eq 2 ]; then
  echo "[PASS] mirbuilder_loop_varstep_ne_else_continue_core_exec_canary_vm"
  exit 0
fi
echo "[FAIL] mirbuilder_loop_varstep_ne_else_continue_core_exec_canary_vm (rc=$rc, expect 2)" >&2; exit 1
