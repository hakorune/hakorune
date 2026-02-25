#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp="/tmp/prog_registry_logical_or_$$.json"
cat >"$tmp" <<'JSON'
{"version":0,"kind":"Program","body":[
 {"type":"Local","name":"a","expr":{"type":"Bool","value":1}},
 {"type":"Local","name":"b","expr":{"type":"Bool","value":0}},
 {"type":"Return","expr":{"type":"Logical","op":"||","lhs":{"type":"Var","name":"a"},"rhs":{"type":"Var","name":"b"}}}
]}
JSON

set +e
HAKO_MIR_BUILDER_INTERNAL=1 HAKO_MIR_BUILDER_REGISTRY=1 HAKO_VERIFY_PRIMARY=core verify_program_via_builder_to_core "$tmp" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp" || true

if [ "$rc" -eq 1 ]; then
  echo "[PASS] mirbuilder_registry_return_logical_or_varvar_core_exec_canary_vm"
  exit 0
fi
echo "[FAIL] mirbuilder_registry_return_logical_or_varvar_core_exec_canary_vm (rc=$rc, expect 1)" >&2; exit 1

