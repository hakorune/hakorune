---
Status: SSOT
Decision: accepted
Date: 2026-06-14
Scope: ARC-RETIRE-006..010 family gate through first-family scaffold.
Related:
  - docs/development/current/main/design/arc-retirement-and-ownership-substrate-ssot.md
  - docs/development/current/main/design/object-handle-box-identity-contract-ssot.md
  - docs/development/current/main/design/box-object-model-replacement-map-ssot.md
  - docs/development/current/main/workstreams/arc-retirement-current.md
  - src/runtime/arc_retirement.rs
---

# Arc Retirement Family Gate And First Family (SSOT)

## Decision

ARC-RETIRE-006..010 defines the first family-retirement gate and selects the
first safe family scope.

```text
arc_retirement_mode=first_family_scaffold
arc_retirement_family_gate_defined=1
first_arc_retirement_candidate=vm_scalar_value_boxes
first_arc_retirement_scope=vmvalue_carrier
first_family_vm_carrier=direct_vm_scalar
first_family_vm_carrier_arc_free=1
first_family_box_trait_arc_replaced=0
global_arc_replaced=0
```

The first scope is deliberately narrow: VM scalar value boxes are already
carried directly as `VMValue::{Integer,Bool,String,Float,Void}` instead of
`VMValue::BoxRef(Arc<dyn NyashBox>)`.

This is a real first-family scaffold for the VM carrier, not a global Box trait
carrier replacement.

## Task Slices

```text
ARC-RETIRE-006:
  family retirement gate contract

ARC-RETIRE-007:
  first candidate family selection

ARC-RETIRE-008:
  refcount storage owner prototype

ARC-RETIRE-009:
  atomic retain/release/free-on-zero contract

ARC-RETIRE-010:
  first-family Arc-retirement scaffold
```

## Family Gate

A family may claim Arc retirement only when the gate is satisfied for a named
scope.

```text
object_identity_owner_exists=1
refcount_storage_owner_exists=1
atomic_free_on_zero_exists=1
dispatch_route_owner_exists=1
clone_share_semantics_preserved=1
weak_behavior_defined=1
fini_owner_defined=1
backend_unsupported_surfaces_fail_fast=1
```

For `vm_scalar_value_boxes`, the scope is `vmvalue_carrier`.

```text
object identity:
  direct VM scalar value identity

refcount storage:
  immediate scalar values need no runtime refcount storage

atomic/free-on-zero:
  no-op for the first family, but substrate contract is fixed for later
  refcounted families

dispatch route:
  VM scalar operations route through direct VMValue variants

weak behavior:
  no WeakBox carrier for the first family scope

fini:
  no fini owner for immediate scalar values
```

## First Candidate

```text
family=vm_scalar_value_boxes
scope=vmvalue_carrier
reason=VM scalar values are already direct VMValue carriers and do not use VMValue::BoxRef
```

This includes:

```text
VMValue::Integer
VMValue::Bool
VMValue::String
VMValue::Float
VMValue::Void
```

It does not claim that `IntegerBox`, `BoolBox`, `StringBox`, or `VoidBox`
have been removed from `dyn NyashBox` APIs. `to_nyash_box()` may still create
boxed values for compatibility.

## Refcount Storage Prototype

First family:

```text
refcount_storage_strategy=immediate_scalar_no_refcount
storage_owner=VMValue scalar variant
```

Future refcounted object families:

```text
strategy=object_header_or_side_table
storage_owner=ownership substrate object header or object table
```

## Atomic Retain/Release Contract

The first family does not use runtime refcounts, but ARC-RETIRE-009 fixes the
primitive vocabulary for later refcounted families.

```text
retain_symbol=hako_atomic_slot_fetch_add_i64
release_symbol=hako_atomic_slot_fetch_add_i64
release_uses_fetch_add_minus_one=1
free_symbol=hako_mem_free
free_on_zero_owner=ownership substrate
```

This is a contract over existing substrate symbols. It does not add new
externcall symbols.

## Stop Line

```text
do not claim global Arc replacement
do not change VMValue::BoxRef layout in this slice
do not rewrite Box trait APIs
do not replace plugin carriers
do not make TypeAbiCatalog identity truth
do not hide Arc behind a new wrapper and call it retired
```

## Report Vocabulary

```text
arc_retirement_mode=first_family_scaffold
arc_retirement_family_gate_defined=1
arc_retirement_family_gate_satisfied=1
object_identity_owner_exists=1
refcount_storage_owner_exists=1
atomic_free_on_zero_exists=1
dispatch_route_owner_exists=1
clone_share_semantics_preserved=1
weak_behavior_defined=1
fini_owner_defined=1
backend_unsupported_surfaces_fail_fast=1
first_arc_retirement_candidate=vm_scalar_value_boxes
first_arc_retirement_scope=vmvalue_carrier
refcount_storage_owner_defined=1
refcount_storage_strategy=immediate_scalar_no_refcount
atomic_retain_release_contract_defined=1
retain_symbol=hako_atomic_slot_fetch_add_i64
release_symbol=hako_atomic_slot_fetch_add_i64
release_uses_fetch_add_minus_one=1
free_symbol=hako_mem_free
first_family_arc_retirement_scaffold=1
first_family_vm_carrier=direct_vm_scalar
first_family_vm_carrier_arc_free=1
first_family_box_trait_arc_replaced=0
global_arc_replaced=0
typeabi_identity_truth_count=0
```
