---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-131.
Related:
  - docs/development/current/main/design/fastmem-verified-direct-default-retirement-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-629-MIM-PORT-FMEM-130-BRANCH-ROUTE-CONDITION-FACT-RETIREMENT.md
  - src/mir/builder/fastmem.rs
  - src/mir/builder/fastmem/branch.rs
  - tools/hako_check/fastmem_check.py
---

# 296x-630 MIM-PORT-FMEM-131 FastMemory Dedicated Lowerer Remaining Task Order

## Purpose

Record the post-007 status of the FastMemory dedicated lowerer retirement lane
and split the remaining work into rows that a smaller implementation model can
continue without reopening allocator activation or remote-head behavior.

The branch CFG route is no longer bespoke after 296x-629, but the FastMemory
source lowerer is still transitional and still interprets broad AST shapes.

## Current Evaluation

```text
docs / policy:
  pass

inventory / guard:
  mostly pass

implementation separation:
  partial

legacy retirement:
  partial

remaining task clarity:
  this card closes the inventory gap
```

## Correction To Worker Feedback

```text
stale claim:
  branch is still a dedicated CFG route

current state:
  branch CFG lowering delegates to ordinary if lowering
  fastmem still owns the ownerEq branch condition gate
```

The important remaining debt is not branch CFG generation. It is the broad AST
interpretation still living in `src/mir/builder/fastmem.rs`.

## 008 Status

```text
MIRBUILDER-FMEM-008 is landed:
  the post-007 inventory now exposes local / literal / variable / call /
  method-call counts plus the branch condition gate count.
```

## 009 Status

```text
MIRBUILDER-FMEM-009 is landed:
  local, print, return, and variable assignment now share the mechanical
  statement-shell helpers while fastmem expression lowering remains in place.
```

## 010 Status

```text
MIRBUILDER-FMEM-010 is landed:
  field accesses now share the ordinary FieldGet / FieldSet builder path while
  verified-direct field evidence remains visible in inventory and check output.
```

## Remaining Rows

```text
MIRBUILDER-FMEM-011:
  index route retirement
  Index becomes ordinary index origin plus IndexAccessSite and verified table
  obligations

MIRBUILDER-FMEM-012:
  numeric route retirement
  BinaryOp becomes ordinary BinOp plus numeric route facts

MIRBUILDER-FMEM-013:
  intrinsic registry cleanup
  mem.* stays fastmem-specific vocabulary but stops being scattered string
  matching

MIRBUILDER-FMEM-014:
  branch condition gate generalization
  ownerEq condition proof moves toward ordinary condition route facts

MIRBUILDER-FMEM-015:
  dedicated lowerer closeout
  fastmem.rs keeps only region entry and obligation shell
```

## Non-Goals

```text
delete fastmem.rs immediately
open AtomicRemoteHead behavior
open TLS backing transfer
open product activation
install hooks
claim global allocator replacement
claim winner status
add generic IndexGet/IndexSet without a separate accepted row
```

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed

```text
The retirement lane now has a post-007 status correction and a concrete
MIRBUILDER-FMEM-009..015 task order, with MIRBUILDER-FMEM-008 landed as the
inventory slice.
```

## Closeout

```text
next: MIRBUILDER-FMEM-011 index route retirement
```
