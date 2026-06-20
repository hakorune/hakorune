# Rust-to-Hako Lifecycle Projection Reference

Status: Reference
Scope: How Rust ownership / borrow / move / Drop information is migrated into
Hako lifecycle plans.

## Summary

The short answer is:

```text
Yes, the converter can translate Rust ownership into .hako,
but only after ownership is represented as a verified Hako lifecycle plan.
```

The converter is not the policy owner.

```text
rustc adapter:
  extracts lifecycle facts

Hako lifecycle resolver:
  chooses Hako representation / borrow / cleanup plan

Hako lifecycle verifier:
  checks the plan satisfies the Rust facts

converter / emitter:
  renders the verified plan as .hako or canonical MIR
```

This keeps the migration structural. Rust syntax is not rewritten directly into
Hako ownership behavior.

## What "Converter" Means Here

In this reference, "converter" is shorthand for the verified projection
pipeline.

```text
RustSubsetModule-v0:
  source structure

RustLifecycleFacts-v0:
  ownership / borrow / move / Drop facts

HakoLifecyclePlan-v0:
  Hako-owned representation and cleanup choice

VerifierResult:
  positive proof that the plan satisfies the facts

converter / emitter:
  renders the verified plan
```

The converter may be the final tool that prints `.hako`, but it must not choose
ownership policy from Rust syntax alone. If a lifecycle-aware conversion is
requested and no verified plan exists, the conversion must fail fast. A lossy
skeleton route may still emit TODO comments, but it cannot claim lifecycle
parity.

## Migration Granularity

Use different units for different jobs:

```text
crate:
  inventory, transport, coverage sweep

module:
  source provenance and focused materialization fixture

family:
  semantic migration, lifecycle projection, oracle parity, authority promotion
```

A crate-wide converter run is useful evidence that input transport and coverage
work. It is not enough to promote semantic authority.

Authority promotion happens family by family:

```text
1. choose one family, such as BindingContext ordered-binding behavior
2. extract rustc facts for that family
3. build a HakoLifecyclePlan
4. verify the plan
5. render .hako / canonical MIR
6. compare with Rust oracle vectors
7. switch the selfhost mainline authority for that family only
```

The Rust implementation stays as bootstrap, oracle, and explicit compatibility
route. Do not read family promotion as Rust bootstrap removal.

## Practical Migration Pipeline

```text
Rust crate
  -> HIR inventory contract
  -> selected family
  -> THIR body inventory
  -> MIR lifecycle facts
  -> HakoLifecyclePlan
  -> verifier result
  -> .hako / canonical MIR emitter
  -> Rust oracle parity
  -> family authority promotion
```

This is the practical answer to "how does the converter migrate Rust to Hako?"
The converter is developed so each layer produces a stable, testable artifact
that the next layer can consume.

## Non-Goal

Do not add Rust lifetime syntax to `.hako` as the migration model.

```text
not:
  &T / &mut T / lifetime parameters as source-level Hako syntax

instead:
  rustc-proven facts -> HakoLifecyclePlan -> verifier -> emitter
```

## Inputs

### RustSubsetModule-v0

Owns source structure:

```text
module
items
source names
emitted names
statements
expressions
unsupported handoff comments
```

It is enough for skeleton generation. It is not enough for ownership parity.

### RustLifecycleFacts-v0

Owns Rust semantic evidence:

```text
copy class
move operands
borrow kind
borrow escape
identity observation
drop obligation
initializedness
resource boundary
thread / atomic observation
deterministic iteration requirement
```

The adapter must describe Rust facts only. It must not choose Hako containers
or cleanup surfaces.

### HakoLifecyclePlan-v0

Owns selected Hako projection:

```text
Immediate
AggregateLocal
BorrowView
TransferOwned
LocalBox
OrderedMapBox
StableHandle
MutableCell
HostResource
ArcCompat
CompatShim
```

The plan is produced by Hako-side resolver logic, not by syntax spelling in the
Rust adapter.

### VerifierResult

Owns acceptance:

```text
Allow:
  plan satisfies facts

Deny:
  projection boundary is unsupported or unsafe
```

The lifecycle-aware emitter requires a positive verifier result.

## Projection Rules

### Scalars

```text
Rust scalar Copy
  -> Immediate
```

Examples:

```text
i64
bool
small id wrappers when represented as scalar contracts
```

