# Rust Lifecycle Projection SSOT

Status: SSOT
Scope: Rust semantic migration facts to Hako lifecycle plans.

## Decision

Hakorune does not import Rust lifetime / borrow syntax into `.hako` source as
the migration model.

Instead, Rust lifecycle information is handled as facts and projected into
Hako lifecycle plans:

```text
rustc semantic adapter:
  emits Rust facts only

Hako lifecycle resolver:
  chooses Hako representation / lifecycle plan

Hako lifecycle verifier:
  checks the plan satisfies the Rust facts

emitter:
  emits only verified .hako / canonical MIR
  SSOT: docs/development/current/main/design/rust-to-hako-lifecycle-emitter-contract.md
```

The converter/emitter must not infer ownership from syntax by itself.
Operational reference for implementers:

```text
docs/development/current/main/design/rust-to-hako-ownership-converter-reference.md
docs/reference/architecture/rust-to-hako-lifecycle-projection.md
```

Short form:

```text
correct:
  converter emits .hako from a verified HakoLifecyclePlan

incorrect:
  converter sees Rust syntax and directly chooses ownership semantics
```

This is the key boundary. The long-term goal is still to generate useful
`.hako`, but ownership is not a text-rewrite rule. Ownership is a projection
from rustc-proven facts into a Hako-owned plan.

## Short Answer

The migration goal can be described as:

```text
converterで所有権などをうまく.hakoに変換する
```

but only with this ownership split:

```text
correct:
  rustc adapter proves lifecycle facts
  Hako resolver chooses the Hako lifecycle plan
  verifier checks the plan
  converter/emitter prints the verified plan as .hako / canonical MIR

incorrect:
  converter reads Rust syntax and directly decides ownership / borrow / Drop
```

So the converter is the final emission surface, not the policy owner.
It may become good enough to generate mostly useful `.hako` automatically, but
the safety contract comes from facts + plan + verifier, not from textual
pattern replacement.

## Converter Meaning

When we say "the converter translates Rust ownership into `.hako`", the word
`converter` means the whole verified projection pipeline, not one direct text
rewriter.

```text
accepted meaning:
  RustSubsetModule-v0
  + RustLifecycleFacts-v0
  + verified HakoLifecyclePlan-v0
  -> .hako / canonical MIR

rejected meaning:
  Rust syntax
  -> converter guesses ownership / borrow / Drop policy
  -> .hako
```

The practical implementation has four roles:

```text
external rustc adapter:
  produce Rust facts only

Hako lifecycle resolver:
  choose representation / borrow / cleanup plan

Hako lifecycle verifier:
  allow only plans that satisfy the facts

converter / emitter:
  render the verified plan
```

If a task needs executable ownership parity and no verified lifecycle plan is
available, the converter must fail fast. The existing skeleton route may still
emit TODO comments, but that route must not claim ownership, borrow, move, or
Drop parity.

## Operational Answer

When a task asks whether the converter can translate Rust ownership into
`.hako`, answer with this boundary:

```text
Yes, but only after ownership is expressed as verified lifecycle plans.

Rust syntax:
  not enough

rustc facts:
  required input

HakoLifecyclePlan:
  required owner of representation / cleanup / borrow choices

converter/emitter:
  renders verified plans only
```

This means a future ownership-aware converter is a two-input renderer:

```text
RustSubsetModule-v0:
  source structure and skeleton shape

Verified HakoLifecyclePlan-v0:
  ownership / borrow / move / Drop projection
```

The converter must fail fast when a lifecycle plan is missing for a semantic
claim. It may still emit lossy skeleton TODOs on the existing skeleton route,
but that route must not claim ownership, borrow, or Drop parity.

## Pipeline

```text
Rust source
  ↓
rustc semantic adapter
  ↓
ResolvedRustStructure
RustLifecycleFacts
  ↓
HakoLifecycleResolver
  ↓
Allow(HakoLifecyclePlan)
or Deny(ProjectionDenyReason)
  ↓
HakoLifecycleVerifier
  ↓
.hako emitter
  ↓
canonical MIR
  ↓
Rust oracle semantic parity
```

Keep the existing skeleton lane separate:

```text
RustSubsetModule-v0:
  structure / skeleton transport

RustLifecycleFacts-v0:
  semantic migration sidecar
  SSOT: docs/development/current/main/design/rust-lifecycle-facts-vocab-v0.md

HakoLifecyclePlan-v0:
  Hako-owned projection result
  SSOT: docs/development/current/main/design/hako-lifecycle-plan-vocab-v0.md
```

## Ownership Boundary

### Rust Semantic Adapter

Owns Rust facts:

```text
item identity
module path
concrete type
copy/move operand class
borrow kind and escape
drop obligation
initializedness
identity observation
thread / atomic observation
resource boundary
```

Does not own Hako representation policy.

It may provide evidence such as:

```text
deterministic_order_required=true
identity_observed=false
borrow_escapes=false
drop_observable=false
thread_atomic_observed=false
```

It must not emit:

```text
use OrderedMapBox
use ArcCompat
erase Drop
make this a record
make this a box
```

