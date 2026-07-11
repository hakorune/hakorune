# 296x-1384 RUST-LIFECYCLE-FACTS-VOCAB-000

Status: closed
Date: 2026-06-20

## Purpose

Add passive vocabulary for the Rust-side lifecycle facts consumed by the Hako
lifecycle resolver.

This row does not choose Hako representation policy and does not emit `.hako`.
It only names the facts that a future rustc semantic adapter must provide.

## Selected By

```text
296x-1381-RUST-LIFECYCLE-PROJECTION-SSOT-001
```

## SSOT

```text
docs/development/current/main/design/rust-lifecycle-projection-ssot.md
docs/development/current/main/design/rust-lifecycle-facts-vocab-v0.md
```

## Scope

Define `RustLifecycleFacts-v0` passive vocabulary:

```text
CopyKind:
  ImmediateValue
  AggregateValue
  SharedAlias
  CallablePointer
  RawAddress

MoveKind:
  TransferOwned
  TakeLocal
  ReplaceOwned
  ConsumeArgument

BorrowFact:
  SharedRead
  UniqueWrite
  scope
  escapes

DropFact:
  TrivialMemory
  StructuralOwned
  CustomSemanticDrop
  HostResource
  Conditional
  Open

EscapeFact:
  LocalOnly
  Returned
  Stored
  HostBoundary
  ThreadBoundary

ObservationFact:
  identity_observed
  address_observed
  layout_observed
  arc_count_observed
  weak_observed
  atomic_observed
```

Implementation shape is passive only:

```text
schema_or_docs_only=preferred
runtime_behavior_changed=0
converter_behavior_changed=0
resolver_behavior_changed=0
emitter_behavior_changed=0
```

## Acceptance

```text
rust_lifecycle_facts_vocab_exists=1
copy_move_borrow_drop_escape_facts_named=1
adapter_facts_only_boundary_preserved=1
hako_representation_policy_added=0
converter_direct_ownership_policy_added=0
implementation_behavior_changed=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_add_HakoLifecyclePlan_in_this_row=1
do_not_add_converter_emission_in_this_row=1
do_not_add_rust_lifetime_syntax=1
do_not_let_adapter_choose_Hako_representation=1
do_not_start_BindingContext_lifecycle_pilot=1
```

## Next

```text
296x-1385-HAKO-LIFECYCLE-PLAN-VOCAB-000
```

## Closeout Evidence

```text
rust_lifecycle_facts_vocab_exists=1
copy_move_borrow_drop_escape_facts_named=1
adapter_facts_only_boundary_preserved=1
hako_representation_policy_added=0
converter_direct_ownership_policy_added=0
implementation_behavior_changed=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
