#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

MODULE="src/mir/loop_route_detection/mod.rs"
SUPPORT_FACADE="src/mir/loop_route_detection/support/mod.rs"

echo "[route-detector-legacy-surface-guard] checking route detector legacy boundary"

for file in "$MODULE" "$SUPPORT_FACADE"; do
  if [ ! -f "$file" ]; then
    echo "[route-detector-legacy-surface-guard] ERROR: missing file: $file" >&2
    exit 1
  fi
done

bad_parent_surface="$(
  rg -n '^\s*pub(\([^)]*\))?\s+(mod\s+legacy\s*;|use\s+legacy::)' "$MODULE" || true
)"
if [ -n "$bad_parent_surface" ]; then
  echo "[route-detector-legacy-surface-guard] ERROR: legacy surface re-exported from parent module" >&2
  printf '%s\n' "$bad_parent_surface" >&2
  exit 1
fi

bad_compat_callers="$(
  rg -n 'loop_route_detection::(break_condition_analyzer|function_scope_capture|loop_body_carrier_promoter|loop_body_cond_promoter|loop_condition_scope|mutable_accumulator_analyzer|pinned_local_analyzer|trim_loop_helper)' \
    src tests -g '*.rs' || true
)"
if [ -n "$bad_compat_callers" ]; then
  echo "[route-detector-legacy-surface-guard] ERROR: old route detector compatibility path caller found" >&2
  printf '%s\n' "$bad_compat_callers" >&2
  exit 1
fi

bad_legacy_callers="$(
  rg -n 'loop_route_detection::legacy' src tests -g '*.rs' \
    | grep -v "^${SUPPORT_FACADE}:" || true
)"
if [ -n "$bad_legacy_callers" ]; then
  echo "[route-detector-legacy-surface-guard] ERROR: direct private legacy caller outside support facade" >&2
  printf '%s\n' "$bad_legacy_callers" >&2
  exit 1
fi

echo "[route-detector-legacy-surface-guard] ok"
