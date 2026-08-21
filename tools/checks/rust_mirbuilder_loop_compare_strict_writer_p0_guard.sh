#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-loop-compare-strict-writer-p0"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

BUILDER_EMIT="$ROOT_DIR/src/mir/builder/builder_emit.rs"
APPEND_CORE="$ROOT_DIR/src/mir/builder/builder_emit_core.rs"
WRITER="$ROOT_DIR/src/mir/builder/resolved_lowering/loop_recipe_physicalizer/compare_i64_writer.rs"
WRITER_TESTS="$ROOT_DIR/src/mir/builder/resolved_lowering/loop_recipe_physicalizer/compare_i64_writer_tests.rs"
PHYSICALIZER_MOD="$ROOT_DIR/src/mir/builder/resolved_lowering/loop_recipe_physicalizer/mod.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mirbuilder-loop-compare-strict-writer-p0-2026-08-22.md"
README="$ROOT_DIR/src/mir/builder/resolved_lowering/README.md"
REFERENCE="$ROOT_DIR/docs/reference/mir/canonical-loop-compare-same-block.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/rust_mirbuilder_loop_compare_strict_writer_p0_guard.sh"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$BUILDER_EMIT" "$APPEND_CORE" "$WRITER" \
  "$WRITER_TESTS" "$PHYSICALIZER_MOD" "$CARD" "$README" "$REFERENCE" "$INDEX"

guard_expect_fixed_in_file "$TAG" "append_instruction_core" "$APPEND_CORE" \
  "strict writer must use the shared append core"
guard_expect_fixed_in_file "$TAG" "PreparedCanonicalCompareAppendV1" "$APPEND_CORE" \
  "strict writer must expose a prepared append state"
guard_expect_fixed_in_file "$TAG" "CanonicalCompareDefinitionSourceV1" "$APPEND_CORE" \
  "strict commit must return the writer-owned definition source"
guard_expect_fixed_in_file "$TAG" "CanonicalLoopCompareI64WriterV1" "$WRITER" \
  "Loop Compare must have one named strict writer"
guard_expect_fixed_in_file "$TAG" "mod compare_i64_writer;" "$PHYSICALIZER_MOD" \
  "physicalizer must retain the strict writer child"
guard_expect_fixed_in_file "$TAG" "Status: landed fast task" "$CARD" \
  "strict writer card must record landed status"
guard_expect_fixed_in_file "$TAG" "Loop Compare strict writer" "$README" \
  "resolved-lowering README must document the strict writer"
guard_expect_fixed_in_file "$TAG" "strict physical writer" "$REFERENCE" \
  "Compare reference must document the strict writer"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" \
  "check index must list the strict writer guard"

# The strict child and shared preparation facade must not reach the repair-capable
# emitter or reconstruct a target from ambient state.
for file in "$WRITER" "$APPEND_CORE"; do
  if rg -n -- 'emit_instruction_at\(|emit_instruction\(|ensure_block_exists\(|current_block|compute_def_blocks\(|compute_dominators\(' "$file"; then
    guard_fail "$TAG" "strict writer path reaches a legacy/ambient repair hook: ${file#"$ROOT_DIR/"}"
  fi
done

# There must be exactly one direct instruction mutation, owned by the shared
# append child. The legacy front door delegates to it but does not mutate MIR.
direct_append_count="$(rg -F -o -- 'add_instruction_with_span(' "$BUILDER_EMIT" "$APPEND_CORE" | wc -l | tr -d '[:space:]')"
if [[ "$direct_append_count" -ne 1 ]]; then
  guard_fail "$TAG" "physical append count must be one; found $direct_append_count"
fi
if rg -n -F -- 'add_instruction_with_span(' "$BUILDER_EMIT"; then
  guard_fail "$TAG" "builder_emit.rs must not own a second direct append"
fi

# P0 intentionally proves the writer in focused tests but does not connect it
# to a production dispatcher or caller.
non_test_writer_callers=()
while IFS= read -r file; do
  [[ -z "$file" ]] && continue
  case "$file" in
    *_tests.rs) continue ;;
  esac
  non_test_writer_callers+=("$file")
done < <(rg -l --glob '*.rs' -F 'CanonicalLoopCompareI64WriterV1::emit(' "$ROOT_DIR/src" || true)
if [[ "${#non_test_writer_callers[@]}" -ne 0 ]]; then
  guard_fail "$TAG" "P0 must remain caller-zero; found ${non_test_writer_callers[*]}"
fi

# Keep the split trigger visible; do not compress a growing physical owner.
for file in "$BUILDER_EMIT" "$APPEND_CORE" "$WRITER" "$WRITER_TESTS"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "strict-writer source reached the 800-line hard boundary: ${file#"$ROOT_DIR/"}=$lines"
  fi
done
builder_lines="$(wc -l < "$BUILDER_EMIT" | tr -d '[:space:]')"
if (( builder_lines >= 700 )); then
  guard_fail "$TAG" "builder_emit.rs reached the 700-line split trigger: $builder_lines"
fi

echo "[$TAG] ok (one shared append, strict preparation, non-test writer callers=0)"
