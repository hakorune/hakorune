---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-075.
Related:
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
  - src/mir/fastmem_access_plan.rs
  - tools/hako_check/fastmem_check_smoke.sh
---

# 296x-573 MIM-PORT-FMEM-075 Hako Alloc Body Migration Reentry Selection

## Purpose

Return from the FastMemory BoxShape cleanup mini-series to the mimalloc port
mainline. Select the next `.hako` hako_alloc body slice that should use landed
FastMemory substrate, without opening a new lowering or product activation row
in the same commit.

## Required Boundaries

```text
selection/report-only row
no new MemOp kind
no new lowering
no report/check behavior change unless the selected row explicitly requires it
no product activation / hook / global allocator claim / winner claim changes
```

## Selection Questions

```text
Which landed substrate is ready to consume next?
Which hako_alloc body still has the highest C-bridge duplication pressure?
Can the next slice be one durable semantic row with fixture/gate coverage?
Which existing checks prove the selected boundary stays non-activating?
```

## Acceptance Sketch

```text
next body migration row is named
explicit non-goals and closed claims are recorded
required fixture/gate evidence for the next row is listed
current FastMemory smoke remains green
git diff --check passes
```

## Non-goals

```text
implementing the selected body slice
retiring the diagnostic Python-template C bridge
changing product activation or allocator claim behavior
```
