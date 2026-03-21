#!/bin/bash
# phase29cc_wsm_p8_min1_bridge_retire_readiness_vm.sh
# Contract pin:
# - WSM-P8-min1: compat bridge retire execution stays accepted-but-blocked
#   while default-only route contract remains active.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/phase29cc_wsm_cargo_test_common.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-188-wsm-p8-min1-bridge-retire-readiness-lock-ssot.md"
if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_p8_min1_bridge_retire_readiness_vm: lock doc missing"
  exit 1
fi

for needle in \
  "WSM-P8-min1" \
  "accepted-but-blocked" \
  "BridgeRustBackend" \
  "bridge-rust-backend" \
  "default-only" \
  "portability"; do
  if ! grep -Fq "$needle" "$doc"; then
    test_fail "phase29cc_wsm_p8_min1_bridge_retire_readiness_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

bash tools/smokes/v2/profiles/integration/phase29cc_wsm/p7/phase29cc_wsm_p7_min4_compat_retention_lock_vm.sh
run_wsm_targeted_contract_test "wasm_hako_default_lane_plan_bridge_for_non_pilot_shape_contract"
run_wsm_targeted_contract_test "wasm_hako_default_lane_trace_has_none_shape_id_for_bridge_contract"
run_wsm_targeted_contract_test "wasm_demo_default_hako_lane_bridge_non_pilot_contract"

test_pass "phase29cc_wsm_p8_min1_bridge_retire_readiness_vm: PASS (WSM-P8-min1 bridge retire readiness lock)"
