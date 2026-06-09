---
Status: Active
Date: 2026-05-29
Scope: authority contract for C-like representation/direct lowering after exact-slot micro-helper closeout.
Related:
  - docs/development/current/main/phases/phase-296x/296x-297-MICRO-HELPER-LANE-CLOSEOUT-AND-REPRESENTATION-DIRECT-LOWERING-SELECTION.md
  - docs/development/current/main/phases/phase-296x/296x-278-CAPSULE-VALUE-RESULT-CONTRACT-SSOT.md
  - docs/development/current/main/phases/phase-296x/296x-207-MIR-TYPED-FIELD-RESIDENCE-SSOT.md
  - docs/development/current/main/design/typed-object-exact-slot-abi-ssot.md
---

# Representation Direct Lowering SSOT

## Purpose

Move the mimalloc parity lane from exact-slot helper micro-optimizations to a
compiler-owned representation contract.

The target shape is:

```text
.hako field / capsule / array operation
  -> MIR semantic op
  -> RepresentationFact / RepresentationPlan
  -> ResidentScalar / ValueAggregate / NativeDirect
  -> LLVM scalar or direct load/store
  -> runtime helper only at escape, materialization, or fallback
```

The old hot path often stopped here:

```text
MIR FieldGet / FieldSet
  -> exact-slot helper
  -> runtime typed-object or array storage
```

That path remains the fallback/proof/debug surface, not the next hot-path goal.

Typed-object exact slot helper calls are a bridge, not the final keeper. The
ABI split owner is:

```text
docs/development/current/main/design/typed-object-exact-slot-abi-ssot.md
```

`field_get_hii` belongs to compat / legacy. The exact route truth is
`typed_object.slot_load_*` / `typed_object.slot_store_*`, with
`hako.object.exact_slot_get_*` / `hako.object.exact_slot_set_*` as the
helper-backed transition before NativeDirect.

## Representation Ladder

```text
PublicObject
  Visible object identity, public observer, unknown escape, or dynamic shape.
  Runtime helper path required.

ExactSlotObject
  Receiver type, slot, and storage are known.
  Exact-slot helper or narrow fused helper is allowed, but helpers still remain
  on the hot path.

ResidentScalar
  Object identity remains real, but selected field values are carried as
  scalars inside a proven region. Helper load/writeback happens only at
  materialization or escape boundaries.

ValueAggregate
  Capsule/tuple/result-like value is represented as an aggregate delta.
  Public object materialization happens only at observer/public-return/escape
  boundaries.

NativeDirect
  Backend lowers directly to scalar, slot, array, or stack aggregate access.
  No runtime helper appears inside the selected hot region.
```

## Ownership

```text
MIRBuilder:
  Emits semantic ops and source-shape facts.
  It must not guess representation policy.

Representation planner:
  Owns RepresentationFact and RepresentationPlan.
  It chooses PublicObject / ExactSlotObject / ResidentScalar /
  ValueAggregate / NativeDirect.

Lowerer:
  Consumes the selected plan only.
  It must not independently re-prove semantic eligibility.

Runtime helpers:
  Own fallback, materialization, debug/proof paths, and unsupported dynamic
  shapes.
```

## Required Inventory Fields

The next candidate inventory must compare typed-object, capsule, and ArraySlot
regions using the same vocabulary:

```text
candidate_family
current_representation
candidate_representation
hot_pct
helper_ops_before
helper_ops_erased
materialization_ops_added
net_helper_delta
escape_barrier_count
observer_barrier_count
unknown_call_barrier_count
storage_or_slot_proven
implementation_risk
selected_as_first_pilot
```

## Fail-Fast

```text
selected_representation != PublicObject
and materialization_policy_known == 0
  -> fail-fast

selected_representation != PublicObject
and net_helper_delta_positive == 0
  -> fail-fast

unknown escape crosses region
  -> no plan or explicit fallback plan

public observer boundary unknown
  -> no ValueAggregate plan

slot/storage/index not proven
  -> no NativeDirect plan

selected plan silently falls back
  -> row failure
```

## Non-Goals

```text
generic MIR CSE
new exact-slot helper family
name-based hako_alloc special cases
provider activation
allocator replacement
hooks
global allocator
winner claim
```

## First Pilot Selection Rule

Do not select the first implementation target directly from intuition.

The next row must inventory at least these candidate families with the shared
fields above:

```text
typed_object_exact_slot_residence
result_capsule_value_aggregate
array_slot_native_direct
```

Then a separate selection row chooses one first pilot:

```text
if typed_object has positive net delta and manageable barriers:
  choose typed-object ResidentScalar/NativeDirect
elif ArraySlot has a small safe positive-net region:
  choose ArraySlot NativeDirect as the pipeline proof
elif capsule has a non-public-return positive-net region:
  choose capsule ValueAggregate
else:
  return to measurement owner refresh
```
