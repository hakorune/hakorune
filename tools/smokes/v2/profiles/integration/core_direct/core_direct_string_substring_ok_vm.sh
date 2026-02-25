#!/bin/bash
# core_direct_string_substring_ok_vm.sh — Core Direct: substring positive

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
source "$ROOT/tools/smokes/v2/lib/stageb_helpers.sh"
require_env || exit 2

code='static box Main { method main(args) { local s="abcdef"; local t=s.substring(2,5); print(t); return 0 } }'
json=$(stageb_compile_to_json "$code") || { echo "[FAIL] core_direct_string_substring_ok_vm (emit failed)" >&2; exit 1; }

out=$(NYASH_GATE_C_CORE=1 HAKO_GATE_C_CORE=1 HAKO_CORE_DIRECT=1 \
  NYASH_QUIET=0 HAKO_QUIET=0 NYASH_CLI_VERBOSE=0 \
  "$NYASH_BIN" --json-file "$json" 2>&1)
rm -f "$json"
if echo "$out" | tail -n1 | grep -qx "cde"; then
  echo "[PASS] core_direct_string_substring_ok_vm"
else
  echo "[FAIL] core_direct_string_substring_ok_vm" >&2; echo "$out" >&2; exit 1
fi

