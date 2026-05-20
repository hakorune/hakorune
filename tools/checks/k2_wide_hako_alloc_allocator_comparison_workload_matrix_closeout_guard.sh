#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-workload-matrix-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -ne 0 ]; then
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
  case "$VALIDATION_LEVEL" in
    L0|L1|L2) ;;
    L3|L4)
      echo "[$TAG] ERROR: MIMAP-432A has no L3/L4 benchmark evidence" >&2
      exit 2
      ;;
  esac
fi

CARD_430A="docs/development/current/main/phases/phase-293x/293x-1052-MIMAP-430A-ALLOCATOR-COMPARISON-WORKLOAD-MATRIX-INVENTORY.md"
CARD_431A="docs/development/current/main/phases/phase-293x/293x-1053-MIMAP-431A-ALLOCATOR-COMPARISON-WORKLOAD-MATRIX-DIAGNOSTICS.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1054-MIMAP-432A-ALLOCATOR-COMPARISON-WORKLOAD-MATRIX-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1055-MIMAP-433A-ALLOCATOR-COMPARISON-MEASUREMENT-PLAN-INVENTORY.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-workload-matrix-closeout-ssot.md"
DESIGN_430A="docs/development/current/main/design/hako-alloc-allocator-comparison-workload-matrix-inventory-ssot.md"
DESIGN_431A="docs/development/current/main/design/hako-alloc-allocator-comparison-workload-matrix-diagnostics-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
OWNER_430A="lang/src/hako_alloc/memory/allocator_comparison_workload_matrix_inventory_box.hako"
OWNER_431A="lang/src/hako_alloc/memory/allocator_comparison_workload_matrix_diagnostic_box.hako"
GUARD_430A="tools/checks/k2_wide_hako_alloc_allocator_comparison_workload_matrix_inventory_guard.sh"
GUARD_431A="tools/checks/k2_wide_hako_alloc_allocator_comparison_workload_matrix_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_workload_matrix_closeout_guard.sh"

printf '[%s] checking MIMAP-432A allocator comparison workload matrix closeout\n' "$TAG"

guard_require_files "$TAG" "$CARD_430A" "$CARD_431A" "$CARD" "$NEXT_CARD" "$DESIGN" "$DESIGN_430A" "$DESIGN_431A" "$INDEX" "$OWNER_430A" "$OWNER_431A" "$GUARD_430A" "$GUARD_431A" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$GUARD_430A" "$GUARD_431A" "$SELF_SCRIPT"

for card in "$CARD_430A" "$CARD_431A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-433A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-432A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_430A" "MIMAP-430A design must remain accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_431A" "MIMAP-431A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-432A guard"
guard_expect_in_file "$TAG" 'benchmark_executed: 0' "$OWNER_430A" "MIMAP-430A must keep benchmark execution closed"
guard_expect_in_file "$TAG" 'benchmark_executed: 0' "$OWNER_431A" "MIMAP-431A must keep benchmark execution closed"
guard_expect_in_file "$TAG" 'process_replacement_executed: 0' "$OWNER_430A" "MIMAP-430A must keep process replacement closed"
guard_expect_in_file "$TAG" 'process_replacement_executed: 0' "$OWNER_431A" "MIMAP-431A must keep process replacement closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER_430A" "MIMAP-430A must keep hook install closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER_431A" "MIMAP-431A must keep hook install closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER_430A" "MIMAP-430A must keep backend matcher additions closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER_431A" "MIMAP-431A must keep backend matcher additions closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER_430A" "MIMAP-430A must keep global allocator install closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER_431A" "MIMAP-431A must keep global allocator install closed"

if rg -n 'run_benchmark[[:space:]]*\(|replace_process_allocator|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER_430A" "$OWNER_431A" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: allocator comparison workload matrix owners must keep execution seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'AllocatorComparisonWorkloadMatrixInventory|AllocatorComparisonWorkloadMatrixDiagnostic|allocator-comparison-workload-matrix-inventory-proof|allocator-comparison-workload-matrix-diagnostics-proof|run_benchmark|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: allocator comparison workload matrix owner/app matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

bash "$GUARD_430A" --level L2
bash "$GUARD_431A" --level L2

printf '[%s] ok\n' "$TAG"
