---
Status: Landed
Date: 2026-05-29
Scope: implement the selected-method typed-object exact-slot RMW fusion keeper.
Blocker: TYPED-OBJECT-FIELD-RMW-FUSION-KEEPER-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-221-TYPED-OBJECT-FIELD-RMW-FUSION-SELECTION.md
---

# 296x-222 Typed-Object Field RMW Fusion Keeper

## Purpose

Replace selected-method same-block `field_get -> add -> field_set` patterns
with a runtime-owned fused exact-slot helper.

The helper keeps typed-object storage ownership inside Rust runtime code. It
does not expose `TypedSlotObject` layout to LLVM and does not reopen generic
typed-field residence.

## Contract

```text
output_contract=typed-object-field-rmw-fusion-keeper-v0
input_contract=typed-object-field-rmw-fusion-selection-v0
selected_method=HakoAllocPageModel.acquire_usize/1
implementation_owner=c_abi_same_module_typed_field_rmw_fusion
runtime_export_owner=crates/nyash_kernel/src/exports/typed_object.rs
runtime_store_owner=crates/nyash_kernel/src/exports/typed_object_store.rs
fused_runtime_symbol=nyash.object.exact_slot_rmw_add_u64_hiii
planned_erased_get_set_helper_calls=10
planned_added_fused_helper_calls=5
planned_net_helper_call_delta=5
rejected_extra_get_use_count=1
runtime_storage_owner_preserved=1
helper_free_direct_op_rejected=1
generic_residence_open=0
source_rewrite=0
by_name_hako_alloc_special_case=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
```

The `used` RMW-looking pattern is intentionally not fused because its loaded
value is consumed again by the later `peak_used` comparison.

## Acceptance

```text
runtime_helper_exported=1
selected_method_fused=1
exact_exe_fused_symbol_count=1
semantic_proof_summary=ok
single_thread_backend_smoke=ok
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_typed_object_field_rmw_fusion_keeper_guard.sh
```
