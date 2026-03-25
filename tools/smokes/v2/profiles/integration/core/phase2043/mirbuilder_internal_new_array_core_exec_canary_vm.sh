#!/usr/bin/env bash
# Program(JSON v0) with New(ArrayBox) → MirBuilder(INTERNAL) → MIR(JSON) → Core exec rc=0
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_prog="/tmp/prog_internal_new_array_$$.json"
cat >"$tmp_prog" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Local","name":"a","expr":{"type":"New","class":"ArrayBox","args":[]}},
  {"type":"Return","expr":{"type":"Int","value":0}}
]}
JSON

trap 'rm -f "$tmp_prog" || true' EXIT

run_verify_canary_and_expect_rc \
  run_verify_program_via_internal_builder_no_methods_to_core \
  "$tmp_prog" \
  0 \
  "mirbuilder_internal_new_array_core_exec_canary_vm" \
  "mirbuilder_internal_new_array_core_exec_canary_vm"
