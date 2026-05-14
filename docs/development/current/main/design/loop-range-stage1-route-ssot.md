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
  keep loop-carried body writes frozen in the pilot

LOOP-003C:
  add verifier facts
  enforce read-only index
  expose bounds facts
  define carrier policy
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

## Current Pilot Contract

LOOP-003B enables the first executable JSON v0 bridge pilot:

```text
entry-bound start/end capture
header index PHI
end-exclusive compare
continue jumps to the step block
step is fixed 1
```

The pilot still freezes unsupported body writes instead of silently miscompiling
loop-carried variables:

```text
[freeze:contract][json_v0_bridge/loop_range_index_write]
[freeze:contract][json_v0_bridge/loop_range_carrier_unsupported]
```

## LOOP-003C Fact Surface

The executable JSON v0 bridge route emits function-level
`loop_range_facts` metadata.

Each fact records:

```text
index_name
start_value
end_value
index_phi
preheader_bb
header_bb
body_bb
step_bb
exit_bb
step = 1
end_exclusive = true
index_read_only = true
body_writes_supported = false
```

Verifier/backend rows must consume this metadata instead of rediscovering
range-loop shape from raw block layout.

`body_writes_supported=false` keeps the LOOP-003B carrier stop-line explicit
until `LOOP-003D` defines the carrier policy.
