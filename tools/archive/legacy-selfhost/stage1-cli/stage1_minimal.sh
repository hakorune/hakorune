#!/usr/bin/env bash
# stage1_minimal.sh — Stage‑1 CLI 経路の最小実行ヘルパー
#
# 目的:
#   - 「環境変数をあまり意識せずに」 Stage‑1 CLI 経路で
#     emit program-json / emit mir-json / run-vm を叩くための
#     ごく薄いラッパ。
#   - 中身は既存の NYASH_USE_STAGE1_CLI / STAGE1_EMIT_* に
#     マッピングしているが、外形としては
#       NYASH_STAGE1_MODE / NYASH_STAGE1_INPUT / NYASH_FEATURES
#     の 3 変数イメージで扱えるようにしておく。
#
# 使い方（暫定仕様）:
#   NYASH_STAGE1_MODE=emit-program-json \
#   NYASH_STAGE1_INPUT=apps/tests/minimal.hako \
#   ./tools/archive/legacy-selfhost/stage1-cli/stage1_minimal.sh
#
#   # 省略時デフォルト:
#   #   MODE = emit-program-json
#   #   INPUT = 第1引数 or 必須
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT_DIR="$ROOT_GIT"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/../../../.." && pwd)"
fi

usage() {
  cat <<'USAGE' >&2
Usage: tools/archive/legacy-selfhost/stage1-cli/stage1_minimal.sh [<source.hako>]

Environment (logical vars):
  NYASH_STAGE1_MODE   : emit-program-json | emit-mir-json | run-vm (default: emit-program-json)
  NYASH_STAGE1_INPUT  : source.hako path（未設定なら第1引数）
  NYASH_FEATURES      : カンマ区切り using,parser-stage3,plugins など（暫定）

Examples:
  NYASH_STAGE1_MODE=emit-program-json \
    NYASH_STAGE1_INPUT=apps/tests/minimal.hako \
    ./tools/archive/legacy-selfhost/stage1-cli/stage1_minimal.sh

  ./tools/archive/legacy-selfhost/stage1-cli/stage1_minimal.sh apps/tests/minimal.hako
    # → MODE=emit-program-json, INPUT=apps/tests/minimal.hako で実行
USAGE
}

MODE="${NYASH_STAGE1_MODE:-emit-program-json}"
INPUT="${NYASH_STAGE1_INPUT:-}"

if [[ $# -ge 1 && -z "$INPUT" ]]; then
  INPUT="$1"
fi

if [[ -z "$INPUT" ]]; then
  echo "[stage1_minimal] missing NYASH_STAGE1_INPUT or <source.hako>" >&2
  usage
  exit 2
fi

if [[ ! -f "$INPUT" ]]; then
  echo "[stage1_minimal] source not found: $INPUT" >&2
  exit 1
fi

# Resolve nyash/hakorune binary
if [[ -z "${NYASH_BIN:-}" ]]; then
  if [[ -x "$ROOT_DIR/target/release/hakorune" ]]; then
    NYASH_BIN="$ROOT_DIR/target/release/hakorune"
  else
    NYASH_BIN="$ROOT_DIR/target/release/nyash"
  fi
fi

if [[ ! -x "$NYASH_BIN" ]]; then
  echo "[stage1_minimal] nyash binary not found at $NYASH_BIN" >&2
  echo "  build with: cargo build --release" >&2
  exit 1
fi

echo "[stage1_minimal] ROOT_DIR = $ROOT_DIR"
echo "[stage1_minimal] NYASH_BIN = $NYASH_BIN"
echo "[stage1_minimal] MODE      = $MODE"
echo "[stage1_minimal] INPUT     = $INPUT"

# Map logical flags to current envs
export STAGE1_SOURCE="$INPUT"
export NYASH_USE_STAGE1_CLI=1

case "$MODE" in
  emit-program-json)
    export STAGE1_EMIT_PROGRAM_JSON=1
    unset STAGE1_EMIT_MIR_JSON
    export STAGE1_BACKEND="${STAGE1_BACKEND:-vm}"
    ;;
  emit-mir-json)
    export STAGE1_EMIT_MIR_JSON=1
    unset STAGE1_EMIT_PROGRAM_JSON
    export STAGE1_BACKEND="${STAGE1_BACKEND:-vm}"
    ;;
  run-vm)
    unset STAGE1_EMIT_PROGRAM_JSON STAGE1_EMIT_MIR_JSON
    export STAGE1_BACKEND=vm
    ;;
  *)
    echo "[stage1_minimal] unknown NYASH_STAGE1_MODE: $MODE" >&2
    exit 2
    ;;
esac

# nyash_debug (統合案) は現状未使用。従来どおり CLI_VERBOSE/STAGE1_CLI_DEBUG を直接見る。
export NYASH_CLI_VERBOSE=1
export STAGE1_CLI_DEBUG="${STAGE1_CLI_DEBUG:-1}"

# Features (暫定: using / parser-stage3 / plugins のみ見ておく)
# Stage-3 は既定ON: 明示がなければ NYASH_FEATURES=stage3 を付与する。
FEATURES="${NYASH_FEATURES:-stage3}"
export NYASH_FEATURES="$FEATURES"
if [[ "$FEATURES" == *"using"* ]]; then
  export NYASH_ENABLE_USING=1
  export HAKO_ENABLE_USING=1
fi
if [[ "$FEATURES" == *"plugins"* ]]; then
  export NYASH_DISABLE_PLUGINS=0
else
  # デフォルトは安全側: plugin許可だが config に委ねる
  : # ここでは特に上書きしない
fi

echo "[stage1_minimal] executing via Stage1 CLI..."
echo "  $NYASH_BIN $INPUT"
echo

set +e
"$NYASH_BIN" "$INPUT"
rc=$?
set -e

echo
echo "[stage1_minimal] exit code: $rc"
exit "$rc"
