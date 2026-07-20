# Binding-SSA-First Canonical Control Lowering SSOT

Status: Accepted
Decision: D′ — SSA-first, control-contract-preserving, function-owner-atomic
Date: 2026-07-14
Implementation taskboard:
`../investigations/mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md`

JoinIR/CorePlan succession, normalized parity, diagnostic rehome, and physical
retirement child task:
`../investigations/mirbuilder-joinir-dprime-succession-task-2026-07-20.md`

## Purpose

This document is the long-lived architecture authority for canonical mutable
local lowering. The implementation taskboard owns milestone order, fixtures,
and the current blocker. If historical RegionFlow, If, Loop, or task documents
conflict with this document, this document wins for future canonical work.

The final rule is:

> Control contracts decide where execution may go. One function-owned Binding
> SSA builder decides which `ValueId` reaches each local `BindingRefV1` use and
> which PHIs are required.

Canonical syntax families must not precompute carrier lists, branch write
sets, join-source rows, or name-keyed final value maps as a second PHI
authority.

## Final architecture

```text
canonical syntax
+ VerifiedResolvedFunctionV1
        │
        ├─ exact lexical/control identity
        │    BindingRefV1 / ScopeId / RegionId / exact targets
        │
        ├─ VerifiedLocated*ControlV1
        │    exact source coverage
        │    family topology and typed ports
        │    cleanup obligations
        │    no ValueId / BasicBlockId / binding-effect rows
        │
        ▼
CanonicalSsaFunctionLowererV2
  ResolvedIdentityLedgerV2
  ResolvedSemanticStackV1
  CanonicalCfgSessionV1
  BindingSsaBuilderV1
        │
        ▼
verified MIR SSA
        │
        ▼
optional post-MIR derived analysis
```

There is exactly one `BindingSsaBuilderV1` per canonical function session.
If, Loop, nested control, and straight-line lowering all use that instance.

## Authority boundaries

### Resolver and verified semantics

Own:

```text
BindingRefV1 declaration/use/assignment identity
ScopeId and RegionId topology
exact Break/Continue/Return/Throw targets
assignment target kind
owner and source provenance
```

Do not own:

```text
ValueId or BasicBlockId
PHI placement
loop carriers
branch merge values
MIR predecessor state
```

### Located control contracts

Own:

```text
exact structural source coverage
family-specific region topology
reachable typed control ports
exact target RegionId
ordered cleanup obligations
unsupported-family preflight
```

Do not own:

```text
may_rebind_outer
carrier rows
If join-source matrices
ValueId / BasicBlockId
PHI rows
name-keyed state
```

Coverage may use a reusable private sidecar schema, but a verified family
product and its coverage are co-sealed and cannot be recombined by Lower.

### Canonical CFG session

MIR terminators are the actual CFG truth. Cached predecessor lists are checked
derivatives, not an independently mutable graph.

All canonical edge and seal operations pass through one fallible facade:

```text
emit edge:
  validate target and open state
  emit terminator
  update cached witness atomically

seal block:
  derive predecessors from terminators
  verify cached equality
  freeze the predecessor set

after seal:
  adding a predecessor is a typed error
```

`compute_predecessors()` over cached successors is not a terminator-truth
witness. Canonical seal derives the set directly from terminators. PHI
analysis and input materialization must not call `update_cfg()` or otherwise
repair the graph as a side effect.

### Binding SSA

Own:

```text
(BasicBlockId, BindingRefV1) reaching definitions
recursive predecessor reads
incomplete PHIs for open blocks
PHI completion after block seal
missing-definition and unfinished-state errors
```

Do not accept or inspect:

```text
AST nodes
SourceSite or Span
ScopeId or RegionId
names
control-family policy
```

The minimum conceptual API is:

```rust
define(binding, block, value)
read(binding, block)
seal(block, verified_predecessors)
finish()
```

`read` follows standard sealed-block SSA construction:

```text
local definition exists:
  return it

block is open:
  define a provisional PHI first
  record it as incomplete

sealed block with one predecessor:
  recursively read that predecessor

sealed block with multiple predecessors:
  define a provisional PHI before recursive reads
  collect one input per exact predecessor
  patch the PHI

sealed block with no definition path:
  typed missing-definition error
```

A block first read only after single-predecessor seal forwards directly and
does not create a PHI. If an open-block read already created a Defined
provisional PHI, sealing it with one predecessor may keep that valid
single-input PHI until the later generic simplifier.

