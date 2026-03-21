#!/bin/bash
# phase29cc_wsm_p10_min5_expansion_inventory_lock_vm.sh
# Contract pin:
# - WSM-P10-min5: expansion inventory is analysis-only and does not change native helper boundary.

set -euo pipefail

source "$(dirname "$0")/phase29cc_wsm_p10_common.sh"

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-198-wsm-p10-min5-expansion-inventory-lock-ssot.md"
require_p10_doc_keywords \
  "phase29cc_wsm_p10_min5_expansion_inventory_lock_vm" \
  "$doc" \
  "WSM-P10-min5" \
  "analysis-only" \
  "wsm.p10.main_loop_extern_call.warn.fixed3.inventory.v0" \
  "wsm.p10.main_loop_extern_call.info.fixed3.inventory.v0" \
  "wsm.p10.main_loop_extern_call.error.fixed3.inventory.v0" \
  "wsm.p10.main_loop_extern_call.debug.fixed3.inventory.v0" \
  "WSM-P10-min6"

run_p10_contract_tests \
  "cargo test --features wasm-backend wasm_shape_table_detects_p10_min5_expansion_warn_inventory_contract -- --nocapture" \
  "cargo test --features wasm-backend wasm_shape_table_detects_p10_min5_expansion_info_inventory_contract -- --nocapture" \
  "cargo test --features wasm-backend wasm_shape_table_detects_p10_min5_expansion_error_inventory_contract -- --nocapture" \
  "cargo test --features wasm-backend wasm_shape_table_detects_p10_min5_expansion_debug_inventory_contract -- --nocapture" \
  "cargo test --features wasm-backend wasm_shape_table_rejects_p10_min5_expansion_unknown_method_contract -- --nocapture" \
  "cargo test --features wasm-backend wasm_demo_default_route_p10_min5_warn_inventory_rejected_by_native_helper_contract -- --nocapture" \
  "cargo test --features wasm-backend wasm_demo_default_route_p10_min5_info_inventory_rejected_by_native_helper_contract -- --nocapture"

test_pass "phase29cc_wsm_p10_min5_expansion_inventory_lock_vm: PASS (WSM-P10-min5 expansion inventory lock)"
