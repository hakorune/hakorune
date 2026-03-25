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

trap 'rm -f "$tmp" || true' EXIT

run_verify_canary_and_expect_rc \
  run_verify_program_via_registry_internal_to_core \
  "$tmp" \
  0 \
  "mirbuilder_registry_return_logical_and_varvar_core_exec_canary_vm" \
  "mirbuilder_registry_return_logical_and_varvar_core_exec_canary_vm"