### Hako Lifecycle Resolver

Owns Hako plan selection:

```text
Immediate
AggregateLocal
BorrowView
LocalBox
StableHandle
MutableCell
OrderedMapBox
HostResource
ArcCompat
CompatShim
```

### Verifier

Owns projection validity:

```text
escape
alias
cleanup
publication
identity
initializedness
concurrency
drop ownership
```

### Converter / Emitter

Owns text / canonical MIR emission only:

```text
input:
  verified HakoLifecyclePlan

output:
  .hako skeleton/source
  or canonical MIR
```

The emitter does not re-run escape, ownership, drop, or borrow decisions.

Allowed responsibilities:

```text
render record / box / function / method text
render plan-selected cleanup / birth / field initializer shape
render verified BorrowView / TransferOwned lowering surface
preserve diagnostics and source provenance
fail-fast when a verified plan is missing
```

Forbidden responsibilities:

```text
choose record vs box from Rust syntax alone
choose OrderedMapBox because the adapter saw BTreeMap
erase Drop because a Rust value looks memory-only
turn &mut into direct mutation without non-escape proof
turn Arc/Rc into ordinary boxes without observation facts
invent fallback ownership when facts are unknown
```

## Projection Categories

```text
Rust scalar Copy:
  Hako immediate copy

Rust aggregate Copy:
  Hako record / aggregate local copy
  only if identity, address, and layout are not observed

Rust reference / pointer Copy:
  shared alias / BorrowView
  not record copy

Rust unique owned value:
  aggregate local or local box

Rust owned identity value:
  unpublished local box

Rust move:
  TransferOwned / TakeLocal / ReplaceOwned / ConsumeArgument plan

temporary &T:
  direct read or BorrowView when non-escaping

temporary &mut T:
  direct local / field / container update when non-escaping

returned or stored borrow:
  owner-carrying BorrowView or Deny

Cell / RefCell:
  MutableCell shim

Vec / Map / BTreeMap:
  Array / Map / OrderedMap local box

Weak:
  Hako weak reference

trivial memory Drop:
  erase only with positive fact

structural Drop:
  target ownership or generated cleanup

custom/resource Drop:
  scope cleanup or HostHandle release

conditional/open Drop:
  initialized flags or Deny in v0

shared-only Rc / Arc:
  Hako box reference if count, weak, ptr_eq, atomics, Send/Sync are not observed

atomic/shared Arc:
  ArcCompat / atomic shim

Pin / self-reference:
  pinned host shim or Deny

raw / unsafe / layout-sensitive:
  explicit shim or Deny
```

## Required Hako Primitives

Use existing primitives first:

```text
record
box
stored field initializer
birth
cleanup
object fini()
weak
Result / Option
local-first unpublished object
publication facts
HostHandle boundary
OrderedMapBox
```

Prefer internal plan vocabulary over source syntax:

```text
TransferOwned:
  move / consume / replace plan

BorrowView:
  non-owning verifier-owned carrier

MutableCellBox:
  Cell / RefCell style shared mutation

OwnedResource:
  valid / release / released / cleanup registration

ConditionalCleanupState:
  initialized whole / field flags for later OpenDrop support

ArcCompat:
  only when Arc behavior is observed
```

## Adapter Fact Sources

Recommended inputs:

```text
HIR:
  item/module/type identity, visibility, attributes, unsafe boundary

THIR:
  typed structured body, resolved method/operator calls,
  auto-ref/deref, pattern binding, destruction scope

MIR + borrowck:
  Place, Operand copy/move, borrow kind/scope, initialization,
  moved path, concrete call target, CFG

drop-elaborated MIR:
  Static / Dead / Conditional / Open drop class,
  drop path, glue target, field obligations

Instance graph:
  concrete generic arguments, selected impl, drop glue,
  monomorphized call target
```

Do not use raw rustc debug dumps as the stable handoff schema.

## First Pilot

Selected first practical pilot:

```text
MIRBUILDER-BINDING-CONTEXT-LIFECYCLE-PILOT-001
```

Reason:

```text
BTreeMap<String, BindingId>
&self read methods
&mut self mutation methods
memory-only Drop
no Arc
no unsafe
no FFI
direct MirBuilder migration value
```

Expected projection:

```text
Rust BTreeMap:
  OrderedMapBox

Rust &self:
  direct read / BorrowView

Rust &mut self:
  owner method mutation

Rust BindingId Copy:
  scalar / record value copy

Rust memory-only Drop:
  erase
```

## Task Decomposition

