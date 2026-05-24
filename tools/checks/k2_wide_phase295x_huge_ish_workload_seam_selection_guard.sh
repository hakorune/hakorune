#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-huge-ish-workload-seam-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-24-MIMALLOC-COMPARISON-HUGE-ISH-WORKLOAD-SEAM-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-23-MIMALLOC-COMPARISON-MIXED-SIZE-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_huge_ish_workload_seam_selection_guard.sh"

echo "[$TAG] checking phase-295x huge-ish workload seam selection"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HUGE-ISH-WORKLOAD-SEAM-SELECTION-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HUGE-ISH-CONTRACT-295X-REFRESH-001' "$CARD" "card must select huge-ish contract refresh"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HUGE-ISH-WORKLOAD-SEAM-SELECTION-295X-001' "$PREV_CARD" "previous row must select this seam selection"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HUGE-ISH-CONTRACT-295X-REFRESH-001' "$TASKBOARD" "taskboard must expose contract refresh follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'workload=representative-huge-ish-v0' "$CARD" "card must select workload id"
guard_expect_in_file "$TAG" 'operation_family=huge-ish' "$CARD" "card must select operation family"
guard_expect_in_file "$TAG" 'operation_sequence_id=representative-huge-ish-v0-seq' "$CARD" "card must select operation sequence id"
guard_expect_in_file "$TAG" 'free_order_id=ascending-release-v0' "$CARD" "card must select free order"
guard_expect_in_file "$TAG" 'allocation_count=2' "$CARD" "card must define allocation count"
guard_expect_in_file "$TAG" 'free_count=2' "$CARD" "card must define free count"
guard_expect_in_file "$TAG" 'requested_bytes=4194321' "$CARD" "card must define requested bytes"
guard_expect_in_file "$TAG" 'large_request_count=1' "$CARD" "card must define large request count"
guard_expect_in_file "$TAG" 'OSVM/page-source equivalence' "$CARD" "card must keep OSVM equivalence closed"

if rg -n 'LD_PRELOAD|replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|winner_claim=1|provider_activation=1|host_replacement=1|thread::|worker_local|nowait|await' "$CARD" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: huge-ish seam selection opened replacement/provider/thread/winner seams" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

echo "[$TAG] ok"
