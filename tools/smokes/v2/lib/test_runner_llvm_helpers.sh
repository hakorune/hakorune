#!/bin/bash
# test_runner_llvm_helpers.sh - split from test_runner.sh
# Nyash実行ヘルパー（LLVM）
run_nyash_llvm() {
    local program="$1"
    shift
    # Allow developer to force LLVM run (env guarantees availability)
    if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ]; then
        # Skip gracefully when LLVM backend is not available in this build
        if ! can_run_llvm; then
            log_warn "LLVM backend not available in this build; skipping LLVM run"
            log_info "Hint: build ny-llvmc + enable harness: cargo build --release -p nyash-llvm-compiler && cargo build --release --features llvm"
            return 0
        fi
    fi
    # -c オプションの場合は一時ファイル経由で実行
    if [ "$program" = "-c" ]; then
        local code="$1"
        shift
        local tmpfile="/tmp/nyash_test_$$.hako"
        echo "$code" > "$tmpfile"
        # 軽量ASIFix（テスト用）: ブロック終端の余剰セミコロンを寛容に除去
        if [ "${SMOKES_ASI_STRIP_SEMI:-1}" = "1" ]; then
            sed -i -E 's/;([[:space:]]*)(\}|$)/\1\2/g' "$tmpfile" || true
        fi
        # プラグイン初期化メッセージを除外
        PYTHONPATH="${PYTHONPATH:-$NYASH_ROOT}" NYASH_NY_LLVM_COMPILER="$NYASH_ROOT/target/release/ny-llvmc" NYASH_LLVM_USE_HARNESS=1 NYASH_EMIT_EXE_NYRT="$NYASH_ROOT/target/release" NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 "$NYASH_BIN" --backend llvm "$tmpfile" "$@" 2>&1 | \
            grep -v "^\[UnifiedBoxRegistry\]" | grep -v "^\[FileBox\]" | grep -v "^Net plugin:" | grep -v "^\[.*\] Plugin" | \
            grep -v '^\[plugin-loader\] backend=' | \
            grep -v '^🔌 plugin host initialized' | grep -v '^✅ plugin host fully configured' | \
            grep -v '^⚡ Hakorune LLVM Backend' | \
            grep -v '^✅ LLVM (harness) execution completed' | grep -v '^📊 MIR Module compiled successfully' | grep -v '^📊 Functions:' | grep -v 'JSON Parse Errors:' | grep -v 'Parsing errors' | grep -v 'No parsing errors' | grep -v 'Error at line ' | \
            grep -v '^\[using\]' | grep -v '^\[using/resolve\]' | grep -v '^\[using/cache\]' | \
            grep -v '^\[ny-llvmc\]' | grep -v '^\[harness\]' | grep -v '^Compiled to ' | grep -v '^/usr/bin/ld:'
        local exit_code=${PIPESTATUS[0]}
        rm -f "$tmpfile"
        return $exit_code
    else
        local runprog="$program"
        local workfile=""
        # 軽量ASIFix（テスト用）: 元ファイルは変更せず一時コピーへ適用
        if [ "${SMOKES_ASI_STRIP_SEMI:-1}" = "1" ] && [ -f "$program" ]; then
            workfile="$(mktemp /tmp/nyash_test_llvm_file.XXXXXX.hako)"
            cp "$program" "$workfile"
            sed -i -E 's/;([[:space:]]*)(\}|$)/\1\2/g' "$workfile" || true
            runprog="$workfile"
        fi
        # プラグイン初期化メッセージを除外
        PYTHONPATH="${PYTHONPATH:-$NYASH_ROOT}" NYASH_NY_LLVM_COMPILER="$NYASH_ROOT/target/release/ny-llvmc" NYASH_LLVM_USE_HARNESS=1 NYASH_EMIT_EXE_NYRT="$NYASH_ROOT/target/release" NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 "$NYASH_BIN" --backend llvm "$runprog" "$@" 2>&1 | \
            grep -v "^\[UnifiedBoxRegistry\]" | grep -v "^\[FileBox\]" | grep -v "^Net plugin:" | grep -v "^\[.*\] Plugin" | \
            grep -v '^\[plugin-loader\] backend=' | \
            grep -v '^🔌 plugin host initialized' | grep -v '^✅ plugin host fully configured' | \
            grep -v '^⚡ Hakorune LLVM Backend' | \
            grep -v '^✅ LLVM (harness) execution completed' | grep -v '^📊 MIR Module compiled successfully' | grep -v '^📊 Functions:' | grep -v 'JSON Parse Errors:' | grep -v 'Parsing errors' | grep -v 'No parsing errors' | grep -v 'Error at line ' | \
            grep -v '^\[using\]' | grep -v '^\[using/resolve\]' | grep -v '^\[using/cache\]' | \
            grep -v '^\[ny-llvmc\]' | grep -v '^\[harness\]' | grep -v '^Compiled to ' | grep -v '^/usr/bin/ld:'
        local exit_code=${PIPESTATUS[0]}
        rm -f "${workfile:-}" 2>/dev/null || true
        return $exit_code
    fi
}
