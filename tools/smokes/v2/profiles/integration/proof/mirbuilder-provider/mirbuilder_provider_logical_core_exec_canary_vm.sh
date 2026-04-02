#!/usr/bin/env bash
set -euo pipefail
# Purpose: Provider route with Logical → Core rc=1

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

prog_json_path="/tmp/prog_2044_logical_$$.json"
cat >"$prog_json_path" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Return","expr":{"type":"Logical","op":"||","lhs":{"type":"Logical","op":"&&","lhs":{"type":"Bool","value":true},"rhs":{"type":"Bool","value":false}},"rhs":{"type":"Bool","value":true}}}
]}
JSON

trap 'rm -f "$prog_json_path" || true' EXIT
run_preferred_mirbuilder_canary_and_expect_rc \
  "$prog_json_path" \
  1 \
  "provider logical → core" \
  "phase2044/mirbuilder_provider_logical_core_exec_canary_vm"
