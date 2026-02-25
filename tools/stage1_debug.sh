#!/usr/bin/env bash
# stage1_debug.sh — Stage‑1 CLI デバッグ用ヘルパー
#
# 目的:
#   - Stage0（Rust）→ Stage1 CLI stub（stage1_cli.hako）の経路を
#     再現性のある形で叩き、関連する環境変数を一括で診断する。
#   - 「いまどのフラグが効いていて、何が不足しているか」を
#     1 コマンドで見えるようにする。
#
# 現状の方針（2025-11 時点）:
#   - 既存の NYASH_USE_STAGE1_CLI / STAGE1_EMIT_PROGRAM_JSON などを
#     内部でセットしつつ、将来の NYASH_STAGE1_MODE などへの移行を見据えた
#     薄いラッパとして実装する。
#
# 使い方:
#   tools/stage1_debug.sh <source.hako>
#   tools/stage1_debug.sh --mode emit-program-json <source.hako>
#

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

usage() {
  cat <<'USAGE' >&2
Usage: tools/stage1_debug.sh [--mode <mode>] <source.hako>

Modes (current stub):
  emit-program-json   : Stage-1 CLI で Program(JSON v0) を emit
  emit-mir-json       : Stage-1 CLI で MIR(JSON) を emit
  run-vm              : Stage-1 CLI で vm backend 実行（予定）

Examples:
  tools/stage1_debug.sh apps/tests/minimal.hako
  tools/stage1_debug.sh --mode emit-mir-json apps/tests/minimal.hako

Note:
  - 現時点では、内部的には既存の NYASH_USE_STAGE1_CLI / STAGE1_EMIT_* を
    マッピングするだけのヘルパーです。
  - 将来 NYASH_STAGE1_MODE などが導入された際は、このスクリプトから
    そちらに橋渡しする予定です。
USAGE
}

MODE="emit-program-json"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --mode)
      [[ $# -ge 2 ]] || { echo "[stage1_debug] --mode requires value" >&2; usage; exit 2; }
      MODE="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    --)
      shift
      break
      ;;
    *)
      break
      ;;
  esac
done

if [[ $# -lt 1 ]]; then
  echo "[stage1_debug] missing <source.hako>" >&2
  usage
  exit 2
fi

SRC="$1"
if [[ ! -f "$SRC" ]]; then
  echo "[stage1_debug] source not found: $SRC" >&2
  exit 1
fi

# Resolve nyash/hakorune binary (Stage0)
if [[ -z "${NYASH_BIN:-}" ]]; then
  if [[ -x "$ROOT_DIR/target/release/hakorune" ]]; then
    NYASH_BIN="$ROOT_DIR/target/release/hakorune"
  else
    NYASH_BIN="$ROOT_DIR/target/release/nyash"
  fi
fi

if [[ ! -x "$NYASH_BIN" ]]; then
  echo "[stage1_debug] nyash binary not found at $NYASH_BIN" >&2
  echo "  build with: cargo build --release" >&2
  exit 1
fi

echo "[stage1_debug] ROOT_DIR = $ROOT_DIR"
echo "[stage1_debug] NYASH_BIN = $NYASH_BIN"
echo "[stage1_debug] MODE      = $MODE"
echo "[stage1_debug] SRC       = $SRC"
echo
echo "[stage1_debug] effective env snapshot (selected keys):"
for k in \
  NYASH_USE_STAGE1_CLI STAGE1_EMIT_PROGRAM_JSON STAGE1_EMIT_MIR_JSON \
  STAGE1_BACKEND STAGE1_SOURCE STAGE1_PROGRAM_JSON \
  NYASH_ENABLE_USING HAKO_ENABLE_USING HAKO_STAGEB_APPLY_USINGS \
  NYASH_PARSER_STAGE3 HAKO_PARSER_STAGE3 \
  NYASH_FILEBOX_MODE NYASH_BOX_FACTORY_POLICY \
  NYASH_SCRIPT_ARGS_JSON NYASH_CLI_VERBOSE STAGE1_CLI_DEBUG \
  NYASH_STAGE1_MODE NYASH_STAGE1_INPUT NYASH_FEATURES
do
  v="${!k-}"
  if [[ -n "$v" ]]; then
    printf "  %-28s= %s\n" "$k" "$v"
  fi
done
echo

# Map logical MODE → current env toggles
case "$MODE" in
  emit-program-json)
    export NYASH_USE_STAGE1_CLI=1
    export STAGE1_EMIT_PROGRAM_JSON=1
    unset STAGE1_EMIT_MIR_JSON
    export STAGE1_BACKEND="${STAGE1_BACKEND:-vm}"
    export NYASH_TO_I64_FORCE_ZERO=1
    ;;
  emit-mir-json)
    export NYASH_USE_STAGE1_CLI=1
    export STAGE1_EMIT_MIR_JSON=1
    unset STAGE1_EMIT_PROGRAM_JSON
    export STAGE1_BACKEND="${STAGE1_BACKEND:-vm}"
    export NYASH_TO_I64_FORCE_ZERO=1
    ;;
  run-vm)
    export NYASH_USE_STAGE1_CLI=1
    unset STAGE1_EMIT_PROGRAM_JSON STAGE1_EMIT_MIR_JSON
    export STAGE1_BACKEND=vm
    export NYASH_TO_I64_FORCE_ZERO=1
    ;;
  *)
    echo "[stage1_debug] unknown mode: $MODE" >&2
    exit 2
    ;;
esac

export STAGE1_SOURCE="$SRC"

# Recommended debug toggles for Stage1 path
export NYASH_CLI_VERBOSE="${NYASH_CLI_VERBOSE:-1}"
export STAGE1_CLI_DEBUG="${STAGE1_CLI_DEBUG:-1}"
export NYASH_ALLOW_NYASH="${NYASH_ALLOW_NYASH:-1}"
export HAKO_ALLOW_NYASH="${HAKO_ALLOW_NYASH:-1}"
# Stage-3 is now default: prefer NYASH_FEATURES=stage3, keep legacy env only when explicitly provided.
export NYASH_FEATURES="${NYASH_FEATURES:-stage3}"
export NYASH_ENABLE_USING="${NYASH_ENABLE_USING:-1}"
export HAKO_ENABLE_USING="${HAKO_ENABLE_USING:-1}"

echo "[stage1_debug] running Stage1 CLI via Stage0..."
echo "  $NYASH_BIN $SRC"
echo

set +e
"$NYASH_BIN" "$SRC"
rc=$?
set -e

echo
echo "[stage1_debug] exit code: $rc"
exit "$rc"
