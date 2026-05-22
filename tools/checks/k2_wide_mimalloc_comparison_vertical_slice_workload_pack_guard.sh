#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-comparison-vertical-slice-workload-pack"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

CARD="docs/development/current/main/phases/phase-294x/294x-53-MIMALLOC-COMPARISON-VERTICAL-SLICE-WORKLOAD-PACK.md"
TASKBOARD="docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md"
README="docs/development/current/main/phases/phase-294x/README.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
MIMAP430_CARD="docs/development/current/main/phases/phase-293x/293x-1052-MIMAP-430A-ALLOCATOR-COMPARISON-WORKLOAD-MATRIX-INVENTORY.md"
MIMAP430_GUARD="tools/checks/k2_wide_hako_alloc_allocator_comparison_workload_matrix_inventory_guard.sh"
OWNER="lang/src/hako_alloc/memory/allocator_comparison_workload_matrix_inventory_box.hako"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.c"
C_RUNNER_SH="tools/allocator/c_mimalloc_explicit_runner.sh"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_comparison_vertical_slice_workload_pack_guard.sh"

printf '[%s] checking 294x-53 mimalloc comparison vertical-slice workload pack\n' "$TAG"

guard_require_files "$TAG" "$CARD" "$TASKBOARD" "$README" "$CURRENT_STATE" \
  "$INDEX" "$MIMAP430_CARD" "$MIMAP430_GUARD" "$OWNER" "$C_RUNNER" \
  "$C_RUNNER_SH" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$MIMAP430_GUARD" "$C_RUNNER_SH" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Landed' "$CARD" "294x-53 card must be landed"
guard_expect_in_file "$TAG" 'Decision' "$CARD" "294x-53 card must carry a decision"
guard_expect_in_file "$TAG" 'small_fixed_alloc_free_reuse' "$CARD" "V0 must select fixed small alloc/free/reuse"
guard_expect_in_file "$TAG" 'mixed_small_sizes' "$CARD" "V0 must select mixed small-size workload"
guard_expect_in_file "$TAG" 'realloc_same_class_and_grow' "$CARD" "V0 must select realloc same-class/grow workload"
guard_expect_in_file "$TAG" 'aligned_small' "$CARD" "V0 must select aligned-small workload"
guard_expect_in_file "$TAG" 'huge_osvm_backed' "$CARD" "V0 must select huge/OSVM workload"
guard_expect_in_file "$TAG" 'allocator_id' "$CARD" "V0 must keep allocator_id in the schema anchor"
guard_expect_in_file "$TAG" 'runner_kind' "$CARD" "V0 must keep runner_kind in the schema anchor"
guard_expect_in_file "$TAG" 'workload_id' "$CARD" "V0 must keep workload_id in the schema anchor"
guard_expect_in_file "$TAG" 'requested_bytes' "$CARD" "V0 must keep requested_bytes in the schema anchor"
guard_expect_in_file "$TAG" 'rss_or_memory_evidence' "$CARD" "V0 must keep memory evidence in the schema anchor"

guard_expect_in_file "$TAG" 'Mimalloc Comparison Vertical Slice Queue' "$TASKBOARD" "taskboard must expose vertical slice queue"
guard_expect_in_file "$TAG" 'Select the comparison workload pack' "$TASKBOARD" "taskboard must keep V0 visible"
guard_expect_in_file "$TAG" 'C mimalloc vs `.hako` report closeout' "$TASKBOARD" "taskboard must keep V5 closeout visible"
guard_expect_in_file "$TAG" 'not a full allocator-wide port' "$TASKBOARD" "taskboard must reject full-port interpretation"
guard_expect_in_file "$TAG" 'comparison-quality vertical slice' "$README" "phase README must explain acceleration correction"
guard_expect_in_file "$TAG" 'Do not wait for every remaining field group' "$README" "phase README must avoid broad field drain"

guard_expect_in_file "$TAG" 'allocator_comparison_workload_matrix_inventory_box' "$MIMAP430_GUARD" "V0 must reuse MIMAP-430A workload matrix guard"
guard_expect_in_file "$TAG" 'small_allocation_workload_present' "$OWNER" "MIMAP-430A owner must keep small allocation family"
guard_expect_in_file "$TAG" 'small_free_workload_present' "$OWNER" "MIMAP-430A owner must keep small free family"
guard_expect_in_file "$TAG" 'realloc_workload_present' "$OWNER" "MIMAP-430A owner must keep realloc family"
guard_expect_in_file "$TAG" 'huge_allocation_workload_present' "$OWNER" "MIMAP-430A owner must keep huge family"
guard_expect_in_file "$TAG" 'throughput_workload_present' "$OWNER" "MIMAP-430A owner must keep throughput family"
guard_expect_in_file "$TAG" 'memory_usage_workload_present' "$OWNER" "MIMAP-430A owner must keep memory family"
guard_expect_in_file "$TAG" 'benchmark_executed: 0' "$OWNER" "V0 must keep benchmark execution closed in workload inventory"
guard_expect_in_file "$TAG" 'process_replacement_executed: 0' "$OWNER" "V0 must keep process replacement closed in workload inventory"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER" "V0 must keep hook install closed in workload inventory"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER" "V0 must keep global allocator closed in workload inventory"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list 294x-53 guard"

if rg -n 'replace_process_allocator|install_hook[[:space:]]*\(|backendMatcherInstall|provider_activate[[:space:]]*\(|worker_local[[:space:]]*\(|atomic_bitmap_execute' "$CARD" "$TASKBOARD" "$README" >/tmp/"$TAG".leak 2>&1; then
  echo "[$TAG] ERROR: V0 workload-pack docs reopened a parked execution/provider seam" >&2
  cat /tmp/"$TAG".leak >&2
  rm -f /tmp/"$TAG".leak
  exit 1
fi
rm -f /tmp/"$TAG".leak

printf '[%s] ok\n' "$TAG"
