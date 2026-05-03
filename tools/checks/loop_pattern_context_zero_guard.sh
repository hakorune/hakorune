#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="loop-pattern-context-zero-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
TMP_FILE="${TMPDIR:-/tmp}/loop_pattern_context_hits.$$"
trap 'rm -f "$TMP_FILE"' EXIT

if rg -n "\\bLoopPatternContext\\b" "$ROOT_DIR/src" --glob '*.rs' >"$TMP_FILE"; then
  echo "[FAIL] LoopPatternContext must be fully removed from src/**/*.rs" >&2
  cat "$TMP_FILE" >&2
  exit 1
fi

echo "[$TAG] ok"
