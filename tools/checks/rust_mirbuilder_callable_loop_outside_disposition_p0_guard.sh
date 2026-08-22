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
require_match "owner: FunctionOwnerIdV1" "$handoff"
require_match "rows: Box<[CallableLoopOutsideRowV1]>" "$handoff"
require_match "CallableLoopObservedBindingRowV1" "$handoff"
require_match "CallableLoopReadyBindingClassV1" "$handoff"
require_match "CallableLoopOutsideKindV1::BodyOnlyRebind" "$handoff"
require_match "build_callable_loop_ready_row" "$handoff"
require_match "build_callable_loop_outside_row" "$handoff"
require_match "validate_ready_remainder" "$handoff"
require_match "CallableLoopBindingProjectionDispositionV1::Ready(schedule)" "$raw_entry"
require_match "CallableLoopBindingProjectionDispositionV1::Outside(reason)" "$raw_entry"
require_match "lower_outside_callable_loop_v1" "$raw_entry"
require_match "Err(reason.into_terminal_error())" "$raw_entry"
require_match ".project_disposition(loop_site)" "$recursive"

if rg -n --fixed-strings "CallableLoopBindingClassV1" "$repo_root/src/mir/builder" \
  || rg -n --fixed-strings "CallableLoopBindingCoverageRowV1" "$repo_root/src/mir/builder"; then
  echo "[callable-loop-outside-p0] old shared Ready/Outside row vocabulary remains" >&2
  exit 1
fi

if rg -n --fixed-strings "bindings: Box<[BindingRefV1]>" "$handoff" \
  || rg -n --fixed-strings "sites: Box<[SourceNodeSiteV1]>" "$handoff"; then
  echo "[callable-loop-outside-p0] Outside reason still stores unpaired bindings/sites" >&2
  exit 1
fi

if rg -n -A 8 "struct CallableLoopOutsideReasonV1" "$handoff" | rg -q -- 'Clone|Copy'; then
  echo "[callable-loop-outside-p0] Outside reason must remain move-only" >&2
  exit 1
fi

ready_row_builder_count="$(rg -F -o -- 'build_callable_loop_ready_row(' "$handoff" | wc -l | tr -d '[:space:]')"
if [[ "$ready_row_builder_count" -ne 2 ]]; then
  echo "[callable-loop-outside-p0] Ready row builder must have one definition and one caller; found $ready_row_builder_count" >&2
  exit 1
fi

outside_row_builder_count="$(rg -F -o -- 'build_callable_loop_outside_row(' "$handoff" | wc -l | tr -d '[:space:]')"
if [[ "$outside_row_builder_count" -ne 2 ]]; then
  echo "[callable-loop-outside-p0] Outside row builder must have one definition and one caller; found $outside_row_builder_count" >&2
  exit 1
fi

require_match "reason.rows()" "$repo_root/src/mir/builder/normal_callable_loop_handoff_tests.rs"
require_match "BodyRebind" "$repo_root/src/mir/builder/normal_callable_loop_handoff_tests.rs"
require_match "CallableLoopOutsideKindV1::BodyOnlyRebind" "$repo_root/src/mir/builder/normal_callable_loop_handoff_tests.rs"

outside_branch="$(awk '
/if !outside_bindings\.is_empty\(\)/ { inside = 1 }
inside { print }
inside && /return Ok\(CallableLoopBindingProjectionDispositionV1::Outside/ { exit }
' "$handoff")"
if rg -q --fixed-strings "VerifiedCallableSemanticLoopBindingScheduleV1::seal" <<<"$outside_branch"; then
  echo "[callable-loop-outside-p0] Outside branch still issues a throwaway Verified schedule" >&2
  exit 1
fi

if rg -n --fixed-strings ".project(loop_site)" "$recursive"; then
  echo "[callable-loop-outside-p0] recursive child path still uses Ready-only project()" >&2
  exit 1
fi

if rg -n -A 12 "fn lower_outside_callable_loop_v1" "$raw_entry" | rg -q "lower_loop_or_freeze_v1"; then
  echo "[callable-loop-outside-p0] Outside helper still enters ordinary JoinIR" >&2
  exit 1
fi

for file in "$handoff" "$raw_entry" "$recursive"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    echo "[callable-loop-outside-p0] source reached the 800-line hard boundary: ${file#$repo_root/}=$lines" >&2
    exit 1
  fi
done

echo "[callable-loop-outside-p0] ok"
