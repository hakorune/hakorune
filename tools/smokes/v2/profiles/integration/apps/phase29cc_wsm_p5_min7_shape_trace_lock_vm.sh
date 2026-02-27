#!/bin/bash
# phase29cc_wsm_p5_min7_shape_trace_lock_vm.sh
# Contract pin:
# - WSM-P5-min7: shape-id route trace is observable for wasm route decisions.
# - default(native/bridge) and legacy policy plans are emitted as stable one-line tags.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-166-wsm-p5-min7-shape-trace-lock-ssot.md"
if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_p5_min7_shape_trace_lock_vm: lock doc missing"
  exit 1
fi

for needle in \
  "WSM-P5-min7" \
  "NYASH_WASM_ROUTE_TRACE" \
  "[wasm/route-trace]" \
  "shape_id" \
  "legacy-rust"; do
  if ! grep -q "$needle" "$doc"; then
    test_fail "phase29cc_wsm_p5_min7_shape_trace_lock_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

cargo test --features wasm-backend wasm_route_policy_name_contract -- --nocapture
cargo test --features wasm-backend wasm_hako_default_lane_trace_ -- --nocapture
cargo test --features wasm-backend wasm_demo_route_trace_reports_ -- --nocapture

test_pass "phase29cc_wsm_p5_min7_shape_trace_lock_vm: PASS (WSM-P5-min7 shape trace lock)"
