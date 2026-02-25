#!/bin/bash
# map_basic_get_set_vm.sh — Map.set/get with string key prints the value

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

code='static box Main { main() { local m=new MapBox(); m.set("a", 42); print(m.get("a")); return 0 } }'
out=$(run_nyash_vm -c "$code")
if echo "$out" | grep -q "^42$"; then
  echo "[PASS] map_basic_get_set_vm"
else
  echo "[FAIL] map_basic_get_set_vm" >&2; echo "$out" >&2; exit 1
fi

