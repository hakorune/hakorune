#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-post-keeper-taxonomy-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_59="docs/development/current/main/phases/phase-296x/296x-59-HAKO-MIMALLOC-PERF-POST-KEEPER-TAXONOMY-REFRESH.md"
CARD_60="docs/development/current/main/phases/phase-296x/296x-60-HAKO-MIMALLOC-PERF-PHASE-COST-ABLATION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_post_keeper_taxonomy_refresh.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_post_keeper_taxonomy_refresh_guard.sh"

echo "[$TAG] checking phase-296x post-keeper taxonomy refresh"

guard_require_files "$TAG" "$CARD_59" "$CARD_60" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_59" "post-keeper taxonomy card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_60" "phase cost ablation card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-keeper-taxonomy-refresh-v0' "$CARD_59" "card must record refresh contract"
guard_expect_fixed_in_file "$TAG" 'remaining_gap_ms=276' "$CARD_59" "card must preserve remaining gap"
guard_expect_fixed_in_file "$TAG" 'external_timing_collectors_same=0' "$CARD_59" "card must record timing collector bias"
guard_expect_fixed_in_file "$TAG" 'same_workload_semantics=partial' "$CARD_59" "card must record partial semantics"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=phase_cost_ablation_reset_alloc_release' "$CARD_59" "card must select phase cost ablation"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=0' "$CARD_59" "card must keep optimization closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_59" "card must keep winner closed"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-59-HAKO-MIMALLOC-PERF-POST-KEEPER-TAXONOMY-REFRESH"' "$CURRENT_STATE" "current state latest card must advance to row 59"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-PHASE-COST-ABLATION-296X-001"' "$CURRENT_STATE" "current state must select row 60"
guard_expect_fixed_in_file "$TAG" '| 59 | `HAKO-MIMALLOC-PERF-POST-KEEPER-TAXONOMY-REFRESH-296X-001` | Landed |' "$TASKBOARD" "taskboard row 59 must be landed"
guard_expect_fixed_in_file "$TAG" '| 60 | `HAKO-MIMALLOC-PERF-PHASE-COST-ABLATION-296X-001` | Current |' "$TASKBOARD" "taskboard row 60 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list refresh tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_post_keeper.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
previous="$tmp_dir/previous.out"
current="$tmp_dir/current.out"
report="$tmp_dir/refresh.out"

cat > "$previous" <<'EOF'
output_contract=hako-mimalloc-in-process-operation-repeat-measurement-v0
measurement_profile=hako-mimalloc-in-process-operation-repeat-v0
timing_repeat_kind=in-process-operation-loop-v0
workload_id=representative-small-block-v0
operation_repeat=8192
process_repeat=3
same_workload=1
same_operation_count=1
process_invocation_repeat=0
hako_external_elapsed_median_ms=330
c_external_elapsed_median_ms=4
external_elapsed_median_gap_ms=326
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
EOF

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
hako_external_elapsed_median_ms=280
c_external_elapsed_median_ms=4
external_elapsed_median_gap_ms=276
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
EOF

python3 "$TOOL" --previous "$previous" --current "$current" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-keeper-taxonomy-refresh-v0' "$report" "tool must emit refresh contract"
guard_expect_fixed_in_file "$TAG" 'previous_hako_external_elapsed_median_ms=330' "$report" "tool must preserve previous median"
guard_expect_fixed_in_file "$TAG" 'current_hako_external_elapsed_median_ms=280' "$report" "tool must preserve current median"
guard_expect_fixed_in_file "$TAG" 'improvement_ms=50' "$report" "tool must compute improvement"
guard_expect_fixed_in_file "$TAG" 'remaining_gap_ms=276' "$report" "tool must preserve remaining gap"
guard_expect_fixed_in_file "$TAG" 'external_timing_collector_hako=usr_bin_time_elapsed' "$report" "tool must name hako collector"
guard_expect_fixed_in_file "$TAG" 'external_timing_collector_c=python_perf_counter_subprocess' "$report" "tool must name c collector"
guard_expect_fixed_in_file "$TAG" 'external_timing_collectors_same=0' "$report" "tool must record collector mismatch"
guard_expect_fixed_in_file "$TAG" 'body_elapsed_comparable=0' "$report" "tool must keep body timing secondary/unavailable"
guard_expect_fixed_in_file "$TAG" 'same_workload_semantics=partial' "$report" "tool must scope semantics"
guard_expect_fixed_in_file "$TAG" 'interpretation_scope=operation-count-parity-only' "$report" "tool must scope interpretation"
guard_expect_fixed_in_file "$TAG" 'gap_owner=allocator_algorithm' "$report" "tool must keep allocator owner"
guard_expect_fixed_in_file "$TAG" 'gap_confidence=high' "$report" "tool must keep high confidence"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=phase_cost_ablation_reset_alloc_release' "$report" "tool must select phase cost ablation"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=0' "$report" "tool must keep optimization closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$report" "tool must keep provider closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$report" "tool must keep hook closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$report" "tool must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
