---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-020.
Related:
  - docs/development/current/main/phases/phase-296x/296x-517-MIM-PORT-FMEM-019B-FREE-HEAD-PUSH-DERIVED-NON-EMPTY-PROOF.md
  - lang/src/hako_alloc/memory/page_meta_refill_then_free_head_alloc_body_box.hako
  - src/mir/builder/fastmem.rs
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

## Decision

Choose **C. Single-route allocator wrapper / route-selection report surface** as
the next durable slice.

Do not open `fastmem` branch execution yet.

Reason:

```text
src/mir/builder/fastmem.rs currently lowers ASTNode::If by evaluating the
condition and then lowering the then/else bodies in source order. It does not
emit CFG Branch/Join structure for fastmem regions.
```

That is good enough for parse/source observation, but not safe as allocator
route execution. Opening branch behavior now would risk treating a linearized
observation path as real mutually-exclusive allocation routing.

The next implementation row must therefore add route-selection evidence over
the already-verified straight-line bodies before any branch lowering.

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

## Selected Next Row

```text
MIM-PORT-FMEM-021:
  Page-local allocation route report surface

Goal:
  classify existing verified single-route bodies as route candidates:
    local_free_alloc
    free_head_alloc
    refill_then_free_head_alloc

Acceptance:
  - route classification is derived from verified MemOp plans
  - no branch semantics are claimed
  - no fastmem CFG lowering is opened
  - no ordinary free_head/local_free_head FieldLoad/FieldStore mutation opens
  - no remote owner routing / AtomicRemoteHead / TLS transfer opens
  - no product allocator activation or winner claim appears
```

## Deferred Branch Work

Real branch routing needs a separate design row because it must define:

```text
fastmem branch CFG representation
proof dominance across branches
route exclusivity evidence
LayoutRef / free-list token lifetime across branch joins
report/check fields for selected runtime route vs observed candidate route
```

Until that row lands, branch-shaped `.hako` bodies are observation-only and must
not be treated as allocator execution truth.

## Acceptance

```text
The selected next slice is documented before implementation.
No new product allocator claim appears.
No ABI hot lookup appears.
No Python-template C bridge becomes semantic truth again.
The next implementation row has one clear fixture/smoke boundary.
```
