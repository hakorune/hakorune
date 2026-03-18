#!/bin/bash
# array_length_vm.sh — Canary: ArrayBox.length returns correct values

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

code='static box Main {
  main() {
    local a = new ArrayBox()
    print(a.length())
    a.push(1)
    print(a.length())
    a.push(2)
    print(a.length())
    a.push(3)
    print(a.length())
    return 0
  }
}'

output=$(run_nyash_vm -c "$code" --dev)
expected=$'0\n1\n2\n3'

if [ "$output" = "$expected" ]; then
  echo "[PASS] array_length_vm"
  exit 0
else
  echo "[FAIL] array_length_vm" >&2
  echo "--- expected ---" >&2
  echo "$expected" >&2
  echo "--- actual ---" >&2
  echo "$output" >&2
  exit 1
fi
