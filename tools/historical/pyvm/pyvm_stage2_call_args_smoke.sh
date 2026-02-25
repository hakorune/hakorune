#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/../../.." && pwd)
source "$SCRIPT_DIR/common.sh"

TMP_DIR="$ROOT_DIR/tmp"
mkdir -p "$TMP_DIR"

pass() { echo "✅ $1" >&2; }
fail() { echo "❌ $1" >&2; echo "$2" | sed -n '1,160p' >&2; exit 1; }

run_exit_code() {
  local src="$1"; local f="$TMP_DIR/stage2_call_args_tmp.ny"
  printf '%s\n' "$src" > "$f"
  local code
  set +e
  pyvm_run_source_capture "$f" >/dev/null 2>&1
  code=$?
  set -e
  echo "$code"
}

# Nested args: substring with expression argument
SRC1=$'static box Main {\n  main(args){\n    return ("abcdef").substring(1, 1+2).length()\n  }\n}'
CODE=$(run_exit_code "$SRC1")
[[ "$CODE" -eq 2 ]] && pass "call args: substring(1,1+2).length -> 2" || fail "call args: nested expr arg" "__EXIT_CODE__=$CODE"

# Nested chain with nested calls in args (single line)
SRC2=$'static box Main {\n  main(args){\n    return ("abcdef").substring(1, 1+3).substring(0,2).length()\n  }\n}'
CODE=$(run_exit_code "$SRC2")
[[ "$CODE" -eq 2 ]] && pass "call args: nested calls and expr args -> 2" || fail "call args: nested chain" "__EXIT_CODE__=$CODE"

echo "All Stage-2 call/args smokes (PyVM) PASS" >&2
