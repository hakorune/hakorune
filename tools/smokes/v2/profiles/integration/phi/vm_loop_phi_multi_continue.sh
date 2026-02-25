#!/bin/bash
# vm_loop_phi_multi_continue.sh — multi-continue paths with single loop carrier

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2
preflight_plugins || exit 2

test_multi_continue_sum() {
  # Expect: sum of odd numbers 1..9 except 7 -> 1+3+5+9 = 18
  local code='static box Main { main() { local i, sum; i=0; sum=0; loop(i<10) {
    if i%2==0 { i=i+1; continue }
    if i==7 { i=i+1; continue }
    sum=sum+i; i=i+1
  }; print(sum); return 0 } }'
  local out
  out=$(run_nyash_vm -c "$code")
  [ "$out" = "18" ]
}

run_test "loop_phi_multi_continue" test_multi_continue_sum

