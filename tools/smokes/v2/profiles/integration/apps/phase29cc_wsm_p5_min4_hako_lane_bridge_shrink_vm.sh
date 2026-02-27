#!/bin/bash
# phase29cc_wsm_p5_min4_hako_lane_bridge_shrink_vm.sh
# Contract pin:
# - WSM-P5-min4 bridge shrink lock for default(hako-lane):
#   one-shape native pilot + explicit bridge fallback contract.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-163-wsm-p5-min4-hako-lane-bridge-shrink-lock-ssot.md"
if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_p5_min4_hako_lane_bridge_shrink_vm: lock doc missing"
  exit 1
fi

for needle in \
  "WSM-P5-min4" \
  "NativePilotShape" \
  "BridgeRustBackend" \
  "default(hako-lane)" \
  "bridge"; do
  if ! grep -q "$needle" "$doc"; then
    test_fail "phase29cc_wsm_p5_min4_hako_lane_bridge_shrink_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

cargo test --features wasm-backend wasm_hako_default_lane_plan_ -- --nocapture
cargo test --features wasm-backend wasm_demo_default_hako_lane_ -- --nocapture

test_pass "phase29cc_wsm_p5_min4_hako_lane_bridge_shrink_vm: PASS (WSM-P5-min4 bridge shrink lock)"
