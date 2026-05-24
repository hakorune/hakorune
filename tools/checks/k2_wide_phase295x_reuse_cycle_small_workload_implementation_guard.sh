#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-reuse-cycle-small-workload-implementation"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-80-MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-WORKLOAD-IMPLEMENTATION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-79-MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-WORKLOAD-CONTRACT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_reuse_cycle_small_workload_implementation_guard.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"
H_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
REPORTER="tools/allocator/mimalloc_comparison_memory_report.py"
APP="apps/hako-alloc-mimalloc-comparison-reuse-cycle-small-exe-proof/main.hako"

echo "[$TAG] checking phase-295x reuse-cycle small workload implementation"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$C_RUNNER" "$H_RUNNER" "$REPORTER" "$APP"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$C_RUNNER" "$H_RUNNER" "$REPORTER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-WORKLOAD-IMPLEMENTATION-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-WORKLOAD-CLOSEOUT-295X-001' "$CARD" "card must select closeout"
guard_expect_in_file "$TAG" 'representative-reuse-cycle-small-v0' "tools/allocator/c_mimalloc_explicit_runner.c" "C source must implement workload"
guard_expect_in_file "$TAG" 'representative-reuse-cycle-small-v0' "$APP" ".hako app must expose workload"
guard_expect_in_file "$TAG" 'reuse_cycle_count' "$H_RUNNER" "hako runner must parse reuse cycle count"
guard_expect_in_file "$TAG" 'reuse_cycle_count' "$REPORTER" "normalizer must publish reuse cycle count"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-WORKLOAD-IMPLEMENTATION-295X-001' "$PREV_CARD" "previous card must select implementation"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-WORKLOAD-CLOSEOUT-295X-001' "$TASKBOARD" "taskboard must expose closeout"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_reuse_cycle.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
c_out="$tmp_dir/c.out"
hako_out="$tmp_dir/hako.out"
report_out="$tmp_dir/report.out"

bash "$C_RUNNER" \
  --out "$c_out" \
  --workload representative-reuse-cycle-small-v0 \
  --operation-repeat 1 \
  --allow-ldconfig-discovery >/tmp/"$TAG".c.stdout

bash "$H_RUNNER" \
  --app "$APP" \
  --workload representative-reuse-cycle-small-v0 \
  --runtime-config empty \
  --out "$hako_out" >/tmp/"$TAG".hako.stdout

python3 "$REPORTER" --hako "$hako_out" --c "$c_out" --out "$report_out"

for out in "$c_out" "$hako_out"; do
  rg -F -q 'workload=representative-reuse-cycle-small-v0' "$out"
  rg -F -q 'operation_family=reuse-cycle-small' "$out"
  rg -F -q 'operation_sequence_id=representative-reuse-cycle-small-v0-seq' "$out"
  rg -F -q 'free_order_id=even-odd-release-then-reacquire-v0' "$out"
  rg -F -q 'allocation_count=128' "$out"
  rg -F -q 'free_count=128' "$out"
  rg -F -q 'requested_bytes=66508' "$out"
  rg -F -q 'reuse_cycle_count=1' "$out"
  rg -F -q 'summary=ok' "$out"
done

rg -F -q 'workload_match=1' "$report_out"
rg -F -q 'operation_family_match=1' "$report_out"
rg -F -q 'operation_sequence_match=1' "$report_out"
rg -F -q 'free_order_match=1' "$report_out"
rg -F -q 'allocation_count_delta=0' "$report_out"
rg -F -q 'free_count_delta=0' "$report_out"
rg -F -q 'requested_bytes_delta=0' "$report_out"
rg -F -q 'reuse_cycle_count_delta=0' "$report_out"
rg -F -q 'winner_claim=0' "$report_out"
rg -F -q 'summary=ok' "$report_out"

cat "$report_out"
echo "[$TAG] ok"
