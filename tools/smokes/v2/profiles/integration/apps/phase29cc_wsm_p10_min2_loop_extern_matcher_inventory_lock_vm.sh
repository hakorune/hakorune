#!/bin/bash
# phase29cc_wsm_p10_min2_loop_extern_matcher_inventory_lock_vm.sh
# Contract pin:
# - WSM-P10-min2: loop+extern matcher inventory is analysis-only and does not change default bridge route.

set -euo pipefail

source "$(dirname "$0")/phase29cc_wsm_p10_common.sh"

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-195-wsm-p10-min2-loop-extern-matcher-inventory-lock-ssot.md"
require_p10_doc_keywords \
  "phase29cc_wsm_p10_min2_loop_extern_matcher_inventory_lock_vm" \
  "$doc" \
  "WSM-P10-min2" \
  "analysis-only" \
  "wsm.p10.main_loop_extern_call.v0" \
  "Branch + Jump + Extern Call" \
  "WSM-P10-min3"

run_p10_contract_tests \
  "cargo test --features wasm-backend wasm_shape_table_detects_p10_loop_extern_candidate_contract -- --nocapture" \
  "cargo test --features wasm-backend wasm_shape_table_rejects_p10_candidate_without_loop_contract -- --nocapture" \
  "cargo test --features wasm-backend wasm_hako_default_lane_plan_bridge_for_non_pilot_shape_contract -- --nocapture"

test_pass "phase29cc_wsm_p10_min2_loop_extern_matcher_inventory_lock_vm: PASS (WSM-P10-min2 loop/extern matcher inventory lock)"
