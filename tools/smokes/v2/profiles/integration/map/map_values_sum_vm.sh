#!/bin/bash
# map_values_sum_vm.sh — Sum Map.values via loop → expect 3

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
    local vals = m.values();
    local n = vals.size();
    local i = 0; local s = 0;
    loop(i < n) {
      local v = vals.get(i);
      if v != null { s = s + v }
      i = i + 1
    }
    print(s);
    return 0
  }
}
NY
)
out=$(run_nyash_vm -c "$code")
if echo "$out" | grep -qx "3"; then
  echo "[PASS] map_values_sum_vm"
else
  echo "[FAIL] map_values_sum_vm" >&2; echo "$out" >&2; exit 1
fi
