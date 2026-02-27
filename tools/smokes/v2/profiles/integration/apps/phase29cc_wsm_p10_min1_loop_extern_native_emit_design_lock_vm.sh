#!/bin/bash
# phase29cc_wsm_p10_min1_loop_extern_native_emit_design_lock_vm.sh
# Contract pin:
# - WSM-P10-min1: loop/extern call native emit stays design-locked (bridge boundary unchanged).

set -euo pipefail
source "$(dirname "$0")/phase29cc_wsm_p10_common.sh"

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-194-wsm-p10-min1-loop-extern-native-emit-design-lock-ssot.md"
require_p10_doc_keywords \
  "phase29cc_wsm_p10_min1_loop_extern_native_emit_design_lock_vm" \
  "$doc" \
  "WSM-P10-min1" \
  "accepted-but-blocked" \
  "loop/extern call native emit" \
  "wsm.p10.main_loop_extern_call.v0" \
  "WSM-P10-min2"

run_p10_contract_tests \
  "cargo test --features wasm-backend wasm_demo_default_hako_lane_bridge_webcanvas_fixture_contract -- --nocapture" \
  "cargo test --features wasm-backend wasm_demo_default_hako_lane_bridge_canvas_advanced_fixture_contract -- --nocapture" \
  "cargo test --features wasm-backend wasm_demo_route_trace_reports_bridge_for_webcanvas_fixture_contract -- --nocapture"

test_pass "phase29cc_wsm_p10_min1_loop_extern_native_emit_design_lock_vm: PASS (WSM-P10-min1 loop/extern native emit design lock)"
