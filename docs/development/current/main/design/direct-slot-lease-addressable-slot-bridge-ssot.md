---
Status: Accepted
Date: 2026-05-29
Scope: SSOT for the helper-free addressable slot bridge required before DirectSlotLease lowering.
Related:
  - docs/development/current/main/design/direct-slot-lease-guard-surface-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-318-DIRECT-SLOT-LEASE-LOWERING-PILOT-FEASIBILITY-CLOSEOUT.md
---

# Direct Slot Lease Addressable Slot Bridge SSOT

## Purpose

Define the storage bridge required for DirectSlotLease lowering.

DirectSlotLease is not allowed to become another helper-call layer. Its purpose
is to let a proven hot region lower selected typed-object slot operations to
direct load/store form while preserving `.hako` allocator semantics.

## Layer Ownership

```text
hako_alloc_policy_state_owner=.hako
representation_owner=compiler_direct_lowering
storage_substrate_owner=typed_object_direct_slot_storage
runtime_helper_owner=fallback_materialization_debug
```

The compiler must not special-case `HakoAlloc*` names. Selection must be driven
by receiver/slot/storage/barrier facts.

## Accepted Bridge Shape

The first acceptable bridge is a dedicated direct-slot storage substrate.

```text
bridge_kind=direct_slot_cell_storage
slot_cell_layout=stable_abi
slot_cell_visibility=compiler_consumable
handle_to_storage_resolution=contracted
raw_runtime_vec_pointer_exposure=0
rust_refcell_borrow_exposure=0
typed_slot_rust_enum_layout_exposure=0
```

This bridge may be backed by Rust internally, but the layout consumed by LLVM
must be an explicit stable contract. Current `TypedSlot` / `TypedSlotValue` Rust
enum layout is not a direct lowering ABI.

## Rejected Bridge Shapes

```text
raw_vec_slot_pointer_bridge=0
thread_local_refcell_pointer_bridge=0
rust_enum_layout_direct_load=0
c_abi_load_writeback_helper_bridge=0
by_name_hako_alloc_bridge=0
silent_helper_fallback_bridge=0
```

Reasons:

- raw `Vec` / `RefCell` pointers have lifetime, relocation, and borrow rules the
  lowerer cannot prove;
- Rust enum layout is not the stable slot ABI for LLVM direct load/store;
- C ABI load/writeback helpers recreate the zero-net ResidentScalar problem;
- by-name specialization would make hako_alloc a compiler magic case.

## Required Facts

```text
receiver_exact_plan_required=1
slot_constant_required=1
storage_class_exact_required=1
storage_layout_stable_required=1
handle_resolution_contract_required=1
generation_or_identity_validation_required=1
unknown_call_barrier_policy=no_plan
escape_barrier_policy=no_plan_or_materialize
observer_barrier_policy=no_plan_or_materialize
positive_net_helper_delta_required=1
selected_plan_silent_fallback_allowed=0
```

## Direct Slot Cell Minimum

The minimum stable cell vocabulary for the first pilot:

```text
cell_storage_classes=i64|u64|handle
cell_read_i64=direct_load
cell_write_i64=direct_store
cell_read_u64=direct_load
cell_write_u64=direct_store
cell_read_handle=direct_load
cell_write_handle=direct_store
```

The cell shape must be explicit before implementation. A future row may choose
the exact Rust representation, but it must not rely on the private layout of
`TypedSlot`.

## Runtime Helper Path

Existing typed-object helpers remain the fallback/materialization/debug path.

```text
existing_helper_abi_unchanged=1
default_backend_direct_slot_emission=0
helper_path=fallback_materialization_debug
provider_activation=0
allocator_replacement=0
hook_installed=0
global_allocator=0
winner_claim=0
```

## Next Implementation Gate

Before retrying DirectSlotLease lowering, a row must prove:

```text
addressable_slot_bridge_available=1
stable_cell_layout_defined=1
llvm_consumable_slot_address_defined=1
helper_free_bridge_possible=1
raw_runtime_vec_pointer_exposure=0
c_abi_load_writeback_helper_count=0
```

If any of these are false, lowering remains closed.