Reserve-only values are never published as binding definitions. Provisional
PHIs must be Defined before recursive exposure, and all insertion, patch, and
rollback operations use the repository PHI lifecycle SSOT.

### Optimizer analysis

Loop carriers, induction variables, recurrences, and invariants are derived
from completed MIR only when a named consumer exists. Such results are
invalidated by MIR/CFG mutation and never influence source route selection.

If a future structured-loop IR requires explicit `iter_args` and yields, that
is a separate IR decision. It must not coexist with generic CFG SSA as a
second baseline value authority.

## Identity, lifetime, and storage classes

`ResolvedIdentityLedgerV2` owns exact source claims and lexical activity.
`BindingSsaBuilderV1` owns reaching values. Scope exit retires lexical access
but does not erase historical SSA definitions needed by predecessor reads.

Only owner-local mutable bindings enter Binding SSA:

```text
local/parameter/receiver binding rebind:
  Binding SSA define/read

captured-by-reference or Upvar state:
  cell/capture owner, or preflight reject until designed

field/index writes:
  heap/place owner, never a local Binding SSA definition
```

Same-name shadows need no value-map snapshot or restoration because their
`BindingRefV1` identities differ.

Assignment ownership cleanup reads the old value through Binding SSA before
the new definition is installed. Scope-exit cleanup also reads the current
SSA value. RC behavior is part of each atomic production gate.

Ownership SSA is distinct from Binding SSA:

```text
Binding SSA:
  which ValueId reaches a BindingRef use

MirOwnershipKindV1:
  None | Borrowed | Owned discipline for that MIR ValueId
```

The portable ownership pair is:

```text
CopyOwned(dst, src):
  non-consuming src use; fresh independently consumable Owned dst

DestroyOwned(value):
  exactly one consuming use of the named Owned value
```

Ordinary `Copy` remains ownership-neutral. Legacy `ReleaseStrong` is not the
canonical consume operation and its meaning is not changed during migration.
It is isolated, guarded to zero canonical callers, and retired only after
repository-wide exact caller zero.

Owned PHI input selection and Return forward one ownership token rather than
create an implicit copy. At each finite reachable function exit, every Owned
value has exactly one consuming or forwarding disposition; a non-terminating
path may keep its token live. `Copy` on an Owned value, duplicate consume, use
after consume, and missing exit disposition are canonical verification
failures.

V1 seals this classification and its path-sensitive dispositions as one
owner-branded `VerifiedOwnershipSsaV1` consumed by verification and supported
backends. It is value-lifetime metadata, not a second
`BindingRef -> ValueId` authority. Canonical V1 requires edge arguments to be
absent; `Phi.inputs` is the sole owned edge-transfer vocabulary. Borrowed V1
roots exist only at a sealed function ABI boundary and cannot escape through
Phi/Return without `CopyOwned`.

The first executable representation profile is deliberately closed:

```text
BoxRef:
  CopyOwned / DestroyOwned allowed after exact static proof

InlineI64 / InlineBool / InlineF64:
  None; reuse ValueId and emit no ownership instruction

BorrowedText / Array / Future / WeakRef / Void / Opaque / Unknown:
  reject before Builder effects
```

Before scope close is activated, the pure plan distinguishes self-assignment,
owned temporary transfer, a borrowed strong alias, and a BlockExpr tail/current
value that escapes the closing scope. It materializes the next value before
destroying the previous value and destroys remaining locals in reverse source
declaration order. Error cleanup of an unpublished draft restores compiler
state but does not emit runtime ownership cleanup for discarded code.

## Open PHI facts

An incomplete Loop PHI has unknown incoming facts. A concrete type,
representation, origin, or optimization fact must not be inferred from only
the entry input.

Allowed:

```text
conservative unknown/open facts
generic operations valid for the conservative fact
fact join/refinement after all inputs are patched
```

Forbidden:

```text
entry-only concrete inference
route selection from an open PHI
representation-sensitive lowering without a separate proof
```

Unsupported cases fail before Builder effects until the required fact contract
is independently accepted.

## Structured nesting

If and Loop do not become one universal control node. They keep small,
family-specific CFG boxes over the same function SSA and CFG substrate.

```text
If in If:
  inner merge definitions feed the outer branch

If in Loop:
  inner merge definitions feed the Loop latch/backedge

Loop in If:
  inner after definitions feed the outer branch merge

Loop in Loop:
  inner after definitions feed the outer latch/backedge
```

