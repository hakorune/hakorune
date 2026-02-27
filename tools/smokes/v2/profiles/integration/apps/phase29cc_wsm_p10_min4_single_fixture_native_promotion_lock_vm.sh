#!/bin/bash
# phase29cc_wsm_p10_min4_single_fixture_native_promotion_lock_vm.sh
# Contract pin:
# - WSM-P10-min4: promote exactly one loop/extern fixture to native emit, keep existing bridge fixtures unchanged.

set -euo pipefail

source "$(dirname "$0")/phase29cc_wsm_p10_common.sh"

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-197-wsm-p10-min4-single-fixture-native-promotion-lock-ssot.md"
require_p10_doc_keywords \
  "phase29cc_wsm_p10_min4_single_fixture_native_promotion_lock_vm" \
  "$doc" \
  "WSM-P10-min4" \
  "phase29cc_wsm_p10_min4_loop_extern_native.hako" \
  "wsm.p10.main_loop_extern_call.fixed3.v0" \
  "build_loop_extern_call_skeleton_module(3)" \
  "WSM-P10-min5"

run_p10_contract_tests \
  "cargo test --features wasm-backend wasm_demo_default_hako_lane_native_p10_min4_loop_extern_shape_contract -- --nocapture" \
  "cargo test --features wasm-backend wasm_demo_default_route_p10_min4_uses_native_helper_contract -- --nocapture" \
  "cargo test --features wasm-backend wasm_demo_route_trace_reports_shape_id_for_native_p10_min4_contract -- --nocapture" \
  "cargo test --features wasm-backend wasm_demo_default_hako_lane_bridge_webcanvas_fixture_contract -- --nocapture"

test_pass "phase29cc_wsm_p10_min4_single_fixture_native_promotion_lock_vm: PASS (WSM-P10-min4 single fixture native promotion lock)"
