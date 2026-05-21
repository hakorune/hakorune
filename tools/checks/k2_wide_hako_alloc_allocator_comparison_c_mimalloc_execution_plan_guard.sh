#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-allocator-comparison-c-mimalloc-execution-plan"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

CARD_446A="docs/development/current/main/phases/phase-293x/293x-1068-MIMAP-446A-ALLOCATOR-COMPARISON-REPRESENTATIVE-BENCHMARK-EXECUTION-CLOSEOUT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1069-MIMAP-447A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXECUTION-PLAN.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1070-MIMAP-448A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXECUTION-INVENTORY.md"
DESIGN="docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-execution-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_execution_plan_guard.sh"

printf '[%s] checking MIMAP-447A C mimalloc execution plan\n' "$TAG"

guard_require_files "$TAG" "$CARD_446A" "$CARD" "$NEXT_CARD" "$DESIGN" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_446A" "MIMAP-446A must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-447A must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-448A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-447A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-447A guard"
guard_expect_in_file "$TAG" 'MIMAP-448A Allocator Comparison C Mimalloc Execution Inventory' "$DESIGN" "MIMAP-447A must select MIMAP-448A"
guard_expect_in_file "$TAG" 'same representative workload as MIMAP-444A' "$DESIGN" "C mimalloc plan must reuse the Hako representative workload shape"
guard_expect_in_file "$TAG" 'explicit runner/tool input' "$DESIGN" "C mimalloc plan must require explicit runner input"
guard_expect_in_file "$TAG" 'stable line/record contract' "$DESIGN" "C mimalloc plan must define output contract"
guard_expect_in_file "$TAG" 'No process allocator replacement' "$NEXT_CARD" "MIMAP-448A must keep process replacement closed"
guard_expect_in_file "$TAG" 'No `#\[global_allocator\]`' "$NEXT_CARD" "MIMAP-448A must keep global allocator closed"
guard_expect_in_file "$TAG" 'No implicit C mimalloc execution' "$NEXT_CARD" "MIMAP-448A must keep execution explicit"

if rg -n 'replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$CARD" "$NEXT_CARD" "$DESIGN" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-447A docs must not open replacement/hook/backend/source-concurrency seams" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

printf '[%s] ok\n' "$TAG"