### Aggregate Copy

```text
Rust aggregate Copy
  -> AggregateLocal
```

Only when identity, address, layout, and Drop timing are not observed.

Do not treat every `Copy` type as a value aggregate. References and pointers can
also be `Copy`, but they are aliases.

### Borrow

```text
temporary &T
  -> BorrowView(read)

temporary &mut T
  -> BorrowView(write) or direct owner update
```

Allowed only when the borrow does not escape.

Returned, stored, or owner-independent borrows require an owner-carrying plan or
must deny.

```text
returned &mut map:
  Deny(ReturnedMutableBorrow)
```

### Move

Rust move is not always ordinary assignment.

```text
pure aggregate move:
  assignment may be valid

resource / identity / Drop-sensitive move:
  TransferOwned / TakeLocal / ReplaceOwned style plan required
```

The emitter must not introduce a lingering alias that changes Rust release or
ownership timing.

### Collections

```text
Vec:
  Array-like local box / selected collection plan

HashMap:
  Map-like plan only when unordered semantics are valid

BTreeMap:
  OrderedMapBox when deterministic String-key iteration is required
```

The Rust adapter may emit:

```text
deterministic_order_required=true
```

It must not directly say:

```text
use OrderedMapBox
```

### Drop

Drop must not be erased without positive evidence.

```text
TrivialMemory:
  erase

StructuralOwned:
  target ownership or explicit cleanup plan

HostResource / semantic Drop:
  explicit cleanup / release owner

Conditional or Open Drop:
  init flags or Deny in v0
```

Do not map every Rust `Drop` implementation to `box.fini()`.

```text
scope cleanup:
  lexical release owner

box.fini():
  object-level logical finalization
```

They are not interchangeable.

### Arc / Rc

```text
shared-only Rc/Arc:
  ordinary Hako reference may be valid

weak / ptr_eq / strong_count / thread / atomic behavior observed:
  Weak / identity / ArcCompat / atomic shim required
```

Do not globally translate `Arc<T>` to one Hako shape.

## Pipeline

```text
Rust source
  ↓
rustc semantic adapter
  ↓
RustSubsetModule-v0
RustLifecycleFacts-v0
  ↓
HakoLifecycleResolver
  ↓
HakoLifecyclePlan-v0
  ↓
HakoLifecycleVerifier
  ↓
converter / emitter
  ↓
.hako source or canonical MIR
  ↓
Rust oracle semantic parity
```

## Converter Contract

The converter has two routes.

### Skeleton Route

```text
input:
  RustSubsetModule-v0

output:
  lossy .hako skeleton with Unsupported comments

claims:
  structure / parser / MIR surface only
```

It must not claim ownership, borrow, move, or Drop parity.

### Lifecycle-Aware Route

```text
input:
  RustSubsetModule-v0
  verified HakoLifecyclePlan-v0

output:
  .hako / canonical MIR with lifecycle claims
```

Missing plan or failed verification is fail-fast.

## Stop Lines

```text
do not add Rust lifetime syntax as the migration shortcut
do not let the Rust adapter choose Hako representation policy
do not infer lifecycle policy from Rust spelling alone
do not erase Drop without a positive TrivialMemory fact
do not lower returned/stored borrow as a naked alias
do not model resource moves as ordinary box assignment
do not choose OrderedMapBox inside the adapter
do not map every Arc/Rc to ordinary Hako boxes
do not claim lifecycle parity on the skeleton route
```

## First Practical Profile

The first useful profile is MirBuilder context migration:

```text
included:
  safe Rust
  concrete call targets
  single-thread execution
  memory-only Drop erase
  BTreeMap deterministic iteration via plan
  Option / Result surface

excluded:
  unsafe / raw pointers
  FFI
  true parallel execution
  observable Arc counts
  panic unwind parity
  open Drop
  returned mutable borrow as naked alias
```

Good first targets:

```text
BindingContext:
  BTreeMap -> OrderedMapBox plan
  &self -> read BorrowView / direct read
  &mut self -> owner method mutation
  memory-only Drop -> erase

VariableContext:
  simple map operations first
  returned mutable map borrow remains denied
```

## Owners

```text
Rust adapter:
  facts only

Hako resolver:
  plan selection

Verifier:
  correctness gate

Converter/emitter:
  rendering only
```

If a change is not clearly in one of these owners, stop and add a design card
before implementation.
