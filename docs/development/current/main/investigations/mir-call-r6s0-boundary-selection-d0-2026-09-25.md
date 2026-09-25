# MIR-CALL-R6S0-BOUNDARY-SELECTION-D0 — R6-S0 exact boundary selection

Status: open
Date: 2026-09-25
Parent: SELFHOST-RESUME-ENTRY-RECHECK0-P0 (closed 2026-09-25)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
  (staged R6-S0..R7 queue, "remains unopened until its exact boundary
  is selected")
Implementation permission: false; this is a design_stop selection row.
No code, route, fixture, caller, or fallback change.

## Purpose

The Call/R7 queue is unopened only because the R6-S0 split boundary was
never selected. This row selects exactly which instruction-schema/visitor
owners and which published-view/transport owners split before any
semantic growth — or records `NoSafeSlice` with an explicit reopen
trigger. It does not re-run the forbidden aggregate inventory and does
not grant implementation permission to any downstream row.

## Six-line brief

```text
Decision: name the R6-S0 boundary — one instruction-schema/visitor
  owner set and one published-view/transport owner set — as the first
  behavior-neutral split of the staged queue, or NoSafeSlice.
Source authority + canonical issuer: the R7 retirement card's staged
  queue definition and the existing owner census fixtures; this card
  selects a boundary, it does not mint authority.
Non-authority: no inventory repeat, no semantic growth, no caller
  switch, no deletion, no new semantic receipt.
Fail-fast boundary: if no boundary has a finite caller set plus a
  behavior-neutral split shape, the row records NoSafeSlice and parks
  the queue again with a reopen trigger.
Smallest next slice: boundary candidate table (owner, callers, split
  shape, delete-set impact) plus one accepted Decision.
Non-claims: does not open R6-S1..R7; does not fix the EXE suite reds;
  does not touch B3 or language-v1 rows.
```

## Census boundary

`このcensusが覆う境界: staged R6-S0 queue description -> one accepted
split boundary; includes instruction-schema/visitor and
published-view/transport owner candidates under src/mir/**; excludes
semantic changes, caller migration, deletion, and any other queue stage.`

## Exit

An accepted Decision naming the R6-S0 boundary (owners, files, split
shape, finite caller set, planned delete-set impact) or `NoSafeSlice`
with an observable reopen trigger. On acceptance, work_mode returns to
`fast` for the bounded split row and `next_execution_card` names it.
