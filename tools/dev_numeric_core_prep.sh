#!/usr/bin/env bash
# dev_numeric_core_prep.sh — numeric_core + AotPrep 付きで MIR(JSON) を吐く開発用ヘルパー
#
# Usage:
#   tools/dev_numeric_core_prep.sh <input.hako> <out.json>
# Notes:
#   - HAKO_APPLY_AOT_PREP=1 と NYASH_AOT_NUMERIC_CORE=1 を必ず立てて、
#     emit_mir_route.sh (--route hako-helper) を呼び出す。
#   - NYASH_SKIP_TOML_ENV / NYASH_DISABLE_PLUGINS など、開発用の最小クリーン環境を既定ONにする。

set -euo pipefail

if [ "$#" -ne 2 ]; then
  echo "Usage: $0 <input.hako> <out.json>" >&2
  exit 2
fi

IN="$1"
OUT="$2"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

if [ ! -f "$IN" ]; then
  echo "[FAIL] input not found: $IN" >&2
  exit 1
fi

export HAKO_APPLY_AOT_PREP=1
export NYASH_AOT_NUMERIC_CORE="${NYASH_AOT_NUMERIC_CORE:-1}"
export NYASH_AOT_NUMERIC_CORE_TRACE="${NYASH_AOT_NUMERIC_CORE_TRACE:-1}"

# 開発用: 余計な TOML/env/plugin の影響を避ける
export NYASH_SKIP_TOML_ENV="${NYASH_SKIP_TOML_ENV:-1}"
export NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}"

# Stage‑B/AotPrep のトレースを既定ON（必要に応じて上書き可）
export HAKO_SELFHOST_TRACE="${HAKO_SELFHOST_TRACE:-1}"

# JSON だけを期待する（stdoutノイズ対策）
export NYASH_JSON_ONLY="${NYASH_JSON_ONLY:-1}"

echo "[dev_numeric_core] input=$IN out=$OUT" >&2
echo "[dev_numeric_core] HAKO_APPLY_AOT_PREP=$HAKO_APPLY_AOT_PREP NYASH_AOT_NUMERIC_CORE=$NYASH_AOT_NUMERIC_CORE" >&2

bash "$ROOT/tools/smokes/v2/lib/emit_mir_route.sh" --route hako-helper --timeout-secs "${HAKO_BUILD_TIMEOUT:-60}" --out "$OUT" --input "$IN"

echo "[dev_numeric_core] MIR JSON written with numeric_core+AotPrep: $OUT" >&2
