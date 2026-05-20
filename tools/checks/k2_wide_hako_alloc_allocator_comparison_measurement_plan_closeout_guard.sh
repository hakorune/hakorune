#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-measurement-plan-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -ne 0 ]; then
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
  case "$VALIDATION_LEVEL" in
    L0|L1|L2) ;;
    L3|L4)
      echo "[$TAG] ERROR: MIMAP-435A has no L3/L4 benchmark evidence" >&2
      exit 2
      ;;
  esac
fi

CARD_433A="docs/development/current/main/phases/phase-293x/293x-1055-MIMAP-433A-ALLOCATOR-COMPARISON-MEASUREMENT-PLAN-INVENTORY.md"
CARD_434A="docs/development/current/main/phases/phase-293x/293x-1056-MIMAP-434A-ALLOCATOR-COMPARISON-MEASUREMENT-PLAN-DIAGNOSTICS.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1057-MIMAP-435A-ALLOCATOR-COMPARISON-MEASUREMENT-PLAN-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1058-MIMAP-436A-ALLOCATOR-COMPARISON-BENCHMARK-EXECUTION-PREFLIGHT-INVENTORY.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-measurement-plan-closeout-ssot.md"
DESIGN_433A="docs/development/current/main/design/hako-alloc-allocator-comparison-measurement-plan-inventory-ssot.md"
DESIGN_434A="docs/development/current/main/design/hako-alloc-allocator-comparison-measurement-plan-diagnostics-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
OWNER_433A="lang/src/hako_alloc/memory/allocator_comparison_measurement_plan_inventory_box.hako"
OWNER_434A="lang/src/hako_alloc/memory/allocator_comparison_measurement_plan_diagnostic_box.hako"
GUARD_433A="tools/checks/k2_wide_hako_alloc_allocator_comparison_measurement_plan_inventory_guard.sh"
GUARD_434A="tools/checks/k2_wide_hako_alloc_allocator_comparison_measurement_plan_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_measurement_plan_closeout_guard.sh"

printf '[%s] checking MIMAP-435A allocator comparison measurement plan closeout\n' "$TAG"

guard_require_files "$TAG" "$CARD_433A" "$CARD_434A" "$CARD" "$NEXT_CARD" "$DESIGN" "$DESIGN_433A" "$DESIGN_434A" "$INDEX" "$OWNER_433A" "$OWNER_434A" "$GUARD_433A" "$GUARD_434A" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$GUARD_433A" "$GUARD_434A" "$SELF_SCRIPT"

for card in "$CARD_433A" "$CARD_434A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-436A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-435A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_433A" "MIMAP-433A design must remain accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_434A" "MIMAP-434A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-435A guard"
guard_expect_in_file "$TAG" 'benchmark_executed: 0' "$OWNER_433A" "MIMAP-433A must keep benchmark execution closed"
guard_expect_in_file "$TAG" 'benchmark_executed: 0' "$OWNER_434A" "MIMAP-434A must keep benchmark execution closed"
guard_expect_in_file "$TAG" 'process_replacement_executed: 0' "$OWNER_433A" "MIMAP-433A must keep process replacement closed"
guard_expect_in_file "$TAG" 'process_replacement_executed: 0' "$OWNER_434A" "MIMAP-434A must keep process replacement closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER_433A" "MIMAP-433A must keep hook install closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER_434A" "MIMAP-434A must keep hook install closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER_433A" "MIMAP-433A must keep backend matcher additions closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER_434A" "MIMAP-434A must keep backend matcher additions closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER_433A" "MIMAP-433A must keep global allocator install closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER_434A" "MIMAP-434A must keep global allocator install closed"

if rg -n 'run_benchmark[[:space:]]*\(|replace_process_allocator|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER_433A" "$OWNER_434A" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: allocator comparison measurement plan owners must keep execution seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'AllocatorComparisonMeasurementPlanInventory|AllocatorComparisonMeasurementPlanDiagnostic|allocator-comparison-measurement-plan-inventory-proof|allocator-comparison-measurement-plan-diagnostics-proof|run_benchmark|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: allocator comparison measurement plan owner/app matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

bash "$GUARD_433A" --level L2
bash "$GUARD_434A" --level L2

printf '[%s] ok\n' "$TAG"
