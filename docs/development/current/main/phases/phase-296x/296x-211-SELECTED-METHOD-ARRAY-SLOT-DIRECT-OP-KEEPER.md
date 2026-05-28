---
Status: Landed
Date: 2026-05-28
Scope: implement the selected-method Array slot direct-op keeper for `HakoAllocPageModel.acquire_usize/1`.
Blocker: SELECTED-METHOD-ARRAY-SLOT-DIRECT-OP-KEEPER-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-210-SELECTED-METHOD-ARRAY-SLOT-DIRECT-OP-OWNER-SELECTION.md
  - docs/development/current/main/phases/phase-296x/296x-209-MIR-ARRAY-SLOT-RESIDENCE-SELECTED-METHOD-GUARD-SURFACE.md
---

# 296x-211 Selected Method Array Slot Direct Op Keeper

## Purpose

Replace the selected same-block `ArrayBox.get` / copy-carrier /
`ArrayBox.set` pair in `HakoAllocPageModel.acquire_usize/1` with one fused
runtime direct-slot op.

This row does not open generic ArrayBox residence. The implementation is a
selected-method keeper that proves whether this seam can erase one hot helper
pair before a broader MIR transform is considered.

## Contract

```text
output_contract=selected-method-array-slot-direct-op-keeper-v0
input_contract=selected-method-array-slot-direct-op-owner-selection-v0
selected_method=HakoAllocPageModel.acquire_usize/1
selected_block=45
implementation_owner=c_abi_same_module_array_slot_direct_op_fusion
helper_owner_file=lang/c-abi/shims/hako_llvmc_ffi_same_module_body_emit.inc
declaration_owner_file=lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc
runtime_export_owner=crates/nyash_kernel/src/plugin/array_direct_slot_op.rs
fused_runtime_symbol=nyash.array.slot_load_store_i64_hihi
planned_erased_get_set_helper_calls=2
planned_added_fused_helper_calls=1
planned_net_helper_call_delta=1
generic_array_residence_open=0
by_name_hako_alloc_special_case=0
source_rewrite=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
```

## Implementation Boundary

Allowed:

- add a fused runtime helper that loads one i64 slot and stores an i64 value
  through the existing Array slot backend seam;
- add a narrow C-ABI same-module body-emitter pattern for the selected
  method/block;
- preserve the loaded value in the original get destination and copy-carrier
  destinations so later PHI/return users see the same value.

Rejected:

- generic ArrayBox residence transform;
- source-level `.hako` rewrite;
- by-name hako_alloc semantic special case;
- changing the default `safe_rwlock` ArrayBox behavior.

## Acceptance

```text
runtime_helper_exported=1
selected_block_fused=1
erased_get_set_helper_calls=2
added_fused_helper_calls=1
net_helper_call_delta=1
semantic_proof_summary=ok
default_backend_smoke=ok
single_thread_backend_smoke=ok
exact_exe_fused_symbol_count=1
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_selected_method_array_slot_direct_op_keeper_guard.sh
```
