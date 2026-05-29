---
Status: Landed
Date: 2026-05-29
Scope: define the ArraySlot NativeDirect guard surface after runtime helper micro-optimization closeout.
Blocker: ARRAY-SLOT-NATIVEDIRECT-GUARD-SURFACE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-347-POST-ARRAY-HANDLE-CACHE-OWNER-REFRESH.md
  - crates/nyash_kernel/src/plugin/array_slot_backend.rs
---

# 296x-348 ArraySlot NativeDirect Guard Surface

## Purpose

Close the Array runtime/helper micro-optimization lane and define the first
ArraySlot NativeDirect surface.

Row347 showed that after the `single_thread_exact` handle-entry cache removed
the `HashMap` owner, the remaining large cost is the Array slot helper boundary
itself. The next work should not make plugin helpers thinner again. It should
introduce a compiler-consumable representation for hot exact i64 Array slots.

This row is docs/guard only. It keeps implementation and LLVM lowering closed.

## Contract

```text
output_contract=array-slot-nativedirect-guard-surface-v0
input_contract=array-post-handle-cache-owner-refresh-v0
selected_owner=array_slot_nativedirect
selected_reason=array_helper_call_boundary_dominates_after_hash_removed
public_arraybox_semantics_unchanged=1
default_safe_rwlock_path_unchanged=1
plugin_arraybox_public_owner=1
single_thread_exact_helper_path=fallback_materialization_debug
selected_representation=DirectArrayI64BufferV0
element_storage=i64
mixed_storage_supported=0
boxed_storage_supported=0
string_storage_supported=0
bool_f64_storage_supported=0
direct_i64_load_store_selected=1
fused_load_store_selected=0
method_local_residence_selected=0
runtime_helper_internal_fast_lane_repeat=0
public_arraybox_storage_change=0
hako_source_workaround=0
mirbuilder_changes_allowed=0
hako_source_changes_allowed=0
required_fact_receiver_array_exact=1
required_fact_element_storage_i64=1
required_fact_index_i64=1
required_fact_bounds_policy_known=1
required_fact_append_policy_known=1
required_fact_materialization_policy_known=1
required_positive_net_helper_delta=1
unsupported_storage_policy=no_plan
oob_policy=preserve_or_no_plan
append_at_end_policy=preserve_if_capacity_known_else_no_plan
selected_plan_silent_fallback_allowed=0
implementation_open=0
llvm_lowering_open=0
provider_activation=0
host_replacement=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Representation Boundary

`ArrayBox` stays the public runtime/plugin owner:

```text
PublicArrayBox:
  dynamic/mixed storage
  safe_rwlock default path
  boxed/string/bool/f64/generic semantics
  observer/debug/materialization boundary
```

The NativeDirect representation is a separate exact i64 storage substrate:

```text
DirectArrayI64BufferV0:
  exact-EXE / proven i64 slot region only
  repr(C) stable layout selected in a later row
  contiguous i64 payload planned
  no public ArrayBox semantics by itself
```

This matches the DirectSlot separation: public semantics remain in the runtime
object, while hot proven storage gets a compiler-consumable representation.

## First Pilot Choice

The first pilot is direct i64 slot load/store.

```text
direct_i64_load_store_selected=1
fused_load_store_selected=0
method_local_residence_selected=0
```

Fused Array ops and method-local ArraySlot residence remain later consumers.
They should not be used to prove the storage substrate.

## Fail-Fast Boundary

Planning-time unsupported shapes produce no NativeDirect plan. If a selected
plan later fails its facts, the row must fail; it must not silently fall back to
helper calls and report success.

```text
selected_plan_silent_fallback_allowed=0
unsupported_storage_policy=no_plan
```

Append and OOB semantics must be preserved by proof or closed by no-plan:

```text
idx < len:
  direct store/load may be planned

idx == len:
  append may be planned only when capacity policy is known

idx > len or idx < 0:
  preserve current result or produce no plan
```

## Non-Goals

```text
rejected=plugin_runtime_helper_micro_optimization_repeat
reason=row347 selected helper boundary removal, not another helper internal fast lane

rejected=public_arraybox_storage_rewrite
reason=public ArrayBox semantics stay runtime-owned

rejected=ArraySlot_residence_first
reason=residence needs a direct buffer substrate first, otherwise it repeats zero-net helper load/writeback risk
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_array_slot_nativedirect_guard_surface_guard.sh
```
