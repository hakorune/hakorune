#!/bin/bash
# arithmetic_ops.sh - 四則演算のテスト
# set -eは使わない（個々のテストが失敗しても続行するため）

# 共通ライブラリ読み込み（必須）
source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/result_checker.sh"

# 環境チェック（必須）
require_env || exit 2

# プラグイン整合性チェック（必須）
preflight_plugins || exit 2

test_arithmetic_ops_suite() {
    local tmpfile
    tmpfile="$(mktemp /tmp/arithmetic_ops.XXXXXX.hako)"
    cat >"$tmpfile" <<'EOF'
static box Main {
    main() {
        print(10 + 25)
        print(100 - 42)
        print(7 * 6)
        print(84 / 2)
        print((10 + 5) * 2 - 8)
        return 0
    }
}
EOF

    local output
    output=$(run_quick_vm_release "$tmpfile")
    rm -f "$tmpfile"

    local expected
    expected=$(cat <<'TXT'
35
58
42
42
22
TXT
)
    compare_outputs "$expected" "$output" "arithmetic_ops"
}

run_test "arithmetic_ops" test_arithmetic_ops_suite
