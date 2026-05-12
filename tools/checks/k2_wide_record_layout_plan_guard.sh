#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-record-layout-plan"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-210-C203B-RECORD-LAYOUT-PLANS.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
RECORD_SSOT="docs/development/current/main/design/record-and-packed-array-lowering-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
DECLARED_STORAGE="src/mir/declared_type_storage.rs"
MIR_TYPES="src/mir/function/types.rs"
RECORD_LAYOUT="src/mir/record_layout_plan.rs"
MIR_MOD="src/mir/mod.rs"
MODULE_LIFECYCLE="src/mir/builder/module_lifecycle.rs"
SEMANTIC_REFRESH="src/mir/semantic_refresh.rs"
BRIDGE_LOWERING="src/runner/json_v0_bridge/lowering.rs"
BRIDGE_TESTS="src/runner/json_v0_bridge/tests.rs"
MIR_JSON_DECLS="src/runner/mir_json_emit/decls.rs"
MIR_JSON_ROOT="src/runner/mir_json_emit/root.rs"
MIR_JSON_TESTS="src/runner/mir_json_emit/tests/decl_values.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_record_layout_plan_guard.sh"

echo "[$TAG] checking C203b record layout plans"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PLAN" \
  "$RECORD_SSOT" \
  "$PHASE_README" \
  "$DECLARED_STORAGE" \
  "$MIR_TYPES" \
  "$RECORD_LAYOUT" \
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

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "C203b card must be complete"
guard_expect_in_file "$TAG" 'C203b status:' "$PLAN" "mimalloc plan must record C203b status"
guard_expect_in_file "$TAG" '`C203b` is complete as `293x-210`' "$RECORD_SSOT" "record SSOT must mark C203b complete"
guard_expect_in_file "$TAG" '`293x-210`' "$PHASE_README" "phase README must list C203b row"
guard_expect_in_file "$TAG" 'storage_for_declared_type' "$DECLARED_STORAGE" "declared-type storage helper must be shared"
guard_expect_in_file "$TAG" 'pub struct RecordLayoutPlan' "$MIR_TYPES" "MIR metadata must define RecordLayoutPlan"
guard_expect_in_file "$TAG" 'pub record_layout_plans: Vec<RecordLayoutPlan>' "$MIR_TYPES" "MIR metadata must carry record_layout_plans"
guard_expect_in_file "$TAG" 'RECORD_LAYOUT_KIND_VALUE_AGGREGATE_V0' "$RECORD_LAYOUT" "record layout owner must define layout kind"
guard_expect_in_file "$TAG" 'refresh_module_record_layout_plans' "$RECORD_LAYOUT" "record layout owner must expose refresh entry"
guard_expect_in_file "$TAG" 'record_layout_plan' "$MIR_MOD" "MIR root must expose record layout owner"
guard_expect_in_file "$TAG" 'refresh_module_record_layout_plans' "$MODULE_LIFECYCLE" "module lifecycle must refresh record layout plans"
guard_expect_in_file "$TAG" 'refresh_module_record_layout_plans' "$SEMANTIC_REFRESH" "semantic refresh must include record layout plans"
guard_expect_in_file "$TAG" 'refresh_module_record_layout_plans' "$BRIDGE_LOWERING" "JSON bridge must derive record layout plans"
guard_expect_in_file "$TAG" 'collect_record_layout_plan_values' "$MIR_JSON_DECLS" "MIR JSON decls must expose record layout collector"
guard_expect_in_file "$TAG" '"record_layout_plans"' "$MIR_JSON_ROOT" "MIR JSON root must emit record_layout_plans"
guard_expect_in_file "$TAG" 'build_record_layout_plans_accepts_concrete_typed_fields' "$RECORD_LAYOUT" "record layout unit test must cover accepted concrete records"
guard_expect_in_file "$TAG" 'build_record_layout_plans_skips_generic_and_weak_records' "$RECORD_LAYOUT" "record layout unit test must cover unsupported records"
guard_expect_in_file "$TAG" 'parse_json_v0_to_module_derives_concrete_record_layout_plans' "$BRIDGE_TESTS" "JSON bridge test must cover concrete record layouts"
guard_expect_in_file "$TAG" 'collect_record_layout_plan_values_preserves_record_layout_truth' "$MIR_JSON_TESTS" "MIR JSON test must cover record layout lane"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C203b guard"

if rg -n 'record_layout_plans|RecordLayoutPlan|record_value_aggregate_v0' lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: C203b record layout matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

cargo test -q build_record_layout_plans_accepts_concrete_typed_fields
cargo test -q build_record_layout_plans_skips_generic_and_weak_records
cargo test -q parse_json_v0_to_module_derives_concrete_record_layout_plans
cargo test -q collect_record_layout_plan_values_preserves_record_layout_truth

echo "[$TAG] ok"
