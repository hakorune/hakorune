---
Status: design stop; implementation forbidden until selection
Date: 2026-07-16
Decision: pending
Previous row: P0c-MR-R0-G0 closed
Recommended next row: HMI-P0 inventory and ingress selection
---

# HMI-P0 MIR Semantic-Reference Migration Consultation

## Outcome already closed

P0c-R0 has one final callable source authority:

```text
owned function-only Program
  -> immutable callable catalog
  -> canonical-keyed resolved functions
  -> shared graph inventory
  -> deterministic SCC partition
  -> typed finite direct-call plans
  -> unpublished drafts
  -> atomic module publication
```

Singleton self recursion and multi-function recursion use this same route.
The old bare-function `RootCallable`, one-entry index facades, and exact-one
call policy are physically retired. Body-only `compile_resolved` remains
explicitly call-disabled. No compatibility adapter or route retry exists.

## Why HMI-P0 is the next genuine selfhost design stop

The `.hako` MirBuilder/parser backlog is failure-driven and currently has no
active blocker. It must remain monitor-only until an existing probe produces a
stable first freeze/reject. Ownership V2 grammar is also a separate parked
language lane.

The D-prime dependency graph instead names HMI-P0 as the first unselected row
of the required Rust semantic-reference retirement branch:

```text
{SSA-RC-A1c, SSA-RC-V0, SSA-RC-RET-P0}
  -> HMI-P0 -> HMI-S0 -> HMI-S1 -> HMI-I0 -> HMI-P1

{SSA-I1-O1, HMI-P1}
  -> HMI-C0 -> HMI-X0 -> HMI-R1
```

The three HMI-P0 prerequisites are closed. HMI-C0 remains correctly blocked on
the future exact BoxRef SSA-I1-O1 owner, but disconnected inventory and parity
preparation through HMI-P1 can proceed independently.

## HMI-P0 task boundary

HMI-P0 is read-only inventory plus one ingress decision. It changes no
execution owner and activates no opcode.

Inventory exactly:

```text
all Rust MirInterpreter instruction handlers
all semantic-reference fixtures and callers
all VM-only product/compat callers
available sealed MIR transports and their lossiness
backend-specific values hidden behind VMValue
```

Deliverables:

1. One machine-readable handler/caller/transport inventory.
2. One normalized human-readable summary and zero/coverage guard.
3. One selected sealed MIR input transport.
4. Exact typed fail-fast rules for unsupported or lossy transport.
5. HMI-S0 implementation packet, still with production callers zero.

## Source authority

The selected input must be an already sealed MIR transport carrying the
portable semantic facts needed by the first subset. HMI-P0 must identify the
single existing carrier and prove its exact coverage/lossiness before naming
it authority.

The following are forbidden authorities:

```text
raw Rust MirModule access from .hako
source AST
reconstructed ProgramV0
a second semantic MIR schema
Rust VMValue layout
backend helper names
runtime handler discovery
```

If no existing transport is lossless for the first subset, stop and report the
exact missing fields. Do not silently invent an adapter schema inside HMI-P0.

## First subset reserved for HMI-S0

HMI-P0 inventories, but does not activate, this already accepted subset:

```text
Const
Copy
CopyOwned
DestroyOwned
BinOp
Jump
Branch
Phi
Return
```

`ReleaseStrong` is legacy vocabulary and is not part of the portable subset.
Unsupported instructions must eventually fail before interpreter effects, with
no Rust fallback.

## Required decisions

Please decide:

1. Which existing sealed MIR transport is the sole `.hako` interpreter input?
2. What machine-readable row owns handler, caller, fixture, transport-lossiness,
   and VMValue-hidden-representation classification?
3. Which callers are semantic-reference, VM-only compatibility, or product
   callers, and what is each retirement condition?
4. What stable typed error identifies unsupported/lossy ingress before effects?
5. Confirm that HMI-P0 has execution-owner delta zero and HMI-S0 production
   callers remain zero.

## Recommended task order after selection

```text
HMI-P0-D0  decision lock for sole sealed MIR ingress and inventory schema
HMI-P0-I0  read-only machine inventory with exact coverage and caller classes
HMI-P0-G0  normalized report, drift guard, and transport-lossiness proof
HMI-S0-D0  first portable opcode subset implementation packet
```

Do not split HMI-P0 by opcode. It is one inventory/ingress authority row, not
an interpreter implementation series.

## Non-claims

```text
no .hako interpreter execution
no semantic-reference owner cutover
no Rust fallback or Rust handler retirement
no product VM revival
no parser or .hako MirBuilder migration
no BoxRef SSA-I1-O1 activation
no Ownership V2 grammar work
no broad Rust VM translation
no AST or ProgramV0 execution authority
```

## Stop conditions

Stop before implementation if HMI-P0 would:

```text
create a second MIR semantic schema
read raw Rust MirModule from .hako
use AST or ProgramV0 as interpreter authority
translate handlers before the inventory and ingress decision
activate the accepted opcode subset
add Rust fallback after an unsupported .hako result
mix product VM compatibility with semantic-reference ownership
infer transport completeness from tests instead of exact fields
open BoxRef/O1, parser, MirBuilder, ownership grammar, or another backend
touch a source/check file at or above 800 lines
```
