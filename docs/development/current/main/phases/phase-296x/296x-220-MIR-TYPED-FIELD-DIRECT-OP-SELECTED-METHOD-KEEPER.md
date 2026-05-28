---
Status: Rejected
Date: 2026-05-29
Scope: assess the selected-method helper-free typed-field direct-op keeper before implementation.
Blocker: MIR-TYPED-FIELD-DIRECT-OP-SELECTED-METHOD-KEEPER-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-219-MIR-TYPED-FIELD-DIRECT-OP-GUARD-SURFACE.md
  - docs/development/current/main/phases/phase-296x/296x-218-MIR-TYPED-FIELD-DIRECT-OP-NET-INVENTORY.md
---

# 296x-220 MIR Typed-Field Direct-Op Selected-Method Keeper

## Purpose

Check whether the row219 selected-method helper-free typed-field direct-op
keeper can be implemented cleanly.

## Decision

```text
output_contract=mir-typed-field-direct-op-selected-method-feasibility-v0
input_contract=mir-typed-field-direct-op-guard-surface-v0
selected_method=HakoAllocPageModel.acquire_usize/1
requested_helper_free_direct_op=1
feasible_with_current_storage_abi=0
selected_method_keeper_open=0
rejected_owner=helper_free_typed_field_direct_op
rejected_reason=typed_object_storage_is_rust_tls_vec_and_llvm_only_has_opaque_handles
next_owner=typed_object_field_rmw_fusion_selection
by_name_special_case=0
source_rewrite=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

The current typed-object storage lives behind Rust-owned
`SINGLE_THREAD_OBJECTS: RefCell<Vec<TypedSlotObject>>`. The same-module LLVM
emitter has opaque handles and constant slots, but it does not own a stable
pointer layout for `TypedSlotObject`, `TypedSlotField`, or `TypedSlotValue`.

Implementing helper-free loads/stores here would require exporting or assuming
Rust `Vec` / enum / struct layout across the C ABI. That would violate the
typed-object storage seam and make future storage backends brittle.

## Next Owner

The next row should select a helper-reduction seam that keeps storage ownership
inside Rust runtime code, for example exact-slot RMW/fusion helpers for repeated
patterns such as:

```text
field_get reject_count
binop + 1
field_set reject_count
```

This is not as ideal as helper-free scalar residence, but it avoids freezing
Rust storage layout while still attacking the hot exact-slot helper calls.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_mir_typed_field_direct_op_selected_method_feasibility_guard.sh
```
