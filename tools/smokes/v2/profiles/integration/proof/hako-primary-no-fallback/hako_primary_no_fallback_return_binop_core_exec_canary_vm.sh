#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

prog_json_path="/tmp/prog_2044_hako_primary_binop_$$.json"
cat >"$prog_json_path" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Return","expr":{"type":"Binary","op":"+","lhs":{"type":"Int","value":40},"rhs":{"type":"Int","value":2}}}
]}
JSON

trap 'rm -f "$prog_json_path" || true' EXIT
run_hako_primary_no_fallback_canary_and_expect_rc \
  "$prog_json_path" \
  42 \
  "Hako PRIMARY no-fallback return-binop →" \
  "phase2044/hako_primary_no_fallback_return_binop_core_exec_canary_vm"
