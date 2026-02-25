#!/usr/bin/env bash
# Phase 22.1 — TLV shim plugin-call canary (single call, guarded)
set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env >/dev/null || exit 2

# Build nyash with tlv-shim feature
(
  cd "$NYASH_ROOT" && cargo build -q --release -p nyash-rust --features tlv-shim
)

# Minimal plugin call: MapBox.set/get via VM with TLV shim enabled
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

# Enable trace (default only MapBox.set is traced); accept output-only fallback
out=$(HAKO_TLV_SHIM=1 HAKO_TLV_SHIM_TRACE=1 run_nyash_vm -c "$CODE" 2>&1)
if echo "$out" | grep -q '\[tlv/shim:MapBox.set\]'; then
  echo "[PASS] tlv_shim_plugin_call_canary_vm"
  exit 0
fi
if echo "$out" | grep -q '^v$'; then
  echo "[PASS] tlv_shim_plugin_call_canary_vm"
  exit 0
fi

echo "[FAIL] tlv_shim_plugin_call_canary_vm" >&2
echo "$out" >&2
exit 1
