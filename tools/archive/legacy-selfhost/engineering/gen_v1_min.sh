#!/usr/bin/env bash
# Archived minimal v1 JSON generator.
# Historical engineering helper only; keep it frozen and non-growing.
set -euo pipefail

# gen_v1_min.sh — archived minimal v1 JSON generator (historical helper only)
# 標準出力に v1 JSON を出す。

ROOT="$(cd "$(dirname "$0")/../../../.." && pwd)"
NYASH_BIN="${NYASH_BIN:-}"
if [ -z "$NYASH_BIN" ]; then
  if [ -x "$ROOT/target/release/hakorune" ]; then
    NYASH_BIN="$ROOT/target/release/hakorune"
  else
    NYASH_BIN="$ROOT/target/release/nyash"
  fi
fi
if [ ! -x "$NYASH_BIN" ]; then
  echo "[FAIL] nyash/hakorune not found; build first: cargo build --release" >&2
  exit 2
fi

CODE='include "lang/src/mir/min_emitter.hako"
static box Main { method main(args) {
  local j = MinMirEmitter.emit_if_gt_i64(3,5)
  print(j)
  return 0
} }'

tmp="/tmp/gen_v1_min_$$.nyash"; echo "$CODE" > "$tmp"
pref="/tmp/gen_v1_min_pre_$$.nyash"
# Text preinclude（.hako using を安全に統合）
if [ -x "$ROOT/tools/dev/hako_preinclude.sh" ]; then
  "$ROOT/tools/dev/hako_preinclude.sh" "$tmp" "$pref" >/dev/null || cp "$tmp" "$pref"
else
  cp "$tmp" "$pref"
fi
# Remove optional helper using that is not needed for inline run and may not resolve in compat path
sed -i '/using selfhost\.shared\.common\.entry_point_base/d' "$pref" 2>/dev/null || true

# できるだけ静かに実行して JSON のみを出力（explicit compat bridge）
NYASH_ROOT="$ROOT" NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 \
NYASH_FEATURES=stage3 NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 NYASH_USING_AST=0 \
"$NYASH_BIN" --backend vm "$pref" 2>/dev/null | tr -d '\r'
rm -f "$tmp" "$pref" 2>/dev/null || true
