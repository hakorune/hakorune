---
Status: Landed
Date: 2026-05-29
Scope: refresh the hot owner after DirectSlot bootstrap/materialization compatibility.
Blocker: POST-DIRECT-SLOT-BOOTSTRAP-MATERIALIZATION-OWNER-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-341-DIRECT-SLOT-BOOTSTRAP-MATERIALIZATION-COMPATIBILITY.md
  - tools/allocator/direct_slot_post_bootstrap_owner_refresh.py
---

# 296x-342 Post DirectSlot Bootstrap Owner Refresh

## Purpose

Classify the remaining exact-EXE owner after row341 made DirectSlot positive
handles semantic-safe for bootstrap and fallback helper paths.

The result is clear: the selected `acquire_usize/1` NativeDirect pilot remains
valid, but non-selected methods still lower supported typed-object fields through
legacy field helpers. Those helpers now dominate the sample. The next row should
not optimize helper internals; it should open a fact-driven DirectSlot
NativeDirect guard surface for supported storage where TypedObjectPlan already
proves receiver type, slot, and storage.

## Contract

```text
output_contract=direct-slot-post-bootstrap-owner-refresh-v0
input_contract=direct-slot-bootstrap-materialization-compatibility-v0
workload_id=representative-object-lifecycle-small-block-v0
attribution_source=perf_callgraph
callgraph_attribution_available=1
field_helper_pct=65.02
exact_slot_helper_pct=0.00
array_slot_backend_pct=16.36
array_hash_pct=13.91
array_total_pct=30.27
hako_method_pct=4.64
family_0_name=object_lifecycle_facade
family_0_pct=20.54
family_1_name=page_queue_helpers
family_1_pct=19.63
family_2_name=alloc_result_capsule
family_2_pct=12.94
family_3_name=release_result_capsule
family_3_pct=9.59
helper_0_symbol=nyash.object.field_set_hii
helper_0_pct=29.21
helper_1_symbol=nyash.object.field_get_hii
helper_1_pct=15.20
helper_2_symbol=nyash.object.field_get_u64_hii
helper_2_pct=11.76
helper_3_symbol=nyash.object.field_set_u64_hiu
helper_3_pct=8.85
selected_boundary=direct_slot_supported_storage_nativedirect_guard_surface
next_diagnostic=direct_slot_supported_storage_nativedirect_guard_surface
selected_reason=legacy_field_helpers_dominate_after_direct_slot_bootstrap_compatibility
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

`direct_slot_exact` now has enough storage substrate and fallback/materialization
compatibility to consider a wider lowering guard. The next row may design
fact-driven direct payload lowering for supported storage:

```text
allowed_storage=i64,u64,usize,handle
required_facts=TypedObjectPlan receiver binding + constant runtime slot + storage support
rejected=helper_internal_fast_lane
rejected=by_name_hako_alloc_special_case
rejected=unsupported_narrow_integer_direct_store
```

The guard surface must keep unsupported storage on the existing helper route and
must preserve unsigned nonnegative traps and exact-status continuation labels.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_post_direct_slot_bootstrap_owner_refresh_guard.sh
```
