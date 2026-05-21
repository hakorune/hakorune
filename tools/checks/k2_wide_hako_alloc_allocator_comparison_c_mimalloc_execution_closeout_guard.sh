#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-execution-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -ne 0 ]; then
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
  case "$VALIDATION_LEVEL" in
    L0|L1|L2) ;;
    L3|L4)
      echo "[$TAG] ERROR: MIMAP-450A does not run C mimalloc; execution belongs to the explicit runner pilot" >&2
      exit 2
      ;;
  esac
fi

CARD_448A="docs/development/current/main/phases/phase-293x/293x-1070-MIMAP-448A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXECUTION-INVENTORY.md"
CARD_449A="docs/development/current/main/phases/phase-293x/293x-1071-MIMAP-449A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXECUTION-DIAGNOSTICS.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1072-MIMAP-450A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXECUTION-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1073-MIMAP-451A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXPLICIT-RUNNER-EXECUTION-PILOT.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-execution-closeout-ssot.md"
DESIGN_448A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-execution-inventory-ssot.md"
DESIGN_449A="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-execution-diagnostics-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
OWNER_448A="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_execution_inventory_box.hako"
OWNER_449A="lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_execution_diagnostic_box.hako"
GUARD_448A="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_execution_inventory_guard.sh"
GUARD_449A="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_execution_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_execution_closeout_guard.sh"

printf '[%s] checking MIMAP-450A C mimalloc execution closeout\n' "$TAG"

guard_require_files "$TAG" "$CARD_448A" "$CARD_449A" "$CARD" "$NEXT_CARD" "$DESIGN" "$DESIGN_448A" "$DESIGN_449A" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$OWNER_448A" "$OWNER_449A" "$GUARD_448A" "$GUARD_449A" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$GUARD_448A" "$GUARD_449A" "$SELF_SCRIPT"

for card in "$CARD_448A" "$CARD_449A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-451A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-450A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_448A" "MIMAP-448A design must remain accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_449A" "MIMAP-449A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-450A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-448A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-448A"
guard_expect_in_file "$TAG" 'id = "MIMAP-449A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-449A"
guard_expect_in_file "$TAG" 'closeout_pack = "allocator-comparison-c-mimalloc-execution"' "$PROOF_MANIFEST_INCLUDE" "C mimalloc execution rows must share closeout pack"
guard_expect_in_file "$TAG" 'MIMAP-451A Allocator Comparison C Mimalloc Explicit Runner Execution Pilot' "$DESIGN" "closeout must select explicit runner pilot"
guard_expect_in_file "$TAG" 'No process allocator replacement' "$NEXT_CARD" "MIMAP-451A must keep process replacement closed"
guard_expect_in_file "$TAG" 'No `#\[global_allocator\]`' "$NEXT_CARD" "MIMAP-451A must keep global allocator closed"
guard_expect_in_file "$TAG" 'explicit C mimalloc comparison runner' "$NEXT_CARD" "MIMAP-451A must require explicit runner"
guard_expect_in_file "$TAG" 'c_mimalloc_executed: 0' "$OWNER_448A" "MIMAP-448A must keep C execution closed"
guard_expect_in_file "$TAG" 'c_mimalloc_executed: 0' "$OWNER_449A" "MIMAP-449A must keep C execution closed"
guard_expect_in_file "$TAG" 'process_replacement_executed: 0' "$OWNER_448A" "MIMAP-448A must keep process replacement closed"
guard_expect_in_file "$TAG" 'process_replacement_executed: 0' "$OWNER_449A" "MIMAP-449A must keep process replacement closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER_448A" "MIMAP-448A must keep global allocator install closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER_449A" "MIMAP-449A must keep global allocator install closed"

if rg -n 'run_c_mimalloc[[:space:]]*\(|run_benchmark[[:space:]]*\(|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER_448A" "$OWNER_449A" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: C mimalloc execution inventory/diagnostics owners must keep execution seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'AllocatorComparisonCMimallocExecutionInventory|AllocatorComparisonCMimallocExecutionDiagnostic|allocator-comparison-c-mimalloc-execution-inventory-proof|allocator-comparison-c-mimalloc-execution-diagnostics-proof|run_c_mimalloc|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: C mimalloc execution inventory/diagnostic matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

bash "$GUARD_448A" --level L2
bash "$GUARD_449A" --level L2

printf '[%s] ok\n' "$TAG"
