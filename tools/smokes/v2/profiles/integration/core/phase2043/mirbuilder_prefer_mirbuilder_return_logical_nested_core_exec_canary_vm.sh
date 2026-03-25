#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_prog="/tmp/prog_prefer_mirbuilder_return_logical_nested_$$.json"
cat >"$tmp_prog" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Local","name":"b1","expr":{"type":"Bool","value":true}},
  {"type":"Local","name":"b2","expr":{"type":"Bool","value":false}},
  {"type":"Local","name":"b3","expr":{"type":"Bool","value":true}},
  {"type":"Return","expr":{"type":"Logical","op":"&&",
    "lhs":{"type":"Var","name":"b1"},
    "rhs":{"type":"Logical","op":"||",
      "lhs":{"type":"Var","name":"b2"},
      "rhs":{"type":"Var","name":"b3"}
    }
  }}
]}
JSON

trap 'rm -f "$tmp_prog" || true' EXIT

run_verify_canary_and_expect_rc \
  run_verify_program_via_preferred_mirbuilder_core_to_core \
  "$tmp_prog" \
  1 \
  "mirbuilder_prefer_mirbuilder_return_logical_nested_core_exec_canary_vm" \
  "mirbuilder_prefer_mirbuilder_return_logical_nested_core_exec_canary_vm"
