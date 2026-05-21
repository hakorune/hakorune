#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-representative-benchmark-execution-row-selection"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

CARD_442A="docs/development/current/main/phases/phase-293x/293x-1064-MIMAP-442A-ALLOCATOR-COMPARISON-CONTROLLED-BENCHMARK-EXECUTION-CLOSEOUT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1065-MIMAP-443A-ALLOCATOR-COMPARISON-REPRESENTATIVE-BENCHMARK-EXECUTION-ROW-SELECTION.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1066-MIMAP-444A-ALLOCATOR-COMPARISON-REPRESENTATIVE-BENCHMARK-EXECUTION-PILOT.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_representative_benchmark_execution_row_selection_guard.sh"

printf '[%s] checking MIMAP-443A representative benchmark execution row selection\n' "$TAG"

guard_require_files "$TAG" "$CARD_442A" "$CARD" "$NEXT_CARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_442A" "MIMAP-442A must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-443A must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-444A must be selected current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-443A guard"
guard_expect_in_file "$TAG" 'MIMAP-444A Allocator Comparison Representative Benchmark Execution Pilot' "$CARD" "MIMAP-443A must select MIMAP-444A"
guard_expect_in_file "$TAG" 'first narrow representative benchmark execution seam' "$NEXT_CARD" "MIMAP-444A must be a narrow execution seam"
guard_expect_in_file "$TAG" 'must not replace the process allocator' "$NEXT_CARD" "MIMAP-444A must keep replacement separate"
guard_expect_in_file "$TAG" 'No process allocator replacement' "$NEXT_CARD" "MIMAP-444A must keep process replacement closed"
guard_expect_in_file "$TAG" 'No `#\[global_allocator\]`' "$NEXT_CARD" "MIMAP-444A must keep global allocator closed"

if rg -n 'replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$CARD" "$NEXT_CARD" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-443A docs must not open replacement/hook/backend/source-concurrency seams" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

printf '[%s] ok\n' "$TAG"
