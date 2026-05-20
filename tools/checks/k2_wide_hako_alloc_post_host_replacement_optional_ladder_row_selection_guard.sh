#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-post-host-replacement-optional-ladder-row-selection"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

CARD_425A="docs/development/current/main/phases/phase-293x/293x-1047-MIMAP-425A-OPTIONAL-PROCESS-ALLOCATOR-REPLACEMENT-PROPOSAL.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1048-MIMAP-426A-POST-HOST-REPLACEMENT-OPTIONAL-LADDER-ROW-SELECTION.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1049-MIMAP-427A-ALLOCATOR-COMPARISON-BASELINE-INVENTORY.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_post_host_replacement_optional_ladder_row_selection_guard.sh"

printf '[%s] checking MIMAP-426A post host replacement optional ladder row selection\n' "$TAG"

guard_require_files "$TAG" "$CARD_425A" "$CARD" "$NEXT_CARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_425A" "MIMAP-425A must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-426A must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-427A must be selected current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-426A guard"
guard_expect_in_file "$TAG" 'MIMAP-427A Allocator Comparison Baseline Inventory' "$CARD" "MIMAP-426A must select MIMAP-427A"
guard_expect_in_file "$TAG" 'performance and memory usage can be compared against C' "$CARD" "MIMAP-426A must return to comparison evidence"
guard_expect_in_file "$TAG" 'optional replacement execution .*parked' "$NEXT_CARD" "MIMAP-427A must keep optional replacement parked"
guard_expect_in_file "$TAG" 'C mimalloc performance and memory' "$NEXT_CARD" "MIMAP-427A must keep C mimalloc comparison target explicit"

if rg -n 'replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$CARD" "$NEXT_CARD" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-426A docs must not open replacement/hook/backend/source-concurrency seams" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

printf '[%s] ok\n' "$TAG"
