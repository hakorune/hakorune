---
Status: Active Proposal
Date: 2026-04-19
Card: 289x-3a
Scope: first runtime-private storage pilot selection before behavior/code edits.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/phases/phase-289x/README.md
  - docs/development/current/main/phases/phase-289x/289x-91-runtime-value-object-task-board.md
  - docs/development/current/main/phases/phase-289x/289x-94-container-demand-table.md
---

# Phase 289x Array Text Residence Pilot

## Decision

The first runtime-private storage pilot is:

```text
Array text residence through KernelTextSlot store
```

This is not a full `TextLane` rewrite.
It is the smallest pilot that already has proof from phase-137x and matches the
runtime-wide demand model:

```text
Array element read:
  ValueDemand::ReadRef

Text producer:
  ValueDemand::OwnedPayload

Array element write:
  StorageDemand::CellResidence

Boundary only if requested:
  PublishDemand::NeedStableObject / ExternalBoundary
```

## Selected Pilot Surface

| Surface | Current symbol/function | Demand |
| --- | --- | --- |
| read text from array slot | `array_string_len_by_index`, `array_string_indexof_by_index` | `ValueDemand::ReadRef` |
| produce same-slot suffix text | `array_string_concat_const_suffix_by_index_into_slot` / `nyash.array.kernel_slot_concat_his` | `ValueDemand::ReadRef` + `ValueDemand::OwnedPayload` |
| store unpublished text into array | `array_string_store_kernel_text_slot_at` / `nyash.array.kernel_slot_store_hi` | `StorageDemand::CellResidence` |
| publish only on downstream demand | `publish_kernel_text_slot`, `objectize_kernel_text_slot_stable_box` | `PublishDemand::*` |

## Required Existing Proof

The pilot starts from keeper `49c356339`:

```text
array.get -> indexOf("line") -> compare -> branch
then branch:
  fetched string used only as copy -> const suffix -> Add -> same array.set(idx, value)
```

Accepted lowering:

```text
nyash.array.string_indexof_hih
nyash.array.kernel_slot_concat_his
nyash.array.kernel_slot_store_hi
```

Reject seam:

```text
no call to nyash.array.slot_load_hi on the exact same-slot suffix path
live-after-get reuse shapes still keep slot_load_hi
```

## Implementation Order

1. Add code-level demand vocabulary in a runtime-private module.
2. Map existing Array text-residence calls to demand constants without changing behavior.
3. Add/keep unit tests that prove `KernelTextSlot` store does not publish a fresh handle.
4. Keep C shim lowering shape unchanged until the demand vocabulary exists in code.
5. Only then consider replacing helper-name route checks with demand facts.

## First Code Cut

The first code cut should be BoxShape:

```text
crates/nyash_kernel/src/plugin/value_demand.rs
```

It may define runtime-private enums and constants:

```text
ValueDemand::{ReadRef, EncodeImmediate, EncodeAlias, OwnedPayload, StableObject}
StorageDemand::{CellResidence, ImmediateResidence, GenericResidence, DegradeGeneric}
PublishDemand::{ExternalBoundary, GenericFallback, ExplicitApi, NeedStableObject}
MutationDemand::{InvalidateAliases, DropEpoch}
```

It must not change exported ABI or lowering behavior.

Status:

```text
landed in code as runtime-private vocabulary
behavior unchanged
```

## Acceptance

- public Array semantics stay identity-container based
- the exact same-slot suffix path remains closed
- `slot_load_hi` remains rejected on that exact path
- live-after-get reuse keeps the existing stable fallback behavior
- demand vocabulary is runtime-private and docs-backed

## No-Go

- do not introduce full `ArrayStorage::Text`
- do not change public ABI
- do not rewrite Map in this pilot
- do not start allocator/arena work
- do not remove fallback publication paths before MIR/lowering can name demand
