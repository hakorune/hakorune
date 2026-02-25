#!/bin/bash
# map_delete_has_size_vm.sh — Map.delete then has/size

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
    m.delete("a");
    print(m.has("a"));
    print(m.size());
    return 0
  }
}
NY
)
out=$(run_nyash_vm -c "$code")
first=$(echo "$out" | sed -n '1p')
second=$(echo "$out" | sed -n '2p')
if [ "$first" = "false" ] && [ "$second" = "1" ]; then
  echo "[PASS] map_delete_has_size_vm"
else
  echo "[FAIL] map_delete_has_size_vm" >&2; echo "$out" >&2; exit 1
fi

