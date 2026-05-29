---
Status: Active
Date: 2026-05-29
Scope: storage substrate contract for NativeDirect representation after ResidentScalar zero-net evidence.
Related:
  - docs/development/current/main/design/representation-direct-lowering-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-304-TYPED-OBJECT-RESIDENT-SCALAR-FEASIBILITY-CLOSEOUT.md
---

# Representation Direct Storage Substrate SSOT

## Purpose

Define the substrate required for C-like hot-path lowering.

The row304 feasibility evidence showed that `ResidentScalar` implemented with
helper load/writeback is zero-net:

```text
erased_field_get_count=11
erased_field_set_count=8
inserted_helper_load_count=11
inserted_helper_writeback_count=8
net_helper_call_delta=0
```

Therefore the next representation target is not another helper-backed scalar
cache. The compiler needs a storage representation that can make selected hot
regions helper-free.

## Extended Ladder

```text
PublicObject
  Visible identity, public observer, unknown escape, or dynamic shape.
  Runtime helper path required.

ExactSlotObject
  Receiver type, slot, and storage are known.
  Exact-slot helper path allowed, but helper calls remain on the hot path.

ResidentScalarCache
  Field values are cached as scalars in a region, but load/writeback still use
  helpers. This is only valid when repeated use gives positive net helper delta.

AddressableSlot
  A selected object/slot has stable address or offset identity for a proven
  region.

DirectSlotLease
  Runtime/storage grants a bounded direct-access lease for one object/slot.
  No raw runtime Vec pointer exposure is allowed.

MaterializedLocalStruct
  Compiler-owned local aggregate for born-local or no-escape object/value
  shapes. Public object state is materialized only at known boundaries.

ValueAggregateDelta
  Result/capsule-like update represented as a value delta. It materializes at a
  known observer/public-return/escape boundary.

NativeDirect
  Backend emits scalar/direct load-store with no runtime helper in the selected
  hot region.
```

## DirectSlotLease

`DirectSlotLease` is a bridge from opaque handles to direct storage. It is not
raw pointer exposure.

Required facts:

```text
single_thread_exact=1
receiver_type_known=1
slot_constant=1
storage_class_known=1
object_storage_pinned=1
no_vec_reallocation_in_region=1
no_unknown_escape=1
no_unknown_call=1
no_aliasing_write=1
materialization_policy_known=1
```

If any required fact is absent, no lease is produced.

## MaterializedLocalStruct

`MaterializedLocalStruct` is the compiler-owned path for born-local/no-escape
objects and value-like aggregates.

It may be selected only when:

```text
object_birth_or_materialization_point_known=1
public_identity_required_inside_region=0
observer_boundary_known=1
escape_boundary_known=1
materialization_policy_known=1
net_helper_delta_positive=1
```

## Fail-Fast

```text
raw_runtime_vec_pointer_exposure_allowed=0

DirectSlotLease selected
and object_storage_pinned != 1
  -> fail-fast

NativeDirect selected
and no AddressableSlot or MaterializedLocalStruct
  -> fail-fast

selected plan silently falls back to exact-slot helper
  -> row failure

net_helper_delta_positive == 0
  -> no implementation
```

## Next Feasibility Candidates

```text
typed_object_direct_slot_lease
method_local_stack_aggregate
array_slot_native_direct
result_capsule_value_aggregate_region
```

The first feasibility row should answer whether the current typed-object store
can support `DirectSlotLease` without a storage rewrite.
