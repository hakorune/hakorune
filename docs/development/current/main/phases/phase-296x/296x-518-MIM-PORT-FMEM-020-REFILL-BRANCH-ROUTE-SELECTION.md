---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-020.
Related:
  - docs/development/current/main/phases/phase-296x/296x-517-MIM-PORT-FMEM-019B-FREE-HEAD-PUSH-DERIVED-NON-EMPTY-PROOF.md
  - lang/src/hako_alloc/memory/page_meta_refill_then_free_head_alloc_body_box.hako
---

# 296x-518 MIM-PORT-FMEM-020 Refill Branch/Route Selection

## Purpose

Select the next smallest durable slice after the refill-then-free_head alloc
body.

The current `.hako hako_alloc` path can already compose:

```text
LocalFreePop(page)
FreeHeadPush(page, block)
FreeHeadPop(page)
page.used = page.used + 1
```

The next missing shape is the real allocation route boundary:

```text
if local_free is non-empty:
  pop local_free
else if free_head is non-empty:
  pop free_head
else:
  refill local_free -> free_head, then pop free_head
```

## Selection Rules

The selection must stay narrow.

```text
Allowed:
  - choose one next route proof / branch preflight slice
  - add source/MIR/report evidence for that one slice
  - keep existing free-list MemOps as the mutation substrate

Forbidden:
  - multi-block refill transfer
  - generic fastmem CFG semantics
  - ordinary free_head / local_free_head FieldLoad or FieldStore mutation
  - remote owner routing
  - AtomicRemoteHead
  - TLS backing transfer
  - provider activation
  - process allocator replacement
  - hook installation
  - global allocator claim
  - winner claim
```

## Candidate Next Slices

```text
A. Branch preflight:
   observe the minimal fastmem route shape for local_free/free_head/refill
   without lowering multi-branch mutation.

B. Non-empty proof selector:
   add verifier-owned route evidence that distinguishes source assumptions
   from facts derived by previous MemOps.

C. Single-route allocator wrapper:
   keep the body straight-line but introduce an explicit route-selection
   report surface for later branch lowering.
```

## Acceptance

```text
The selected next slice is documented before implementation.
No new product allocator claim appears.
No ABI hot lookup appears.
No Python-template C bridge becomes semantic truth again.
The next implementation row has one clear fixture/smoke boundary.
```
