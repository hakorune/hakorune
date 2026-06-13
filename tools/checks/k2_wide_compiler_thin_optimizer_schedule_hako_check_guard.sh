#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-compiler-thin-optimizer-schedule-hako-check"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/compiler-pipeline-thinning-ssot.md"
README="tools/hako_check/README.md"
INDEX="docs/tools/check-scripts-index.md"
WRAPPER="tools/hako_check.sh"
TOOL="tools/hako_check/optimizer_schedule.py"
SELF_SCRIPT="tools/checks/k2_wide_compiler_thin_optimizer_schedule_hako_check_guard.sh"
CORE="src/mir/optimizer/core.rs"

echo "[$TAG] checking hako_check optimizer schedule surface"

guard_require_files "$TAG" "$SSOT" "$README" "$INDEX" "$WRAPPER" "$TOOL" "$SELF_SCRIPT" "$CORE"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-check-optimizer-schedule-v0' "$SSOT" "SSOT must record optimizer schedule contract"
guard_expect_fixed_in_file "$TAG" 'hako_check optimizer-schedule' "$README" "README must document optimizer schedule surface"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-check-optimizer-schedule-v0' "$README" "README must include optimizer schedule contract"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" 'tools/hako_check.sh optimizer-schedule' "$INDEX" "check index must list wrapper entry"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list optimizer schedule tool"

tmp_dir="$(mktemp -d /tmp/hakorune_optimizer_schedule.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.kv"

bash "$WRAPPER" optimizer-schedule --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-check-optimizer-schedule-v0' "$report" "tool must emit contract"
guard_expect_fixed_in_file "$TAG" 'tool_surface=hako_check_optimizer_schedule' "$report" "tool must name surface"
guard_expect_fixed_in_file "$TAG" 'observation_only=1' "$report" "tool must be observation only"
guard_expect_fixed_in_file "$TAG" 'rewrite_executed=0' "$report" "tool must not rewrite"
guard_expect_fixed_in_file "$TAG" 'keeper_selection=0' "$report" "tool must not select keepers"
guard_expect_fixed_in_file "$TAG" 'optimizer_schedule_truth_source=src/mir/optimizer/core.rs::MIR_OPT_PIPELINE_GROUPS' "$report" "tool must cite Rust const truth source"
guard_expect_fixed_in_file "$TAG" 'hako_check_optimizer_truth_count=0' "$report" "hako_check must not own optimizer truth"
guard_expect_fixed_in_file "$TAG" 'optimizer_behavior_changed=0' "$report" "tool must not imply behavior change"
guard_expect_fixed_in_file "$TAG" 'optimizer_physical_pass_merge_count=0' "$report" "tool must not merge physical passes"
guard_expect_fixed_in_file "$TAG" 'visible_group_count=7' "$report" "tool must report seven visible groups"
guard_expect_fixed_in_file "$TAG" 'visible_group_order_matches_expected=1' "$report" "tool must lock visible order"
guard_expect_fixed_in_file "$TAG" 'schedule_group[0]=normalize_frontend_surface' "$report" "tool must report group 0"
guard_expect_fixed_in_file "$TAG" 'schedule_group[6]=optional_and_diagnostics' "$report" "tool must report group 6"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
