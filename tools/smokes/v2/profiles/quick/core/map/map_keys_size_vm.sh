#!/bin/bash
# map_keys_size_vm.sh — Map.keys().size() positive case → expect 2

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

code=$(cat <<'NY'
static box Main {
  main() {
    local m = new MapBox();
    m.set("a", 1); m.set("b", 2);
    local ks = m.keys();
    print(ks.size());
    return 0
  }
}
NY
)
out=$(run_nyash_vm -c "$code")
if echo "$out" | grep -qx "2"; then
  echo "[PASS] map_keys_size_vm"
else
  echo "[FAIL] map_keys_size_vm" >&2; echo "$out" >&2; exit 1
fi

