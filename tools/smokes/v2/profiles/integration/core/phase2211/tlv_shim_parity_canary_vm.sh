#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env >/dev/null || exit 2

# Build nyash with tlv-shim feature (even if we run OFF for parity)
( cd "$NYASH_ROOT" && cargo build -q --release -p nyash-rust --features tlv-shim )

CODE='
static box Main {
  main() {
    local m = new MapBox()
    m.set("k", "v")
    print(m.get("k"))
    return 0
  }
}
'

o1=$(HAKO_TLV_SHIM=0 run_nyash_vm -c "$CODE" 2>&1 | tr -d '\r')
o2=$(HAKO_TLV_SHIM=1 run_nyash_vm -c "$CODE" 2>&1 | tr -d '\r')

# Strip shim trace if any to compare pure program output
o1p=$(printf "%s\n" "$o1" | awk '!/^\[tlv\/shim:/')
o2p=$(printf "%s\n" "$o2" | awk '!/^\[tlv\/shim:/')

if diff -u <(printf "%s\n" "$o1p") <(printf "%s\n" "$o2p") >/dev/null; then
  echo "[PASS] tlv_shim_parity_canary_vm"
  exit 0
fi
echo "[FAIL] tlv_shim_parity_canary_vm" >&2
echo "--- OFF ---" >&2; echo "$o1" >&2
echo "--- ON ----" >&2; echo "$o2" >&2
exit 1

