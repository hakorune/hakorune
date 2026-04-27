#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

MODULE="src/mir/loop_route_detection/mod.rs"
LEGACY_DIR="src/mir/loop_route_detection/legacy"

echo "[route-detector-legacy-surface-guard] checking route detector legacy boundary"

if [ ! -f "$MODULE" ]; then
  echo "[route-detector-legacy-surface-guard] ERROR: missing file: $MODULE" >&2
  exit 1
fi

bad_parent_legacy="$(
  rg -n '^\s*(pub(\([^)]*\))?\s+)?mod\s+legacy\s*;|^\s*pub(\([^)]*\))?\s+use\s+legacy::' "$MODULE" || true
)"
if [ -n "$bad_parent_legacy" ]; then
  echo "[route-detector-legacy-surface-guard] ERROR: legacy module surface reintroduced" >&2
  printf '%s\n' "$bad_parent_legacy" >&2
  exit 1
fi

if [ -d "$LEGACY_DIR" ]; then
  legacy_files="$(find "$LEGACY_DIR" -type f | sort || true)"
  if [ -n "$legacy_files" ]; then
    echo "[route-detector-legacy-surface-guard] ERROR: legacy storage files reintroduced" >&2
    printf '%s\n' "$legacy_files" >&2
    exit 1
  fi
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
  rg -n 'loop_route_detection::legacy' src tests -g '*.rs' || true
)"
if [ -n "$bad_legacy_callers" ]; then
  echo "[route-detector-legacy-surface-guard] ERROR: direct route detector legacy caller found" >&2
  printf '%s\n' "$bad_legacy_callers" >&2
  exit 1
fi

bad_loop_pattern_kind="$(
  rg -n '\bLoopPatternKind\b' src tests -g '*.rs' || true
)"
if [ -n "$bad_loop_pattern_kind" ]; then
  echo "[route-detector-legacy-surface-guard] ERROR: LoopPatternKind legacy alias found in live code" >&2
  printf '%s\n' "$bad_loop_pattern_kind" >&2
  exit 1
fi

echo "[route-detector-legacy-surface-guard] ok"
