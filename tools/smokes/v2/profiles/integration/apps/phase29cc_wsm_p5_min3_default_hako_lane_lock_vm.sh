#!/bin/bash
# phase29cc_wsm_p5_min3_default_hako_lane_lock_vm.sh
# Contract pin:
# - WSM-P5-min3 default route cutover to hako-lane (bridge implementation) with legacy parity lock.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-162-wsm-p5-min3-default-hako-lane-lock-ssot.md"
if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_p5_min3_default_hako_lane_lock_vm: lock doc missing"
  exit 1
fi

for needle in \
  "WSM-P5-min3" \
  "default" \
  "hako-lane" \
  "bridge" \
  "legacy-wasm-rust"; do
  if ! grep -q "$needle" "$doc"; then
    test_fail "phase29cc_wsm_p5_min3_default_hako_lane_lock_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

cargo test --features wasm-backend wasm_compile_route_policy_ -- --nocapture
cargo test --features wasm-backend wasm_demo_min_fixture_legacy_route_retired_failfast_contract -- --nocapture

test_pass "phase29cc_wsm_p5_min3_default_hako_lane_lock_vm: PASS (WSM-P5-min3 default hako-lane lock)"
