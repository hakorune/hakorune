#!/bin/bash
# phase29cc_plg_hm1_min2_core_module_route_skip_lock_vm.sh
# Contract pin:
# - PLG-HM1-min2: module_first mode skips dynamic plugin route for Core4 (Array/String/Map/Console).

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

set +e
output=$(cd "$NYASH_ROOT" && cargo test should_skip_dynamic_route_contract -- --nocapture 2>&1)
rc=$?
set -e
if [ "$rc" -ne 0 ]; then
  test_fail "phase29cc_plg_hm1_min2_core_module_route_skip_lock_vm: route skip tests failed (rc=$rc)"
  printf '%s\n' "$output" | sed -n '1,200p'
  exit 1
fi

if ! printf '%s\n' "$output" | grep -q "should_skip_dynamic_route_contract"; then
  test_fail "phase29cc_plg_hm1_min2_core_module_route_skip_lock_vm: expected test marker missing"
  printf '%s\n' "$output" | sed -n '1,200p'
  exit 1
fi

test_pass "phase29cc_plg_hm1_min2_core_module_route_skip_lock_vm: PASS (PLG-HM1-min2 core module route skip lock)"
