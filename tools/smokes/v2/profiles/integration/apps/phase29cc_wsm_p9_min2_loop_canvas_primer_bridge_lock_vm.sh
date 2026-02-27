#!/bin/bash
# phase29cc_wsm_p9_min2_loop_canvas_primer_bridge_lock_vm.sh
# Contract pin:
# - WSM-P9-min2: webcanvas/loop primer fixture remains on bridge route.

set -euo pipefail
source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-191-wsm-p9-min2-loop-canvas-primer-bridge-lock-ssot.md"
if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_p9_min2_loop_canvas_primer_bridge_lock_vm: lock doc missing"
  exit 1
fi

for needle in "WSM-P9-min2" "accepted-but-blocked" "BridgeRustBackend" "bridge-rust-backend"; do
  if ! grep -Fq "$needle" "$doc"; then
    test_fail "phase29cc_wsm_p9_min2_loop_canvas_primer_bridge_lock_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

cargo test --features wasm-backend wasm_demo_default_hako_lane_bridge_webcanvas_fixture_contract -- --nocapture
cargo test --features wasm-backend wasm_demo_route_trace_reports_bridge_for_webcanvas_fixture_contract -- --nocapture

test_pass "phase29cc_wsm_p9_min2_loop_canvas_primer_bridge_lock_vm: PASS (WSM-P9-min2 loop/canvas primer bridge lock)"
