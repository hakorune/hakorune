#!/bin/bash
# early_return.sh - 早期returnの合流確認

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/result_checker.sh"

require_env || exit 2
preflight_plugins || exit 2

test_early_return_then() {
  local tmpfile
  tmpfile="$(mktemp /tmp/early_return.XXXXXX.hako)"
  cat >"$tmpfile" <<'EOF'
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
  local output
  output=$(
    NYASH_JOINIR_DEV=0 \
    HAKO_JOINIR_STRICT=0 \
    NYASH_JOINIR_STRICT=0 \
    HAKO_JOINIR_PLANNER_REQUIRED=0 \
    "$NYASH_BIN" --backend vm "$tmpfile" 2>&1 | filter_noise
  )
  rm -f "$tmpfile"
  check_exact "A" "$output" "early_return_then"
}

run_test "early_return_then" test_early_return_then
