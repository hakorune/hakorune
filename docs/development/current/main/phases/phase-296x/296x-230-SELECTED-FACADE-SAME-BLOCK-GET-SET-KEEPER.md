---
Status: Landed
Date: 2026-05-29
Scope: implement the selected facade same-block get/set fusion keeper.
Blocker: SELECTED-FACADE-SAME-BLOCK-GET-SET-KEEPER-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-229-SELECTED-FACADE-SAME-BLOCK-GET-SET-GUARD-SURFACE.md
---

# 296x-230 Selected Facade Same-Block Get/Set Keeper

## Purpose

Implement the row229 selected facade-family `field_get -> add -> field_set`
fusion by reusing the runtime-owned exact-slot RMW helper.

This keeps storage ownership inside the Rust runtime, preserves all non-selected
typed-field access paths, and does not open generic typed-field residence or
source rewrites.

## Evidence

```text
output_contract=selected-facade-same-block-get-set-keeper-v0
input_contract=selected-facade-same-block-get-set-guard-surface-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_owner=selected_facade_same_block_get_set_fusion
implementation_owner=c_abi_same_module_typed_field_rmw_fusion
target_family=object_lifecycle_facade
candidate_count=6
candidate_usize_count=6
fused_runtime_symbol=nyash.object.exact_slot_rmw_add_u64_hiii
status_continue_label_contract=exact_status_continue
planned_erased_get_set_helper_calls=12
planned_added_fused_helper_calls=6
planned_net_helper_call_delta=6
exact_exe_fused_symbol_count=1
semantic_proof_summary=ok
single_thread_backend_smoke=ok
runtime_storage_owner_preserved=1
helper_free_direct_op_rejected=1
generic_residence_open=0
source_rewrite=0
by_name_hako_alloc_special_case=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_next=selected_facade_same_block_get_set_measurement
next_row=selected_facade_same_block_get_set_measurement
```

The keeper extends the existing selected-method RMW fusion target set to the
four facade methods frozen by row229:

```text
HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
HakoAllocObjectLifecycleFacade.objectLifecycleReleaseKnownPageIndex/1
HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
HakoAllocObjectLifecycleFacade.objectLifecycleSmallAllocAligned/2
```

The fusion remains gated by `HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1` and exact
typed-object storage metadata. Unsupported field access keeps the existing
generic helper route.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_selected_facade_same_block_get_set_keeper_guard.sh
```

Do not claim performance parity from this row. Measure body time and owner
refresh separately.
