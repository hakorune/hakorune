#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-arraybox-inline-record-plan-probe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
source "$ROOT_DIR/tools/checks/lib/phase_card_paths.sh"

CARD="$(guard_require_phase293x_card "$TAG" "293x-221-C206D-ARRAYBOX-INLINE-RECORD-PLAN-PROBE.md")"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
RECORD_SSOT="docs/development/current/main/design/record-and-packed-array-lowering-ssot.md"
ARRAY_MOD="src/boxes/array/mod.rs"
PROBE="src/boxes/array/inline_record_probe.rs"
PLAN_PROBE="src/boxes/array/inline_record_plan_probe.rs"
ARRAY_TESTS="src/boxes/array/tests.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_arraybox_inline_record_plan_probe_guard.sh"

echo "[$TAG] checking C206d ArrayBox inline-record plan probe"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PLAN" \
  "$RECORD_SSOT" \
  "$ARRAY_MOD" \
  "$PROBE" \
  "$PLAN_PROBE" \
  "$ARRAY_TESTS" \
  "$INDEX" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "C206d card must be complete"
guard_expect_in_file "$TAG" 'C206d status:' "$PLAN" "mimalloc plan must record C206d status"
guard_expect_in_file "$TAG" '`C206d` is complete as `293x-221`' "$RECORD_SSOT" "record SSOT must mark C206d complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C206d guard"

guard_expect_in_file "$TAG" '#\[cfg\(test\)\]' "$ARRAY_MOD" "inline-record plan probe module must stay cfg(test)"
guard_expect_in_file "$TAG" 'mod inline_record_plan_probe;' "$ARRAY_MOD" "array module must expose the plan probe only to tests"
guard_expect_in_file "$TAG" 'struct ArrayInlineRecordPlanProbe' "$PLAN_PROBE" "plan probe owner must be explicit"
guard_expect_in_file "$TAG" 'ArrayRecordStoragePlan' "$PLAN_PROBE" "plan probe must consume MIR storage plans"
guard_expect_in_file "$TAG" 'ARRAY_RECORD_STORAGE_KIND_INLINE_RECORD_COLUMNS_V0' "$PLAN_PROBE" "plan probe must require the inline-record descriptor kind"
guard_expect_in_file "$TAG" 'uses_integer_lane' "$PLAN_PROBE" "plan probe must restrict itself to integer-lane storage"
guard_expect_in_file "$TAG" 'ArrayInlineRecordProbe::build' "$PLAN_PROBE" "plan probe must delegate to the explicit runtime probe"
guard_expect_in_file "$TAG" 'inline_record_plan_probe_builds_integer_lane_array' "$ARRAY_TESTS" "tests must cover plan-to-runtime probe success"
guard_expect_in_file "$TAG" 'inline_record_plan_probe_rejects_handle_columns' "$ARRAY_TESTS" "tests must cover unsupported plan column rejection"

if rg -n 'ArrayInlineRecordPlanProbe|inline_record_plan_probe' src -g'*.rs' \
  | rg -v '^src/boxes/array/(inline_record_plan_probe|mod|tests)\.rs:' \
  >/tmp/"$TAG".source_leak 2>&1; then
  echo "[$TAG] ERROR: ArrayInlineRecordPlanProbe leaked outside array test owner files" >&2
  cat /tmp/"$TAG".source_leak >&2
  rm -f /tmp/"$TAG".source_leak
  exit 1
fi
rm -f /tmp/"$TAG".source_leak

if {
  rg -n 'ArrayInlineRecordPlanProbe|inline_record_plan_probe|InlineRecord|inline_record' lang/src/hako_alloc -g'*.hako'
  rg -n 'ArrayInlineRecordPlanProbe|inline_record_plan_probe|InlineRecord|inline_record' lang/c-abi/shims src/llvm_py/instructions
} >/tmp/"$TAG".runtime_leak 2>&1; then
  echo "[$TAG] ERROR: C206d plan probe leaked into hako_alloc/backend lowering surfaces" >&2
  cat /tmp/"$TAG".runtime_leak >&2
  rm -f /tmp/"$TAG".runtime_leak
  exit 1
fi
rm -f /tmp/"$TAG".runtime_leak

cargo test -q boxes::array::tests::inline_record_plan_probe_builds_integer_lane_array
cargo test -q boxes::array::tests::inline_record_plan_probe_rejects_handle_columns

echo "[$TAG] ok"
