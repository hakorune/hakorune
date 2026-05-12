---
Status: Complete
Date: 2026-05-12
Scope: M168 `.hako` mimalloc OSVM page-source composition.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/memory/osvm_backed_fast_path_heap_box.hako
  - lang/src/hako_alloc/memory/page_source_policy_box.hako
  - apps/mimalloc-osvm-page-source-composition-proof/
---

# 293x-176 M168 Mimalloc OSVM Page-Source Composition

## Goal

Compose the M167 page queue + page-local free-list model with the existing
`HakoAllocPageSourcePolicy` OSVM reserve/commit/decommit seam.

`HakoAllocOsVmBackedFastPathHeap` owns orchestration only:

- reserve and commit backing bytes for each fresh modeled page;
- add the backed page to `HakoAllocPageQueue`;
- decommit backing pages as cleanup evidence.

## Stop Line

M168 does not add a native OSVM leaf, OSVM unreserve/release, local-free
collection/retire, remote-free atomics, page-map lookup, provider activation,
hook install, process allocator replacement, `.inc` name matching, or
production `usize` field migration.

The M167 heap remains OSVM-free. M168 is a separate adapter so the M167 guard
continues proving the pure page/queue/free-list allocation path, while this row
proves the page-source composition boundary.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_osvm_page_source_composition_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
