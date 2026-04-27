#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-builder-calltarget-owner-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
cd "$ROOT_DIR"

BUILDER_ROOT="src/mir/builder.rs"
COMPAT_SHELL="src/mir/builder/builder_calls.rs"

guard_require_command "$TAG" rg
guard_require_files "$TAG" "$BUILDER_ROOT" "$COMPAT_SHELL" "src/mir/builder/calls/call_target.rs"

echo "[$TAG] checking CallTarget owner path"

guard_expect_in_file \
  "$TAG" \
  '^pub\(crate\) use calls::CallTarget;' \
  "$BUILDER_ROOT" \
  "builder root must re-export CallTarget from calls::CallTarget"

bad_builder_calls_path="$(
  rg -n 'builder_calls::CallTarget' src/mir/builder -g '*.rs' || true
)"
if [ -n "$bad_builder_calls_path" ]; then
  echo "[$TAG] ERROR: CallTarget imported through builder_calls compatibility path" >&2
  printf '%s\n' "$bad_builder_calls_path" >&2
  exit 1
fi

bad_compat_reexport="$(
  rg -n 'pub use super::calls::call_target::CallTarget|pub use super::calls::CallTarget' \
    "$COMPAT_SHELL" || true
)"
if [ -n "$bad_compat_reexport" ]; then
  echo "[$TAG] ERROR: builder_calls CallTarget compatibility re-export found" >&2
  printf '%s\n' "$bad_compat_reexport" >&2
  exit 1
fi

echo "[$TAG] ok"
