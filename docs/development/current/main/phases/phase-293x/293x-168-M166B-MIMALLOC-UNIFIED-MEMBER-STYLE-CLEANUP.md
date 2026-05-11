---
Status: Complete
Date: 2026-05-11
Scope: M166B `.hako` mimalloc unified-member style cleanup.
Related:
  - docs/reference/language/EBNF.md
  - docs/reference/language/LANGUAGE_REFERENCE_2025.md
  - lang/src/hako_alloc/memory/page_box.hako
  - lang/src/hako_alloc/memory/page_queue_box.hako
---

# 293x-168 M166B Mimalloc Unified Member Style Cleanup

## Goal

Align newly added mimalloc `.hako` state boxes with the current language
reference before continuing to M167.

`init { ... }` remains supported as a legacy compatibility slot list, but new
code should prefer Unified Members stored declarations such as `field: Type`.

## Changes

- Converted `HakoAllocPageModel` fields from legacy `init { ... }` to stored
  member declarations.
- Converted `HakoAllocPageQueue` fields from legacy `init { ... }` to stored
  member declarations.
- Tightened the M165 and M166 focused guards so these new state boxes cannot
  drift back to legacy slot lists.
- Kept existing proof behavior unchanged.

## Stop Line

This is a style/structure cleanup only. It does not add allocator fast-path
behavior, OSVM page sourcing, local-free collection/retire, remote-free
integration, provider activation, hook install, process allocator replacement,
or `.inc` name matching.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_page_model_guard.sh
bash tools/checks/k2_wide_mimalloc_page_queue_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

M167 should introduce the alloc fast path using Unified Members from the start.
