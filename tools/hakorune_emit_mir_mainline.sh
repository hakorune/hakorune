#!/usr/bin/env bash
# hakorune_emit_mir_mainline.sh — Program→MIR mainline route (no compat fallback)
#
# Usage: tools/hakorune_emit_mir_mainline.sh <input.hako> <out.json>

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

exec env \
  HAKO_SELFHOST_BUILDER_FIRST=1 \
  HAKO_SELFHOST_NO_DELEGATE=1 \
  HAKO_SELFHOST_TRY_MIN=0 \
  HAKO_EMIT_MIR_MAINLINE_ONLY=1 \
  bash "$SCRIPT_DIR/hakorune_emit_mir.sh" "$@"
