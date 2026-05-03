#!/usr/bin/env bash
set -euo pipefail

# program_json_v0_dev_stageb.sh — archived Stage-B Hako -> JSON v0 -> Gate-C helper
# Usage:
#   tools/archive/legacy-selfhost/engineering/program_json_v0_dev_stageb.sh 'box Main { static method main() { print(1+2); } }'
#   tools/archive/legacy-selfhost/engineering/program_json_v0_dev_stageb.sh -f path/to/code.hako

ROOT="${NYASH_ROOT:-$(cd "$(dirname "$0")/../../../.." && pwd)}"
BIN="$ROOT/target/release/nyash"
STAGEA="$ROOT/tools/archive/legacy-selfhost/engineering/program_json_v0_dev_stagea.sh"

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

RAW="/tmp/stageb_raw_$$.txt"
OUT="/tmp/stageb_v0_$$.json"
trap 'rm -f "$RAW" "$OUT"' EXIT

# Stage‑B: compiler_stageb.hako に --source を渡して JSON v0 を 1 行出力
# 必要な開発ENV（using/file許可とStage3）を付与
NYASH_PARSER_ALLOW_SEMICOLON=1 \
NYASH_SYNTAX_SUGAR_LEVEL=full \
NYASH_ENABLE_ARRAY_LITERAL=1 \
HAKO_ALLOW_USING_FILE=1 NYASH_ALLOW_USING_FILE=1 \
NYASH_FEATURES=stage3 NYASH_FEATURES=stage3 \
NYASH_VARMAP_GUARD_STRICT=0 NYASH_BLOCK_SCHEDULE_VERIFY=0 NYASH_PHI_VERIFY=0 \
NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 \
"$BIN" --backend vm "$ROOT/lang/src/compiler/entry/compiler_stageb.hako" -- --source "$CODE" >"$RAW" 2>&1 || true

if ! awk '/"version":0/ && /"kind":"Program"/ {print > out; found=1; exit} END{exit (!found)}' out="$OUT" "$RAW"; then
  echo "[warn] Stage‑B emit failed or empty; falling back to Stage‑A" >&2
  exec "$STAGEA" -f <(printf '%s\n' "$CODE")
fi

# Gate‑C 実行（MIR Interpreter）
NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 NYASH_NYRT_SILENT_RESULT=1 \
"$BIN" --json-file "$OUT"

exit $?
