#!/bin/bash
# set -eは使わない（個々のテストが失敗しても続行するため）
# string_concat.sh - 文字列結合のテスト

# 共通ライブラリ読み込み（必須）
source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/result_checker.sh"

# 環境チェック（必須）
require_env || exit 2

# プラグイン整合性チェック（必須）
preflight_plugins || exit 2

test_string_concat_suite() {
    local tmpfile
    tmpfile="$(mktemp /tmp/string_concat.XXXXXX.hako)"
    cat >"$tmpfile" <<'EOF'
static box Main {
    main() {
        print("Hello" + " " + "World")

        local greeting, name, message
        greeting = "Hello"
        name = "Nyash"
        message = greeting + ", " + name + "!"
        print(message)

        local num, text
        num = 42
        text = "The answer is " + ("" + num)
        print(text)

        return 0
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

    local expected
    expected=$(cat <<'TXT'
Hello World
Hello, Nyash!
The answer is 42
TXT
)
    compare_outputs "$expected" "$output" "string_concat"
}

run_test "string_concat" test_string_concat_suite
