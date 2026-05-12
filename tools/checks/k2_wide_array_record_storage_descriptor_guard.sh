#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-array-record-storage-descriptor"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-212-C204A-ARRAY-RECORD-STORAGE-DESCRIPTORS.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
RECORD_SSOT="docs/development/current/main/design/record-and-packed-array-lowering-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
ARRAY_RECORD_PLAN="src/mir/array_record_storage_plan.rs"
MIR_TYPES="src/mir/function/types.rs"
MIR_MOD="src/mir/mod.rs"
MODULE_LIFECYCLE="src/mir/builder/module_lifecycle.rs"
SEMANTIC_REFRESH="src/mir/semantic_refresh.rs"
BRIDGE_LOWERING="src/runner/json_v0_bridge/lowering.rs"
BRIDGE_TESTS="src/runner/json_v0_bridge/tests.rs"
MIR_JSON_DECLS="src/runner/mir_json_emit/decls.rs"
MIR_JSON_ROOT="src/runner/mir_json_emit/root.rs"
MIR_JSON_TESTS="src/runner/mir_json_emit/tests/decl_values.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_array_record_storage_descriptor_guard.sh"

echo "[$TAG] checking C204a array record storage descriptors"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PLAN" \
  "$RECORD_SSOT" \
  "$PHASE_README" \
  "$ARRAY_RECORD_PLAN" \
  "$MIR_TYPES" \
  "$MIR_MOD" \
  "$MODULE_LIFECYCLE" \
  "$SEMANTIC_REFRESH" \
  "$BRIDGE_LOWERING" \
  "$BRIDGE_TESTS" \
  "$MIR_JSON_DECLS" \
  "$MIR_JSON_ROOT" \
  "$MIR_JSON_TESTS" \
  "$INDEX" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "C204a card must be complete"
guard_expect_in_file "$TAG" 'C204a status:' "$PLAN" "mimalloc plan must record C204a status"
guard_expect_in_file "$TAG" '`C204a` is complete as `293x-212`' "$RECORD_SSOT" "record SSOT must mark C204a complete"
guard_expect_in_file "$TAG" '`293x-212`' "$PHASE_README" "phase README must list C204a row"
guard_expect_in_file "$TAG" 'pub struct ArrayRecordStoragePlan' "$MIR_TYPES" "MIR metadata must define ArrayRecordStoragePlan"
guard_expect_in_file "$TAG" 'pub array_record_storage_plans: Vec<ArrayRecordStoragePlan>' "$MIR_TYPES" "MIR metadata must carry array_record_storage_plans"
guard_expect_in_file "$TAG" 'ARRAY_RECORD_STORAGE_KIND_INLINE_RECORD_COLUMNS_V0' "$ARRAY_RECORD_PLAN" "array record storage owner must define storage kind"
guard_expect_in_file "$TAG" 'refresh_module_array_record_storage_plans' "$ARRAY_RECORD_PLAN" "array record storage owner must expose refresh entry"
guard_expect_in_file "$TAG" 'array_record_storage_plan' "$MIR_MOD" "MIR root must expose array record storage owner"
guard_expect_in_file "$TAG" 'refresh_module_array_record_storage_plans' "$MODULE_LIFECYCLE" "module lifecycle must refresh array record storage plans"
guard_expect_in_file "$TAG" 'refresh_module_array_record_storage_plans' "$SEMANTIC_REFRESH" "semantic refresh must include array record storage plans"
guard_expect_in_file "$TAG" 'refresh_module_array_record_storage_plans' "$BRIDGE_LOWERING" "JSON bridge must derive array record storage plans"
guard_expect_in_file "$TAG" 'collect_array_record_storage_plan_values' "$MIR_JSON_DECLS" "MIR JSON decls must expose array record storage collector"
guard_expect_in_file "$TAG" '"array_record_storage_plans"' "$MIR_JSON_ROOT" "MIR JSON root must emit array_record_storage_plans"
guard_expect_in_file "$TAG" 'build_array_record_storage_plans_maps_record_layout_to_columns' "$ARRAY_RECORD_PLAN" "array record storage unit test must cover layout-to-column mapping"
guard_expect_in_file "$TAG" 'array_record_storage_plans' "$BRIDGE_TESTS" "JSON bridge tests must cover descriptor lane"
guard_expect_in_file "$TAG" 'collect_array_record_storage_plan_values_preserves_column_truth' "$MIR_JSON_TESTS" "MIR JSON tests must cover descriptor lane"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C204a guard"

if rg -n 'InlineRecord|array_record_storage_plans|inline_record_columns_v0' src/boxes/array lang/c-abi/shims src/llvm_py/instructions lang/src/hako_alloc >/tmp/"$TAG".leak 2>&1; then
  echo "[$TAG] ERROR: C204a descriptor leaked into runtime/backend/hako_alloc surfaces" >&2
  cat /tmp/"$TAG".leak >&2
  rm -f /tmp/"$TAG".leak
  exit 1
fi
rm -f /tmp/"$TAG".leak

cargo test -q build_array_record_storage_plans_maps_record_layout_to_columns
cargo test -q parse_json_v0_to_module_derives_concrete_record_layout_plans
cargo test -q collect_array_record_storage_plan_values_preserves_column_truth

echo "[$TAG] ok"