```text
RUST-LIFECYCLE-PROJECTION-SSOT-001:
  document adapter / resolver / verifier / emitter ownership boundaries
  document the converter-as-emitter answer explicitly

RUST-LIFECYCLE-FACTS-VOCAB-000:
  passive schema vocabulary for RustLifecycleFacts-v0
  includes CopyKind / MoveKind / BorrowFact / DropFact / EscapeFact
  no conversion behavior

HAKO-LIFECYCLE-PLAN-VOCAB-000:
  passive schema vocabulary for HakoLifecyclePlan-v0
  includes Immediate / AggregateLocal / BorrowView / TransferOwned /
  LocalBox / OrderedMapBox / HostResource / CompatShim
  no emitter behavior

RUST-TO-HAKO-LIFECYCLE-EMITTER-CONTRACT-000:
  passive emitter contract for rendering verified lifecycle plans
  converter direct ownership policy remains forbidden

MIRBUILDER-BINDING-CONTEXT-LIFECYCLE-PILOT-001:
  BindingContext facts + plan + verifier evidence
  OrderedMapBox projection
  memory-only Drop erase only with positive fact

MIRBUILDER-BINDING-CONTEXT-LIFECYCLE-ORACLE-PARITY-001:
  compare Hako result against Rust oracle vectors
  promote only the BindingContext family when green
```

Do not start the pilot until the current crate-bundle transport milestone is
closed. The lifecycle lane consumes crate transport evidence; it does not
replace the crate-bundle input-route work.

## Current Follow-up Task Queue

The lifecycle lane now has enough passive vocabulary and focused fixture
evidence to proceed by named owners. Keep these as separate rows; do not merge
them into a general converter rewrite.

```text
PHI-CARRIER-LIFECYCLE-CONSUMER-INVENTORY-001:
  inventory join_id / promoted_body_locals / trim_helper consumers before a
  general resolver is introduced
  implementation_started=0

HAKO-LIFECYCLE-RESOLVER-READONLY-SKELETON-001:
  consume frozen RustLifecycleFacts + HakoLifecyclePlan fixtures
  produce Allow/Deny diagnostics only
  converter_emission_added=0

LIFECYCLE-VERIFIER-RESULT-VOCAB-000:
  define the positive verifier result required by the emitter contract
  no emission behavior

RUST-TO-HAKO-LIFECYCLE-EMITTER-PROBE-001:
  render one verified BindingContext/VariableContext plan slice
  no direct Rust syntax ownership decisions

RUSTC-SEMIR-LIFECYCLE-FACTS-ADAPTER-PROBE-001:
  later external-adapter probe for producing lifecycle facts from rustc
  do not use raw rustc dumps as stable schema
```

## Ownership-Aware Converter Task Queue

Use this queue when the next task is specifically "make the converter handle
Rust ownership well".

```text
RUST-TO-HAKO-OWNERSHIP-CONVERTER-TASK-SEQUENCE-001:
  document the converter as a verified-plan renderer
  choose the next concrete converter/emitter row
  implementation_started=0

RUST-TO-HAKO-CONVERTER-TWO-INPUT-BOUNDARY-001:
  define the converter input as RustSubsetModule-v0 plus a verified
  HakoLifecyclePlan-v0
  lifecycle_claim_without_verified_plan=fail_fast
  skeleton_todo_route_still_allowed=1

RUST-TO-HAKO-LIFECYCLE-EMITTER-SURFACE-001:
  render one existing verified plan fixture into `.hako`
  no direct Rust syntax ownership decisions

RUST-TO-HAKO-LIFECYCLE-PARITY-GATE-001:
  compare emitted `.hako` / canonical MIR against the Rust oracle for the
  selected family only
  no crate-wide executable parity claim

RUSTC-SEMIR-LIFECYCLE-FACTS-ADAPTER-PROBE-001:
  later external adapter probe
  rustc facts only
  Hako representation policy remains resolver-owned
```

Selection rule:

```text
If a plan consumer is not named yet:
  inventory first

If facts and plan exist but no verifier result exists:
  add verifier vocabulary before emitter work

If verifier is missing or negative:
  converter/emitter must fail fast
```

## Stop Lines

```text
do not add Rust lifetime syntax to Hako
do not let the Rust adapter choose Hako representation policy
do not use raw rustc MIR/THIR dump as stable handoff schema
do not erase Drop without positive TrivialMemory fact
do not map every Drop impl to box.fini()
do not collapse Copy, Clone, Share, Borrow, and Move into assignment
do not model resource or identity moves as ordinary box aliasing
do not return or store a naked borrowed alias without an owner-carrying view
do not map interior mutability to ordinary &mut-style direct access
do not classify Array/Map/OrderedMap as identity-free record values
do not infer publication or escape legality from type/helper names
do not turn unknown lifecycle facts into ordinary box fallback
do not generate both scope cleanup and object fini for the same release owner
do not mix panic=abort and unwind semantics in one profile
do not auto-project Pin, MaybeUninit, ManuallyDrop, union, raw pointer,
repr(C), repr(packed), inline asm, or layout-observing code
do not map Arc to ordinary box when count, weak, ptr_eq, atomics,
Send/Sync, or cross-thread behavior is observed
do not claim crate-wide executable parity before lifecycle oracle tests pass
```

## Migration Order

```text
1. Close current crate-bundle transport milestone.
2. Add passive RustLifecycleFacts / HakoLifecyclePlan vocabulary.
3. Build BindingContext lifecycle pilot.
4. Compare against Rust oracle vectors.
5. Promote one Hako authority family only after verifier evidence.
```
