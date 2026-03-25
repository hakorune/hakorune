#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_prog="/tmp/prog_registry_binop_intint_$$.json"
cat >"$tmp_prog" <<'JSON'
{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Binary","op":"+","lhs":{"type":"Int","value":2},"rhs":{"type":"Int","value":3}}}]}
JSON

trap 'rm -f "$tmp_prog" || true' EXIT

run_verify_canary_and_expect_rc \
  run_verify_program_via_registry_internal_to_core \
  "$tmp_prog" \
  5 \
  "mirbuilder_registry_return_binop_intint_core_exec_canary_vm" \
  "mirbuilder_registry_return_binop_intint_core_exec_canary_vm"
