#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-arraybox-inline-record-autouse-pilot"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
source "$ROOT_DIR/tools/checks/lib/phase_card_paths.sh"

CARD="$(guard_require_phase293x_card "$TAG" "293x-226-C209-NON-ESCAPING-PACKED-ARRAYBOX-AUTOUSE-PILOT.md")"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
RECORD_SSOT="docs/development/current/main/design/record-and-packed-array-lowering-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
MIR_MOD="src/mir/mod.rs"
MIR_TYPES="src/mir/function/types.rs"
PLANNER="src/mir/array_record_packed_autouse_pilot.rs"
SEMANTIC_REFRESH="src/mir/semantic_refresh.rs"
MODULE_LIFECYCLE="src/mir/builder/module_lifecycle.rs"
JSON_BRIDGE="src/runner/json_v0_bridge/lowering.rs"
JSON_DECLS="src/runner/mir_json_emit/decls.rs"
JSON_ROOT="src/runner/mir_json_emit/root.rs"
JSON_TESTS="src/runner/mir_json_emit/tests/decl_values.rs"
ARRAY_STORAGE="src/boxes/array/storage.rs"
ARRAY_SHARED="src/boxes/array/ops/shared.rs"
ARRAY_ACCESS="src/boxes/array/ops/access.rs"
ARRAY_TESTS="src/boxes/array/tests.rs"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_arraybox_inline_record_autouse_pilot_guard.sh"

echo "[$TAG] checking C209 non-escaping packed ArrayBox auto-use pilot"

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
  "$ARRAY_STORAGE" \
  "$ARRAY_SHARED" \
  "$ARRAY_ACCESS" \
  "$ARRAY_TESTS" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "C209 card must be complete"
guard_expect_in_file "$TAG" 'C209 status:' "$PLAN" "mimalloc plan must record C209 status"
guard_expect_in_file "$TAG" '`C209` is complete as' "$RECORD_SSOT" "record SSOT must mark C209 complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C209 guard"

guard_expect_in_file "$TAG" 'array_record_packed_autouse_pilot' "$MIR_MOD" "MIR root must expose C209 planner"
guard_expect_in_file "$TAG" 'ArrayRecordPackedAutoUsePilotPlan' "$MIR_TYPES" "MIR metadata type must exist"
guard_expect_in_file "$TAG" 'array_record_packed_autouse_pilot_plans' "$MIR_TYPES" "ModuleMetadata must carry C209 pilot rows"
guard_expect_in_file "$TAG" 'ARRAY_RECORD_PACKED_AUTOUSE_PILOT_KIND_INTEGER_LANE_DIRECT_READS_V0' "$PLANNER" "planner must define integer-lane direct-read pilot"
guard_expect_in_file "$TAG" 'private_runtime_storage_enabled: true' "$PLANNER" "C209 opens only the private runtime pilot seam"
guard_expect_in_file "$TAG" 'public_array_get_materialization_enabled: false' "$PLANNER" "C209 must keep public materialization disabled"
guard_expect_in_file "$TAG" 'hako_alloc_migration_enabled: false' "$PLANNER" "C209 must not migrate hako_alloc"
guard_expect_in_file "$TAG" 'backend_lowering_enabled: false' "$PLANNER" "C209 must not add backend lowering"
guard_expect_in_file "$TAG" 'refresh_module_array_record_packed_autouse_pilot_plans' "$SEMANTIC_REFRESH" "semantic refresh must populate C209 metadata"
guard_expect_in_file "$TAG" 'refresh_module_array_record_packed_autouse_pilot_plans' "$MODULE_LIFECYCLE" "MIR builder lifecycle must populate C209 metadata"
guard_expect_in_file "$TAG" 'refresh_module_array_record_packed_autouse_pilot_plans' "$JSON_BRIDGE" "JSON bridge lowering must populate C209 metadata"
guard_expect_in_file "$TAG" 'collect_array_record_packed_autouse_pilot_plan_values' "$JSON_DECLS" "MIR JSON decls must expose C209 rows"
guard_expect_in_file "$TAG" '"array_record_packed_autouse_pilot_plans"' "$JSON_ROOT" "MIR JSON root must include C209 rows"
guard_expect_in_file "$TAG" 'collect_array_record_packed_autouse_pilot_plan_values_preserves_pilot_limits' "$JSON_TESTS" "MIR JSON tests must cover C209 rows"

guard_expect_in_file "$TAG" 'from_i64_columns' "$ARRAY_STORAGE" "runtime storage must expose private i64-column construction"
guard_expect_in_file "$TAG" 'load_i64_column' "$ARRAY_STORAGE" "runtime storage must expose direct i64-column read"
guard_expect_in_file "$TAG" 'new_with_inline_record_i64_columns_for_compiler_autouse' "$ARRAY_SHARED" "ArrayBox must expose private C209 construction seam"
guard_expect_in_file "$TAG" 'inline_record_load_i64_column_raw' "$ARRAY_ACCESS" "ArrayBox must expose private C209 read seam"
guard_expect_in_file "$TAG" 'inline_record_autouse_pilot_reads_i64_columns_without_materializing' "$ARRAY_TESTS" "runtime tests must cover C209 direct-read seam"
guard_expect_in_file "$TAG" 'inline_record_autouse_pilot_rejects_ragged_i64_columns' "$ARRAY_TESTS" "runtime tests must cover C209 ragged rejection"

cargo test -q mir::array_record_packed_autouse_pilot
cargo test -q runner::mir_json_emit::tests::decl_values::collect_array_record_packed_autouse_pilot_plan_values_preserves_pilot_limits
cargo test -q boxes::array::tests::inline_record_autouse_pilot_reads_i64_columns_without_materializing
cargo test -q boxes::array::tests::inline_record_autouse_pilot_rejects_ragged_i64_columns

if rg -n 'array_record_packed_autouse|ArrayRecordPackedAutoUse|new_with_inline_record_i64_columns_for_compiler_autouse|inline_record_load_i64_column_raw|integer_lane_direct_reads_v0' \
  lang/src/hako_alloc lang/c-abi/shims src/llvm_py/instructions \
  >/tmp/"$TAG".leak 2>&1; then
  echo "[$TAG] ERROR: C209 pilot leaked into hako_alloc/backend surfaces" >&2
  cat /tmp/"$TAG".leak >&2
  rm -f /tmp/"$TAG".leak
  exit 1
fi
rm -f /tmp/"$TAG".leak

if rg -n 'public_array_get_materialization_enabled: true|hako_alloc_migration_enabled: true|backend_lowering_enabled: true' \
  src/mir src/runner src/boxes/array docs/development/current/main/phases/phase-293x \
  >/tmp/"$TAG".enabled 2>&1; then
  echo "[$TAG] ERROR: C209 must not enable public materialization, hako_alloc migration, or backend lowering" >&2
  cat /tmp/"$TAG".enabled >&2
  rm -f /tmp/"$TAG".enabled
  exit 1
fi
rm -f /tmp/"$TAG".enabled

if rg -n 'k2_wide_arraybox_inline_record_autouse_pilot_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: C209 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

echo "[$TAG] ok"
