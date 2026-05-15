#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-aligned-small-metadata-packed-store-pilot"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
source "$ROOT_DIR/tools/checks/lib/phase_card_paths.sh"

CARD="$(guard_require_phase293x_card "$TAG" "293x-227-C210-ALIGNED-SMALL-METADATA-PACKED-STORE-PILOT.md")"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
RECORD_SSOT="docs/development/current/main/design/record-and-packed-array-lowering-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
MIR_MOD="src/mir/mod.rs"
MIR_TYPES="src/mir/function/types.rs"
PLANNER="src/mir/hako_alloc_aligned_small_packed_store_pilot.rs"
SEMANTIC_REFRESH="src/mir/semantic_refresh.rs"
MODULE_LIFECYCLE="src/mir/builder/module_lifecycle.rs"
JSON_BRIDGE="src/runner/json_v0_bridge/lowering.rs"
JSON_DECLS="src/runner/mir_json_emit/decls.rs"
JSON_ROOT="src/runner/mir_json_emit/root.rs"
JSON_TESTS="src/runner/mir_json_emit/tests/decl_values.rs"
ARRAY_TESTS="src/boxes/array/tests.rs"
ALIGNED_STORE="lang/src/hako_alloc/memory/aligned_small_meta_store_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_aligned_small_metadata_packed_store_pilot_guard.sh"

echo "[$TAG] checking C210 aligned-small metadata packed-store pilot"

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
  "$ALIGNED_STORE" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "C210 card must be complete"
guard_expect_in_file "$TAG" 'C210 status:' "$PLAN" "mimalloc plan must record C210 status"
guard_expect_in_file "$TAG" '`C210` is complete as' "$RECORD_SSOT" "record SSOT must mark C210 complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C210 guard"

guard_expect_in_file "$TAG" 'hako_alloc_aligned_small_packed_store_pilot' "$MIR_MOD" "MIR root must expose C210 planner"
guard_expect_in_file "$TAG" 'HakoAllocAlignedSmallPackedStorePilotPlan' "$MIR_TYPES" "MIR metadata type must exist"
guard_expect_in_file "$TAG" 'hako_alloc_aligned_small_packed_store_pilot_plans' "$MIR_TYPES" "ModuleMetadata must carry C210 pilot rows"
guard_expect_in_file "$TAG" 'HAKO_ALLOC_ALIGNED_SMALL_META_RECORD' "$PLANNER" "planner must target aligned-small metadata record"
guard_expect_in_file "$TAG" 'HAKO_ALLOC_ALIGNED_SMALL_META_STORE_OWNER' "$PLANNER" "planner must name the source store owner"
guard_expect_in_file "$TAG" 'HAKO_ALLOC_ALIGNED_SMALL_PACKED_STORE_PILOT_KIND' "$PLANNER" "planner must define C210 pilot kind"
guard_expect_in_file "$TAG" 'hako_alloc_source_mentions_compiler: false' "$PLANNER" "C210 must keep hako_alloc source compiler-internal-free"
guard_expect_in_file "$TAG" 'live_scalar_columns_retained: true' "$PLANNER" "C210 must retain source scalar-column compatibility"
guard_expect_in_file "$TAG" 'public_array_get_materialization_enabled: false' "$PLANNER" "C210 must keep public materialization disabled"
guard_expect_in_file "$TAG" 'backend_lowering_enabled: false' "$PLANNER" "C210 must not add backend lowering"
guard_expect_in_file "$TAG" 'refresh_module_hako_alloc_aligned_small_packed_store_pilot_plans' "$SEMANTIC_REFRESH" "semantic refresh must populate C210 metadata"
guard_expect_in_file "$TAG" 'refresh_module_hako_alloc_aligned_small_packed_store_pilot_plans' "$MODULE_LIFECYCLE" "MIR builder lifecycle must populate C210 metadata"
guard_expect_in_file "$TAG" 'refresh_module_hako_alloc_aligned_small_packed_store_pilot_plans' "$JSON_BRIDGE" "JSON bridge lowering must populate C210 metadata"
guard_expect_in_file "$TAG" 'collect_hako_alloc_aligned_small_packed_store_pilot_plan_values' "$JSON_DECLS" "MIR JSON decls must expose C210 rows"
guard_expect_in_file "$TAG" '"hako_alloc_aligned_small_packed_store_pilot_plans"' "$JSON_ROOT" "MIR JSON root must include C210 rows"
guard_expect_in_file "$TAG" 'collect_hako_alloc_aligned_small_packed_store_pilot_plan_values_preserves_pilot_limits' "$JSON_TESTS" "MIR JSON tests must cover C210 rows"
guard_expect_in_file "$TAG" 'aligned_small_metadata_packed_store_pilot_reads_metadata_columns' "$ARRAY_TESTS" "runtime tests must cover C210 aligned-small packed read proof"

cargo test -q mir::hako_alloc_aligned_small_packed_store_pilot
cargo test -q runner::mir_json_emit::tests::decl_values::collect_hako_alloc_aligned_small_packed_store_pilot_plan_values_preserves_pilot_limits
cargo test -q boxes::array::tests::aligned_small_metadata_packed_store_pilot_reads_metadata_columns

if rg -n 'InlineRecord|ArrayStorage|PlanProbe|AutoUse|compiler_autouse|hako_alloc_aligned_small_packed_store_pilot|array_record_packed_autouse' \
  "$ALIGNED_STORE" lang/src/hako_alloc/memory/page_map_aligned_small_path_box.hako \
  >/tmp/"$TAG".hako_leak 2>&1; then
  echo "[$TAG] ERROR: C210 compiler/runtime vocabulary leaked into hako_alloc source" >&2
  cat /tmp/"$TAG".hako_leak >&2
  rm -f /tmp/"$TAG".hako_leak
  exit 1
fi
rm -f /tmp/"$TAG".hako_leak

if rg -n 'hako_alloc_aligned_small_packed_store_pilot|aligned_small_metadata_i64_columns_v0' \
  lang/c-abi/shims src/llvm_py/instructions \
  >/tmp/"$TAG".backend_leak 2>&1; then
  echo "[$TAG] ERROR: C210 pilot leaked into backend shim surfaces" >&2
  cat /tmp/"$TAG".backend_leak >&2
  rm -f /tmp/"$TAG".backend_leak
  exit 1
fi
rm -f /tmp/"$TAG".backend_leak

if rg -n 'public_array_get_materialization_enabled: true|backend_lowering_enabled: true|hako_alloc_source_mentions_compiler: true|live_scalar_columns_retained: false' \
  src/mir src/runner src/boxes/array docs/development/current/main/phases/phase-293x \
  >/tmp/"$TAG".enabled 2>&1; then
  echo "[$TAG] ERROR: C210 must not enable materialization/backend lowering/source compiler leakage" >&2
  cat /tmp/"$TAG".enabled >&2
  rm -f /tmp/"$TAG".enabled
  exit 1
fi
rm -f /tmp/"$TAG".enabled

if rg -n 'k2_wide_aligned_small_metadata_packed_store_pilot_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: C210 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

echo "[$TAG] ok"
