# Hako Alloc Numeric Field Inventory

Status: SSOT
Date: 2026-05-12
Scope: stored numeric fields under `lang/src/hako_alloc/memory/`.
Related:
- `docs/development/current/main/phases/phase-294x/294x-16-HAKO-ALLOC-NUMERIC-FIELD-INVENTORY.md`
- `docs/development/current/main/design/usize-semantic-foundation-ssot.md`

## Decision

All live `hako_alloc` numeric stored fields remain `i64` for now.

This inventory classifies migration candidates before any field is changed to
`usize`. It is not a migration row and it does not change runtime behavior.

## Categories

- `signed-sentinel`: uses a negative value such as `-1`; do not migrate until
  the state shape is split.
- `signed-delta`: may intentionally move above and below zero.
- `index`: non-negative id / slot / bin index.
- `size`: object or block size.
- `capacity`: count of available storage slots or reserved blocks.
- `count`: event, occupancy, stack-top, or statistic count.
- `byte-length`: accumulated or requested bytes.

## Stored Field Inventory

Current stored numeric field count: 37.

No stored `signed-delta` field is live today.
No stored `signed-sentinel` field is live after 294x-17.

Probe-only exact `usize` stored fields live in `usize_field_probe_box.hako`.
They are intentionally excluded from the production migration inventory below.

| File | Box | Field | Current Type | Category | Migration Note |
| --- | --- | --- | --- | --- | --- |
| `page_box.hako` | `HakoAllocPageModel` | `page_id` | `i64` | `index` | Candidate after id/index call sites use exact non-negative semantics. |
| `page_box.hako` | `HakoAllocPageModel` | `block_size` | `i64` | `size` | Candidate after exact `usize` backend/storage lowering exists. |
| `page_box.hako` | `HakoAllocPageModel` | `capacity` | `i64` | `capacity` | First migration candidate group. |
| `page_box.hako` | `HakoAllocPageModel` | `reserved` | `i64` | `capacity` | Candidate with `capacity`; keep invariant `reserved <= capacity`. |
| `page_box.hako` | `HakoAllocPageModel` | `used` | `i64` | `count` | Candidate after dynamic range checks cover decrement paths. |
| `page_box.hako` | `HakoAllocPageModel` | `free_top` | `i64` | `count` | Candidate, but preserve stack-top underflow checks first. |
| `page_box.hako` | `HakoAllocPageModel` | `local_free_top` | `i64` | `count` | Candidate with local-free collection row. |
| `page_box.hako` | `HakoAllocPageModel` | `alloc_count` | `i64` | `count` | Low-risk stats candidate. |
| `page_box.hako` | `HakoAllocPageModel` | `local_free_count` | `i64` | `count` | Low-risk stats candidate. |
| `page_box.hako` | `HakoAllocPageModel` | `reject_count` | `i64` | `count` | Low-risk stats candidate. |
| `page_box.hako` | `HakoAllocPageModel` | `peak_used` | `i64` | `count` | Candidate with `used`. |
| `page_box.hako` | `HakoAllocPageModel` | `requested_bytes` | `i64` | `byte-length` | Candidate after checked add/overflow diagnostics are live for byte sums. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `bin` | `i64` | `index` | Candidate after bin vocabulary is exact non-negative. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `page_count` | `i64` | `count` | Candidate with queue length/capacity rows. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `has_direct_page` | `i64` | `count` | Binary presence state split from the old `-1` direct-page sentinel. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `direct_page_index` | `i64` | `index` | Non-negative after 294x-17; migration candidate after queue index contracts. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `add_count` | `i64` | `count` | Low-risk stats candidate. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `select_count` | `i64` | `count` | Low-risk stats candidate. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `direct_hit_count` | `i64` | `count` | Low-risk stats candidate. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `refresh_count` | `i64` | `count` | Low-risk stats candidate. |
| `page_queue_box.hako` | `HakoAllocPageQueue` | `reject_count` | `i64` | `count` | Low-risk stats candidate. |
| `page_heap_box.hako` | `HakoAllocHandle` | `page_id` | `i64` | `index` | Candidate after handle id contracts are exact non-negative. |
| `page_heap_box.hako` | `HakoAllocHandle` | `block_id` | `i64` | `index` | Candidate after block-id sentinel returns are split. |
| `page_heap_box.hako` | `HakoAllocHandle` | `requested_size` | `i64` | `size` | Candidate after requested-size callers use exact non-negative semantics. |
| `page_heap_box.hako` | `HakoAllocPage` | `page_id` | `i64` | `index` | Candidate after page id contracts are exact non-negative. |
| `page_heap_box.hako` | `HakoAllocPage` | `block_size` | `i64` | `size` | Candidate with size-class migration. |
| `page_heap_box.hako` | `HakoAllocPage` | `capacity` | `i64` | `capacity` | Candidate, but this prototype may be superseded by `HakoAllocPageModel`. |
| `page_heap_box.hako` | `HakoAllocPage` | `free_top` | `i64` | `count` | Candidate, preserve underflow checks first. |
| `page_heap_box.hako` | `HakoAllocPage` | `alloc_count` | `i64` | `count` | Low-risk stats candidate. |
| `page_heap_box.hako` | `HakoAllocPage` | `free_count` | `i64` | `count` | Low-risk stats candidate. |
| `page_heap_box.hako` | `HakoAllocPage` | `reuse_count` | `i64` | `count` | Low-risk stats candidate. |
| `page_heap_box.hako` | `HakoAllocPage` | `current_used` | `i64` | `count` | Candidate after decrement paths are guarded. |
| `page_heap_box.hako` | `HakoAllocPage` | `peak_used` | `i64` | `count` | Candidate with `current_used`. |
| `page_heap_box.hako` | `HakoAllocPage` | `requested_bytes` | `i64` | `byte-length` | Candidate after checked add/overflow diagnostics are live for byte sums. |
| `allocator_facade_box.hako` | `HakoAllocProductionFacade` | `alloc_count` | `i64` | `count` | Low-risk stats candidate. |
| `allocator_facade_box.hako` | `HakoAllocProductionFacade` | `free_count` | `i64` | `count` | Low-risk stats candidate. |
| `allocator_facade_box.hako` | `HakoAllocProductionFacade` | `reject_count` | `i64` | `count` | Low-risk stats candidate. |

## Sentinel Notes

Stored negative sentinel:

- none.

Non-stored sentinel seams that must be considered in the next row:

- `HakoAllocPageModel.acquire(...)` returns `-1` on reject.
- `HakoAllocPageQueue.addPage(...)` returns `-1` on reject.
- `HakoAllocPageQueue.directPageId()` returns `-1` when no direct page exists.

## Migration Order

1. Keep `signed-sentinel` fields as `i64` or split them first.
2. Probe low-risk stats `count` fields.
3. Probe `capacity` / stack-top fields with underflow checks.
4. Probe `size` and `byte-length` fields only after checked arithmetic
   diagnostics are stable enough for allocator byte sums.
5. Probe `index` fields after sentinel returns and not-found states are
   explicit.
