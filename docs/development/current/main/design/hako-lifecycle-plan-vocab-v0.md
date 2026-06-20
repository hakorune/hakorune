# Hako Lifecycle Plan Vocabulary v0

Status: SSOT
Scope: Passive Hako-side lifecycle plan vocabulary for Rust-to-Hako migration.

## Purpose

This vocabulary names the plans selected by a future HakoLifecycleResolver from
`RustLifecycleFacts-v0`.

It does not implement the resolver, verifier, converter emitter, or
BindingContext pilot.

Pipeline position:

```text
RustLifecycleFacts-v0
  -> HakoLifecycleResolver
  -> HakoLifecyclePlan-v0
  -> HakoLifecycleVerifier
  -> converter/emitter
```

## Non-Goals

```text
Rust fact vocabulary
resolver implementation
verifier implementation
converter emission
BindingContext lifecycle pilot
Rust lifetime syntax
runtime behavior
```

## Immediate

Use for scalar immediate values whose copy / move behavior is value-only.

```text
input examples:
  integer
  bool
  small enum-like immediate where identity is not observed

requirements:
  copy_kind=ImmediateValue
  identity_observed=false
  address_observed=false
  layout_observed=false
```

## AggregateLocal

Use for aggregate values that can remain identity-free local data.

```text
requirements:
  copy_kind=AggregateValue or NonCopyOwned
  escape=LocalOnly
  identity_observed=false
  address_observed=false
  layout_observed=false
  drop=TrivialMemory or verifier-approved structural cleanup
```

Stop line:

```text
do_not_project_containers_or_identity_objects_as_AggregateLocal=1
```

## BorrowView

Use for non-owning access that remains tied to a real owner.

```text
fields:
  base_owner
  access=read|write
  scope=CallOnly|LexicalScope|Returned|Stored
  escapes=true|false
```

Rules:

```text
non_escaping_shared_read:
  may become direct read or BorrowView

non_escaping_unique_write:
  may become direct owner mutation or BorrowView

returned_or_stored_borrow:
  must be owner-carrying BorrowView or Deny
```

## TransferOwned

Use for move-like ownership transfer where ordinary assignment may not preserve
Rust lifetime/drop semantics.

```text
shapes:
  TransferOwned
  TakeLocal
  ReplaceOwned
  ConsumeArgument
```

Rules:

```text
resource_or_identity_move:
  TransferOwned required

pure_immediate_move:
  may lower as Immediate copy/move
```

## LocalBox

Use for local identity-capable Hako objects that are not yet published.

```text
requirements:
  escape=LocalOnly or verifier-approved publication site
  identity/lifecycle requires object form
  host/thread/atomic behavior not observed
```

Rule:

```text
LocalBox is not a fallback for unknown facts.
```

## StableHandle

Use when a stable external or host-visible identity is required.

```text
examples:
  HostHandle boundary
  plugin/extern-visible resource
  identity that must survive publication
```

## MutableCell

Use for Rust interior mutability patterns.

```text
input examples:
  Cell
  RefCell
  shared mutable callback state
```

Rule:

```text
ordinary non-escaping &mut does not require MutableCell.
```

## OrderedMapBox

Use for deterministic string-key map behavior, especially BTreeMap-like
compiler contexts.

```text
requirements:
  deterministic_order_required=true
  key_domain supported by OrderedMapBox
  collection identity is acceptable
```

Rule:

```text
BTreeMap fact alone does not let the adapter choose OrderedMapBox.
The resolver chooses OrderedMapBox from deterministic-order facts.
```

## HostResource

Use for observable external resource ownership.

```text
fields:
  acquire
  release
  cleanup_owner
  double_release_policy
```

Rules:

```text
resource Drop:
  scope cleanup or HostHandle release

do_not_map_every_Drop_to_fini:
  cleanup and object fini are distinct release owners
```

## ArcCompat

Use only when Arc/Rc-like behavior is observed.

```text
requires_one_of:
  arc_count_observed
  weak_observed
  atomic_observed
  thread_boundary
  ptr_eq_or_identity_observed
```

Rule:

```text
shared-only Arc without observed Arc behavior may use another plan.
Observed Arc behavior must not be erased.
```

## CompatShim

Use for explicitly unsupported or deferred semantics that need a compatibility
boundary instead of silent fallback.

```text
examples:
  raw pointer
  Pin / self-reference
  layout-sensitive repr
  panic/unwind parity
  unsafe / FFI
```

Rule:

```text
CompatShim must be explicit and diagnostic-visible.
Unknown facts do not silently become CompatShim.
```

## Minimal Record Shape

```text
plan_subject:
  stable id from RustLifecycleFacts-v0

plan_kind:
  one HakoLifecyclePlan-v0 kind

required_facts:
  fact ids / fact classes that justified the plan

cleanup_policy:
  erase | scope_cleanup | object_fini | host_release | denied

publication_policy:
  unpublished | materialize_on_publication | already_public | denied

diagnostics:
  source context only
```

## Stop Lines

```text
do not implement resolver here
do not emit .hako here
do not start BindingContext lifecycle pilot here
do not treat unknown facts as LocalBox or CompatShim fallback
do not erase Drop without TrivialMemory or verifier-approved cleanup
do not collapse BorrowView / TransferOwned into ordinary assignment
```
