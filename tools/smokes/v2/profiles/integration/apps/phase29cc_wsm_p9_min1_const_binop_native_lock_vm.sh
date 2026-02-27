#!/bin/bash
# phase29cc_wsm_p9_min1_const_binop_native_lock_vm.sh
# Contract pin:
# - WSM-P9-min1: const-const-binop-return fixture must run on native shape lane.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-190-wsm-p9-min1-const-binop-native-shape-lock-ssot.md"
if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_p9_min1_const_binop_native_lock_vm: lock doc missing"
  exit 1
fi

for needle in \
  "WSM-P9-min1" \
  "wsm.p9.main_return_i32_const_binop.v0" \
  "const -> const -> binop -> return" \
  "phase29cc_wsm_p9_min1_const_binop_return.hako"; do
  if ! grep -Fq "$needle" "$doc"; then
    test_fail "phase29cc_wsm_p9_min1_const_binop_native_lock_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

cargo test --features wasm-backend wasm_demo_default_hako_lane_native_const_binop_shape_contract -- --nocapture
cargo test --features wasm-backend wasm_demo_default_route_const_binop_uses_native_helper_contract -- --nocapture
cargo test --features wasm-backend wasm_demo_route_trace_reports_shape_id_for_native_const_binop_contract -- --nocapture

test_pass "phase29cc_wsm_p9_min1_const_binop_native_lock_vm: PASS (WSM-P9-min1 const-binop native shape lock)"
