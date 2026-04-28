#!/bin/bash
# set -eは使わない（個々のテストが失敗しても続行するため）
# if_statement.sh - if文のテスト

# 共通ライブラリ読み込み（必須）
source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/result_checker.sh"

# 環境チェック（必須）
require_env || exit 2

# プラグイン整合性チェック（必須）
preflight_plugins || exit 2

test_if_statement_suite() {
    local tmpfile
    tmpfile="$(mktemp /tmp/if_statement.XXXXXX.hako)"
    cat >"$tmpfile" <<'EOF'
static box Main {
  method main(args) {
    local x = 10
    if x > 5 {
      print("greater")
    } else {
      print("smaller")
    }

    local score = 75
    if score >= 80 {
      print("A")
    } else if score >= 60 {
      print("B")
    } else {
      print("C")
    }

    local a = 10
    local b = 20
    if a < b {
      if a == 10 {
        print("correct")
      } else {
        print("wrong")
      }
    } else {
      print("error")
    }

    local px = 5
    local py = 10
    if px > 0 {
      if py > 0 {
        print("both positive")
      } else {
        print("not both positive")
      }
    } else {
      print("not both positive")
    }

    return 0
  }
}
EOF

    local output
    output=$(run_quick_vm_release "$tmpfile")
    local rc=${PIPESTATUS[0]}
    rm -f "$tmpfile"
    [ "$rc" -eq 0 ] || return "$rc"

    local expected
    expected=$(cat <<'TXT'
greater
B
correct
both positive
TXT
)
    compare_outputs "$expected" "$output" "if_statement"
}

run_test "if_statement" test_if_statement_suite
