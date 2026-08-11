#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="loop-physical-transfer-authority"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

LAYOUT="$ROOT_DIR/src/mir/loop_recipe_contract/physical_layout.rs"
TRANSFER="$ROOT_DIR/src/mir/loop_recipe_contract/physical_transfer.rs"
VIEW="$ROOT_DIR/src/mir/loop_recipe_contract/join_sig/transfer_view_v1.rs"
ALLOCATOR="$ROOT_DIR/src/mir/builder/resolved_lowering/loop_recipe_physicalizer/segment_allocator.rs"
AFTER="$ROOT_DIR/src/mir/builder/resolved_lowering/loop_recipe_physicalizer/recursive_after.rs"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$LAYOUT" "$TRANSFER" "$VIEW" "$ALLOCATOR" "$AFTER"

guard_expect_fixed_in_file "$TAG" \
  "logical_transfer_view()" "$LAYOUT" \
  "physical layout must consume the JoinSig-owned transfer view"
guard_expect_fixed_in_file "$TAG" \
  "segment.role()" "$ALLOCATOR" \
  "segment allocation must consume the placement-owned segment role"
guard_expect_fixed_in_file "$TAG" \
  "bind_predicate" "$LAYOUT" \
  "predicate transfer binding must have one physical owner"
guard_expect_fixed_in_file "$TAG" \
  "bind_backedge" "$LAYOUT" \
  "backedge transfer binding must have one physical owner"
guard_expect_fixed_in_file "$TAG" \
  "bind_nested_loop" "$LAYOUT" \
  "nested-loop entry binding must have one physical owner"

for file in "$LAYOUT" "$ALLOCATOR" "$AFTER"; do
  if rg -n -F "LoopConditionV1" "$file" >/dev/null 2>&1; then
    guard_fail "$TAG" "physical consumer still reads Recipe condition authority: ${file#"$ROOT_DIR/"}"
  fi
done

for forbidden in \
  'LoopPhysicalTransferV1::Predicate {' \
  'LoopPhysicalTransferV1::Jump {' \
  'LoopPhysicalTransferV1::OpenNestedLoop {'
do
  production_layout="$(mktemp "${TMPDIR:-/tmp}/loop-physical-layout.XXXXXX")"
  sed '/^#\[cfg(test)\]/,$d' "$LAYOUT" >"$production_layout"
  if rg -n -F -- "$forbidden" "$production_layout" >/dev/null 2>&1; then
    rm -f "$production_layout"
    guard_fail "$TAG" "physical layout still constructs transfer directly: $forbidden"
  fi
  rm -f "$production_layout"
done

for forbidden in \
  'as_recipe()' \
  'segment_role' \
  'LoopConditionV1'
do
  if rg -n -F -- "$forbidden" "$ALLOCATOR" >/dev/null 2>&1; then
    guard_fail "$TAG" "segment allocator still reconstructs placement role: $forbidden"
  fi
done

for file in "$LAYOUT" "$TRANSFER" "$VIEW" "$ALLOCATOR" "$AFTER"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "800-line boundary exceeded: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

echo "[$TAG] ok"
