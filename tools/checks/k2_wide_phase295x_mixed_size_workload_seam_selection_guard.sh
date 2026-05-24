#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mixed-size-workload-seam-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-20-MIMALLOC-COMPARISON-MIXED-SIZE-WORKLOAD-SEAM-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-19-MIMALLOC-COMPARISON-REALLOC-ALIGNED-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mixed_size_workload_seam_selection_guard.sh"

echo "[$TAG] checking phase-295x mixed-size workload seam selection"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MIXED-SIZE-WORKLOAD-SEAM-SELECTION-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MIXED-SIZE-CONTRACT-295X-REFRESH-001' "$CARD" "card must select mixed-size contract refresh"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MIXED-SIZE-WORKLOAD-SEAM-SELECTION-295X-001' "$PREV_CARD" "previous row must select this mixed-size seam selection"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MIXED-SIZE-CONTRACT-295X-REFRESH-001' "$TASKBOARD" "taskboard must expose contract refresh follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'workload=representative-mixed-small-v0' "$CARD" "card must select workload id"
guard_expect_in_file "$TAG" 'operation_family=mixed-small' "$CARD" "card must select operation family"
guard_expect_in_file "$TAG" 'operation_sequence_id=representative-mixed-small-v0-seq' "$CARD" "card must select operation sequence id"
guard_expect_in_file "$TAG" 'free_order_id=ascending-release-v0' "$CARD" "card must select free order"
guard_expect_in_file "$TAG" 'allocation_count=16' "$CARD" "card must define allocation count"
guard_expect_in_file "$TAG" 'free_count=16' "$CARD" "card must define free count"
guard_expect_in_file "$TAG" 'requested_bytes=3096' "$CARD" "card must define requested bytes"

if rg -n 'LD_PRELOAD|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|winner_claim=1|provider_activation=1|host_replacement=1|thread::|worker_local|nowait|await' "$CARD" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: mixed-size seam selection opened replacement/provider/thread/winner seams" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

echo "[$TAG] ok"
