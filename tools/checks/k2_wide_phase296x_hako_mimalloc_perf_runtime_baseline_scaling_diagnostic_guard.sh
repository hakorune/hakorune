#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-runtime-baseline-scaling-diagnostic"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_52="docs/development/current/main/phases/phase-296x/296x-52-HAKO-MIMALLOC-PERF-RUNTIME-BASELINE-SCALING-DIAGNOSTIC.md"
CARD_53="docs/development/current/main/phases/phase-296x/296x-53-HAKO-MIMALLOC-PERF-RUNTIME-VS-WORKLOAD-REPEAT-SPLIT-DIAGNOSTIC.md"
CARD_54="docs/development/current/main/phases/phase-296x/296x-54-HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DIAGNOSTIC="tools/allocator/hako_mimalloc_runtime_baseline_scaling_diagnostic.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_runtime_baseline_scaling_diagnostic_guard.sh"

echo "[$TAG] checking phase-296x runtime baseline scaling diagnostic"

guard_require_files "$TAG" "$CARD_52" "$CARD_53" "$CARD_54" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$DIAGNOSTIC" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$DIAGNOSTIC" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_52" "runtime scaling card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_53" "owner split diagnostic card must be current"
guard_expect_fixed_in_file "$TAG" 'Status: Planned' "$CARD_54" "first keeper optimization card must be planned"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-runtime-baseline-scaling-diagnostic-v0' "$CARD_52" "row 52 card must define scaling diagnostic contract"
guard_expect_fixed_in_file "$TAG" 'per_invocation_growth_observed=1' "$CARD_52" "row 52 card must record invocation growth"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=runtime_vs_workload_repeat_split_diagnostic' "$CARD_52" "row 52 card must select runtime-vs-workload split"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=0' "$CARD_52" "row 52 card must keep optimization closed"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-52-HAKO-MIMALLOC-PERF-RUNTIME-BASELINE-SCALING-DIAGNOSTIC"' "$CURRENT_STATE" "current state latest card must advance to row 52"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-RUNTIME-VS-WORKLOAD-REPEAT-SPLIT-DIAGNOSTIC-296X-001"' "$CURRENT_STATE" "current state must select row 53"
guard_expect_fixed_in_file "$TAG" '| 52 | `HAKO-MIMALLOC-PERF-RUNTIME-BASELINE-SCALING-DIAGNOSTIC-296X-001` | Landed |' "$TASKBOARD" "taskboard row 52 must be landed"
guard_expect_fixed_in_file "$TAG" '| 53 | `HAKO-MIMALLOC-PERF-RUNTIME-VS-WORKLOAD-REPEAT-SPLIT-DIAGNOSTIC-296X-001` | Current |' "$TASKBOARD" "taskboard row 53 must be current"
guard_expect_fixed_in_file "$TAG" '| 54 | `HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001` | Planned |' "$TASKBOARD" "taskboard row 54 must be planned"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$DIAGNOSTIC" "$INDEX" "check index must list diagnostic tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_runtime_scaling.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

make_report() {
  local path="$1"
  local repeat="$2"
  local hako_ms="$3"
  local c_ms="$4"
  cat >"$path" <<EOF
output_contract=mimalloc-comparison-repeated-measurement-v0
workload_0_id=representative-small-block-v0
workload_0_operation_repeat=$repeat
workload_0_sample_count=3
workload_0_hako_external_elapsed_median_ms=$hako_ms
workload_0_c_external_elapsed_median_ms=$c_ms
provider_activation=0
host_replacement=0
hook_installed=0
global_allocator_installed=0
winner_claim=0
summary=ok
EOF
}

make_report "$tmp_dir/repeat-128.out" 128 90 80
make_report "$tmp_dir/repeat-1024.out" 1024 660 570
make_report "$tmp_dir/repeat-8192.out" 8192 5230 4480

diagnostic="$tmp_dir/scaling.out"
python3 "$DIAGNOSTIC" \
  --report 128:"$tmp_dir/repeat-128.out" \
  --report 1024:"$tmp_dir/repeat-1024.out" \
  --report 8192:"$tmp_dir/repeat-8192.out" \
  --out "$diagnostic"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-runtime-baseline-scaling-diagnostic-v0' "$diagnostic" "scaling diagnostic must emit contract"
guard_expect_fixed_in_file "$TAG" 'gap_growth_ms=740' "$diagnostic" "scaling diagnostic must record gap growth"
guard_expect_fixed_in_file "$TAG" 'per_invocation_growth_observed=1' "$diagnostic" "scaling diagnostic must detect invocation growth"
guard_expect_fixed_in_file "$TAG" 'runtime_baseline_fixed_gap_observed=0' "$diagnostic" "scaling diagnostic must reject fixed-only baseline"
guard_expect_fixed_in_file "$TAG" 'refreshed_gap_owner=process_invocation_scaling_gap' "$diagnostic" "scaling diagnostic must classify invocation scaling owner"
guard_expect_fixed_in_file "$TAG" 'refreshed_gap_confidence=medium' "$diagnostic" "scaling diagnostic must keep medium confidence"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=runtime_vs_workload_repeat_split_diagnostic' "$diagnostic" "scaling diagnostic must select runtime-vs-workload split"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=0' "$diagnostic" "scaling diagnostic must block optimization"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$diagnostic" "scaling diagnostic must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$diagnostic" "scaling diagnostic must keep provider closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$diagnostic" "scaling diagnostic must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$diagnostic" "scaling diagnostic must end ok"

echo "[$TAG] ok"
