#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-phase-cost-ablation"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_60="docs/development/current/main/phases/phase-296x/296x-60-HAKO-MIMALLOC-PERF-PHASE-COST-ABLATION.md"
CARD_61="docs/development/current/main/phases/phase-296x/296x-61-HAKO-MIMALLOC-PERF-SECOND-KEEPER-OPTIMIZATION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
RESET_APP="apps/hako-alloc-mimalloc-comparison-in-process-reset-only-proof/main.hako"
RESET_ALLOC_APP="apps/hako-alloc-mimalloc-comparison-in-process-reset-alloc-only-proof/main.hako"
FULL_APP="apps/hako-alloc-mimalloc-comparison-in-process-small-block-proof/main.hako"
TOOL="tools/allocator/hako_mimalloc_phase_cost_ablation.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_phase_cost_ablation_guard.sh"

echo "[$TAG] checking phase-296x phase cost ablation"

guard_require_files "$TAG" "$CARD_60" "$CARD_61" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$RESET_APP" "$RESET_ALLOC_APP" "$FULL_APP" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_60" "phase cost ablation card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_61" "second keeper card must be current"
guard_expect_fixed_in_file "$TAG" 'hako_level_vs_mirbuilder_level=hako_allocator_model_primary' "$CARD_60" "card must classify hako level as primary"
guard_expect_fixed_in_file "$TAG" 'dominant_phase=alloc' "$CARD_60" "card must select alloc phase"
guard_expect_fixed_in_file "$TAG" 'next_optimization_target=acquire_usize_fast_path_and_invariant_hoist' "$CARD_60" "card must select acquire target"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=1' "$CARD_60" "card must allow next optimization"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_60" "card must keep winner closed"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-60-HAKO-MIMALLOC-PERF-PHASE-COST-ABLATION"' "$CURRENT_STATE" "current state latest card must advance to row 60"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-SECOND-KEEPER-OPTIMIZATION-296X-001"' "$CURRENT_STATE" "current state must select row 61"
guard_expect_fixed_in_file "$TAG" '| 60 | `HAKO-MIMALLOC-PERF-PHASE-COST-ABLATION-296X-001` | Landed |' "$TASKBOARD" "taskboard row 60 must be landed"
guard_expect_fixed_in_file "$TAG" '| 61 | `HAKO-MIMALLOC-PERF-SECOND-KEEPER-OPTIMIZATION-296X-001` | Current |' "$TASKBOARD" "taskboard row 61 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list ablation tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_phase_cost.XXXXXX)"
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
external_elapsed_ms=190
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
external_elapsed_ms=280
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
guard_expect_fixed_in_file "$TAG" 'process_repeat=3' "$report" "tool must preserve sample count"
guard_expect_fixed_in_file "$TAG" 'hako_level_vs_mirbuilder_level=hako_allocator_model_primary' "$report" "tool must classify hako model"
guard_expect_fixed_in_file "$TAG" 'reset_only_elapsed_median_ms=60' "$report" "tool must compute reset median"
guard_expect_fixed_in_file "$TAG" 'reset_alloc_only_elapsed_median_ms=190' "$report" "tool must compute reset alloc median"
guard_expect_fixed_in_file "$TAG" 'full_elapsed_median_ms=280' "$report" "tool must compute full median"
guard_expect_fixed_in_file "$TAG" 'alloc_only_estimated_ms=130' "$report" "tool must compute alloc estimate"
guard_expect_fixed_in_file "$TAG" 'release_only_elapsed_median_ms=90' "$report" "tool must compute release estimate"
guard_expect_fixed_in_file "$TAG" 'dominant_phase=alloc' "$report" "tool must select alloc phase"
guard_expect_fixed_in_file "$TAG" 'next_optimization_target=acquire_usize_fast_path_and_invariant_hoist' "$report" "tool must select acquire target"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=1' "$report" "tool must allow next optimization"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$report" "tool must keep provider closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$report" "tool must keep hook closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$report" "tool must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
