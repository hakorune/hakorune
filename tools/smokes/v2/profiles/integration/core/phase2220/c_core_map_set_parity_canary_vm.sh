#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env >/dev/null || exit 2

# Build with c-core feature to make probe callable
(cd "$NYASH_ROOT" && cargo build -q --release -p nyash-rust --features c-core >/dev/null)

CODE='
static box Main {
  main() {
    local m = new MapBox()
    m.set("k", "v")
    print(m.size())
    m.set("k", "v2")
    print(m.size())
    print(m.get("k"))
    return 0
  }
}
'

out0=$(HAKO_C_CORE_ENABLE=0 run_nyash_vm -c "$CODE" 2>&1)
out1=$(HAKO_C_CORE_ENABLE=1 HAKO_C_CORE_TARGETS=MapBox.set run_nyash_vm -c "$CODE" 2>&1)

if [ "$out0" = "$out1" ] && echo "$out1" | grep -q '^1$' && echo "$out1" | grep -q '^v2$'; then
  echo "[PASS] c_core_map_set_parity_canary_vm"
  exit 0
fi
echo "[FAIL] c_core_map_set_parity_canary_vm" >&2
echo "--- off ---" >&2; echo "$out0" >&2
echo "---  on ---" >&2; echo "$out1" >&2
exit 1

