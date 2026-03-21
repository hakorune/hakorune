#!/bin/bash
# phase29cc_wsm_p10_min3_loop_extern_writer_section_lock_vm.sh
# Contract pin:
# - WSM-P10-min3: loop/branch/call writer section contract lock (route remains bridge).

set -euo pipefail

source "$(dirname "$0")/phase29cc_wsm_p10_common.sh"

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-196-wsm-p10-min3-loop-extern-writer-section-lock-ssot.md"
require_p10_doc_keywords \
  "phase29cc_wsm_p10_min3_loop_extern_writer_section_lock_vm" \
  "$doc" \
  "WSM-P10-min3" \
  "Type -> Import -> Function -> Export -> Code" \
  "env.console_log(i32) -> void" \
  "main() -> i32" \
  "bridge-rust-backend" \
  "WSM-P10-min4"

run_p10_contract_tests \
  "cargo test --features wasm-backend wasm_binary_writer_loop_extern_ -- --nocapture" \
  "cargo test --features wasm-backend wasm_binary_writer_loop_extern_skeleton_contract -- --nocapture" \
  "cargo test --features wasm-backend wasm_hako_default_lane_plan_bridge_for_non_pilot_shape_contract -- --nocapture"

test_pass "phase29cc_wsm_p10_min3_loop_extern_writer_section_lock_vm: PASS (WSM-P10-min3 loop/extern writer section lock)"
