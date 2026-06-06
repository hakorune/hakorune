---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-026.
Related:
  - docs/development/current/main/phases/phase-296x/296x-523-MIM-PORT-FMEM-025-PAGE-LOCAL-FREE-ROUTE-REPORT-SURFACE.md
  - docs/development/current/main/phases/phase-296x/296x-521-MIM-PORT-FMEM-023-FASTMEM-BRANCH-REJECTION-GATE.md
---

# 296x-524 MIM-PORT-FMEM-026 Remote-Free Substrate Selection

## Purpose

Select the next narrow substrate before migrating remote-owner free behavior
into `.hako hako_alloc` fastmem bodies.

MIM-024 and MIM-025 cover the same-owner local-free route as a straight-line
body and report candidate. Remote-owner free is still closed.

## Candidate Next Slices

```text
A. AtomicRemoteHead vocabulary/source preflight:
   add source-visible remote-head MemOp names but keep verifier/lowering closed.

B. Remote-free route report vocabulary:
   add report/check fields for remote route candidates, all candidates `none`
   until AtomicRemoteHead exists in MIR producer evidence.

C. Branch-proof row:
   reopen CFG proof design before any same/remote route body can be expressed.
```

## Selection Guard

Prefer A or B before C unless a concrete branch fixture proves:

```text
facts from an untaken branch are not visible to later MemOps
LayoutRef lifetime does not cross branch joins unsafely
remote route candidate evidence remains producer-neutral
no backend redecision or ABI lookup appears
```

## Still Closed

```text
fastmem branch CFG lowering
remote owner routing
AtomicRemoteHead lowering
TLS backing transfer
process allocator replacement
hook installation
global allocator claim
winner claim
```
