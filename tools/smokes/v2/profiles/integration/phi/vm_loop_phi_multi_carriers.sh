#!/bin/bash
# vm_loop_phi_multi_carriers.sh — two loop carriers (fibonacci-like)

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2
preflight_plugins || exit 2

test_multi_carriers_fib6() {
  # Fibonacci with 6 steps starting a=0,b=1 => b ends at 13
  local code='static box Main { main() { local i, a, b, t; i=0; a=0; b=1; loop(i<6) { t=a+b; a=b; b=t; i=i+1 }; print(b); return 0 } }'
  local out
  out=$(run_nyash_vm -c "$code")
  [ "$out" = "13" ]
}

run_test "loop_phi_multi_carriers" test_multi_carriers_fib6

