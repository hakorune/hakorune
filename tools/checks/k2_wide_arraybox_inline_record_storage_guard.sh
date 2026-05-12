#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-arraybox-inline-record-storage"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-213-C204B-ARRAYBOX-INLINE-RECORD-STORAGE-VOCAB.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
RECORD_SSOT="docs/development/current/main/design/record-and-packed-array-lowering-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
ARRAY_STORAGE="src/boxes/array/storage.rs"
ARRAY_SHARED="src/boxes/array/ops/shared.rs"
ARRAY_TRAITS="src/boxes/array/traits.rs"
ARRAY_ACCESS="src/boxes/array/ops/access.rs"
ARRAY_TESTS="src/boxes/array/tests.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_arraybox_inline_record_storage_guard.sh"

echo "[$TAG] checking C204b ArrayBox inline-record storage vocabulary"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PLAN" \
  "$RECORD_SSOT" \
  "$PHASE_README" \
  "$ARRAY_STORAGE" \
  "$ARRAY_SHARED" \
  "$ARRAY_TRAITS" \
  "$ARRAY_ACCESS" \
  "$ARRAY_TESTS" \
  "$INDEX" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "C204b card must be complete"
guard_expect_in_file "$TAG" 'C204b status:' "$PLAN" "mimalloc plan must record C204b status"
guard_expect_in_file "$TAG" '`C204b` is complete as `293x-213`' "$RECORD_SSOT" "record SSOT must mark C204b complete"
guard_expect_in_file "$TAG" '`293x-213`' "$PHASE_README" "phase README must list C204b row"
guard_expect_in_file "$TAG" 'InlineRecord' "$ARRAY_STORAGE" "ArrayStorage must define inline-record variant"
guard_expect_in_file "$TAG" 'struct ArrayInlineRecordStorage' "$ARRAY_STORAGE" "inline-record storage struct must be private to array runtime"
guard_expect_in_file "$TAG" 'enum ArrayInlineRecordColumn' "$ARRAY_STORAGE" "inline-record storage must define scalar columns"
guard_expect_in_file "$TAG" 'new_with_inline_record_storage' "$ARRAY_SHARED" "array shared ops must expose internal constructor"
guard_expect_in_file "$TAG" 'inline-record/unmaterialized' "$ARRAY_ACCESS" "visible read boundary must be explicit"
guard_expect_in_file "$TAG" 'uses_inline_record_slots' "$ARRAY_TRAITS" "tests must be able to assert inline-record residence"
guard_expect_in_file "$TAG" 'inline_record_storage_keeps_visible_materialization_boundary' "$ARRAY_TESTS" "unit tests must cover materialization stop line"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C204b guard"

if {
  rg -n 'InlineRecord|inline-record|inline_record' lang/src/hako_alloc -g'*.hako'
  rg -n 'InlineRecord|inline-record|inline_record' lang/c-abi/shims src/llvm_py/instructions
} >/tmp/"$TAG".leak 2>&1; then
  echo "[$TAG] ERROR: C204b inline-record storage leaked into hako_alloc/backend lowering surfaces" >&2
  cat /tmp/"$TAG".leak >&2
  rm -f /tmp/"$TAG".leak
  exit 1
fi
rm -f /tmp/"$TAG".leak

cargo test -q boxes::array::tests::inline_record_storage

echo "[$TAG] ok"
