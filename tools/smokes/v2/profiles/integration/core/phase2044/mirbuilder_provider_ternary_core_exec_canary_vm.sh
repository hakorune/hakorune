#!/usr/bin/env bash
set -euo pipefail
# Purpose: Provider route with Ternary → Core rc=42

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

prog_json_path="/tmp/prog_2044_ternary_$$.json"
cat >"$prog_json_path" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Return","expr":{"type":"Ternary","cond":{"type":"Compare","op":"<","lhs":{"type":"Int","value":1},"rhs":{"type":"Int","value":2}},"then":{"type":"Int","value":42},"else":{"type":"Int","value":0}}}
]}
JSON

trap 'rm -f "$prog_json_path" || true' EXIT
run_preferred_mirbuilder_canary_and_expect_rc \
  "$prog_json_path" \
  42 \
  "provider ternary → core" \
  "phase2044/mirbuilder_provider_ternary_core_exec_canary_vm"
