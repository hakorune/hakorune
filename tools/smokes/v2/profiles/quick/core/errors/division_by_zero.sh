#!/bin/bash
# division_by_zero.sh - ゼロ除算エラーパターンの検証

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/result_checker.sh"

require_env || exit 2
preflight_plugins || exit 2

test_division_by_zero() {
  local tmpfile
  tmpfile="$(mktemp /tmp/division_by_zero.XXXXXX.hako)"
  cat >"$tmpfile" <<'EOF'
static box Main {
  main() {
    print(1 / 0)
    return 0
  }
}
EOF
  local output
  output=$(run_quick_vm_release "$tmpfile" || true)
  rm -f "$tmpfile"
  check_regex "Division by zero" "$output" "division_by_zero"
}

run_test "division_by_zero" test_division_by_zero
