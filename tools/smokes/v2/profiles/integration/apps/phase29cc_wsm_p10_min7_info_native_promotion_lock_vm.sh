#!/bin/bash
# phase29cc_wsm_p10_min7_info_native_promotion_lock_vm.sh
# Contract pin:
# - WSM-P10-min7 promotes info fixed4 shape to native emit while keeping min5 fixed3 inventory bridge-only.

set -euo pipefail

source "$(dirname "$0")/phase29cc_wsm_p10_common.sh"

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-200-wsm-p10-min7-info-native-promotion-lock-ssot.md"
require_p10_doc_keywords \
  "phase29cc_wsm_p10_min7_info_native_promotion_lock_vm" \
  "$doc" \
  "WSM-P10-min7" \
  "wsm.p10.main_loop_extern_call.info.fixed4.v0" \
  "LoopExternImport::ConsoleInfo" \
  "info.fixed3.inventory.v0" \
  "WSM-P10-min8"

run_p10_contract_tests \
  "cargo test --features wasm-backend wasm_shape_table_detects_p10_min7_info_native_promotable_contract -- --nocapture" \
  "cargo test --features wasm-backend wasm_shape_table_rejects_p10_min7_info_native_promotable_for_fixed3_contract -- --nocapture" \
  "cargo test --features wasm-backend wasm_binary_writer_loop_extern_info_import_contract -- --nocapture" \
  "cargo test --features wasm-backend wasm_demo_default_hako_lane_native_p10_min7_info_loop_extern_shape_contract -- --nocapture" \
  "cargo test --features wasm-backend wasm_demo_default_route_p10_min7_info_native_uses_native_helper_contract -- --nocapture" \
  "cargo test --features wasm-backend wasm_demo_route_trace_reports_shape_id_for_native_p10_min7_info_contract -- --nocapture" \
  "cargo test --features wasm-backend wasm_demo_default_route_p10_min5_info_inventory_rejected_by_native_helper_contract -- --nocapture"

test_pass "phase29cc_wsm_p10_min7_info_native_promotion_lock_vm: PASS (WSM-P10-min7 info native promotion lock)"
