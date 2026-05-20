#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-benchmark-execution-preflight-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -ne 0 ]; then
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
  case "$VALIDATION_LEVEL" in
    L0|L1|L2) ;;
    L3|L4)
      echo "[$TAG] ERROR: MIMAP-438A has no L3/L4 benchmark evidence" >&2
      exit 2
      ;;
  esac
fi

CARD_436A="docs/development/current/main/phases/phase-293x/293x-1058-MIMAP-436A-ALLOCATOR-COMPARISON-BENCHMARK-EXECUTION-PREFLIGHT-INVENTORY.md"
CARD_437A="docs/development/current/main/phases/phase-293x/293x-1059-MIMAP-437A-ALLOCATOR-COMPARISON-BENCHMARK-EXECUTION-PREFLIGHT-DIAGNOSTICS.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1060-MIMAP-438A-ALLOCATOR-COMPARISON-BENCHMARK-EXECUTION-PREFLIGHT-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1061-MIMAP-439A-ALLOCATOR-COMPARISON-CONTROLLED-BENCHMARK-EXECUTION-PLAN.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-benchmark-execution-preflight-closeout-ssot.md"
DESIGN_436A="docs/development/current/main/design/hako-alloc-allocator-comparison-benchmark-execution-preflight-inventory-ssot.md"
DESIGN_437A="docs/development/current/main/design/hako-alloc-allocator-comparison-benchmark-execution-preflight-diagnostics-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
OWNER_436A="lang/src/hako_alloc/memory/allocator_comparison_benchmark_execution_preflight_inventory_box.hako"
OWNER_437A="lang/src/hako_alloc/memory/allocator_comparison_benchmark_execution_preflight_diagnostic_box.hako"
GUARD_436A="tools/checks/k2_wide_hako_alloc_allocator_comparison_benchmark_execution_preflight_inventory_guard.sh"
GUARD_437A="tools/checks/k2_wide_hako_alloc_allocator_comparison_benchmark_execution_preflight_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_benchmark_execution_preflight_closeout_guard.sh"

printf '[%s] checking MIMAP-438A allocator comparison benchmark execution preflight closeout\n' "$TAG"

guard_require_files "$TAG" "$CARD_436A" "$CARD_437A" "$CARD" "$NEXT_CARD" "$DESIGN" "$DESIGN_436A" "$DESIGN_437A" "$INDEX" "$OWNER_436A" "$OWNER_437A" "$GUARD_436A" "$GUARD_437A" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$GUARD_436A" "$GUARD_437A" "$SELF_SCRIPT"

for card in "$CARD_436A" "$CARD_437A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-439A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-438A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_436A" "MIMAP-436A design must remain accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_437A" "MIMAP-437A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-438A guard"
guard_expect_in_file "$TAG" 'benchmark_executed: 0' "$OWNER_436A" "MIMAP-436A must keep benchmark execution closed"
guard_expect_in_file "$TAG" 'benchmark_executed: 0' "$OWNER_437A" "MIMAP-437A must keep benchmark execution closed"
guard_expect_in_file "$TAG" 'process_replacement_executed: 0' "$OWNER_436A" "MIMAP-436A must keep process replacement closed"
guard_expect_in_file "$TAG" 'process_replacement_executed: 0' "$OWNER_437A" "MIMAP-437A must keep process replacement closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER_436A" "MIMAP-436A must keep hook install closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER_437A" "MIMAP-437A must keep hook install closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER_436A" "MIMAP-436A must keep backend matcher additions closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER_437A" "MIMAP-437A must keep backend matcher additions closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER_436A" "MIMAP-436A must keep global allocator install closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER_437A" "MIMAP-437A must keep global allocator install closed"

if rg -n 'run_benchmark[[:space:]]*\(|replace_process_allocator|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER_436A" "$OWNER_437A" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: allocator comparison benchmark preflight owners must keep execution seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'AllocatorComparisonBenchmarkExecutionPreflightInventory|AllocatorComparisonBenchmarkExecutionPreflightDiagnostic|allocator-comparison-benchmark-execution-preflight-inventory-proof|allocator-comparison-benchmark-execution-preflight-diagnostics-proof|run_benchmark|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: allocator comparison benchmark preflight owner/app matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

bash "$GUARD_436A" --level L2
bash "$GUARD_437A" --level L2

printf '[%s] ok\n' "$TAG"
