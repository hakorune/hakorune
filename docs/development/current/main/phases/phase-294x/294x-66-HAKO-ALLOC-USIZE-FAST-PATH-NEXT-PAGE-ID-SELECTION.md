---
Status: Landed
Date: 2026-05-23
Scope: select fast-path heap next-page id exact `usize` row.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/alloc_fast_path_heap_box.hako
---

# 294x-66 Hako Alloc Usize Fast Path Next Page Id Selection

## Decision

Select `HakoAllocFastPathHeap.next_page_id` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-089`.

`next_page_id` is the owner-local page-array length and next page id source for
the non-OSVM fast-path heap. `addPage()` copies the current value into a page
id, pushes the page into `pages`, then increments `next_page_id`. `release()`
already rejects `handle.page_id < 0` before comparing the non-negative signed
handle page id against `next_page_id`, so the follow-on row can keep the signed
handle payload seam explicit while migrating only the owner-local length/source
field.

## Stop Line

The follow-on row must not migrate:

- `HakoAllocFastPathHeap.bin`;
- `HakoAllocFastPathHandle.page_id` or `block_id`;
- `HakoAllocPageModel.page_id`;
- OSVM-backed `next_page_id` or backing payloads;
- provider activation, host allocator replacement, hooks, TLS, atomics, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
