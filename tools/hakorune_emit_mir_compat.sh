#!/usr/bin/env bash
# hakorune_emit_mir_compat.sh — Program→MIR compat route (delegate/fallback allowed)
#
# Usage: tools/hakorune_emit_mir_compat.sh <input.hako> <out.json>

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

exec env \
  HAKO_SELFHOST_BUILDER_FIRST=0 \
  HAKO_SELFHOST_NO_DELEGATE=0 \
  HAKO_EMIT_MIR_MAINLINE_ONLY=0 \
  bash "$SCRIPT_DIR/hakorune_emit_mir.sh" "$@"
