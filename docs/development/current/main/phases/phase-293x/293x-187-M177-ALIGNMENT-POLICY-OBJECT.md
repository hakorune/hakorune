---
Status: Complete
Date: 2026-05-12
Scope: M177 `.hako` mimalloc alignment policy object.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/memory/alignment_policy_box.hako
  - lang/src/hako_alloc/memory/size_class_box.hako
---

# 293x-187 M177 Alignment Policy Object

## Goal

Add a standalone alignment policy owner before aligned allocation execution.

M177 keeps the contract policy-only:

```text
requested alignment
  -> normalize to the minimum supported alignment
  -> reject non-power-of-two / non-positive values
  -> compute padded request size
  -> ask SizeClassBox for the padded good size
```

This row does not allocate aligned blocks. It only freezes normalization and
padding decisions so M178 can reuse them.

## Stop Line

M177 does not implement aligned allocation execution, page-map widening,
byte-copy alignment shims, huge-page routing, secure-list hardening, OSVM
release, provider activation, hook install, process allocator replacement, or
native/ABI alignment claims.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_alignment_policy_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
