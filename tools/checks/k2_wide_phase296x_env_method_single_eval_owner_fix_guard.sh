#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-env-method-single-eval-owner-fix"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_132="docs/development/current/main/phases/phase-296x/296x-132-MIR-BUILDER-ENV-METHOD-SINGLE-EVAL-OWNER-FIX.md"
CARD_133="docs/development/current/main/phases/phase-296x/296x-133-HAKO-MIMALLOC-POST-SINGLE-EVAL-FIXES-MEASUREMENT.md"
APP="apps/mir-env-method-single-eval-proof/main.hako"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
FIXTURE_TOOL="tools/allocator/env_method_single_eval_fixture.py"
TOOL="tools/allocator/env_method_single_eval_owner_fix.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_env_method_single_eval_owner_fix_guard.sh"

echo "[$TAG] checking env method single-eval owner fix"

guard_require_files "$TAG" "$CARD_132" "$CARD_133" "$APP" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$FIXTURE_TOOL" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$FIXTURE_TOOL" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_132" "row132 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_133" "row133 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=mir-builder-env-method-single-eval-owner-fix-v0' "$CARD_132" "row132 must record output contract"
guard_expect_fixed_in_file "$TAG" 'actual_nested_call_count=1' "$CARD_132" "row132 must record fixed count"
guard_expect_fixed_in_file "$TAG" 'owner_fix=env_method_spec_checked_before_argument_lowering' "$CARD_132" "row132 must record owner fix"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-132-MIR-BUILDER-ENV-METHOD-SINGLE-EVAL-OWNER-FIX"' "$CURRENT_STATE" "current state latest card must advance to row132"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-POST-SINGLE-EVAL-FIXES-MEASUREMENT-296X-001"' "$CURRENT_STATE" "current state must select row133"
guard_expect_fixed_in_file "$TAG" '| 132 | `MIR-BUILDER-ENV-METHOD-SINGLE-EVAL-OWNER-FIX-296X-001` | Landed |' "$TASKBOARD" "taskboard row132 must be landed"
guard_expect_fixed_in_file "$TAG" '| 133 | `HAKO-MIMALLOC-POST-SINGLE-EVAL-FIXES-MEASUREMENT-296X-001` | Current |' "$TASKBOARD" "taskboard row133 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_env_method_owner_fix.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

target/release/hakorune --backend mir --emit-mir-json "$tmp_dir/env_method.mir.json" "$APP" >/dev/null
python3 "$FIXTURE_TOOL" --mir-json "$tmp_dir/env_method.mir.json" --out "$tmp_dir/fixture.out"
guard_expect_fixed_in_file "$TAG" 'actual_nested_call_count=1' "$tmp_dir/fixture.out" "env method fixture must be fixed"

python3 "$TOOL" --fixture-report "$tmp_dir/fixture.out" --out "$tmp_dir/report.out"
guard_expect_fixed_in_file "$TAG" 'output_contract=mir-builder-env-method-single-eval-owner-fix-v0' "$tmp_dir/report.out" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'actual_nested_call_count=1' "$tmp_dir/report.out" "tool must preserve fixed count"
guard_expect_fixed_in_file "$TAG" 'owner_fix=env_method_spec_checked_before_argument_lowering' "$tmp_dir/report.out" "tool must record owner fix"
guard_expect_fixed_in_file "$TAG" 'semantic_summary=ok' "$tmp_dir/report.out" "tool must report semantic ok"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$tmp_dir/report.out" "tool must end ok"

echo "[$TAG] ok"
