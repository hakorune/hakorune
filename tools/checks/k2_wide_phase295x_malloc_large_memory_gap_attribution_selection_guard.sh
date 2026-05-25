#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-malloc-large-memory-gap-attribution-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-190-MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-ATTRIBUTION-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-189-MIMALLOC-COMPARISON-MALLOC-LARGE-CLOSEOUT.md"
SSOT="docs/development/current/main/design/mimalloc-comparison-execution-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_malloc_large_memory_gap_attribution_selection_guard.sh"

echo "[$TAG] checking phase-295x malloc-large memory gap attribution selection"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$SSOT" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-ATTRIBUTION-SELECTION-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-BASELINE-295X-001' "$CARD" "card must select memory-gap baseline follow-on"
guard_expect_in_file "$TAG" 'workload=representative-empty-v0' "$CARD" "card must select empty baseline workload"
guard_expect_in_file "$TAG" 'operation_family=empty-baseline' "$CARD" "card must select empty baseline operation family"
guard_expect_in_file "$TAG" 'operation_sequence_id=representative-empty-v0-seq' "$CARD" "card must select empty baseline operation sequence"
guard_expect_in_file "$TAG" 'free_order_id=no-release-v0' "$CARD" "card must select empty baseline free order"
guard_expect_in_file "$TAG" 'measurement_profile=phase295x-repeated-v0' "$CARD" "card must keep repeated measurement profile"
guard_expect_in_file "$TAG" 'warmup_count=1' "$CARD" "card must keep warmup count"
guard_expect_in_file "$TAG" 'sample_count=5' "$CARD" "card must keep sample count"
guard_expect_in_file "$TAG" 'canonical_rss_collector=external-time' "$CARD" "card must keep canonical RSS collector"
guard_expect_in_file "$TAG" 'winner_claim=0' "$CARD" "card must keep winner claims closed"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-ATTRIBUTION-SELECTION-295X-001' "$PREV_CARD" "previous row must select this attribution seam"
guard_expect_in_file "$TAG" 'Memory Gap Attribution' "$SSOT" "SSOT must define attribution policy"
guard_expect_in_file "$TAG" '| 189 | `MIMALLOC-COMPARISON-MALLOC-LARGE-CLOSEOUT-295X-001` | Landed |' "$TASKBOARD" "taskboard must mark closeout landed"
guard_expect_in_file "$TAG" '| 190 | `MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-ATTRIBUTION-SELECTION-295X-001` | Current |' "$TASKBOARD" "taskboard must expose the attribution selection row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

if rg -n 'provider_activation=1|host_replacement=1|hook_installed=1|global_allocator_installed=1|winner_claim=1|LD_PRELOAD|replace_process_allocator|install_hook|process_allocator_replacement=1' "$CARD" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: malloc-large memory-gap selection opened forbidden seams" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

echo "[$TAG] ok"
