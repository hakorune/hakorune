#!/bin/bash
# map_bad_key_get_vm.sh — Map.get with non-string key returns [map/bad-key]

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

code='static box Main { main() { local m=new MapBox(); print(m.get(123)); return 0 } }'
out=$(run_nyash_vm -c "$code")
if echo "$out" | grep -q "\[map/bad-key\]"; then
  echo "[PASS] map_bad_key_get_vm"
else
  echo "[FAIL] map_bad_key_get_vm" >&2; echo "$out" >&2; exit 1
fi

