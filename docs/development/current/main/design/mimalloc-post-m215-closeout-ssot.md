---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: D209 closeout after M215 thread heap owner-token inventory.
Related:
  - docs/development/current/main/design/hako-alloc-thread-heap-owner-inventory-ssot.md
  - docs/development/current/main/design/mimalloc-migration-closeout-check-ssot.md
  - docs/development/current/main/design/mimalloc-port-remaining-inventory-ssot.md
  - docs/reference/runtime/substrate-capabilities.md
  - docs/reference/language/low-level-capabilities.md
---

# Mimalloc Post-M215 Closeout SSOT

## Decision

The post-M213 mimalloc inventory wave is closed after M215.

Completed in this wave:

```text
D205 post-M213 next-lane selection
D206 mimalloc port remaining inventory
D207 mimalloc next-row selection
M214 allocator options/defaults inventory
D208 mimalloc migration closeout check
M215 thread heap owner-token inventory
```

The next recommended lane is the language minimal-surface lane, starting with a
small documentation reconciliation row before parser implementation resumes.

## Closed allocator surface

M214 and M215 are internal read-only inventory surfaces.

They do not add user-facing syntax, environment variables, mutable runtime
options, allocation policy changes, thread scheduling, atomics expansion,
reclaim execution, unreserve, OS release, provider activation, hooks, or
process allocator replacement.

## Manual/reference sync

D209 adds a short reference note for internal read-only `hako_alloc` inventory
surfaces. Planning cards D205-D208 stay development-only.

Reference sync target:

```text
docs/reference/runtime/substrate-capabilities.md
docs/reference/language/low-level-capabilities.md
```

## Remaining inactive surfaces

These remain future-only and require explicit accepted cards:

```text
reclaim execution
thread scheduling
atomic ownership claim
remote-free drain during reclaim
page ownership migration
page-source release / unreserve / OS release
secure entropy source
generic source-level PackedArray<T> semantics
visible record materialization
provider activation
hooks
process allocator replacement
```

## Next lane

```text
D210 language minimal surface lane switch
```

D210 must name the stop point, first language blocker, guard expectations, and
rollback path before parser or Stage1 implementation rows resume.
