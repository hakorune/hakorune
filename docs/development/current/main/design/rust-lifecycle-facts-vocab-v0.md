# Rust Lifecycle Facts Vocabulary v0

Status: SSOT
Scope: Passive Rust-side lifecycle facts for Rust-to-Hako migration.

## Purpose

This vocabulary names the facts emitted by a future rustc semantic adapter.
It does not choose Hako representation policy and does not emit `.hako`.

Pipeline position:

```text
rustc semantic adapter
  -> RustLifecycleFacts-v0
  -> HakoLifecycleResolver
  -> HakoLifecyclePlan-v0
```

## Non-Goals

```text
HakoLifecyclePlan vocabulary
converter emission
BindingContext lifecycle pilot
Rust lifetime syntax
Rust name/use resolution
runtime behavior
```

## CopyKind

```text
ImmediateValue:
  scalar values such as integer / bool / small immediate values

AggregateValue:
  copyable aggregate value with identity/address/layout not observed

SharedAlias:
  reference-like or pointer-like Copy value that preserves alias semantics

CallablePointer:
  function item / function pointer / callable carrier

RawAddress:
  raw pointer or address-like value; v0 resolver should normally Deny
```

Rule:

```text
CopyKind is not representation policy.
SharedAlias and RawAddress must not be treated as record copies.
```

## MoveKind

```text
TransferOwned:
  ownership moves to a new owner

TakeLocal:
  local value is taken and source becomes unavailable

ReplaceOwned:
  destination is overwritten and previous owner/drop obligation matters

ConsumeArgument:
  call consumes the argument by value
```

Rule:

```text
MoveKind records Rust move semantics.
It does not imply plain Hako assignment is valid.
```

## BorrowFact

```text
SharedRead:
  non-mutating borrow

UniqueWrite:
  exclusive mutable borrow

scope:
  CallOnly | LexicalScope | Returned | Stored | Unknown

escapes:
  true | false | unknown
```

Rule:

```text
Returned / Stored / Unknown borrow escape is not a naked alias.
The resolver must choose an owner-carrying view or Deny.
```

## DropFact

```text
TrivialMemory:
  no observable Drop behavior; may be erased only as a positive fact

StructuralOwned:
  field/owned-value cleanup obligations exist

CustomSemanticDrop:
  user-visible Drop behavior exists

HostResource:
  external resource release obligation exists

Conditional:
  initializedness flag is required

Open:
  field/path-level initializedness is required; v0 may Deny
```

Rule:

```text
Drop is never erased from absence of evidence.
Only TrivialMemory permits erase.
```

## EscapeFact

```text
LocalOnly:
  value stays inside the local function/body scope

Returned:
  value leaves through return

Stored:
  value is stored into another object/container/global

HostBoundary:
  value crosses plugin/extern/host handle boundary

ThreadBoundary:
  value crosses task/thread/channel boundary

Unknown:
  escape state is not proven
```

Rule:

```text
Unknown escape is not fallback-to-box.
Unknown escape is Deny or explicit compat plan in later rows.
```

## ObservationFact

```text
identity_observed:
  pointer/object identity is semantically observed

address_observed:
  address or pointer value is observed

layout_observed:
  repr/layout/field offset is observed

arc_count_observed:
  strong/weak count behavior is observed

weak_observed:
  weak upgrade/failure behavior is observed

atomic_observed:
  atomic ordering or cross-thread sharing is observed
```

Rule:

```text
Observation facts restrict projection.
For example, Arc without count/weak/atomic/thread observation may project
differently from Arc with those observations.
```

## Minimal Record Shape

The exact serialization can change later, but v0 consumers should preserve
these fields:

```text
fact_subject:
  function/item/local/place stable id

copy_kind:
  CopyKind or NonCopy

move_kind:
  optional MoveKind per move/consume site

borrow_facts:
  list of BorrowFact

drop_fact:
  DropFact

escape_fact:
  EscapeFact

observation_facts:
  ObservationFact flags

source_context:
  module/source span diagnostics only
```

## Stop Lines

```text
do not choose Hako representation here
do not emit .hako here
do not add HakoLifecyclePlan here
do not treat unknown facts as ordinary box fallback
do not erase Drop without TrivialMemory
do not collapse Copy / Move / Borrow into assignment
```
