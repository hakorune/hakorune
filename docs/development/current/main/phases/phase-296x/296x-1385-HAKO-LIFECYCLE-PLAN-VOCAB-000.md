# 296x-1385 HAKO-LIFECYCLE-PLAN-VOCAB-000

Status: closed
Date: 2026-06-20

## Purpose

Add passive vocabulary for Hako lifecycle plans produced by a future
HakoLifecycleResolver.

This row consumes the RustLifecycleFacts vocabulary but does not implement the
resolver, verifier, emitter, or BindingContext pilot.

## Selected By

```text
296x-1384-RUST-LIFECYCLE-FACTS-VOCAB-000
```

## SSOT

```text
docs/development/current/main/design/rust-lifecycle-projection-ssot.md
docs/development/current/main/design/hako-lifecycle-plan-vocab-v0.md
```

## Scope

Define `HakoLifecyclePlan-v0` passive vocabulary:

```text
Immediate
AggregateLocal
BorrowView
TransferOwned
LocalBox
StableHandle
MutableCell
OrderedMapBox
HostResource
ArcCompat
CompatShim
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
hako_lifecycle_plan_vocab_exists=1
immediate_aggregate_borrow_transfer_localbox_orderedmap_named=1
host_resource_and_compat_shapes_named=1
rust_lifecycle_facts_vocab_consumed=1
resolver_behavior_added=0
converter_emission_added=0
binding_context_pilot_started=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_implement_resolver_in_this_row=1
do_not_emit_Hako_from_plan_in_this_row=1
do_not_start_BindingContext_lifecycle_pilot=1
do_not_add_Rust_lifetime_syntax=1
do_not_erase_Drop_without_positive_TrivialMemory_fact=1
```

## Next

```text
296x-1386-RUST-TO-HAKO-LIFECYCLE-EMITTER-CONTRACT-000
```

## Closeout Evidence

```text
hako_lifecycle_plan_vocab_exists=1
immediate_aggregate_borrow_transfer_localbox_orderedmap_named=1
host_resource_and_compat_shapes_named=1
rust_lifecycle_facts_vocab_consumed=1
resolver_behavior_added=0
converter_emission_added=0
binding_context_pilot_started=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
