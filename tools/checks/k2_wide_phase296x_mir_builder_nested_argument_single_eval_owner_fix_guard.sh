#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mir-builder-nested-argument-single-eval-owner-fix"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_128="docs/development/current/main/phases/phase-296x/296x-128-MIR-BUILDER-NESTED-ARGUMENT-SINGLE-EVAL-OWNER-FIX.md"
CARD_129="docs/development/current/main/phases/phase-296x/296x-129-HAKO-MIMALLOC-POST-NESTED-ARGUMENT-SINGLE-EVAL-FIX-MEASUREMENT.md"
SSOT="docs/development/current/main/design/nested-argument-single-evaluation-ssot.md"
APP="apps/mir-nested-argument-single-eval-proof/main.hako"
FACADE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/mir_builder_nested_argument_single_eval_owner_fix.py"
GENERIC_TOOL="tools/allocator/generic_nested_argument_single_eval_fixture.py"
INVENTORY_TOOL="tools/allocator/hako_alloc_facade_reason_duplicate_inventory.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mir_builder_nested_argument_single_eval_owner_fix_guard.sh"

echo "[$TAG] checking MIR builder nested argument single-eval owner fix"

guard_require_files "$TAG" "$CARD_128" "$CARD_129" "$SSOT" "$APP" "$FACADE" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$GENERIC_TOOL" "$INVENTORY_TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$GENERIC_TOOL" "$INVENTORY_TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_128" "row128 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_129" "row129 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=mir-builder-nested-argument-single-eval-owner-fix-v0' "$CARD_128" "row128 must record output contract"
guard_expect_fixed_in_file "$TAG" 'actual_nested_call_count=1' "$CARD_128" "row128 must record fixed generic count"
guard_expect_fixed_in_file "$TAG" 'facade_reason_duplicate_failure_count=0' "$CARD_128" "row128 must record facade duplicate fix"
guard_expect_fixed_in_file "$TAG" 'generic_cse_added=0' "$CARD_128" "row128 must not add generic CSE"
guard_expect_fixed_in_file "$TAG" 'static_scalar_lowering_added=0' "$CARD_128" "row128 must not add static scalar lowering"
guard_expect_fixed_in_file "$TAG" 'Nested call arguments must be evaluated exactly once.' "$SSOT" "SSOT must define invariant"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-128-MIR-BUILDER-NESTED-ARGUMENT-SINGLE-EVAL-OWNER-FIX"' "$CURRENT_STATE" "current state latest card must advance to row128"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-POST-NESTED-ARGUMENT-SINGLE-EVAL-FIX-MEASUREMENT-296X-001"' "$CURRENT_STATE" "current state must select row129"
guard_expect_fixed_in_file "$TAG" '| 128 | `MIR-BUILDER-NESTED-ARGUMENT-SINGLE-EVAL-OWNER-FIX-296X-001` | Landed |' "$TASKBOARD" "taskboard row128 must be landed"
guard_expect_fixed_in_file "$TAG" '| 129 | `HAKO-MIMALLOC-POST-NESTED-ARGUMENT-SINGLE-EVAL-FIX-MEASUREMENT-296X-001` | Current |' "$TASKBOARD" "taskboard row129 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_nested_arg_owner_fix.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

target/release/hakorune --backend mir --emit-mir-json "$tmp_dir/generic.mir.json" "$APP" >/dev/null
python3 "$GENERIC_TOOL" --mir-json "$tmp_dir/generic.mir.json" --out "$tmp_dir/generic.out"
guard_expect_fixed_in_file "$TAG" 'actual_nested_call_count=1' "$tmp_dir/generic.out" "generic fixture must be fixed"

target/release/hakorune --backend mir --emit-mir-json "$tmp_dir/facade.mir.json" "$FACADE" >/dev/null
python3 "$INVENTORY_TOOL" --mir-json "$tmp_dir/facade.mir.json" --source "$FACADE" --out "$tmp_dir/facade.out"
guard_expect_fixed_in_file "$TAG" 'failing_method_count=0' "$tmp_dir/facade.out" "facade duplicate inventory must be fixed"
guard_expect_fixed_in_file "$TAG" 'total_unused_duplicate_reason_call_count=0' "$tmp_dir/facade.out" "facade duplicate count must be zero"

python3 "$TOOL" \
  --generic-fixture-report "$tmp_dir/generic.out" \
  --facade-inventory-report "$tmp_dir/facade.out" \
  --out "$tmp_dir/report.out"

guard_expect_fixed_in_file "$TAG" 'output_contract=mir-builder-nested-argument-single-eval-owner-fix-v0' "$tmp_dir/report.out" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'actual_nested_call_count=1' "$tmp_dir/report.out" "tool must preserve generic count"
guard_expect_fixed_in_file "$TAG" 'facade_reason_duplicate_failure_count=0' "$tmp_dir/report.out" "tool must preserve facade failure count"
guard_expect_fixed_in_file "$TAG" 'owner_fix=me_call_argument_lowering_deferred_until_route_selected' "$tmp_dir/report.out" "tool must name owner fix"
guard_expect_fixed_in_file "$TAG" 'generic_cse_added=0' "$tmp_dir/report.out" "tool must keep generic CSE closed"
guard_expect_fixed_in_file "$TAG" 'static_scalar_lowering_added=0' "$tmp_dir/report.out" "tool must keep static scalar closed"
guard_expect_fixed_in_file "$TAG" 'semantic_summary=ok' "$tmp_dir/report.out" "tool must report semantic ok"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$tmp_dir/report.out" "tool must end ok"

echo "[$TAG] ok"
