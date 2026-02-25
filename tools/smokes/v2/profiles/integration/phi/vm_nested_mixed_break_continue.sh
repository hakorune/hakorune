#!/bin/bash
# vm_nested_mixed_break_continue.sh — nested loops with mixed break/continue

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2
preflight_plugins || exit 2

test_nested_mixed() {
  # Outer i=0..4, inner j=0..4, skip j==2 (continue), break inner entirely when i==3.
  # Counts per i: 0->4, 1->4, 2->4, 3->0 (break), 4->4 => total 16
  local code='static box Main { main() { local i, j, sum; i=0; sum=0; loop(i<5) {
    j=0; loop(j<5) {
      if j==2 { j=j+1; continue }
      if i==3 { break }
      sum=sum+1; j=j+1
    }; i=i+1
  }; print(sum); return 0 } }'
  local out
  out=$(run_nyash_vm -c "$code")
  [ "$out" = "16" ]
}

run_test "nested_mixed_break_continue" test_nested_mixed

