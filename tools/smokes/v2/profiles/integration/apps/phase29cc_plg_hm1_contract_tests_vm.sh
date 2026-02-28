#!/bin/bash
# phase29cc_plg_hm1_contract_tests_vm.sh
# Contract pin:
# - PLG-HM1 consolidated contract tests (min1..min4)

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

run_case() {
  local test_name="$1"
  local smoke_name="$2"

  set +e
  local output
  output=$(cd "$NYASH_ROOT" && cargo test "$test_name" -- --nocapture 2>&1)
  local rc=$?
  set -e

  if [ "$rc" -ne 0 ]; then
    test_fail "${smoke_name}: test failed (rc=$rc)"
    printf '%s\n' "$output" | sed -n '1,200p'
    exit 1
  fi

  if ! printf '%s\n' "$output" | grep -q "$test_name"; then
    test_fail "${smoke_name}: expected marker missing: $test_name"
    printf '%s\n' "$output" | sed -n '1,200p'
    exit 1
  fi
}

run_case "plugin_exec_mode_" "phase29cc_plg_hm1_contract_tests_vm"
run_case "should_skip_dynamic_route_core4_contract" "phase29cc_plg_hm1_contract_tests_vm"
run_case "should_skip_dynamic_route_file_path_contract" "phase29cc_plg_hm1_contract_tests_vm"
run_case "should_keep_dynamic_route_math_net_compat_contract" "phase29cc_plg_hm1_contract_tests_vm"

test_pass "phase29cc_plg_hm1_contract_tests_vm: PASS (PLG-HM1 min1..min4 consolidated contract)"
