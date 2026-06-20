# Rustc SemIR BindingContext Adapter Harness Design

Status: Design
Scope: first external rustc semantic adapter harness for lifecycle facts.

## Purpose

Define the smallest harness boundary that can later produce
`RustLifecycleFacts-v0` for `BindingContext`.

This design does not implement the adapter.

## Boundary

```text
input:
  selected Rust source item / module slice
  subject=hakorune_mir_builder::binding_context::BindingContext

output:
  RustLifecycleFacts-v0 JSON only

stable schema:
  repo-owned JSON vocabulary
  existing fixture shape:
    binding-context-adapter-facts-v0.json
```

The adapter is an external facts producer. It is not a Hako policy owner.

## Adapter Owns

```text
Rust item identity
source module / path provenance
concrete field and method facts
copy / move / borrow / drop observations
deterministic iteration requirement
identity / address / layout / thread observation flags
```

## Adapter Does Not Own

```text
HakoLifecyclePlan-v0
Hako representation selection
OrderedMapBox selection
BorrowView / TransferOwned / LocalBox spelling
.hako source emission
backend lowering
resolver / verifier decisions
```

## Toolchain Boundary

The future harness may use rustc internals, HIR, THIR, MIR, borrowck, or drop
facts internally, but the checked-in handoff is only the repo-owned JSON schema.

Forbidden stable inputs:

```text
raw pretty MIR text
raw THIR debug dump
rustc-internal node indexes as public IDs
compiler-version-specific enum dumps
```

Required output properties:

```text
schema_version=0
kind=RustLifecycleAdapterFacts
subject=hakorune_mir_builder::binding_context::BindingContext
target_neutral.hako_policy_owner=false
target_neutral.hako_plan_kind_spelling_allowed=false
target_neutral.rendering_instruction_allowed=false
```

## First Probe Shape

The first implementation row should reproduce the existing target-neutral
fixture shape:

```text
types:
  BindingContext:
    copy_class=NonCopyOwned
    drop_class=TrivialMemory
    identity/address/layout/thread observations=false

  BindingId:
    copy_class=ImmediateValue
    drop_class=TrivialMemory

field:
  BindingContext.binding_map:
    rust_type=BTreeMap<String, BindingId>
    deterministic_order_required=true
    drop_class=TrivialMemory

methods:
  shared read CallOnly:
    is_empty
    len
    contains
    lookup

  unique write CallOnly:
    insert
    remove
    clear_for_function_entry
```

## Stop Lines

```text
do_not_emit_HakoLifecyclePlan_from_adapter=1
do_not_emit_hako_source_from_adapter=1
do_not_choose_OrderedMapBox_in_adapter=1
do_not_use_raw_rustc_dump_as_schema=1
do_not_claim_VariableContext_or_crate_wide_facts=1
```
