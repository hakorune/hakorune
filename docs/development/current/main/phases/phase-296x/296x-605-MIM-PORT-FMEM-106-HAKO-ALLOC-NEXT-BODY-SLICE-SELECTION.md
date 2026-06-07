---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-106.
Related:
  - docs/development/current/main/phases/phase-296x/296x-604-MIM-PORT-FMEM-105-IMPLEMENTATION-REENTRY-SELECTION.md
  - docs/development/current/main/workstreams/mimalloc-current.md
---

# 296x-605 MIM-PORT-FMEM-106 hako_alloc Next Body Slice Selection

## Purpose

Select the next `.hako` hako_alloc fastmem body slice to migrate now that the
post-refresh cleanup series has returned the lane to implementation work.

## Chosen Mode

```text
BoxShape
```

## Candidate Surface

```text
existing hako_alloc memory boxes under lang/src/hako_alloc/memory
existing source-syntax smoke fixtures
existing FastMemory MemOp vocabulary and verifier plans
```

## Required Boundary

```text
do not add code before selecting one body slice
do not add a new MemOp family in this selection card
do not open product activation, hooks, global allocator claim, or winner behavior
do not extend smoke scaffolding unless the selected body slice requires it
```

## Acceptance Sketch

```text
one next hako_alloc body slice is selected
required existing MemOps/proofs are named
missing substrate, if any, is explicitly called out before implementation
CURRENT_STATE points at the selected implementation row
```

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
```
