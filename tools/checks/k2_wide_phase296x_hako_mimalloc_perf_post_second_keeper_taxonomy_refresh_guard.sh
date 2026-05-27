#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-post-second-keeper-taxonomy-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_62="docs/development/current/main/phases/phase-296x/296x-62-HAKO-MIMALLOC-PERF-POST-SECOND-KEEPER-TAXONOMY-REFRESH.md"
CARD_63="docs/development/current/main/phases/phase-296x/296x-63-HAKO-MIMALLOC-PERF-POST-SECOND-PHASE-COST-REFRESH.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_post_second_keeper_taxonomy_refresh.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_post_second_keeper_taxonomy_refresh_guard.sh"

echo "[$TAG] checking phase-296x post-second keeper taxonomy refresh"

guard_require_files "$TAG" "$CARD_62" "$CARD_63" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_62" "post-second taxonomy card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_63" "post-second phase cost card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-second-keeper-taxonomy-refresh-v0' "$CARD_62" "card must record refresh contract"
guard_expect_fixed_in_file "$TAG" 'current_hako_external_elapsed_median_ms=260' "$CARD_62" "card must record hako median"
guard_expect_fixed_in_file "$TAG" 'remaining_gap_ms=256' "$CARD_62" "card must record remaining gap"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=post_second_phase_cost_ablation_refresh' "$CARD_62" "card must select phase cost refresh"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=0' "$CARD_62" "card must keep optimization closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_62" "card must keep winner closed"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-62-HAKO-MIMALLOC-PERF-POST-SECOND-KEEPER-TAXONOMY-REFRESH"' "$CURRENT_STATE" "current state latest card must advance to row 62"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-POST-SECOND-PHASE-COST-REFRESH-296X-001"' "$CURRENT_STATE" "current state must select row 63"
guard_expect_fixed_in_file "$TAG" '| 62 | `HAKO-MIMALLOC-PERF-POST-SECOND-KEEPER-TAXONOMY-REFRESH-296X-001` | Landed |' "$TASKBOARD" "taskboard row 62 must be landed"
guard_expect_fixed_in_file "$TAG" '| 63 | `HAKO-MIMALLOC-PERF-POST-SECOND-PHASE-COST-REFRESH-296X-001` | Current |' "$TASKBOARD" "taskboard row 63 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list refresh tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_post_second.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
current="$tmp_dir/current.out"
report="$tmp_dir/refresh.out"

cat > "$current" <<'EOF'
output_contract=hako-mimalloc-in-process-operation-repeat-measurement-v0
measurement_profile=hako-mimalloc-in-process-operation-repeat-v0
timing_repeat_kind=in-process-operation-loop-v0
workload_id=representative-small-block-v0
operation_repeat=8192
process_repeat=3
same_workload=1
same_operation_count=1
process_invocation_repeat=0
hako_external_elapsed_median_ms=260
c_external_elapsed_median_ms=4
external_elapsed_median_gap_ms=256
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
EOF

python3 "$TOOL" --current "$current" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-second-keeper-taxonomy-refresh-v0' "$report" "tool must emit refresh contract"
guard_expect_fixed_in_file "$TAG" 'current_hako_external_elapsed_median_ms=260' "$report" "tool must preserve hako median"
guard_expect_fixed_in_file "$TAG" 'current_c_external_elapsed_median_ms=4' "$report" "tool must preserve c median"
guard_expect_fixed_in_file "$TAG" 'remaining_gap_ms=256' "$report" "tool must preserve remaining gap"
guard_expect_fixed_in_file "$TAG" 'gap_owner=allocator_algorithm' "$report" "tool must keep allocator owner"
guard_expect_fixed_in_file "$TAG" 'gap_confidence=high' "$report" "tool must keep high confidence"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=post_second_phase_cost_ablation_refresh' "$report" "tool must select phase cost refresh"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=0' "$report" "tool must keep optimization closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$report" "tool must keep provider closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$report" "tool must keep hook closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$report" "tool must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
