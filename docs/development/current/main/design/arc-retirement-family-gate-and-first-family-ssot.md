---
Status: SSOT
Decision: accepted
Date: 2026-06-14
Scope: ARC-RETIRE-006..016 family gate through first real family carrier cutover.
Related:
  - docs/development/current/main/design/arc-retirement-and-ownership-substrate-ssot.md
  - docs/development/current/main/design/object-handle-box-identity-contract-ssot.md
  - docs/development/current/main/design/box-object-model-replacement-map-ssot.md
  - docs/development/current/main/workstreams/arc-retirement-current.md
  - src/runtime/arc_retirement.rs
---

# Arc Retirement Family Gate And First Family (SSOT)

## Decision

ARC-RETIRE-006..016 defines the first family-retirement gate and cuts over the
first real object-adjacent family carrier.

```text
arc_retirement_mode=first_real_family_cutover
arc_family_retirement_started=1
arc_hot_path_retirement_started=0
arc_retirement_family_gate_defined=1
first_arc_retirement_candidate=host_handle_text_payload
first_arc_retirement_scope=host_handle_carrier
first_family_carrier=stable_text_payload
first_family_host_handle_text_arc_free=1
first_family_box_trait_arc_replaced=0
global_arc_replaced=0
```

The first real family is deliberately narrow: host-handle text payloads now
store `String` directly in the handle table instead of storing an
`Arc<dyn NyashBox>` payload.

This is a real carrier cutover for the text-handle payload family. It is not a
global host-handle carrier replacement.

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

ARC-RETIRE-011:
  object refcount storage design

ARC-RETIRE-012:
  retain/release MIR contract

ARC-RETIRE-013:
  ObjectHandle-backed object table prototype

ARC-RETIRE-014:
  WeakObjectHandle behavior

ARC-RETIRE-015:
  first real object family selection

ARC-RETIRE-016:
  first real family Arc carrier cutover
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

For `host_handle_text_payload`, the scope is `host_handle_carrier`.

```text
object identity:
  host handle ObjectHandle identity

refcount storage:
  text payload is owned directly by the handle slot

atomic/free-on-zero:
  no-op for this family, while substrate contract is fixed for later families

dispatch route:
  text-only route uses with_str_handle / TextReadSession

weak behavior:
  no WeakBox carrier for the text payload family

fini:
  no fini owner for direct text payload
```

## First Candidate

```text
family=host_handle_text_payload
scope=host_handle_carrier
reason=host handle text payloads are stored as String and only materialize Arc boxes for compatibility APIs
```

This includes:

```text
host_handles::to_handle_text
host_handles::with_str_handle
host_handles::with_text_read_session
```

It does not claim that `StringBox` has been removed from `dyn NyashBox` APIs.
Compatibility APIs such as `host_handles::get()` may still materialize a
temporary `StringBox`.

## Refcount Storage Prototype

First real family:

```text
refcount_storage_strategy=immediate_scalar_no_refcount
storage_owner=host handle stable text payload
```

Future refcounted object families:

```text
strategy=object_header_or_side_table
storage_owner=ownership substrate object header or object table
```

## Atomic Retain/Release Contract

The first real family does not use runtime refcounts, but ARC-RETIRE-009 fixes the
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
arc_retirement_mode=first_real_family_cutover
arc_family_retirement_started=1
arc_hot_path_retirement_started=0
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
first_arc_retirement_candidate=host_handle_text_payload
first_arc_retirement_scope=host_handle_carrier
refcount_storage_owner_defined=1
refcount_storage_strategy=immediate_scalar_no_refcount
atomic_retain_release_contract_defined=1
retain_symbol=hako_atomic_slot_fetch_add_i64
release_symbol=hako_atomic_slot_fetch_add_i64
release_uses_fetch_add_minus_one=1
free_symbol=hako_mem_free
first_family_arc_retirement_scaffold=1
first_family_carrier=stable_text_payload
host_handle_text_payload_arc_replaced=1
first_family_host_handle_text_arc_free=1
first_family_box_trait_arc_replaced=0
global_arc_replaced=0
typeabi_identity_truth_count=0
```
