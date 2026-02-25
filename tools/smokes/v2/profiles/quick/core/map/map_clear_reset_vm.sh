#!/bin/bash
# map_clear_reset_vm.sh — Map.clear resets size/has/keys

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
    m.clear();
    print(m.size());
    print(m.has("a"));
    local ks = m.keys();
    print(ks.size());
    return 0
  }
}
NY
)
out=$(run_nyash_vm -c "$code")
sz=$(echo "$out" | sed -n '1p')
has=$(echo "$out" | sed -n '2p')
ksz=$(echo "$out" | sed -n '3p')
if [ "$sz" = "0" ] && [ "$has" = "false" ] && [ "$ksz" = "0" ]; then
  echo "[PASS] map_clear_reset_vm"
else
  echo "[FAIL] map_clear_reset_vm" >&2; echo "$out" >&2; exit 1
fi

