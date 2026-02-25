#!/bin/bash
# index_substring_vm.sh — String.indexOf/substring boundary behavior

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
  local s = "hello"
  print("" + s.indexOf("l"))    // 2
  print("" + s.indexOf("z"))    // -1
  print(s.substring(0, 2))       // he
  print(s.substring(2, 5))       // llo
  print("" + s.substring(5, 5).length())  // 0
  return 0
} }'

output=$(run_nyash_vm -c "$code" --dev)
expected=$'2\n-1\nhe\nllo\n0'

if [ "$output" = "$expected" ]; then
  echo "[PASS] string_index_substring_vm"
  exit 0
else
  echo "[FAIL] string_index_substring_vm" >&2
  echo "--- expected ---" >&2
  echo "$expected" >&2
  echo "--- actual ---" >&2
  echo "$output" >&2
  exit 1
fi
