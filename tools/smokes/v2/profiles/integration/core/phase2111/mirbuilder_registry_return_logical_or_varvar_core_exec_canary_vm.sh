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

trap 'rm -f "$tmp" || true' EXIT

run_verify_canary_and_expect_rc \
  run_verify_program_via_registry_internal_to_core \
  "$tmp" \
  1 \
  "mirbuilder_registry_return_logical_or_varvar_core_exec_canary_vm" \
  "mirbuilder_registry_return_logical_or_varvar_core_exec_canary_vm"
