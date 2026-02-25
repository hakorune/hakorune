#!/usr/bin/env bash
# hako_debug_run.sh — Stable wrapper to run .hako with Stage‑3
# Usage:
#   tools/dev/hako_debug_run.sh [--internal|--delegate] [--core] [--print-env] <file.hako>
#   tools/dev/hako_debug_run.sh [--internal|--delegate] -c '<code>'
# Notes:
#   - Enables Stage‑3 + semicolon tolerance（smokes runner と同等）
#   - 実行モード:
#       --raw  （既定）: 直実行。inline Ny コンパイラ有効（timeoutは延長）。include が必要な時はこちら。
#       --safe        : ランナー経由。inline Ny コンパイラ無効化＋ノイズフィルタ。
#       --no-compiler : inline Ny コンパイラを明示的に無効化（--raw と併用可）。
#   - Uses tools/smokes/v2/lib/test_runner.sh under the hood（safe モード時）

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env >/dev/null || exit 2

MODE_CODE=0
CODE=""
FILE=""
USE_INTERNAL=0
USE_DELEGATE=0
USE_CORE=0
PRINT_ENV=0
USE_RAW=1
NO_COMPILER=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    -c|--code)
      MODE_CODE=1
      CODE="${2:-}"
      shift 2
      ;;
    --internal)
      USE_INTERNAL=1; shift ;;
    --delegate)
      USE_DELEGATE=1; shift ;;
    --core)
      USE_CORE=1; shift ;;
    --raw)
      USE_RAW=1; shift ;;
    --safe)
      USE_RAW=0; shift ;;
    --no-compiler)
      NO_COMPILER=1; shift ;;
    --print-env)
      PRINT_ENV=1; shift ;;
    -h|--help)
      echo "Usage: $0 [--internal|--delegate] [--core] [--print-env] <file.hako> | -c '<code>'"; exit 0 ;;
    *)
      FILE="$1"; shift ;;
  esac
done

if [[ "$MODE_CODE" -eq 0 && -z "$FILE" ]]; then
  echo "[ERR] No file or -c '<code>' specified" >&2
  exit 2
fi

# Base env (Stage-3 + tolerance)
export NYASH_FEATURES=stage3
export NYASH_FEATURES=stage3
export NYASH_PARSER_ALLOW_SEMICOLON=1
export NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1

# Compiler policy (default: enabled, longer timeout). Use --no-compiler to disable.
export NYASH_NY_COMPILER_TIMEOUT_MS="${NYASH_NY_COMPILER_TIMEOUT_MS:-8000}"
if [[ "$NO_COMPILER" -eq 1 ]]; then
  export NYASH_DISABLE_NY_COMPILER=1
  export HAKO_DISABLE_NY_COMPILER=1
fi

if [[ "$USE_INTERNAL" -eq 1 ]]; then export HAKO_MIR_BUILDER_INTERNAL=1; fi
if [[ "$USE_DELEGATE" -eq 1 ]]; then export HAKO_MIR_BUILDER_DELEGATE=1; fi
if [[ "$USE_CORE" -eq 1 ]]; then export NYASH_GATE_C_CORE=1; export HAKO_GATE_C_CORE=1; fi

if [[ "$PRINT_ENV" -eq 1 ]]; then
  echo "[ENV] NYASH_BIN=$NYASH_BIN"
  env | grep -E '^(NYASH_|HAKO_)' | sort
fi

if [[ "$USE_RAW" -eq 1 ]]; then
  # Direct run (inline compiler allowed unless --no-compiler)
  if [[ "$MODE_CODE" -eq 1 ]]; then
    tmpf="/tmp/hako_debug_run_$$.hako"; printf "%s\n" "$CODE" > "$tmpf"
    "$NYASH_BIN" --backend vm "$tmpf"; rc=$?; rm -f "$tmpf"; exit $rc
  else
    "$NYASH_BIN" --backend vm "$FILE"
  fi
else
  # Safe run via test runner (inline compiler disabled; noise filtered)
  if [[ "$MODE_CODE" -eq 1 ]]; then
    run_nyash_vm -c "$CODE"
  else
    run_nyash_vm "$FILE"
  fi
fi
