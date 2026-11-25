#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="joinir-nested-resolved-fallback-guard"
COMPILER_DIR="$ROOT_DIR/src/mir/compiler"
COMPILER_MOD="$COMPILER_DIR/mod.rs"

fail() {
  printf '[%s] ERROR: %s\n' "$TAG" "$1" >&2
  exit 1
}

require_file() {
  [[ -f "$1" ]] || fail "missing file: ${1#"$ROOT_DIR/"}"
}

require_file "$COMPILER_MOD"

for forbidden in \
  'route_loop(' \
  'try_cf_loop_joinir(' \
  'lower_loop_or_freeze_v1(' \
  'route_nested_loop_minimal'
do
  if rg -n -F "$forbidden" "$COMPILER_DIR" >/dev/null 2>&1; then
    fail "resolved compiler ingress still references legacy Loop edge: $forbidden"
  fi
done

for path in "$COMPILER_MOD"; do
  lines="$(wc -l < "$path" | tr -d '[:space:]')"
  (( lines < 800 )) || fail "800-line boundary exceeded: ${path#"$ROOT_DIR/"}=$lines"
done

printf '[%s] ok: no legacy Loop edge; shared compiler sources stay below 800 lines\n' "$TAG"
