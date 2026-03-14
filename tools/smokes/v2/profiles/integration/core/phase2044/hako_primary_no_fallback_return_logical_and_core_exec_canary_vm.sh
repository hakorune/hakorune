#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

mk_prog() {
  local op="$1"  # '&&' or '||'
  cat <<JSON
{"version":0,"kind":"Program","body":[
  {"type":"Return","expr":{"type":"Logical","op":"$op","lhs":{"type":"Bool","value":true},"rhs":{"type":"Bool","value":false}}}
]}
JSON
}

# case: true || false => 1
tmp2="/tmp/prog_2044_return_logical_or_$$.json"; mk_prog "||" > "$tmp2"
trap 'rm -f "$tmp2" || true' EXIT
run_hako_primary_no_fallback_canary_and_expect_rc \
  "$tmp2" \
  1 \
  "Return(Logical OR)" \
  "phase2044/hako_primary_no_fallback_return_logical_and_core_exec_canary_vm" \
  1
