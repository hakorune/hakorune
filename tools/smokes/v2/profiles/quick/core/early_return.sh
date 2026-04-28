#!/bin/bash
# early_return.sh - 早期returnの合流確認

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/result_checker.sh"

require_env || exit 2
preflight_plugins || exit 2

test_early_return_then() {
  local output
  output=$(run_hako_fixture "early_return" run_quick_vm_release <<'EOF'
static box Main {
  main() {
    if 1 {
      print("A")
      return 0
    } else {
      print("B")
    }
    print("C")
  }
}
EOF
  )
  check_exact "A" "$output" "early_return_then"
}

run_test "early_return_then" test_early_return_then
