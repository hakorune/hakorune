---
Status: Complete
Date: 2026-05-12
Scope: M180 `.hako` mimalloc huge page model.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/memory/huge_page_model_box.hako
  - apps/mimalloc-huge-page-model-proof/main.hako
---

# 293x-191 M180 Huge Page Model

## Goal

Add the first huge page model without implementing huge release.

M180 owns one-allocation huge page state:

```text
huge ptr -> huge page id -> requested_size / committed_size / live flag
```

Huge handles are registered in `HakoAllocPageMap`, but huge page ids stay
outside the small page index range so they cannot be released through
page-local small free lists.

## Stop Line

M180 does not implement huge release, unregister, OS unreserve/release,
decommit, byte copy, secure-list hardening, provider activation, hook install,
process allocator replacement, native allocator replacement, or `.inc`
allocator-name matching.

M181 owns the huge release seam.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_huge_page_model_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
