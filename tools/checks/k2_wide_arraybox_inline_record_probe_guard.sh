#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-arraybox-inline-record-probe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
source "$ROOT_DIR/tools/checks/lib/phase_card_paths.sh"

CARD="$(guard_require_phase293x_card "$TAG" "293x-219-C206B-ARRAYBOX-INLINE-RECORD-PROBE.md")"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
RECORD_SSOT="docs/development/current/main/design/record-and-packed-array-lowering-ssot.md"
ARRAY_MOD="src/boxes/array/mod.rs"
PROBE="src/boxes/array/inline_record_probe.rs"
ARRAY_TESTS="src/boxes/array/tests.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_arraybox_inline_record_probe_guard.sh"

echo "[$TAG] checking C206b ArrayBox inline-record probe"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PLAN" \
  "$RECORD_SSOT" \
  "$ARRAY_MOD" \
  "$PROBE" \
  "$ARRAY_TESTS" \
  "$INDEX" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "C206b card must be complete"
guard_expect_in_file "$TAG" 'C206b status:' "$PLAN" "mimalloc plan must record C206b status"
guard_expect_in_file "$TAG" '`C206b` is complete as `293x-219`' "$RECORD_SSOT" "record SSOT must mark C206b complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C206b guard"

guard_expect_in_file "$TAG" '#\[cfg\(test\)\]' "$ARRAY_MOD" "inline-record probe module must stay cfg(test)"
guard_expect_in_file "$TAG" 'mod inline_record_probe;' "$ARRAY_MOD" "array module must expose the probe module only to tests"
guard_expect_in_file "$TAG" 'struct ArrayInlineRecordProbe' "$PROBE" "probe owner must be explicit"
guard_expect_in_file "$TAG" 'ArrayInlineRecordStorage::new' "$PROBE" "probe must construct storage through the private runtime vocabulary"
guard_expect_in_file "$TAG" 'new_with_inline_record_storage' "$PROBE" "probe must be the explicit inline-record ArrayBox constructor seam"
guard_expect_in_file "$TAG" 'inline_record_probe_builds_explicit_probe_array' "$ARRAY_TESTS" "array tests must cover explicit probe construction"
guard_expect_in_file "$TAG" 'inline_record_probe_rejects_ragged_columns' "$ARRAY_TESTS" "array tests must cover ragged-column rejection"
guard_expect_in_file "$TAG" 'ArrayInlineRecordProbe::build' "$ARRAY_TESTS" "tests must use the probe owner"

if rg -n 'ArrayInlineRecordProbe|inline_record_probe' src -g'*.rs' \
  | rg -v '^src/boxes/array/(inline_record_probe|inline_record_plan_probe|mod|tests)\.rs:' \
  >/tmp/"$TAG".source_leak 2>&1; then
  echo "[$TAG] ERROR: ArrayInlineRecordProbe leaked outside array test owner files" >&2
  cat /tmp/"$TAG".source_leak >&2
  rm -f /tmp/"$TAG".source_leak
  exit 1
fi
rm -f /tmp/"$TAG".source_leak

if {
  rg -n 'ArrayInlineRecordProbe|inline_record_probe|InlineRecord|inline_record' lang/src/hako_alloc -g'*.hako'
  rg -n 'ArrayInlineRecordProbe|inline_record_probe|InlineRecord|inline_record' lang/c-abi/shims src/llvm_py/instructions
} >/tmp/"$TAG".runtime_leak 2>&1; then
  echo "[$TAG] ERROR: C206b probe leaked into hako_alloc/backend lowering surfaces" >&2
  cat /tmp/"$TAG".runtime_leak >&2
  rm -f /tmp/"$TAG".runtime_leak
  exit 1
fi
rm -f /tmp/"$TAG".runtime_leak

cargo test -q boxes::array::tests::inline_record_probe_builds_explicit_probe_array
cargo test -q boxes::array::tests::inline_record_probe_rejects_ragged_columns
cargo test -q boxes::array::tests::inline_record_storage

echo "[$TAG] ok"
