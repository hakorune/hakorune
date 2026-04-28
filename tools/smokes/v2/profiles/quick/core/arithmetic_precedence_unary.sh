#!/bin/bash
# arithmetic_precedence_unary.sh - 単項と優先順位の基本チェック

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/result_checker.sh"

require_env || exit 2
preflight_plugins || exit 2

test_unary_precedence() {
  local tmpfile
  tmpfile="$(mktemp /tmp/arithmetic_precedence_unary.XXXXXX.hako)"
  cat >"$tmpfile" <<'EOF'
static box Main {
  main() {
    print(-(1 + 2) * 3)
    print(1 + -2)
    return 0
  }
}
EOF
  local out
  out=$(
    NYASH_JOINIR_DEV=0 \
    HAKO_JOINIR_STRICT=0 \
    NYASH_JOINIR_STRICT=0 \
    HAKO_JOINIR_PLANNER_REQUIRED=0 \
    "$NYASH_BIN" --backend vm "$tmpfile" 2>&1 | filter_noise
  )
  rm -f "$tmpfile"
  # 期待: -9 と -1 の2行
  if echo "$out" | grep -q "^-9$" && echo "$out" | grep -q "^-1$"; then
    return 0
  else
    echo "[FAIL] unary_precedence: output mismatch" >&2
    echo "  Actual output:" >&2
    echo "$out" | sed 's/^/    /' >&2
    return 1
  fi
}

run_test "unary_precedence" test_unary_precedence
