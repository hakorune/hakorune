#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-realloc-aligned-contract-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-16-MIMALLOC-COMPARISON-REALLOC-ALIGNED-CONTRACT-REFRESH.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-15-MIMALLOC-COMPARISON-NEXT-WORKLOAD-SEAM-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_realloc_aligned_contract_refresh_guard.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"
C_RUNNER_SRC="tools/allocator/c_mimalloc_explicit_runner.c"
H_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
NORMALIZER="tools/allocator/mimalloc_comparison_memory_report.py"
HAKO_GUARD="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_realloc_aligned_slice_guard.sh"
APP="apps/hako-alloc-mimalloc-comparison-realloc-aligned-slice-proof/main.hako"

echo "[$TAG] checking phase-295x realloc/aligned contract refresh"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PREV_CARD" \
  "$TASKBOARD" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "$C_RUNNER" \
  "$C_RUNNER_SRC" \
  "$H_RUNNER" \
  "$NORMALIZER" \
  "$HAKO_GUARD" \
  "$APP"

guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$C_RUNNER" "$H_RUNNER" "$NORMALIZER" "$HAKO_GUARD"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REALLOC-ALIGNED-CONTRACT-295X-REFRESH-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REALLOC-ALIGNED-HAKO-EXE-ACCEPTANCE-295X-001' "$CARD" "card must select hako EXE acceptance follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REALLOC-ALIGNED-CONTRACT-295X-REFRESH-001' "$PREV_CARD" "previous row must select this contract refresh"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REALLOC-ALIGNED-HAKO-EXE-ACCEPTANCE-295X-001' "$TASKBOARD" "taskboard must expose hako EXE acceptance follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" '--workload' "$C_RUNNER" "C runner wrapper must expose workload selection"
guard_expect_in_file "$TAG" 'representative-realloc-aligned-v0' "$C_RUNNER_SRC" "C runner must implement realloc/aligned workload"
guard_expect_in_file "$TAG" 'operation_sequence_id=representative-realloc-aligned-v0-seq' "$APP" "hako app must expose operation sequence id"
guard_expect_in_file "$TAG" 'free_order_id=ascending-release-v0' "$APP" "hako app must expose free order id"
guard_expect_in_file "$TAG" 'operation_family_match=' "$NORMALIZER" "normalizer must publish operation family match"
guard_expect_in_file "$TAG" 'realloc_count_delta=' "$NORMALIZER" "normalizer must publish realloc count delta"
guard_expect_in_file "$TAG" 'aligned_alloc_count_delta=' "$NORMALIZER" "normalizer must publish aligned alloc count delta"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_realloc_aligned_contract.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
small_out="$tmp_dir/small.out"
realloc_out="$tmp_dir/realloc.out"

bash "$C_RUNNER" --out "$small_out" --allow-ldconfig-discovery --workload representative-small-block-v0 >/tmp/"$TAG".small.stdout
bash "$C_RUNNER" --out "$realloc_out" --allow-ldconfig-discovery --workload representative-realloc-aligned-v0 >/tmp/"$TAG".realloc.stdout

rg -F -q 'workload=representative-small-block-v0' "$small_out"
rg -F -q 'operation_family=small-block' "$small_out"
rg -F -q 'free_order_id=even-odd-release-v0' "$small_out"
rg -F -q 'allocation_count=64' "$small_out"
rg -F -q 'free_count=64' "$small_out"
rg -F -q 'requested_bytes=33254' "$small_out"

rg -F -q 'workload=representative-realloc-aligned-v0' "$realloc_out"
rg -F -q 'operation_family=realloc-aligned' "$realloc_out"
rg -F -q 'operation_sequence_id=representative-realloc-aligned-v0-seq' "$realloc_out"
rg -F -q 'free_order_id=ascending-release-v0' "$realloc_out"
rg -F -q 'requested_bytes=216' "$realloc_out"
rg -F -q 'realloc_count=2' "$realloc_out"
rg -F -q 'aligned_alloc_count=2' "$realloc_out"
rg -F -q 'alignment_request_count=3' "$realloc_out"
rg -F -q 'alignment_ok_count=2' "$realloc_out"
rg -F -q 'alignment_reject_count=1' "$realloc_out"
rg -F -q 'summary=ok' "$realloc_out"

bash "$HAKO_GUARD"

echo "[$TAG] ok"
