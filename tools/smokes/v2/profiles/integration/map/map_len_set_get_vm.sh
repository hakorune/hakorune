#!/bin/bash
# map_len_set_get_vm.sh — MapBox len/set/get sequence sanity

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
  print("" + m.size())
  m.set("a", 1)
  print("" + m.size())
  m.set("b", 2)
  print("" + m.size())
  print("" + m.get("a"))
  print("" + m.get("b"))
  return 0
} }'

output=$(run_nyash_vm -c "$code" --dev)
expected=$'0\n1\n2\n1\n2'

if [ "$output" = "$expected" ]; then
  echo "[PASS] map_len_set_get_vm"
  exit 0
else
  echo "[FAIL] map_len_set_get_vm" >&2
  echo "--- expected ---" >&2
  echo "$expected" >&2
  echo "--- actual ---" >&2
  echo "$output" >&2
  exit 1
fi

