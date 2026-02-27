#!/bin/bash
# phase29cc_wsm_p9_min0_non_native_inventory_lock_vm.sh
# Contract pin:
# - WSM-P9-min0: non-native inventory remains explicit and bridge fallback boundary is observable.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-189-wsm-p9-min0-non-native-inventory-lock-ssot.md"
if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_p9_min0_non_native_inventory_lock_vm: lock doc missing"
  exit 1
fi

for needle in \
  "WSM-P9-min0" \
  "BridgeRustBackend" \
  "bridge-rust-backend" \
  "phase29cc_wsm02d_demo_min.hako" \
  "default-only"; do
  if ! grep -Fq "$needle" "$doc"; then
    test_fail "phase29cc_wsm_p9_min0_non_native_inventory_lock_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

cargo test --features wasm-backend wasm_hako_default_lane_plan_bridge_for_non_pilot_shape_contract -- --nocapture
cargo test --features wasm-backend wasm_hako_default_lane_trace_has_none_shape_id_for_bridge_contract -- --nocapture
cargo test --features wasm-backend wasm_demo_default_hako_lane_bridge_non_pilot_contract -- --nocapture

test_pass "phase29cc_wsm_p9_min0_non_native_inventory_lock_vm: PASS (WSM-P9-min0 non-native inventory lock)"
