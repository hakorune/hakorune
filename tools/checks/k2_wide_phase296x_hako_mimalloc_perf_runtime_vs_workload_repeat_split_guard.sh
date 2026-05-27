#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-runtime-vs-workload-repeat-split"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_53="docs/development/current/main/phases/phase-296x/296x-53-HAKO-MIMALLOC-PERF-RUNTIME-VS-WORKLOAD-REPEAT-SPLIT-DIAGNOSTIC.md"
CARD_54="docs/development/current/main/phases/phase-296x/296x-54-HAKO-MIMALLOC-PERF-IN-PROCESS-OPERATION-REPEAT-CONTRACT.md"
CARD_55="docs/development/current/main/phases/phase-296x/296x-55-HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SPLIT="tools/allocator/hako_mimalloc_runtime_vs_workload_repeat_split.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_runtime_vs_workload_repeat_split_guard.sh"

echo "[$TAG] checking phase-296x runtime-vs-workload repeat split"

guard_require_files "$TAG" "$CARD_53" "$CARD_54" "$CARD_55" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$SPLIT" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SPLIT" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_53" "runtime-vs-workload split card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_54" "in-process operation repeat contract card must be current"
guard_expect_fixed_in_file "$TAG" 'Status: Planned' "$CARD_55" "first keeper optimization card must be planned"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-runtime-vs-workload-repeat-split-v0' "$CARD_53" "row 53 card must define split contract"
guard_expect_fixed_in_file "$TAG" 'selected_gap_owner=benchmark_harness' "$CARD_53" "row 53 card must classify harness owner"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=in_process_operation_repeat_contract' "$CARD_53" "row 53 card must select in-process repeat contract"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=0' "$CARD_53" "row 53 card must keep optimization closed"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-53-HAKO-MIMALLOC-PERF-RUNTIME-VS-WORKLOAD-REPEAT-SPLIT-DIAGNOSTIC"' "$CURRENT_STATE" "current state latest card must advance to row 53"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-IN-PROCESS-OPERATION-REPEAT-CONTRACT-296X-001"' "$CURRENT_STATE" "current state must select row 54"
guard_expect_fixed_in_file "$TAG" '| 53 | `HAKO-MIMALLOC-PERF-RUNTIME-VS-WORKLOAD-REPEAT-SPLIT-DIAGNOSTIC-296X-001` | Landed |' "$TASKBOARD" "taskboard row 53 must be landed"
guard_expect_fixed_in_file "$TAG" '| 54 | `HAKO-MIMALLOC-PERF-IN-PROCESS-OPERATION-REPEAT-CONTRACT-296X-001` | Current |' "$TASKBOARD" "taskboard row 54 must be current"
guard_expect_fixed_in_file "$TAG" '| 55 | `HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001` | Planned |' "$TASKBOARD" "taskboard row 55 must be planned"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$SPLIT" "$INDEX" "check index must list split tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_runtime_vs_workload.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

make_report() {
  local path="$1"
  local workload="$2"
  local repeat="$3"
  local hako_ms="$4"
  local c_ms="$5"
  cat >"$path" <<EOF
output_contract=mimalloc-comparison-repeated-measurement-v0
workload_0_id=$workload
workload_0_operation_repeat=$repeat
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

make_report "$tmp_dir/empty-128.out" representative-empty-v0 128 80 70
make_report "$tmp_dir/empty-1024.out" representative-empty-v0 1024 630 520
make_report "$tmp_dir/empty-8192.out" representative-empty-v0 8192 4780 3960
make_report "$tmp_dir/small-128.out" representative-small-block-v0 128 80 70
make_report "$tmp_dir/small-1024.out" representative-small-block-v0 1024 670 550
make_report "$tmp_dir/small-8192.out" representative-small-block-v0 8192 5390 4620

split="$tmp_dir/split.out"
python3 "$SPLIT" \
  --empty-report 128:"$tmp_dir/empty-128.out" \
  --empty-report 1024:"$tmp_dir/empty-1024.out" \
  --empty-report 8192:"$tmp_dir/empty-8192.out" \
  --small-report 128:"$tmp_dir/small-128.out" \
  --small-report 1024:"$tmp_dir/small-1024.out" \
  --small-report 8192:"$tmp_dir/small-8192.out" \
  --out "$split"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-runtime-vs-workload-repeat-split-v0' "$split" "split tool must emit contract"
guard_expect_fixed_in_file "$TAG" 'runtime_explains_ratio_pct=106' "$split" "split tool must record runtime explanation ratio"
guard_expect_fixed_in_file "$TAG" 'selected_gap_owner=benchmark_harness' "$split" "split tool must classify benchmark harness"
guard_expect_fixed_in_file "$TAG" 'selected_gap_confidence=high' "$split" "split tool must raise confidence"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=in_process_operation_repeat_contract' "$split" "split tool must select in-process repeat contract"
guard_expect_fixed_in_file "$TAG" 'next_optimization_allowed=0' "$split" "split tool must block optimization"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$split" "split tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$split" "split tool must keep provider closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$split" "split tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$split" "split tool must end ok"

echo "[$TAG] ok"
