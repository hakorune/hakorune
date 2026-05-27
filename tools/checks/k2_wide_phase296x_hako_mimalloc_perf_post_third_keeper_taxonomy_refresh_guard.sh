#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-post-third-keeper-taxonomy-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_65="docs/development/current/main/phases/phase-296x/296x-65-HAKO-MIMALLOC-PERF-POST-THIRD-KEEPER-TAXONOMY-REFRESH.md"
CARD_66="docs/development/current/main/phases/phase-296x/296x-66-HAKO-MIMALLOC-PORT-FEATURE-GAP-INVENTORY.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_post_third_keeper_taxonomy_refresh.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_post_third_keeper_taxonomy_refresh_guard.sh"

echo "[$TAG] checking phase-296x post-third keeper taxonomy refresh"

guard_require_files "$TAG" "$CARD_65" "$CARD_66" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_65" "post-third taxonomy card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_66" "port feature inventory card must be current"
guard_expect_fixed_in_file "$TAG" 'current_hako_external_elapsed_median_ms=240' "$CARD_65" "card must record hako median"
guard_expect_fixed_in_file "$TAG" 'remaining_gap_ms=236' "$CARD_65" "card must record remaining gap"
guard_expect_fixed_in_file "$TAG" 'optimization_checkpoint=small_model_fast_path_plateau' "$CARD_65" "card must record checkpoint"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=port_feature_gap_inventory' "$CARD_65" "card must select feature inventory"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=0' "$CARD_65" "card must close immediate optimization"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_65" "card must keep winner closed"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-65-HAKO-MIMALLOC-PERF-POST-THIRD-KEEPER-TAXONOMY-REFRESH"' "$CURRENT_STATE" "current state latest card must advance to row 65"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PORT-FEATURE-GAP-INVENTORY-296X-001"' "$CURRENT_STATE" "current state must select row 66"
guard_expect_fixed_in_file "$TAG" '| 65 | `HAKO-MIMALLOC-PERF-POST-THIRD-KEEPER-TAXONOMY-REFRESH-296X-001` | Landed |' "$TASKBOARD" "taskboard row 65 must be landed"
guard_expect_fixed_in_file "$TAG" '| 66 | `HAKO-MIMALLOC-PORT-FEATURE-GAP-INVENTORY-296X-001` | Current |' "$TASKBOARD" "taskboard row 66 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list refresh tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_post_third.XXXXXX)"
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
hako_external_elapsed_median_ms=240
c_external_elapsed_median_ms=4
external_elapsed_median_gap_ms=236
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
EOF

python3 "$TOOL" --current "$current" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-third-keeper-taxonomy-refresh-v0' "$report" "tool must emit refresh contract"
guard_expect_fixed_in_file "$TAG" 'current_hako_external_elapsed_median_ms=240' "$report" "tool must preserve hako median"
guard_expect_fixed_in_file "$TAG" 'remaining_gap_ms=236' "$report" "tool must preserve remaining gap"
guard_expect_fixed_in_file "$TAG" 'optimization_checkpoint=small_model_fast_path_plateau' "$report" "tool must record checkpoint"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=port_feature_gap_inventory' "$report" "tool must select feature inventory"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=0' "$report" "tool must close immediate optimization"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$report" "tool must keep provider closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$report" "tool must keep hook closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$report" "tool must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
