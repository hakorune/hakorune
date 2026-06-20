# Rust-to-Hako Ownership Converter Reference

Status: reference
Scope: How the RustSubset converter may participate in ownership-aware
Rust-to-Hako migration.

## Short Answer

It is acceptable to say:

```text
converterでRustの所有権などを.hakoに変換する
```

but the precise contract is:

```text
rustc semantic adapter:
  proves lifecycle facts

Hako lifecycle resolver:
  chooses the Hako lifecycle plan

Hako lifecycle verifier:
  accepts or rejects that plan

converter / emitter:
  renders only a verified plan
```

The converter is an emission surface. It is not the ownership policy owner.

## Two Converter Routes

### Skeleton Route

Input:

```text
RustSubsetModule-v0
```

Allowed:

```text
emit structural .hako skeletons
emit TODO comments for known Unsupported nodes
preserve source_name / emitted_name / module provenance
```

Forbidden:

```text
claim ownership parity
claim borrow parity
claim Drop parity
choose record/box/cleanup policy from Rust syntax
```

### Lifecycle-Aware Route

Input:

```text
RustSubsetModule-v0
RustLifecycleFacts-v0
HakoLifecyclePlan-v0
VerifierResult
```

Allowed:

```text
render verified record / box / BorrowView / TransferOwned / cleanup surfaces
render OrderedMapBox only when selected by the verified plan
fail fast when a required plan or verification is missing
```

Forbidden:

```text
invent lifecycle fallback
erase Drop without TrivialMemory evidence
turn &mut into mutation without non-escape proof
turn returned/stored borrow into naked alias
map Arc/Rc to ordinary box without observation facts
```

## Required Ownership Split

The Rust adapter may report facts:

```text
copy_class=AggregateValue
borrow_kind=Mutable
borrow_escapes=false
drop_class=TrivialMemory
deterministic_order_required=true
identity_observed=false
thread_atomic_observed=false
```

The Rust adapter must not report target policy:

```text
use OrderedMapBox
make this a box
erase Drop
use ArcCompat
```

The Hako lifecycle resolver owns those target decisions.

## Fail-Fast Rules

```text
missing_lifecycle_facts:
  lifecycle-aware route fails fast

missing_hako_lifecycle_plan:
  lifecycle-aware route fails fast

negative_verifier_result:
  lifecycle-aware route fails fast

unsupported_plan_kind:
  emitter fails fast with plan kind and source context
```

The skeleton route may still produce TODO output, but it must remain explicit
that it is not a lifecycle-preserving migration route.

## Practical Task Order

```text
1. Keep RustSubsetModule-v0 as the structure/skeleton input.
2. Add rustc-derived lifecycle facts as a sidecar, not inside skeleton JSON.
3. Let the Hako lifecycle resolver produce HakoLifecyclePlan-v0.
4. Require VerifierResult before lifecycle-aware emission.
5. Render only verified plans.
6. Compare generated canonical MIR with the Rust oracle for the selected family.
```

## Current Pilot Boundary

The current lifecycle lane uses narrow owner families:

```text
BindingContext:
  completed as OrderedMapBox + memory-only Drop erase

VariableContext simple map:
  completed without returned mutable borrow

CarrierInfo / PHI carrier:
  being split into named lifecycle owners
```

Do not skip from this state to a whole-crate automatic ownership converter.

## Stop Lines

```text
do not add Rust lifetime syntax to .hako for this route
do not make converter_core choose lifecycle policy
do not mix skeleton TODO emission with lifecycle-preserving claims
do not use raw rustc debug dumps as stable facts
do not promote a family without oracle parity
```

