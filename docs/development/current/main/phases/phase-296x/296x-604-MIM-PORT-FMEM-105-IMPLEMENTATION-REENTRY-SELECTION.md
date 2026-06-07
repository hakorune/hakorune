---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-105.
Related:
  - docs/development/current/main/phases/phase-296x/296x-603-MIM-PORT-FMEM-104-POST-REFRESH-CLEANUP-CLOSEOUT-AUDIT.md
  - docs/development/current/main/phases/phase-296x/296x-596-MIM-PORT-FMEM-097-REFRESHED-WINNER-CLOSEOUT-AUDIT.md
---

# 296x-604 MIM-PORT-FMEM-105 Implementation Reentry Selection

## Purpose

Select the next implementation row after the refreshed terminal ladder and
post-refresh cleanup series. This is a selection card: choose one narrow next
owner before touching code.

## Chosen Mode

```text
BoxShape
```

## Candidate Directions

```text
option A: hako_alloc body migration reentry
  choose the next .hako fastmem body slice now that the refreshed ladder is clean

option B: producer-neutral activation-readiness audit
  keep behavior closed and verify the final refreshed winner state is enough
  for a later activation ladder

option C: source-syntax smoke structure split
  continue large-file cleanup only if implementation reentry is blocked
```

## Required Boundary

```text
do not add new MemOps in this card
do not change report/check behavior in this card
do not open product activation, hooks, global allocator claim, or winner behavior
keep BoxShape selection separate from the next BoxCount feature row
```

## Acceptance Sketch

```text
one next implementation row is selected
the reason for not selecting the other candidates is documented
CURRENT_STATE points at the selected next row
```

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
```

## Decision

```text
selected: option A, hako_alloc body migration reentry
```

Rationale:

```text
The refreshed terminal ladder and cleanup series are complete enough to stop
growing smoke/report scaffolding. The active mimalloc workstream also names
`.hako` mimalloc fastmem migration body work as the next priority after the
smoke-growth brake.
```

## Rejected Candidates

```text
option B: producer-neutral activation-readiness audit
  rejected for now because 596 and 603 already close the refreshed winner and
  SSOT cleanup evidence. Product behavior remains closed until a dedicated
  activation ladder reopens it.

option C: source-syntax smoke structure split
  rejected for now because 602 reduced the refresh helper duplication enough
  for implementation reentry. Further large-file cleanup can wait until the
  next implementation blocker exposes a real readability issue.
```

## Closeout

```text
next: 296x-605 MIM-PORT-FMEM-106 hako_alloc next body slice selection
```
