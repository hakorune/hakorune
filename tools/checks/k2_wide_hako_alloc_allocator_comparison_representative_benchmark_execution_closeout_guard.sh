#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-representative-benchmark-execution-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -ne 0 ]; then
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
  case "$VALIDATION_LEVEL" in
    L0|L1|L2) ;;
    L3|L4)
      echo "[$TAG] ERROR: MIMAP-446A defers C mimalloc/native comparison evidence to later rows" >&2
      exit 2
      ;;
  esac
fi

CARD_444A="docs/development/current/main/phases/phase-293x/293x-1066-MIMAP-444A-ALLOCATOR-COMPARISON-REPRESENTATIVE-BENCHMARK-EXECUTION-PILOT.md"
CARD_445A="docs/development/current/main/phases/phase-293x/293x-1067-MIMAP-445A-ALLOCATOR-COMPARISON-REPRESENTATIVE-BENCHMARK-EXECUTION-DIAGNOSTICS.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1068-MIMAP-446A-ALLOCATOR-COMPARISON-REPRESENTATIVE-BENCHMARK-EXECUTION-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1069-MIMAP-447A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXECUTION-PLAN.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-representative-benchmark-execution-closeout-ssot.md"
DESIGN_444A="docs/development/current/main/design/hako-alloc-allocator-comparison-representative-benchmark-execution-pilot-ssot.md"
DESIGN_445A="docs/development/current/main/design/hako-alloc-allocator-comparison-representative-benchmark-execution-diagnostics-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
OWNER_444A="lang/src/hako_alloc/memory/allocator_comparison_representative_benchmark_execution_pilot_box.hako"
OWNER_445A="lang/src/hako_alloc/memory/allocator_comparison_representative_benchmark_execution_diagnostic_box.hako"
GUARD_444A="tools/checks/k2_wide_hako_alloc_allocator_comparison_representative_benchmark_execution_pilot_guard.sh"
GUARD_445A="tools/checks/k2_wide_hako_alloc_allocator_comparison_representative_benchmark_execution_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_representative_benchmark_execution_closeout_guard.sh"

printf '[%s] checking MIMAP-446A allocator comparison representative benchmark execution closeout\n' "$TAG"

guard_require_files "$TAG" "$CARD_444A" "$CARD_445A" "$CARD" "$NEXT_CARD" "$DESIGN" "$DESIGN_444A" "$DESIGN_445A" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$OWNER_444A" "$OWNER_445A" "$GUARD_444A" "$GUARD_445A" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$GUARD_444A" "$GUARD_445A" "$SELF_SCRIPT"

for card in "$CARD_444A" "$CARD_445A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-447A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-446A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_444A" "MIMAP-444A design must remain accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_445A" "MIMAP-445A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-446A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-444A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-444A"
guard_expect_in_file "$TAG" 'id = "MIMAP-445A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-445A"
guard_expect_in_file "$TAG" 'closeout_pack = "allocator-comparison-representative-benchmark-execution"' "$PROOF_MANIFEST_INCLUDE" "representative benchmark rows must share closeout pack"
guard_expect_in_file "$TAG" 'benchmark_executed: accepted' "$OWNER_444A" "MIMAP-444A must own bounded Hako representative execution"
guard_expect_in_file "$TAG" 'diagnoseAllocatorComparisonRepresentativeBenchmarkExecution' "$OWNER_445A" "MIMAP-445A must own execution diagnostics"
guard_expect_in_file "$TAG" 'process_replacement_executed: 0' "$OWNER_444A" "MIMAP-444A must keep process replacement closed"
guard_expect_in_file "$TAG" 'process_replacement_executed: 0' "$OWNER_445A" "MIMAP-445A must keep process replacement closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER_444A" "MIMAP-444A must keep hook install closed"
guard_expect_in_file "$TAG" 'hook_installed: 0' "$OWNER_445A" "MIMAP-445A must keep hook install closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER_444A" "MIMAP-444A must keep backend matcher additions closed"
guard_expect_in_file "$TAG" 'backend_matcher_added: 0' "$OWNER_445A" "MIMAP-445A must keep backend matcher additions closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER_444A" "MIMAP-444A must keep global allocator install closed"
guard_expect_in_file "$TAG" 'global_allocator_installed: 0' "$OWNER_445A" "MIMAP-445A must keep global allocator install closed"

if rg -n 'replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|#\[global_allocator\]|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER_444A" "$OWNER_445A" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: representative benchmark execution owners must keep process replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'AllocatorComparisonRepresentativeBenchmarkExecutionPilot|AllocatorComparisonRepresentativeBenchmarkExecutionDiagnostic|allocator-comparison-representative-benchmark-execution-pilot-proof|allocator-comparison-representative-benchmark-execution-diagnostics-proof|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: representative benchmark execution owner/app matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

bash "$GUARD_444A" --level L2
bash "$GUARD_445A" --level L2

printf '[%s] ok\n' "$TAG"
