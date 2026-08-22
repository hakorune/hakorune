#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="pure-compile-generic-dispatch-split"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

PARENT="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_op_dispatch.inc"
CALLS="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_op_dispatch_calls.inc"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$PARENT" "$CALLS"

count_fixed() {
  local needle="$1"
  local file="$2"
  (rg -F -o -- "$needle" "$file" || true) | wc -l | tr -d '[:space:]'
}

if [[ "$(count_fixed '#include "hako_llvmc_ffi_pure_compile_generic_lowering_op_dispatch_calls.inc"' "$PARENT")" != "1" ]]; then
  guard_fail "$TAG" "parent must include the call-family child exactly once"
fi

for op in call mir_call; do
  needle="if (strcmp(op, \"$op\")==0)"
  if [[ "$(count_fixed "$needle" "$CALLS")" != "1" ]]; then
    guard_fail "$TAG" "$op arm must live exactly once in the call-family child"
  fi
  if [[ "$(count_fixed "$needle" "$PARENT")" != "0" ]]; then
    guard_fail "$TAG" "$op arm must not remain in the parent"
  fi
done

if [[ "$(count_fixed 'if (strcmp(op, "checked_callout")==0)' "$PARENT")" != "1" ]]; then
  guard_fail "$TAG" "checked_callout must remain owned by the parent"
fi
if [[ "$(count_fixed 'note_pure_unsupported_shape(bid, ii, op, "unknown_op", NULL);' "$PARENT")" != "1" ]]; then
  guard_fail "$TAG" "the parent must retain the sole final unknown-op rejection"
fi

if rg -n 'PinnedTextOp|PinnedTextResidence|pinned_text_(op|residence)' "$CALLS"; then
  guard_fail "$TAG" "behavior-neutral call extraction must not add pinned Text lifecycle support"
fi

for file in "$PARENT" "$CALLS"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 760 )); then
    guard_fail "$TAG" "dispatch source reached the 760-line split trigger: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

echo "[$TAG] ok (parent=$(wc -l < "$PARENT" | tr -d '[:space:]'), calls=$(wc -l < "$CALLS" | tr -d '[:space:]'))"
