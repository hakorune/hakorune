#!/usr/bin/env bash
set -euo pipefail
# Purpose: Program uses method "length" on Array; generator should standardize to size and rc=2

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

prog_json_path="/tmp/prog_2044_arr_len_alias_$$.json"
cat >"$prog_json_path" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Local","name":"a","expr":{"type":"New","class":"ArrayBox","args":[]}},
  {"type":"Expr","expr":{"type":"Method","recv":{"type":"Var","name":"a"},"method":"push","args":[{"type":"Int","value":10}]}},
  {"type":"Expr","expr":{"type":"Method","recv":{"type":"Var","name":"a"},"method":"push","args":[{"type":"Int","value":32}]}},
  {"type":"Return","expr":{"type":"Method","recv":{"type":"Var","name":"a"},"method":"length","args":[]}}
]}
JSON

set +e
run_verify_program_via_preferred_mirbuilder_to_core "$prog_json_path"
rc=$?
set -e
rm -f "$prog_json_path"
if [ "$rc" -ne 2 ]; then
  echo "[FAIL] array length alias → rc=$rc (expected 2)" >&2
  exit 1
fi

echo "[PASS] phase2044/mirbuilder_provider_array_length_alias_core_exec_canary_vm"
exit 0
