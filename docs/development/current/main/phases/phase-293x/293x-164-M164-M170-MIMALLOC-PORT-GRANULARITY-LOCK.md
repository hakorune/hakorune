---
Status: Complete
Date: 2026-05-11
Scope: docs-only M164-M170 `.hako` mimalloc port granularity lock.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-163-M163-MIMALLOC-SIZE-CLASS-POLICY-OWNER.md
---

# 293x-164 M164-M170 Mimalloc Port Granularity Lock

## Goal

Make the post-M163 mimalloc port plan fine-grained enough that implementation
rows do not collapse back into a large `page_heap_box.hako` owner.

## Updated Contract

- `M164` is a layout migration closeout row. M163 already delegated
  `LayoutBox` to `SizeClassBox`; M164 should only clean up stale callsites or
  prove no code delta is needed.
- `M165` creates the page-local owner before queues.
- `M166` creates page queues and direct-page cache before block allocation
  fast paths.
- `M167` owns fast allocation and deterministic fallback, but not OSVM.
- `M168` composes existing OSVM page-source rows without adding native leaves.
- `M169` owns same-thread local free collection and retire only.
- `M170` integrates remote-free through existing pointer atomics only.

## Stop Line

If a row adds algorithm bodies back into `page_heap_box.hako`, split the row
before continuing. `page_heap_box.hako` should remain orchestration-oriented.

M164-M170 still do not add provider activation, hook install, process
allocator replacement, `.inc` name matching, pointer `fetch_add`, page-map
lookup, realloc/aligned allocation, secure free lists, purge, stats, or options.

## Proof

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