Nesting does not pass effect summaries between families. Exact RegionId
targets route nonlocal exits through transaction-local role tables. No loop
depth lookup, AST search, carrier payload, or durable scalar
`RegionId -> BasicBlockId` map is allowed.

Each nesting shape and each new port family lands independently. Only after
the bounded depth-independent witness may the supported grammar claim general
finite nesting under the same rules.

## Exit and cleanup law

SSA decides values on the emitted CFG; it does not decide cleanup semantics.
Before Continue, Break, Return, QMark, Throw, or Try/Finally is activated, a
pre-Builder product must close:

```text
exact target
reachable typed port kind
ordered crossed-scope cleanup obligations
unreachable source disposition
```

Source declaration coverage distinguishes at least:

```text
Materialized
SkippedAfterTerminator
OwnedByChildFunction
```

Unreachable declarations are accounted for, not forced into `ValueId`s.
Unsupported exit kinds are absent from the accepted V1 type and fail preflight;
they are never represented by partial booleans or optional fallback fields.

## Atomic cutover law

Production activation is by whole canonical function owner. The first cutover
must switch together:

```text
receiver/parameter/local/Outbox declarations
variable reads
binding assignments
old-value and scope-exit RC reads
straight-line statements and BlockExpr
statement If
function finish and publication
```

Forbidden intermediate states:

```text
Loop uses SSA while If or straight-line code uses a flat value map
If alone uses SSA while other binding operations bypass it
an old-environment/SSA synchronization bridge
Option<BindingSsaBuilder> or a recursive mode boolean
canonical failure followed by legacy If/Loop/CorePlan retry
```

The old If effect/join products may remain a temporary production oracle until
the atomic owner cutover. After the cutover they have zero production callers
and are physically retired in a separate behavior-neutral slice.

## Finish and publication law

Whole-function PHI repair is not SSA completion. A canonical function cannot
depend on post-publication missing-input fabrication, CFG repair, or unused-PHI
pruning to become valid.

Publication order is strict:

```text
source/control/identity coverage complete
all blocks sealed from terminator-derived predecessors
Binding SSA finish with incomplete PHIs = 0
typed values and semantic stacks verified
function draft finalized and caller context restored
candidate module finalized
RC insertion/validation complete
CFG + SSA + dominance + RC + MIR verification green
module session commit
```

A verifier failure is a typed compilation failure, not a successful compile
result carrying `verification_result = Err`. Duplicate same-name canonical
function publication is also a typed failure; silent replacement is forbidden.

The currently accepted completion contract includes both a function-root
final explicit Return and implicit fallthrough completion. Exact target,
cleanup obligations (including an explicit empty set), and unreachable-source
disposition are sealed before either form participates in an SSA cutover.

## Physical boundary

```text
src/mir/resolved_control_flow/
  README.md
  source_coverage.rs
  function_control.rs
  if_control.rs
  loop_control.rs
  cleanup.rs

src/mir/builder/ssa/binding/
  mod.rs
  error.rs
  state.rs
  read.rs
  phi.rs
  tests.rs

src/mir/builder/resolved_lowering/
  identity_ledger.rs
  canonical_cfg/
  active_control_targets.rs
  located_if.rs
  located_loop.rs
  semantic_stack.rs
```

`resolved_control_flow` is the future control-only authority.
`resolved_region_flow` remains isolated only while the old production If path
needs it, then retires after caller-zero proof.

New or modified source/check files stay below 800 lines.
`src/mir/builder/ssa/phi_input_materializer.rs` is already above the boundary,
so D′ requires one independent behavior-neutral physical split before CFG,
PHI-transaction, or Binding SSA implementation. Do not mix that split with a
semantic acceptance row.

## Final completion condition

The canonical-source architecture is complete when:

```text
one Binding SSA session owns all owner-local BindingRef reaching values
all canonical CFG edges use one late-edge-safe facade
pre-Builder products own only source/control/cleanup semantics
If, Loop, nesting, and accepted exact exits use family boxes over that SSA
all supported source owners cut over atomically with no fallback
old canonical effect/carrier/manual-PHI callers are zero
remaining explicit legacy mechanisms are confined to LegacyModuleLoweringInputV1
global physical deletion occurs only after repository-wide caller zero
publication requires coverage, seal, SSA, CFG, RC, and MIR verification
optional loop optimization facts are post-MIR and consumer-gated
```

This decision does not claim ProgramV0 source authority, REPL owner lifetime,
Lambda capture/cell layout, QMark/Throw/Try/Finally support, durable SA4 region
materialization, default-route cutover, or Hako Lower parity.
