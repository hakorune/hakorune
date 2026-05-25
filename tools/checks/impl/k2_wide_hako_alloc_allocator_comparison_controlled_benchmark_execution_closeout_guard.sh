#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-controlled-benchmark-execution-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -ne 0 ]; then
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
  case "$VALIDATION_LEVEL" in
    L0|L1|L2) ;;
    L3|L4)
      echo "[$TAG] ERROR: MIMAP-442A has no L3/L4 benchmark execution evidence" >&2
      exit 2
      ;;
  esac
fi

CARD_440A="docs/development/current/main/phases/phase-293x/293x-1062-MIMAP-440A-ALLOCATOR-COMPARISON-CONTROLLED-BENCHMARK-EXECUTION-INVENTORY.md"
CARD_441A="docs/development/current/main/phases/phase-293x/293x-1063-MIMAP-441A-ALLOCATOR-COMPARISON-CONTROLLED-BENCHMARK-EXECUTION-DIAGNOSTICS.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1064-MIMAP-442A-ALLOCATOR-COMPARISON-CONTROLLED-BENCHMARK-EXECUTION-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1065-MIMAP-443A-ALLOCATOR-COMPARISON-REPRESENTATIVE-BENCHMARK-EXECUTION-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-controlled-benchmark-execution-closeout-ssot.md"
DESIGN_440A="docs/development/current/main/design/hako-alloc-allocator-comparison-controlled-benchmark-execution-inventory-ssot.md"
DESIGN_441A="docs/development/current/main/design/hako-alloc-allocator-comparison-controlled-benchmark-execution-diagnostics-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
OWNER_440A="lang/src/hako_alloc/memory/allocator_comparison_controlled_benchmark_execution_inventory_box.hako"
OWNER_441A="lang/src/hako_alloc/memory/allocator_comparison_controlled_benchmark_execution_diagnostic_box.hako"
GUARD_440A="tools/checks/k2_wide_hako_alloc_allocator_comparison_controlled_benchmark_execution_inventory_guard.sh"
GUARD_441A="tools/checks/k2_wide_hako_alloc_allocator_comparison_controlled_benchmark_execution_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_controlled_benchmark_execution_closeout_guard.sh"

printf '[%s] checking MIMAP-442A allocator comparison controlled benchmark execution closeout\n' "$TAG"

guard_require_files "$TAG" "$CARD_440A" "$CARD_441A" "$CARD" "$NEXT_CARD" "$DESIGN" "$DESIGN_440A" "$DESIGN_441A" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$OWNER_440A" "$OWNER_441A" "$GUARD_440A" "$GUARD_441A" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$GUARD_440A" "$GUARD_441A" "$SELF_SCRIPT"

for card in "$CARD_440A" "$CARD_441A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-443A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-442A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_440A" "MIMAP-440A design must remain accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_441A" "MIMAP-441A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-442A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-440A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-440A"
guard_expect_in_file "$TAG" 'id = "MIMAP-441A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-441A"
guard_expect_in_file "$TAG" 'closeout_pack = "allocator-comparison-controlled-benchmark-execution"' "$PROOF_MANIFEST_INCLUDE" "controlled benchmark rows must share closeout pack"
guard_expect_in_file "$TAG" 'benchmark_executed: 0' "$OWNER_440A" "MIMAP-440A must keep benchmark execution closed"
guard_expect_in_file "$TAG" 'benchmark_executed: 0' "$OWNER_441A" "MIMAP-441A must keep benchmark execution closed"
guard_expect_in_file "$TAG" 'process_replacement_executed: 0' "$OWNER_440A" "MIMAP-440A must keep process replacement closed"
guard_expect_in_file "$TAG" 'process_replacement_executed: 0' "$OWNER_441A" "MIMAP-441A must keep process replacement closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER_440A" "MIMAP-440A must keep hook install closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER_441A" "MIMAP-441A must keep hook install closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER_440A" "MIMAP-440A must keep backend matcher additions closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER_441A" "MIMAP-441A must keep backend matcher additions closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER_440A" "MIMAP-440A must keep global allocator install closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER_441A" "MIMAP-441A must keep global allocator install closed"

if rg -n 'run_benchmark[[:space:]]*\(|replace_process_allocator|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER_440A" "$OWNER_441A" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: controlled benchmark execution owners must keep execution seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'AllocatorComparisonControlledBenchmarkExecutionInventory|AllocatorComparisonControlledBenchmarkExecutionDiagnostic|allocator-comparison-controlled-benchmark-execution-inventory-proof|allocator-comparison-controlled-benchmark-execution-diagnostics-proof|run_benchmark|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: controlled benchmark execution owner/app matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

bash "$GUARD_440A" --level L2
bash "$GUARD_441A" --level L2

printf '[%s] ok\n' "$TAG"
