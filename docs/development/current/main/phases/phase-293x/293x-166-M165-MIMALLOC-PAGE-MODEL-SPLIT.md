---
Status: Complete
Date: 2026-05-11
Scope: M165 `.hako` mimalloc page model split.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/memory/page_box.hako
  - apps/mimalloc-page-model-proof/
---

# 293x-166 M165 Mimalloc Page Model Split

## Goal

Introduce the first mimalloc-shaped page-local state owner without changing the
existing heap/facade behavior.

`HakoAllocPageModel` owns only the page-local vocabulary needed before queue and
fast-path rows: `free`, `local_free`, `used`, `capacity`, and `reserved`.
`page_heap_box.hako` remains the current small/medium heap prototype and is not
rewired in this row.

## Changes

- Added `lang/src/hako_alloc/memory/page_box.hako` with
  `HakoAllocPageModel`.
- Exported `memory.page_box` from `hako_module.toml`.
- Added `apps/mimalloc-page-model-proof/` to prove page-local invariants
  without a heap:
  - free blocks are seeded into the page-local `free` list;
  - allocation consumes only `free`;
  - same-page local release records into `local_free`;
  - page-local `block_used` rejects double release;
  - `used`, `capacity`, and `reserved` stay separately observable.
- Added a focused M165 guard and kept it out of the wide allocator gate step
  list.

Post-M169 note: `local_free` is now reusable through the page-local collection
behavior added by M169. The M165 guard follows the current page model so it does
not pin the old pre-collection behavior.

## Stop Line

M165 does not add page queues, direct-page cache lookup, allocator fast path
integration, OSVM page sourcing, TLS, atomics, remote-free integration,
provider activation, hook install, process allocator replacement, or `.inc`
name matching.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_page_model_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

M166 may introduce a separate page queue/direct-page cache owner. It should
choose pages, not pop allocation blocks; block allocation remains M167.
