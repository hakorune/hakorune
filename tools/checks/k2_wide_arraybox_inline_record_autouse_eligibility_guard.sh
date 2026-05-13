#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-arraybox-inline-record-autouse-eligibility"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-224-C207-PACKED-ARRAYBOX-AUTOUSE-ELIGIBILITY.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
RECORD_SSOT="docs/development/current/main/design/record-and-packed-array-lowering-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
MIR_MOD="src/mir/mod.rs"
MIR_TYPES="src/mir/function/types.rs"
PLANNER="src/mir/array_record_autouse_eligibility.rs"
SEMANTIC_REFRESH="src/mir/semantic_refresh.rs"
MODULE_LIFECYCLE="src/mir/builder/module_lifecycle.rs"
JSON_BRIDGE="src/runner/json_v0_bridge/lowering.rs"
JSON_DECLS="src/runner/mir_json_emit/decls.rs"
JSON_ROOT="src/runner/mir_json_emit/root.rs"
JSON_TESTS="src/runner/mir_json_emit/tests/decl_values.rs"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_arraybox_inline_record_autouse_eligibility_guard.sh"

echo "[$TAG] checking C207 packed ArrayBox auto-use eligibility gate"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PLAN" \
  "$RECORD_SSOT" \
  "$PHASE_README" \
  "$TASKBOARD" \
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
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "C207 card must be complete"
guard_expect_in_file "$TAG" 'C207 status:' "$PLAN" "mimalloc plan must record C207 status"
guard_expect_in_file "$TAG" '`C207` is complete as' "$RECORD_SSOT" "record SSOT must mark C207 complete"
guard_expect_in_file "$TAG" '`293x-224`' "$PHASE_README" "phase README must list C207 row"
guard_expect_in_file "$TAG" '\[x\] `293x-224`' "$TASKBOARD" "taskboard must mark C207 complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C207 guard"

guard_expect_in_file "$TAG" 'array_record_autouse_eligibility' "$MIR_MOD" "MIR root must expose C207 planner"
guard_expect_in_file "$TAG" 'ArrayRecordAutoUseEligibilityPlan' "$MIR_TYPES" "MIR metadata type must exist"
guard_expect_in_file "$TAG" 'array_record_autouse_eligibility_plans' "$MIR_TYPES" "ModuleMetadata must carry C207 eligibility rows"
guard_expect_in_file "$TAG" 'ARRAY_RECORD_AUTOUSE_DECISION_ELIGIBLE' "$PLANNER" "planner must define eligible decision"
guard_expect_in_file "$TAG" 'ARRAY_RECORD_AUTOUSE_REASON_UNSUPPORTED_COLUMN_KIND' "$PLANNER" "planner must define unsupported-column rejection"
guard_expect_in_file "$TAG" 'ARRAY_RECORD_AUTOUSE_REASON_LAYOUT_MISMATCH' "$PLANNER" "planner must define layout-mismatch rejection"
guard_expect_in_file "$TAG" 'production_auto_use_enabled: false' "$PLANNER" "C207 must not enable production runtime auto-use"
guard_expect_in_file "$TAG" 'refresh_module_array_record_autouse_eligibility_plans' "$SEMANTIC_REFRESH" "semantic refresh must populate C207 metadata"
guard_expect_in_file "$TAG" 'refresh_module_array_record_autouse_eligibility_plans' "$MODULE_LIFECYCLE" "MIR builder lifecycle must populate C207 metadata"
guard_expect_in_file "$TAG" 'refresh_module_array_record_autouse_eligibility_plans' "$JSON_BRIDGE" "JSON bridge lowering must populate C207 metadata"
guard_expect_in_file "$TAG" 'collect_array_record_autouse_eligibility_plan_values' "$JSON_DECLS" "MIR JSON decls must expose C207 rows"
guard_expect_in_file "$TAG" '"array_record_autouse_eligibility_plans"' "$JSON_ROOT" "MIR JSON root must include C207 rows"
guard_expect_in_file "$TAG" 'collect_array_record_autouse_eligibility_plan_values_preserves_gate_truth' "$JSON_TESTS" "MIR JSON tests must cover C207 rows"

cargo test -q mir::array_record_autouse_eligibility
cargo test -q runner::mir_json_emit::tests::decl_values::collect_array_record_autouse_eligibility_plan_values_preserves_gate_truth

if rg -n 'array_record_autouse|ArrayRecordAutoUse|production_auto_use_enabled|integer-lane-non-escaping-candidate' \
  lang/src/hako_alloc src/boxes/array lang/c-abi/shims src/llvm_py/instructions \
  >/tmp/"$TAG".leak 2>&1; then
  echo "[$TAG] ERROR: C207 eligibility metadata leaked into hako_alloc/runtime/backend surfaces" >&2
  cat /tmp/"$TAG".leak >&2
  rm -f /tmp/"$TAG".leak
  exit 1
fi
rm -f /tmp/"$TAG".leak

if rg -n 'k2_wide_arraybox_inline_record_autouse_eligibility_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: C207 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

echo "[$TAG] ok"
