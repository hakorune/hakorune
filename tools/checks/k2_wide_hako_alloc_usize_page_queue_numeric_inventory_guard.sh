#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-usize-page-queue-numeric-inventory"
cd "$ROOT_DIR"

QUEUE="lang/src/hako_alloc/memory/page_queue_box.hako"
NUMERIC="lang/src/hako_alloc/memory/NUMERIC_FIELDS.md"
CARD="docs/development/current/main/phases/phase-294x/294x-252-HAKO-ALLOC-USIZE-PAGE-QUEUE-NUMERIC-INVENTORY-DRIFT-CLEANUP.md"
INDEX="docs/tools/check-scripts-index.md"

for path in "$QUEUE" "$NUMERIC" "$CARD" "$INDEX"; do
  [[ -f "$path" ]] || { echo "[$TAG] ERROR: missing required file: $path" >&2; exit 1; }
done

rg -F -q 'box HakoAllocPageQueue' "$QUEUE"
rg -F -q 'bin: usize' "$QUEUE"
rg -F -q 'page_count: usize = 0' "$QUEUE"
rg -F -q 'has_direct_page: i64 = 0' "$QUEUE"
rg -F -q 'direct_page_index: usize = 0' "$QUEUE"
rg -F -q 'add_count: usize = 0' "$QUEUE"
rg -F -q 'select_count: usize = 0' "$QUEUE"
rg -F -q 'direct_hit_count: usize = 0' "$QUEUE"
rg -F -q 'refresh_count: usize = 0' "$QUEUE"
rg -F -q 'reject_count: usize = 0' "$QUEUE"

rg -F -q '| `page_queue_box.hako` | `HakoAllocPageQueue` | `bin` | `usize` | `index` | Exact queue bin index via `HAKO-ALLOC-USIZE-FIELD-GROUP-093`.' "$NUMERIC"
rg -F -q '| `page_queue_box.hako` | `HakoAllocPageQueue` | `has_direct_page` | `i64` | `enum` | Binary presence state stays signed until bool/flag storage gets a dedicated row.' "$NUMERIC"
rg -F -q '| `page_queue_box.hako` | `HakoAllocPageQueue` | `add_count` | `usize` | `count` | Exact queue stats counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-058`.' "$NUMERIC"
rg -F -q '| `page_queue_box.hako` | `HakoAllocPageQueue` | `select_count` | `usize` | `count` | Exact queue stats counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-058`.' "$NUMERIC"
rg -F -q '| `page_queue_box.hako` | `HakoAllocPageQueue` | `direct_hit_count` | `usize` | `count` | Exact queue stats counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-058`.' "$NUMERIC"
rg -F -q '| `page_queue_box.hako` | `HakoAllocPageQueue` | `refresh_count` | `usize` | `count` | Exact queue stats counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-058`.' "$NUMERIC"
rg -F -q '| `page_queue_box.hako` | `HakoAllocPageQueue` | `reject_count` | `usize` | `count` | Exact queue stats counter via `HAKO-ALLOC-USIZE-FIELD-GROUP-058`.' "$NUMERIC"
rg -F -q 'k2_wide_hako_alloc_usize_page_queue_numeric_inventory_guard.sh' "$INDEX"

echo "[$TAG] ok"
