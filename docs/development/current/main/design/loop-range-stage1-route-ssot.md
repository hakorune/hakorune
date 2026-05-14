---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: LOOP-003A Stage1 LoopRange route decision.
Related:
  - docs/development/current/main/design/loop-range-parser-capsule-ssot.md
  - docs/development/current/main/design/loop-cleanup-before-packedarray-ssot.md
  - docs/development/current/main/joinir-architecture-overview.md
---

# LoopRange Stage1 Route SSOT

## Decision

`loop i in start..end { ... }` remains a first-class LoopRange source shape.

Stage0 owns only parsing and metadata transport. Stage1 owns executable
semantics.

## Canonical Route

LOOP-003 is split into small rows:

```text
LOOP-003A:
  decide the route
  accept LoopRange JSON decode as metadata
  fail-fast before executable lowering

LOOP-003B:
  lower LoopRange through the Stage1/JoinIR route
  capture start/end once at loop entry
  create a block-local index
  route continue to the step block

LOOP-003C:
  add verifier facts
  enforce read-only index
  expose bounds facts
```

## Required Semantics

The executable LoopRange lowering must preserve these rules:

```text
start/end:
  evaluated once at loop entry

range:
  end-exclusive

step:
  fixed 1 in MVP

index:
  block-local
  read-only inside the body

continue:
  jumps to the step path before the next condition check

break:
  jumps directly to the exit path
```

## Non-routes

These routes are rejected:

```text
Stage0 desugar to local i + condition loop
AST rewrite from LoopRange to Loop
re-evaluating end on every iteration
allowing assignment to the range index
silent fallback to legacy for-range lowering
```

## Current Fail-fast Contract

Until LOOP-003B lands, JSON v0 bridge accepts the `LoopRange` shape only as
metadata and freezes before executable lowering:

```text
[freeze:contract][json_v0_bridge/loop_range_route_open]
```

This keeps parser/metadata progress visible without pretending that
continue-safe lowering exists.

