#!/bin/bash
# phase29cc_plg_hm1_min4_math_net_compat_inventory_lock_vm.sh
# Contract pin:
# - PLG-HM1-min4: module_first keeps Math/Net on dynamic compat lane (not route-skipped).

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

set +e
output=$(cd "$NYASH_ROOT" && cargo test should_keep_dynamic_route_math_net_compat_contract -- --nocapture 2>&1)
rc=$?
set -e
if [ "$rc" -ne 0 ]; then
  test_fail "phase29cc_plg_hm1_min4_math_net_compat_inventory_lock_vm: compat inventory tests failed (rc=$rc)"
  printf '%s\n' "$output" | sed -n '1,200p'
  exit 1
fi

if ! printf '%s\n' "$output" | grep -q "should_keep_dynamic_route_math_net_compat_contract"; then
  test_fail "phase29cc_plg_hm1_min4_math_net_compat_inventory_lock_vm: expected test marker missing"
  printf '%s\n' "$output" | sed -n '1,200p'
  exit 1
fi

test_pass "phase29cc_plg_hm1_min4_math_net_compat_inventory_lock_vm: PASS (PLG-HM1-min4 math/net compat inventory lock)"
