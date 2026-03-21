#!/bin/bash
# phase29cc_wsm_p5_min2_route_policy_lock_vm.sh
# Contract pin:
# - WSM-P5-min2 route policy lock for default lane selection.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-161-wsm-p5-min2-route-policy-lock-ssot.md"
if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_p5_min2_route_policy_lock_vm: lock doc missing"
  exit 1
fi

for needle in \
  "WSM-P5-min2" \
  "NYASH_WASM_ROUTE_POLICY" \
  "default" \
  "allowed: default" \
  "fail-fast"; do
  if ! grep -q "$needle" "$doc"; then
    test_fail "phase29cc_wsm_p5_min2_route_policy_lock_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

cargo test wasm_route_policy_ -- --nocapture

test_pass "phase29cc_wsm_p5_min2_route_policy_lock_vm: PASS (WSM-P5-min2 route policy lock)"
