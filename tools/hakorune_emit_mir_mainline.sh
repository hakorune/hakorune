#!/usr/bin/env bash
# hakorune_emit_mir_mainline.sh — Program→MIR mainline route preset (no compat fallback)
#
# Usage: tools/hakorune_emit_mir_mainline.sh <input.hako> <out.json>
# Notes:
#   - thin top-level compatibility wrapper only
#   - operational route SSOT is tools/smokes/v2/lib/emit_mir_route.sh --route hako-mainline

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

if [ "$#" -ne 2 ]; then
  echo "Usage: $0 <input.hako> <out.json>" >&2
  exit 2
fi

IN="$1"
OUT="$2"

case "$IN" in
  lang/src/runner/stage1_cli.hako|"$ROOT/lang/src/runner/stage1_cli.hako")
    IN="$ROOT/lang/src/runner/stage1_cli_env.hako"
    ;;
esac

exec env \
  HAKO_SELFHOST_BUILDER_FIRST=1 \
  HAKO_SELFHOST_NO_DELEGATE=1 \
  HAKO_SELFHOST_TRY_MIN=0 \
  HAKO_EMIT_MIR_MAINLINE_ONLY=1 \
  bash "$SCRIPT_DIR/hakorune_emit_mir.sh" "$IN" "$OUT"
