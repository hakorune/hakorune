#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-baseline-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -ne 0 ]; then
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
  case "$VALIDATION_LEVEL" in
    L0|L1|L2) ;;
    L3|L4)
      echo "[$TAG] ERROR: MIMAP-429A has no L3/L4 benchmark evidence" >&2
      exit 2
      ;;
  esac
fi

CARD_427A="docs/development/current/main/phases/phase-293x/293x-1049-MIMAP-427A-ALLOCATOR-COMPARISON-BASELINE-INVENTORY.md"
CARD_428A="docs/development/current/main/phases/phase-293x/293x-1050-MIMAP-428A-ALLOCATOR-COMPARISON-BASELINE-DIAGNOSTICS.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1051-MIMAP-429A-ALLOCATOR-COMPARISON-BASELINE-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1052-MIMAP-430A-ALLOCATOR-COMPARISON-WORKLOAD-MATRIX-INVENTORY.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-baseline-closeout-ssot.md"
DESIGN_427A="docs/development/current/main/design/hako-alloc-allocator-comparison-baseline-inventory-ssot.md"
DESIGN_428A="docs/development/current/main/design/hako-alloc-allocator-comparison-baseline-diagnostics-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
OWNER_427A="lang/src/hako_alloc/memory/allocator_comparison_baseline_inventory_box.hako"
OWNER_428A="lang/src/hako_alloc/memory/allocator_comparison_baseline_diagnostic_box.hako"
GUARD_427A="tools/checks/k2_wide_hako_alloc_allocator_comparison_baseline_inventory_guard.sh"
GUARD_428A="tools/checks/k2_wide_hako_alloc_allocator_comparison_baseline_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_baseline_closeout_guard.sh"

printf '[%s] checking MIMAP-429A allocator comparison baseline closeout\n' "$TAG"

guard_require_files "$TAG" "$CARD_427A" "$CARD_428A" "$CARD" "$NEXT_CARD" "$DESIGN" "$DESIGN_427A" "$DESIGN_428A" "$INDEX" "$OWNER_427A" "$OWNER_428A" "$GUARD_427A" "$GUARD_428A" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$GUARD_427A" "$GUARD_428A" "$SELF_SCRIPT"

for card in "$CARD_427A" "$CARD_428A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-430A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-429A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_427A" "MIMAP-427A design must remain accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_428A" "MIMAP-428A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-429A guard"
guard_expect_in_file "$TAG" 'process_replacement_executed: 0' "$OWNER_427A" "MIMAP-427A must keep process replacement closed"
guard_expect_in_file "$TAG" 'process_replacement_executed: 0' "$OWNER_428A" "MIMAP-428A must keep process replacement closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER_427A" "MIMAP-427A must keep hook install closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER_428A" "MIMAP-428A must keep hook install closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER_427A" "MIMAP-427A must keep backend matcher additions closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER_428A" "MIMAP-428A must keep backend matcher additions closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER_427A" "MIMAP-427A must keep global allocator install closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER_428A" "MIMAP-428A must keep global allocator install closed"

if rg -n 'replace_process_allocator|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER_427A" "$OWNER_428A" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: allocator comparison baseline owners must keep execution seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'AllocatorComparisonBaselineInventory|AllocatorComparisonBaselineDiagnostic|allocator-comparison-baseline-inventory-proof|allocator-comparison-baseline-diagnostics-proof|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: allocator comparison baseline owner/app matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

bash "$GUARD_427A" --level L2
bash "$GUARD_428A" --level L2

printf '[%s] ok\n' "$TAG"
