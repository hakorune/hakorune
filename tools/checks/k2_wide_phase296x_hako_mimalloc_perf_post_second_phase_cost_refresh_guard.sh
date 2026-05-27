#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-post-second-phase-cost-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_63="docs/development/current/main/phases/phase-296x/296x-63-HAKO-MIMALLOC-PERF-POST-SECOND-PHASE-COST-REFRESH.md"
CARD_64="docs/development/current/main/phases/phase-296x/296x-64-HAKO-MIMALLOC-PERF-THIRD-KEEPER-OPTIMIZATION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_phase_cost_ablation.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_post_second_phase_cost_refresh_guard.sh"

echo "[$TAG] checking phase-296x post-second phase cost refresh"

guard_require_files "$TAG" "$CARD_63" "$CARD_64" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_63" "post-second phase cost card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_64" "third keeper card must be current"
guard_expect_fixed_in_file "$TAG" 'full_elapsed_median_ms=250' "$CARD_63" "card must record full median"
guard_expect_fixed_in_file "$TAG" 'alloc_only_estimated_ms=110' "$CARD_63" "card must record alloc estimate"
guard_expect_fixed_in_file "$TAG" 'release_only_elapsed_median_ms=80' "$CARD_63" "card must record release estimate"
guard_expect_fixed_in_file "$TAG" 'dominant_phase=alloc' "$CARD_63" "card must select alloc"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=1' "$CARD_63" "card must allow next keeper"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_63" "card must keep winner closed"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-63-HAKO-MIMALLOC-PERF-POST-SECOND-PHASE-COST-REFRESH"' "$CURRENT_STATE" "current state latest card must advance to row 63"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-THIRD-KEEPER-OPTIMIZATION-296X-001"' "$CURRENT_STATE" "current state must select row 64"
guard_expect_fixed_in_file "$TAG" '| 63 | `HAKO-MIMALLOC-PERF-POST-SECOND-PHASE-COST-REFRESH-296X-001` | Landed |' "$TASKBOARD" "taskboard row 63 must be landed"
guard_expect_fixed_in_file "$TAG" '| 64 | `HAKO-MIMALLOC-PERF-THIRD-KEEPER-OPTIMIZATION-296X-001` | Current |' "$TASKBOARD" "taskboard row 64 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_post_second_phase.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

for i in 0 1 2; do
  cat > "$tmp_dir/reset-$i.out" <<'EOF'
output_contract=hako-exe-memory-evidence-v0
workload=representative-small-block-reset-only-v0
runtime_config_profile=empty
result_code=0
run_count=1
operation_repeat=1
timing_repeat_kind=process-invocation-v0
in_process_operation_repeat=8192
app_timing_repeat_kind=in-process-operation-loop-v0
provider_activation=0
host_replacement=0
hook_installed=0
global_allocator_installed=0
external_elapsed_ms=60
summary=ok
EOF
  cat > "$tmp_dir/reset-alloc-$i.out" <<'EOF'
output_contract=hako-exe-memory-evidence-v0
workload=representative-small-block-reset-alloc-only-v0
runtime_config_profile=empty
result_code=0
run_count=1
operation_repeat=1
timing_repeat_kind=process-invocation-v0
in_process_operation_repeat=8192
app_timing_repeat_kind=in-process-operation-loop-v0
provider_activation=0
host_replacement=0
hook_installed=0
global_allocator_installed=0
external_elapsed_ms=170
summary=ok
EOF
  cat > "$tmp_dir/full-$i.out" <<'EOF'
output_contract=hako-exe-memory-evidence-v0
workload=representative-small-block-v0
runtime_config_profile=empty
result_code=0
run_count=1
operation_repeat=1
timing_repeat_kind=process-invocation-v0
in_process_operation_repeat=8192
app_timing_repeat_kind=in-process-operation-loop-v0
provider_activation=0
host_replacement=0
hook_installed=0
global_allocator_installed=0
external_elapsed_ms=250
summary=ok
EOF
done

report="$tmp_dir/report.out"
python3 "$TOOL" \
  --reset-only "$tmp_dir"/reset-[0-9].out \
  --reset-alloc-only "$tmp_dir"/reset-alloc-[0-9].out \
  --full "$tmp_dir"/full-[0-9].out \
  --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-phase-cost-ablation-v0' "$report" "tool must emit ablation contract"
guard_expect_fixed_in_file "$TAG" 'reset_only_elapsed_median_ms=60' "$report" "tool must compute reset median"
guard_expect_fixed_in_file "$TAG" 'reset_alloc_only_elapsed_median_ms=170' "$report" "tool must compute reset alloc median"
guard_expect_fixed_in_file "$TAG" 'full_elapsed_median_ms=250' "$report" "tool must compute full median"
guard_expect_fixed_in_file "$TAG" 'alloc_only_estimated_ms=110' "$report" "tool must compute alloc estimate"
guard_expect_fixed_in_file "$TAG" 'release_only_elapsed_median_ms=80' "$report" "tool must compute release estimate"
guard_expect_fixed_in_file "$TAG" 'dominant_phase=alloc' "$report" "tool must select alloc"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=1' "$report" "tool must allow next optimization"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
