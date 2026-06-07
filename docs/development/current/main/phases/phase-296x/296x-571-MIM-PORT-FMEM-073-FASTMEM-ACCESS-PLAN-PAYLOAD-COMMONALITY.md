---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-073.
Related:
  - src/mir/fastmem_access_plan.rs
  - src/mir/fastmem_access_plan/types.rs
  - src/mir/fastmem_access_plan/free_list.rs
  - src/mir/fastmem_access_plan/remote.rs
  - src/mir/fastmem_access_plan/linked_list.rs
---

# 296x-571 MIM-PORT-FMEM-073 FastMemory Access-Plan Payload Commonality

## Purpose

Reduce remaining FastMemory access-plan payload duplication after the
winner-claim ladder closeout. This is BoxShape work only: keep report fields,
MemOp vocabulary, verifier decisions, and producer behavior unchanged.

## Required Boundaries

```text
no new MemOp kind
no report/check behavior change
no new hako_alloc body migration row
no product activation or hook behavior change
```

## Acceptance Sketch

```text
LocalFree / FreeHead / AtomicRemoteHead / DrainRemoteListToLocal payloads share
  common head/block-next metadata structures where the field groups are identical
existing fastmem access-plan tests pass
fastmem_check_smoke passes
git diff --check passes
```

## Non-goals

```text
new lowering
new proof kind
current winner-claim behavior changes
```
