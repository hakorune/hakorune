#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

prog_json_path="/tmp/prog_2044_hako_primary_array_size_$$.json"
cat >"$prog_json_path" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Local","name":"a","expr":{"type":"New","class":"ArrayBox","args":[]}},
  {"type":"Return","expr":{"type":"Method","method":"size","recv":{"type":"Var","name":"a"},"args":[]}}
]}
JSON

set +e
run_verify_program_via_hako_primary_no_fallback_to_core "$prog_json_path"
rc=$?
set -e
rm -f "$prog_json_path"
# size on empty array is 0 (stubbed or real) → rc=0
if [ "$rc" -ne 0 ]; then
  echo "[FAIL] Hako PRIMARY no-fallback array.size → rc=$rc (expected 0)" >&2
  exit 1
fi

echo "[PASS] phase2044/hako_primary_no_fallback_array_size_core_exec_canary_vm"
exit 0
