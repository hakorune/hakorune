#!/bin/bash
# Loop(sum with break/continue) internal → Core exec canary — expect 0+1+3+4 = 8
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
enable_mirbuilder_dev_env

tmp_json="/tmp/program_loop_sum_bc_$$.json"
cat > "$tmp_json" <<'JSON'
{
  "version": 0,
  "kind": "Program",
  "body": [
    { "type":"Local", "name":"i", "expr": {"type":"Int","value":0} },
    { "type":"Local", "name":"s", "expr": {"type":"Int","value":0} },
    { "type":"Loop",
      "cond": {"type":"Compare","op":"<","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":5}},
      "body": [
        { "type":"If", "cond": {"type":"Compare","op":"==","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":2}},
          "then": [ ],
          "else": [ { "type":"Local", "name":"s", "expr": {"type":"Binary","op":"+","lhs":{"type":"Var","name":"s"},"rhs":{"type":"Var","name":"i"}} } ]
        },
        { "type":"Local", "name":"i", "expr": {"type":"Binary","op":"+","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":1}} }
      ]
    },
    { "type":"Return", "expr": {"type":"Var","name":"s"} }
  ]
}
JSON

set +e
HAKO_VERIFY_PRIMARY=core verify_mir_rc "$tmp_json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_json" || true

if [ "$rc" -eq 8 ]; then
  echo "[PASS] mirbuilder_internal_loop_sum_bc_core_exec_canary_vm"
  exit 0
fi
echo "[FAIL] mirbuilder_internal_loop_sum_bc_core_exec_canary_vm (rc=$rc, expect 8)" >&2; exit 1
