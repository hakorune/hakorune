#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-arraybox-inline-record-materialization-boundary"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
source "$ROOT_DIR/tools/checks/lib/phase_card_paths.sh"

CARD="$(guard_require_phase293x_card "$TAG" "293x-225-C208-INLINE-RECORD-MATERIALIZATION-ESCAPE-BOUNDARY.md")"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
RECORD_SSOT="docs/development/current/main/design/record-and-packed-array-lowering-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
MIR_MOD="src/mir/mod.rs"
MIR_TYPES="src/mir/function/types.rs"
PLANNER="src/mir/array_record_materialization_boundary.rs"
SEMANTIC_REFRESH="src/mir/semantic_refresh.rs"
MODULE_LIFECYCLE="src/mir/builder/module_lifecycle.rs"
JSON_BRIDGE="src/runner/json_v0_bridge/lowering.rs"
JSON_DECLS="src/runner/mir_json_emit/decls.rs"
JSON_ROOT="src/runner/mir_json_emit/root.rs"
JSON_TESTS="src/runner/mir_json_emit/tests/decl_values.rs"
ARRAY_TESTS="src/boxes/array/tests.rs"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_arraybox_inline_record_materialization_boundary_guard.sh"

echo "[$TAG] checking C208 inline-record materialization / escape boundary"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PLAN" \
  "$RECORD_SSOT" \
  "$INDEX" \
  "$MIR_MOD" \
  "$MIR_TYPES" \
  "$PLANNER" \
  "$SEMANTIC_REFRESH" \
  "$MODULE_LIFECYCLE" \
  "$JSON_BRIDGE" \
  "$JSON_DECLS" \
  "$JSON_ROOT" \
  "$JSON_TESTS" \
  "$ARRAY_TESTS" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "C208 card must be complete"
guard_expect_in_file "$TAG" 'C208 status:' "$PLAN" "mimalloc plan must record C208 status"
guard_expect_in_file "$TAG" '`C208` is complete as' "$RECORD_SSOT" "record SSOT must mark C208 complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C208 guard"

guard_expect_in_file "$TAG" 'array_record_materialization_boundary' "$MIR_MOD" "MIR root must expose C208 planner"
guard_expect_in_file "$TAG" 'ArrayRecordMaterializationBoundaryPlan' "$MIR_TYPES" "MIR metadata type must exist"
guard_expect_in_file "$TAG" 'array_record_materialization_boundary_plans' "$MIR_TYPES" "ModuleMetadata must carry C208 boundary rows"
guard_expect_in_file "$TAG" 'ARRAY_RECORD_MATERIALIZATION_BOUNDARY_KIND_NON_ESCAPING_DIRECT_FIELD_READS_V0' "$PLANNER" "planner must define non-escaping direct field-read boundary"
guard_expect_in_file "$TAG" 'ARRAY_RECORD_MATERIALIZATION_ACTION_FAIL_FAST_UNMATERIALIZED_RECORD' "$PLANNER" "planner must define fail-fast materialization action"
guard_expect_in_file "$TAG" 'visible_record_materialization_enabled: false' "$PLANNER" "C208 must keep visible materialization disabled"
guard_expect_in_file "$TAG" 'runtime_auto_use_enabled: false' "$PLANNER" "C208 must not enable runtime auto-use"
guard_expect_in_file "$TAG" 'return None' "$PLANNER" "rejected C207 rows must not receive a C208 boundary row"
guard_expect_in_file "$TAG" 'refresh_module_array_record_materialization_boundary_plans' "$SEMANTIC_REFRESH" "semantic refresh must populate C208 metadata"
guard_expect_in_file "$TAG" 'refresh_module_array_record_materialization_boundary_plans' "$MODULE_LIFECYCLE" "MIR builder lifecycle must populate C208 metadata"
guard_expect_in_file "$TAG" 'refresh_module_array_record_materialization_boundary_plans' "$JSON_BRIDGE" "JSON bridge lowering must populate C208 metadata"
guard_expect_in_file "$TAG" 'collect_array_record_materialization_boundary_plan_values' "$JSON_DECLS" "MIR JSON decls must expose C208 rows"
guard_expect_in_file "$TAG" '"array_record_materialization_boundary_plans"' "$JSON_ROOT" "MIR JSON root must include C208 rows"
guard_expect_in_file "$TAG" 'collect_array_record_materialization_boundary_plan_values_preserves_stop_line' "$JSON_TESTS" "MIR JSON tests must cover C208 rows"
guard_expect_in_file "$TAG" 'inline_record_storage_keeps_visible_materialization_boundary' "$ARRAY_TESTS" "runtime tests must preserve visible materialization boundary"

cargo test -q mir::array_record_materialization_boundary
cargo test -q runner::mir_json_emit::tests::decl_values::collect_array_record_materialization_boundary_plan_values_preserves_stop_line
cargo test -q boxes::array::tests::inline_record_storage_keeps_visible_materialization_boundary

if rg -n 'array_record_materialization|ArrayRecordMaterialization|runtime_auto_use_enabled|non_escaping_direct_field_reads_v0' \
  lang/src/hako_alloc lang/c-abi/shims src/llvm_py/instructions \
  >/tmp/"$TAG".leak 2>&1; then
  echo "[$TAG] ERROR: C208 boundary metadata leaked into hako_alloc/backend surfaces" >&2
  cat /tmp/"$TAG".leak >&2
  rm -f /tmp/"$TAG".leak
  exit 1
fi
rm -f /tmp/"$TAG".leak

if rg -n 'runtime_auto_use_enabled: true|visible_record_materialization_enabled: true' \
  src/mir src/runner docs/development/current/main/phases/phase-293x \
  >/tmp/"$TAG".enabled 2>&1; then
  echo "[$TAG] ERROR: C208 must not enable runtime auto-use or visible materialization" >&2
  cat /tmp/"$TAG".enabled >&2
  rm -f /tmp/"$TAG".enabled
  exit 1
fi
rm -f /tmp/"$TAG".enabled

if rg -n 'k2_wide_arraybox_inline_record_materialization_boundary_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: C208 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

echo "[$TAG] ok"
