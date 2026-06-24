---
Status: Landed
Date: 2026-05-30
Scope: refresh ArraySlot NativeDirect lowering readiness after exact-lane DirectArrayI64 constructor lowering landed.
Blocker: ARRAY-SLOT-NATIVEDIRECT-LOWERING-READINESS-REFRESH-AFTER-CONSTRUCTOR-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-362-ARRAY-SLOT-NATIVEDIRECT-LOWERING-SELECTED-METHOD-PILOT-PREFLIGHT.md
  - docs/development/current/main/phases/phase-296x/296x-364-DIRECT-ARRAY-I64-BIRTH-HANDLE-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-366-DIRECT-ARRAY-I64-CONSTRUCTOR-LOWERING-PILOT.md
---

# 296x-367 ArraySlot NativeDirect Lowering Readiness Refresh After Constructor

## Purpose

Refresh the selected-method ArraySlot NativeDirect lowering gate after the
exact-lane DirectArrayI64 constructor hook landed.

Row362 blocked lowering because `ArrayBox` construction still produced a public
ArrayBox host handle. Row366 now records a DirectArray origin fact when
`HAKO_ARRAY_SLOT_STORE=direct_array_i64_exact`, so the selected-method lowering
pilot may proceed to a final guard refresh.

This row is docs/guard only. It does not lower ArraySlot get/set to direct
loads/stores.

## Contract

```text
output_contract=array-slot-nativedirect-lowering-readiness-refresh-after-constructor-v0
input_contract=direct-array-i64-constructor-lowering-pilot-v0
selected_method=HakoAllocPageModel.acquire_usize/1
selected_owner_file=src/llvm_py/instructions/mir_call/collection_method_call.py
selected_backend=direct_array_i64_exact
selected_direct_birth_symbol=nyash.array.direct_i64.birth_h
default_public_birth_symbol=nyash.array.birth_h
direct_array_birth_handle_producer_available=1
constructor_exact_lane_origin_fact_available=1
direct_array_origin_fact=resolver.direct_array_i64_ids
receiver_origin_required=nyash.array.direct_i64.birth_h
public_arraybox_handle_as_direct_buffer_allowed=0
default_public_birth_unchanged=1
default_direct_array_emission=0
generic_arraybox_rewrite_allowed=0
runtime_helper_semantics_changes_allowed=0
direct_array_handle_required=1
element_storage_i64_required=1
index_and_bounds_facts_required=1
materialization_boundary_required=1
positive_net_helper_delta_required=1
silent_fallback_allowed=0
legacy_retirement_policy=defer_until_direct_array_semantic_smoke_and_perf_owner_refresh
legacy_retirement_candidate_0=single_thread_exact_array_helper_backend
legacy_retirement_candidate_1=array_slot_handle_entry_cache
legacy_retirement_candidate_2=array_slot_public_helper_fast_lane
legacy_retirement_now=0
selected_method_lowering_blocked=0
selected_method_lowering_unblocked=1
selected_next=array_slot_nativedirect_selected_method_lowering_guard_refresh
implementation_open=0
optimization_open=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The prior row362 blocker is closed:

```text
before:
  ArrayBox constructor -> public nyash.array.birth_h handle
  direct lowering unsafe

now:
  exact lane ArrayBox constructor -> nyash.array.direct_i64.birth_h
  resolver.direct_array_i64_ids records the DirectArray origin
```

Do not delete the diagnostic Array helper/cache lane yet. The legacy retirement
policy is deliberately deferred until a DirectArray NativeDirect semantic smoke
and a post-keeper perf owner refresh prove those paths are no longer required.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_array_slot_nativedirect_lowering_readiness_refresh_after_constructor_guard.sh
```
