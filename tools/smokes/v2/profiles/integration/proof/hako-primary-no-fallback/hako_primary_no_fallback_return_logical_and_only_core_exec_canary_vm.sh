#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

prog_json_path="/tmp/prog_2044_return_logical_and_only_$$.json"
cat >"$prog_json_path" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Return","expr":{"type":"Logical","op":"&&","lhs":{"type":"Bool","value":true},"rhs":{"type":"Bool","value":false}}}
]}
JSON

trap 'rm -f "$prog_json_path" || true' EXIT
run_hako_primary_no_fallback_canary_and_expect_rc \
  "$prog_json_path" \
  0 \
  "Return(Logical AND only)" \
  "phase2044/hako_primary_no_fallback_return_logical_and_only_core_exec_canary_vm"
