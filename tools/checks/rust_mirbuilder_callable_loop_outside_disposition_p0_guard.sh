#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
handoff="$repo_root/src/mir/builder/normal_callable_loop_handoff.rs"
recursive="$repo_root/src/mir/builder/recursive_child_lowering.rs"
raw_entry="$repo_root/src/mir/builder/raw_loop_child_entry.rs"

require_match() {
  local pattern="$1"
  local path="$2"
  if ! rg -q --fixed-strings "$pattern" "$path"; then
    echo "[callable-loop-outside-p0] missing: $pattern in ${path#$repo_root/}" >&2
    exit 1
  fi
}

require_match "CallableLoopBindingProjectionDispositionV1" "$handoff"
require_match "project_disposition" "$handoff"
require_match "CallableLoopBindingProjectionDispositionV1::Ready(handoff)" "$raw_entry"
require_match "CallableLoopBindingProjectionDispositionV1::Outside(reason)" "$raw_entry"
require_match "lower_outside_callable_loop_v1" "$raw_entry"
require_match ".project_disposition(loop_site)" "$recursive"

if rg -n --fixed-strings ".project(loop_site)" "$recursive"; then
  echo "[callable-loop-outside-p0] recursive child path still uses Ready-only project()" >&2
  exit 1
fi

echo "[callable-loop-outside-p0] ok"
