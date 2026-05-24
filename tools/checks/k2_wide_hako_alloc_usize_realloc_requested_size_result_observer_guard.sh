#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-usize-realloc-requested-size-result-observer"
cd "$ROOT_DIR"

RESULT="lang/src/hako_alloc/memory/object_lifecycle_facade_result_box.hako"
NUMERIC="lang/src/hako_alloc/memory/NUMERIC_FIELDS.md"
CARD="docs/development/current/main/phases/phase-294x/294x-250-HAKO-ALLOC-USIZE-REALLOC-REQUESTED-SIZE-RESULT-OBSERVER.md"
INDEX="docs/tools/check-scripts-index.md"

for path in "$RESULT" "$NUMERIC" "$CARD" "$INDEX"; do
  [[ -f "$path" ]] || { echo "[$TAG] ERROR: missing required file: $path" >&2; exit 1; }
done

rg -F -q 'box HakoAllocObjectLifecycleReallocResult' "$RESULT"
rg -F -q 'last_requested_size: usize = 0' "$RESULT"
rg -F -q 'last_page_id: i64 = -1' "$RESULT"
rg -F -q 'last_block_id: i64 = -1' "$RESULT"
rg -F -q 'last_new_page_id: i64 = -1' "$RESULT"
rg -F -q 'last_new_block_id: i64 = -1' "$RESULT"
rg -F -q 'last_reason: i64 = 0' "$RESULT"
rg -F -q 'last_ok: i64 = 0' "$RESULT"
rg -F -q 'last_requested: i64 = -1' "$RESULT"
rg -F -q 'last_normalized: i64 = -1' "$RESULT"
rg -F -q 'recordRequest(page_id, block_id, requested_size)' "$RESULT"
rg -F -q 'recordSuccess(page_id, block_id, requested_size)' "$RESULT"
rg -F -q 'recordMoveSuccess(old_page_id, old_block_id, new_page_id, new_block_id, requested_size)' "$RESULT"
rg -F -q 'HAKO-ALLOC-USIZE-FIELD-GROUP-249' "$NUMERIC"
rg -F -q 'k2_wide_hako_alloc_usize_realloc_requested_size_result_observer_guard.sh' "$INDEX"

echo "[$TAG] ok"
