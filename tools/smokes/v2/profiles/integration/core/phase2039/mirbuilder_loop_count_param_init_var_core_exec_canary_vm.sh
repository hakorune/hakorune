#!/bin/bash
# Loop count_param — init from Local Var → expect rc == 6 (init=2, step=2, limit=6)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/program_loop_count_param_init_var_$$.json"
cat > "$tmp_json" <<'JSON'
{
  "version": 0,
  "kind": "Program",
  "body": [
    { "type":"Local", "name":"initVal", "expr": {"type":"Int","value":2} },
    { "type":"Local", "name":"i", "expr": {"type":"Var","name":"initVal"} },
    { "type":"Loop",
      "cond": {"type":"Compare","op":"<","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":6}},
      "body": [
        { "type":"Local", "name":"i", "expr": {"type":"Binary","op":"+","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":2}} }
      ]
    },
    { "type":"Return", "expr": {"type":"Var","name":"i"} }
  ]
}
JSON

set +e
HAKO_VERIFY_PRIMARY=core verify_mir_rc "$tmp_json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_json" || true

if [ "$rc" -eq 6 ]; then
  echo "[PASS] mirbuilder_loop_count_param_init_var_core_exec_canary_vm"
  exit 0
fi
echo "[FAIL] mirbuilder_loop_count_param_init_var_core_exec_canary_vm (rc=$rc, expect 6)" >&2; exit 1

