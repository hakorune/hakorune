#!/bin/bash
# phase29cc_wsm_p6_min1_route_policy_default_noop_lock_vm.sh
# Contract pin:
# - WSM-P6-min1: NYASH_WASM_ROUTE_POLICY is default-only and operationally no-op.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/phase29cc_wsm_p5_route_trace_common.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-170-wsm-p6-min1-route-policy-default-noop-lock-ssot.md"
if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_p6_min1_route_policy_default_noop_lock_vm: lock doc missing"
  exit 1
fi

for needle in \
  "WSM-P6-min1" \
  "NYASH_WASM_ROUTE_POLICY" \
  "default-only" \
  "no-op" \
  "allowed: default" \
  "wasm-boundary-lite"; do
  if ! grep -Fq "$needle" "$doc"; then
    test_fail "phase29cc_wsm_p6_min1_route_policy_default_noop_lock_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

run_wsm_p6_route_policy_default_noop_contract_tests

test_pass "phase29cc_wsm_p6_min1_route_policy_default_noop_lock_vm: PASS (WSM-P6-min1 route policy default-only no-op lock)"
