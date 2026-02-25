#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp="/tmp/prog_registry_logical_and_$$.json"
cat >"$tmp" <<'JSON'
{"version":0,"kind":"Program","body":[
 {"type":"Local","name":"x","expr":{"type":"Bool","value":0}},
 {"type":"Local","name":"y","expr":{"type":"Bool","value":1}},
 {"type":"Return","expr":{"type":"Logical","op":"&&","lhs":{"type":"Var","name":"x"},"rhs":{"type":"Var","name":"y"}}}
]}
JSON

set +e
HAKO_MIR_BUILDER_INTERNAL=1 HAKO_MIR_BUILDER_REGISTRY=1 HAKO_VERIFY_PRIMARY=core verify_program_via_builder_to_core "$tmp" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp" || true

if [ "$rc" -eq 0 ]; then
  echo "[PASS] mirbuilder_registry_return_logical_and_varvar_core_exec_canary_vm"
  exit 0
fi
echo "[FAIL] mirbuilder_registry_return_logical_and_varvar_core_exec_canary_vm (rc=$rc, expect 0)" >&2; exit 1

