#!/usr/bin/env bash
# selfhost_canary_minimal.sh — Layer 1: 軽量カナリア（常時実行・30秒以内）
# 目的: セルフホストコンパイラのエントリが"パース可能"かを常時確認（LLVM不要）

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/selfhost_canary_minimal_$$.json"
trap 'rm -f "$tmp_json" || true' EXIT

# Hakoコンパイラ（エントリ）のパース確認（Program JSON emit）
# 目的は「パース可能」なことの常時確認なので、JoinIR ループ制約に依存しない
# `--emit-ast-json` を使用する。
set +e
out=$(NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
      NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
      NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
      "$NYASH_BIN" --emit-ast-json "$tmp_json" "$ROOT/lang/src/compiler/entry/compiler.hako" 2>&1)
rc=$?
set -e
if [ "$rc" -ne 0 ] || [ ! -s "$tmp_json" ]; then
  echo "[FAIL] selfhost_canary_minimal (emit program failed, rc=$rc)" >&2
  printf '%s\n' "$out" | sed -n '1,120p' >&2
  exit 1
fi

# JSON構造の最小検査（Program JSON / AST-Program 互換）
if ! jq -e '.kind == "Program" and ((.body | type=="array") or (.statements | type=="array"))' "$tmp_json" >/dev/null 2>&1; then
  echo "[FAIL] selfhost_canary_minimal (invalid Program JSON)" >&2
  head -n1 "$tmp_json" >&2 || true
  exit 1
fi

echo "[PASS] selfhost_canary_minimal"
exit 0
