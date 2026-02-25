#!/bin/bash
# array_oob_get_tag_vm.sh — Array OOB get emits stable tag under strict

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

code='static box Main { main() {
  local a = [1,2]
  HAKO_OOB_STRICT=1  // note: env used by runtime; program comment
  print(a[5])
  return 0
} }'

output=$(HAKO_OOB_STRICT=1 NYASH_OOB_STRICT=1 run_nyash_vm -c "$code" --dev)

if echo "$output" | grep -q "\[oob/array/get\]"; then
  echo "[PASS] array_oob_get_tag_vm"
  exit 0
else
  echo "[FAIL] array_oob_get_tag_vm" >&2
  echo "--- output ---" >&2
  echo "$output" >&2
  exit 1
fi

