#!/bin/bash
# array_oob_negative_get_tag_vm.sh — Array negative OOB get emits stable tag under strict

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
  local a = new ArrayBox()
  a.push(1)
  a.push(2)
  local idx = 0 - 1
  print(a.get(idx))
  return 0
} }'

set +e
output=$(HAKO_OOB_STRICT=1 NYASH_OOB_STRICT=1 \
  NYASH_JOINIR_DEV=0 HAKO_JOINIR_STRICT=0 \
  run_nyash_vm -c "$code" --dev)
rc=$?
set -e

if echo "$output" | grep -q "\[oob/array/get\]"; then
  echo "[PASS] array_oob_negative_get_tag_vm"
  exit 0
fi

echo "[FAIL] array_oob_negative_get_tag_vm" >&2
echo "rc=$rc" >&2
echo "--- output ---" >&2
echo "$output" >&2
exit 1
