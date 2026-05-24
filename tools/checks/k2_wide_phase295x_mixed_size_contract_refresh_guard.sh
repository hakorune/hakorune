#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mixed-size-contract-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-21-MIMALLOC-COMPARISON-MIXED-SIZE-CONTRACT-REFRESH.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-20-MIMALLOC-COMPARISON-MIXED-SIZE-WORKLOAD-SEAM-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mixed_size_contract_refresh_guard.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"
C_RUNNER_SRC="tools/allocator/c_mimalloc_explicit_runner.c"
HAKO_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
APP="apps/hako-alloc-mimalloc-comparison-mixed-small-exe-proof/main.hako"
APP_README="apps/hako-alloc-mimalloc-comparison-mixed-small-exe-proof/README.md"
APP_TEST="apps/hako-alloc-mimalloc-comparison-mixed-small-exe-proof/test.sh"

echo "[$TAG] checking phase-295x mixed-size contract refresh"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PREV_CARD" \
  "$TASKBOARD" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "$C_RUNNER" \
  "$C_RUNNER_SRC" \
  "$HAKO_RUNNER" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST"

guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$C_RUNNER" "$HAKO_RUNNER" "$APP_TEST"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MIXED-SIZE-CONTRACT-295X-REFRESH-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MIXED-SIZE-EVIDENCE-295X-RUN-001' "$CARD" "card must select evidence run follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MIXED-SIZE-CONTRACT-295X-REFRESH-001' "$PREV_CARD" "previous row must select this contract refresh"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MIXED-SIZE-EVIDENCE-295X-RUN-001' "$TASKBOARD" "taskboard must expose evidence run follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'representative-mixed-small-v0' "$C_RUNNER_SRC" "C runner must implement mixed-small workload"
guard_expect_in_file "$TAG" 'workload=representative-mixed-small-v0' "$APP" "hako app must expose workload id"
guard_expect_in_file "$TAG" 'operation_family=mixed-small' "$APP" "hako app must expose operation family"
guard_expect_in_file "$TAG" 'operation_sequence_id=representative-mixed-small-v0-seq' "$APP" "hako app must expose operation sequence id"
guard_expect_in_file "$TAG" 'free_order_id=ascending-release-v0' "$APP" "hako app must expose free order"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_mixed_contract.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
c_out="$tmp_dir/c.out"
hako_out="$tmp_dir/hako.out"

bash "$C_RUNNER" --out "$c_out" --allow-ldconfig-discovery --workload representative-mixed-small-v0
bash "$HAKO_RUNNER" --app "$APP" --workload representative-mixed-small-v0 --out "$hako_out"

for out in "$c_out" "$hako_out"; do
  rg -F -q 'workload=representative-mixed-small-v0' "$out"
  rg -F -q 'operation_family=mixed-small' "$out"
  rg -F -q 'operation_sequence_id=representative-mixed-small-v0-seq' "$out"
  rg -F -q 'free_order_id=ascending-release-v0' "$out"
  rg -F -q 'allocation_count=16' "$out"
  rg -F -q 'free_count=16' "$out"
  rg -F -q 'requested_bytes=3096' "$out"
  rg -F -q 'summary=ok' "$out"
done

rg -F -q 'output_contract=allocator-comparison-c-mimalloc-explicit-runner-v0' "$c_out"
rg -F -q 'output_contract=hako-exe-memory-evidence-v0' "$hako_out"
rg -F -q 'provider_activation=0' "$hako_out"
rg -F -q 'host_replacement=0' "$hako_out"
rg -F -q 'hook_installed=0' "$hako_out"
rg -F -q 'global_allocator_installed=0' "$hako_out"

echo "[$TAG] ok"
