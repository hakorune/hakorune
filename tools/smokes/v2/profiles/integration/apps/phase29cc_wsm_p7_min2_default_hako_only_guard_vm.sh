#!/bin/bash
# phase29cc_wsm_p7_min2_default_hako_only_guard_vm.sh
# Contract pin:
# - WSM-P7-min2: default route remains hako-only and legacy route-policy values stay rejected.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../phase29cc_wsm/p6/phase29cc_wsm_p6_common.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-185-wsm-p7-min2-default-hako-only-guard-lock-ssot.md"
if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_p7_min2_default_hako_only_guard_vm: lock doc missing"
  exit 1
fi

for needle in \
  "WSM-P7-min2" \
  "default-only" \
  "NYASH_WASM_ROUTE_POLICY" \
  "allowed: default" \
  "wasm_hako_default_lane_trace_" \
  "portability"; do
  if ! grep -Fq "$needle" "$doc"; then
    test_fail "phase29cc_wsm_p7_min2_default_hako_only_guard_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

run_wsm_p6_route_policy_default_noop_contract_tests
run_wsm_targeted_contract_test "wasm_hako_default_lane_trace_"

test_pass "phase29cc_wsm_p7_min2_default_hako_only_guard_vm: PASS (WSM-P7-min2 default hako-only guard lock)"
