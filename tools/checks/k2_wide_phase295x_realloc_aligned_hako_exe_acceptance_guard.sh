#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-realloc-aligned-hako-exe-acceptance"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-17-MIMALLOC-COMPARISON-REALLOC-ALIGNED-HAKO-EXE-ACCEPTANCE.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-16-MIMALLOC-COMPARISON-REALLOC-ALIGNED-CONTRACT-REFRESH.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_realloc_aligned_hako_exe_acceptance_guard.sh"
HAKO_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
MODEL_GUARD="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_realloc_aligned_slice_guard.sh"
APP="apps/hako-alloc-mimalloc-comparison-realloc-aligned-exe-proof/main.hako"
APP_README="apps/hako-alloc-mimalloc-comparison-realloc-aligned-exe-proof/README.md"
APP_TEST="apps/hako-alloc-mimalloc-comparison-realloc-aligned-exe-proof/test.sh"
MODEL_APP="apps/hako-alloc-mimalloc-comparison-realloc-aligned-slice-proof/main.hako"

echo "[$TAG] checking phase-295x realloc/aligned hako EXE acceptance"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PREV_CARD" \
  "$TASKBOARD" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "$HAKO_RUNNER" \
  "$MODEL_GUARD" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$MODEL_APP"

guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$HAKO_RUNNER" "$MODEL_GUARD" "$APP_TEST"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REALLOC-ALIGNED-HAKO-EXE-ACCEPTANCE-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REALLOC-ALIGNED-EVIDENCE-295X-RUN-001' "$CARD" "card must select evidence run follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REALLOC-ALIGNED-HAKO-EXE-ACCEPTANCE-295X-001' "$PREV_CARD" "previous row must select this acceptance blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REALLOC-ALIGNED-EVIDENCE-295X-RUN-001' "$TASKBOARD" "taskboard must expose evidence run follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'workload=representative-realloc-aligned-v0' "$APP" "EXE app must use same workload id"
guard_expect_in_file "$TAG" 'operation_family=realloc-aligned' "$APP" "EXE app must expose operation family"
guard_expect_in_file "$TAG" 'operation_sequence_id=representative-realloc-aligned-v0-seq' "$APP" "EXE app must expose operation sequence id"
guard_expect_in_file "$TAG" 'free_order_id=ascending-release-v0' "$APP" "EXE app must expose free order id"

if rg -n 'LD_PRELOAD|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|winner_claim=1|provider_activation=1|host_replacement=1|thread::|worker_local|nowait|await' "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: hako EXE acceptance opened replacement/provider/thread/winner seams" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

bash "$MODEL_GUARD"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_realloc_aligned_hako_exe.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
hako_out="$tmp_dir/hako.out"

bash "$HAKO_RUNNER" --app "$APP" --workload representative-realloc-aligned-v0 --out "$hako_out"

rg -F -q 'output_contract=hako-exe-memory-evidence-v0' "$hako_out"
rg -F -q 'workload=representative-realloc-aligned-v0' "$hako_out"
rg -F -q 'operation_family=realloc-aligned' "$hako_out"
rg -F -q 'operation_sequence_id=representative-realloc-aligned-v0-seq' "$hako_out"
rg -F -q 'free_order_id=ascending-release-v0' "$hako_out"
rg -F -q 'allocation_count=4' "$hako_out"
rg -F -q 'free_count=4' "$hako_out"
rg -F -q 'requested_bytes=216' "$hako_out"
rg -F -q 'realloc_count=2' "$hako_out"
rg -F -q 'aligned_alloc_count=2' "$hako_out"
rg -F -q 'alignment_request_count=3' "$hako_out"
rg -F -q 'alignment_ok_count=2' "$hako_out"
rg -F -q 'alignment_reject_count=1' "$hako_out"
rg -F -q 'provider_activation=0' "$hako_out"
rg -F -q 'host_replacement=0' "$hako_out"
rg -F -q 'hook_installed=0' "$hako_out"
rg -F -q 'global_allocator_installed=0' "$hako_out"
rg -F -q 'summary=ok' "$hako_out"

cat "$hako_out"
echo "[$TAG] ok"
