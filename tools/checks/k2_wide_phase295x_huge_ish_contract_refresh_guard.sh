#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-huge-ish-contract-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-25-MIMALLOC-COMPARISON-HUGE-ISH-CONTRACT-REFRESH.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-24-MIMALLOC-COMPARISON-HUGE-ISH-WORKLOAD-SEAM-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_huge_ish_contract_refresh_guard.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"
C_RUNNER_SRC="tools/allocator/c_mimalloc_explicit_runner.c"
HAKO_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
NORMALIZER="tools/allocator/mimalloc_comparison_memory_report.py"
APP="apps/hako-alloc-mimalloc-comparison-huge-ish-exe-proof/main.hako"
APP_README="apps/hako-alloc-mimalloc-comparison-huge-ish-exe-proof/README.md"
APP_TEST="apps/hako-alloc-mimalloc-comparison-huge-ish-exe-proof/test.sh"

echo "[$TAG] checking phase-295x huge-ish contract refresh"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$C_RUNNER" "$C_RUNNER_SRC" "$HAKO_RUNNER" "$NORMALIZER" "$APP" "$APP_README" "$APP_TEST"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$C_RUNNER" "$HAKO_RUNNER" "$APP_TEST"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HUGE-ISH-CONTRACT-295X-REFRESH-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HUGE-ISH-EVIDENCE-295X-RUN-001' "$CARD" "card must select evidence run follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HUGE-ISH-CONTRACT-295X-REFRESH-001' "$PREV_CARD" "previous row must select this contract refresh"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HUGE-ISH-EVIDENCE-295X-RUN-001' "$TASKBOARD" "taskboard must expose evidence run follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'representative-huge-ish-v0' "$C_RUNNER_SRC" "C runner must implement huge-ish workload"
guard_expect_in_file "$TAG" 'large_request_count' "$HAKO_RUNNER" "hako runner must publish large request count"
guard_expect_in_file "$TAG" 'large_request_count_delta' "$NORMALIZER" "normalizer must publish large request count delta"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_huge_contract.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
c_out="$tmp_dir/c.out"
hako_out="$tmp_dir/hako.out"

bash "$C_RUNNER" --out "$c_out" --allow-ldconfig-discovery --workload representative-huge-ish-v0
bash "$HAKO_RUNNER" --app "$APP" --workload representative-huge-ish-v0 --out "$hako_out"

for out in "$c_out" "$hako_out"; do
  rg -F -q 'workload=representative-huge-ish-v0' "$out"
  rg -F -q 'operation_family=huge-ish' "$out"
  rg -F -q 'operation_sequence_id=representative-huge-ish-v0-seq' "$out"
  rg -F -q 'free_order_id=ascending-release-v0' "$out"
  rg -F -q 'allocation_count=2' "$out"
  rg -F -q 'free_count=2' "$out"
  rg -F -q 'requested_bytes=4194321' "$out"
  rg -F -q 'large_request_count=1' "$out"
  rg -F -q 'summary=ok' "$out"
done

echo "[$TAG] ok"
