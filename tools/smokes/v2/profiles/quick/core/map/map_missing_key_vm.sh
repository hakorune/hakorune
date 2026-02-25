#!/bin/bash
# map_missing_key_vm.sh — MapBox.get on missing key returns stable string

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
  local m = new MapBox()
  print(m.get("nope"))
  return 0
} }'

output=$(run_nyash_vm -c "$code" --dev)
if echo "$output" | grep -q "Key not found: nope"; then
  echo "[PASS] map_missing_key_vm"
  exit 0
else
  echo "[FAIL] map_missing_key_vm" >&2
  echo "--- output ---" >&2
  echo "$output" >&2
  exit 1
fi

