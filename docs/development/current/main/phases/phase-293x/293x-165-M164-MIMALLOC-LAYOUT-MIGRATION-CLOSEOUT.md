---
Status: Complete
Date: 2026-05-11
Scope: M164 `.hako` mimalloc layout migration closeout.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-163-M163-MIMALLOC-SIZE-CLASS-POLICY-OWNER.md
  - lang/src/hako_alloc/memory/layout_box.hako
  - lang/src/hako_alloc/memory/size_class_box.hako
---

# 293x-165 M164 Mimalloc Layout Migration Closeout

## Goal

Close M164 by proving the M163 layout migration is complete without widening
allocator behavior.

M163 already moved size-class truth into `SizeClassBox`. M164 fixes the
remaining ownership contract: `LayoutBox` is only the current small/medium
compatibility facade, and `page_heap_box.hako` consumes that facade without
bypassing it.

## Changes

- Clarified `layout_box.hako` as the legacy two-class compatibility facade.
- Added a focused M164 guard that checks:
  - `SizeClassBox` owns `size_to_bin` and `bin_size`;
  - `LayoutBox` delegates normalize/class-size decisions to `SizeClassBox`;
  - `page_heap_box.hako` still consumes `LayoutBox` as the compatibility seam;
  - no layout migration matcher leaked into `.inc`;
  - existing VM and representative EXE allocator proofs stay green.
- Kept M164 out of the wide allocator gate step list.

## Stop Line

M164 does not add a page model, page queues, fast-path allocation, OSVM page
source composition, local-free collection, remote-free integration, provider
activation, hook install, process allocator replacement, or `.inc` name
matching.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_layout_migration_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

M165 may introduce `page_box.hako` as a page-local model owner. It must not
add page queues, OSVM, TLS, atomics, or remote-free behavior.
