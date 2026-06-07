---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-081.
Related:
  - docs/development/current/main/phases/phase-296x/296x-579-MIM-PORT-FMEM-080-PAGE-LOCAL-ALLOC-FREE-ROUTE-BODY-JOIN-PRODUCER-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-556-MIM-PORT-FMEM-058-TLS-BACKING-TRANSFER-PREFLIGHT.md
  - docs/development/current/main/phases/phase-296x/296x-557-MIM-PORT-FMEM-059-TLS-BACKING-TRANSFER-PRODUCER-PILOT.md
---

# 296x-580 MIM-PORT-FMEM-081 Post Route-Join Terminal Ladder Reentry Selection

## Purpose

Select the next terminal-ladder row after the page-local alloc/free route body
join producer pilot.

The old terminal ladder already advanced through TLS, owner slot reuse,
abandoned reclaim, product activation, hook install, global allocator claim, and
winner claim using page-local free route CFG evidence. The allocation route CFG
was added later, so this card decides whether to refresh the terminal ladder or
resume at a specific existing boundary.

## Candidate Next Slices

```text
1. terminal ladder refresh preflight that requires page_local_route_body_join_open=1
2. TLS backing transfer preflight refresh
3. product activation preflight refresh
4. winner claim closeout audit after route join
```

## Required Boundaries

```text
selection row only unless a single next slice is explicitly chosen
no new MemOp kind
no product activation / hook / global allocator / winner claim change
no diagnostic Python-template C bridge retirement
```

## Acceptance Sketch

```text
next terminal-ladder slice is named
the selected slice consumes page_local_route_body_join_open=1
closed activation / hook / allocator / winner claims stay explicit until their own rows
current state pointer guard passes
git diff --check passes
```

## Non-goals

```text
opening product allocator replacement
changing already-landed terminal rows without a refresh decision
```
