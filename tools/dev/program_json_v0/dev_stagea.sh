#!/usr/bin/env bash
set -euo pipefail

# dev_stagea.sh — Stage‑A (minimal) Hako → JSON v0 → Gate‑C 実行ヘルパー
# 使い方:
#   tools/dev/program_json_v0/dev_stagea.sh 'box Main { static method main() { print(1+2); } }'
#   tools/dev/program_json_v0/dev_stagea.sh -f path/to/code.hako

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
BIN="$ROOT/target/release/nyash"

if [ ! -x "$BIN" ]; then
  echo "[info] building nyash (release) ..." >&2
  (cd "$ROOT" && cargo build --release -q)
fi

CODE=""
if [ "${1:-}" = "-f" ] && [ -n "${2:-}" ]; then
  CODE="$(cat "$2")"
elif [ -n "${1:-}" ]; then
  CODE="$1"
else
  echo "Usage: $0 '<hako code>' | -f <file.hako>" >&2
  exit 2
fi

RAW="/tmp/stagea_raw_$$.txt"
OUT="/tmp/stagea_v0_$$.json"
trap 'rm -f "$RAW" "$OUT"' EXIT

# Stage‑A: compiler.hako に --source を渡して JSON v0 を 1 行出力
NYASH_PARSER_ALLOW_SEMICOLON=1 \
NYASH_SYNTAX_SUGAR_LEVEL=full \
NYASH_ENABLE_ARRAY_LITERAL=1 \
HAKO_ALLOW_USING_FILE=1 NYASH_ALLOW_USING_FILE=1 \
NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 \
"$BIN" --backend vm "$ROOT/lang/src/compiler/entry/compiler.hako" -- --source "$CODE" >"$RAW" 2>&1
awk '/"version":0/ && /"kind":"Program"/ {print; exit}' "$RAW" >"$OUT" || {
  echo "[error] JSON v0 not found in output" >&2
  sed -n '1,80p' "$RAW" >&2
  exit 1
}

# Gate‑C 実行（MIR Interpreter）
NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 NYASH_NYRT_SILENT_RESULT=1 \
"$BIN" --json-file "$OUT"

exit $?
