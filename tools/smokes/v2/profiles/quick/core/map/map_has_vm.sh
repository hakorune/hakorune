#!/bin/bash
# map_has_vm.sh — Map.has positive/negative

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

code='static box Main { main() { local m=new MapBox(); m.set("a",1); print(m.has("a")); print(m.has("z")); return 0 } }'
out=$(run_nyash_vm -c "$code")
first=$(echo "$out" | sed -n '1p')
second=$(echo "$out" | sed -n '2p')
if [ "$first" = "true" ] && [ "$second" = "false" ]; then
  echo "[PASS] map_has_vm"
else
  echo "[FAIL] map_has_vm" >&2; echo "$out" >&2; exit 1
fi
