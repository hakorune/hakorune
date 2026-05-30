---
Status: Current
Date: 2026-05-30
Scope: close the remaining direct-path fast-path surface check before returning to mimalloc source-level optimization.
Blocker: POST-DIRECTARRAY-REMAINING-DIRECT-PATH-SURFACE-CHECK-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-412-COLLECTION-METHOD-DIRECT-ARRAY-LANE-POST-RETIREMENT-PERF-OWNER-REFRESH.md
  - docs/development/current/main/phases/phase-296x/296x-378-ARRAY-REPR-DESIGN-ROW.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/representation-direct-lowering-ssot.md
  - tools/checks/k2_wide_phase296x_post_directarray_remaining_direct_path_surface_check_guard.sh
---

# 296x-413 Post-DirectArray Remaining Direct Path Surface Check

## Purpose

Close the direct-path fast-path search before returning to mimalloc source-level
optimization.

This row does not implement another fast path. It records whether any remaining
`.hako -> Representation/NativeDirect` surface still has enough evidence to
justify a new owner row.

## Contract

```text
output_contract=post-directarray-remaining-direct-path-surface-check-v0
input_contract=collection-method-direct-array-lane-post-retirement-perf-owner-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0

checked_surface_0=typed_object_legacy_field_helper
checked_surface_1=runtime_databox_consumer_surface
checked_surface_2=object_lifecycle_facade
checked_surface_3=public_arraybox_runtime_surface
checked_surface_4=directarray_optional_member
checked_surface_5=result_capsule_value_aggregate
checked_surface_6=page_model_page_queue

typed_object_legacy_field_helper_new_fast_path_open=0
runtime_databox_consumer_surface_new_fast_path_open=0
object_lifecycle_facade_new_fast_path_open=0
public_arraybox_runtime_surface_new_fast_path_open=0
directarray_optional_member_open=0
result_capsule_value_aggregate_open=0
page_model_page_queue_fast_path_open=0

new_fast_path_open=0
new_fast_path_owner=none
return_to_mimalloc_optimization=1
selected_next=mimalloc_source_level_owner_refresh

open_new_fast_path_only_if_positive_net_helper_delta=1
open_new_fast_path_only_if_perf_owner_pct_above_threshold=1
open_new_fast_path_only_if_selected_callsite_or_family=1
open_new_fast_path_only_if_no_recent_nonkeeper=1
open_new_fast_path_only_if_no_silent_fallback=1

optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Surface Closeout

### Typed Object Legacy Field Helper

Keep closed for now. Symbol presence alone is not perf evidence. Reopen only if
a post-DirectArray report still shows a concrete typed-object legacy helper
callsite or family with positive net helper-call delta.

### RuntimeDataBox Consumer Surface

Keep closed for now. Rows 394-401 split the RuntimeDataBox route and consumer
surface, and row398 extracted the route-policy source. Reopen only if the active
hot path still falls into `RuntimeDataBox.getField/setField` after those route
policy changes.

### Object Lifecycle Facade

Do not treat this as a new substrate fast path. If it remains hot, it belongs to
mimalloc `.hako` source/state-shape optimization, not another helper or
NativeDirect substrate row.

### Public ArrayBox Runtime Surface

Keep as public facade / fallback / materialization owner. Do not reinterpret
public ArrayBox handles as DirectArray handles.

### DirectArray Optional Member

Keep closed. Row388 already closed optional next-member selection, and row412
selects the existing ArrayRepr design handoff rather than another member.

### Result Capsule ValueAggregate

Keep closed for this lane. Earlier ValueAggregate attempts hit materialization
boundaries or small-helper closeout. Reopen only with a new positive-net owner
selection row.

### Page Model / Page Queue

Keep closed as a fast-path substrate candidate. Prior page-model/page-queue
retries have no-effect / non-keeper evidence. If they remain hot, return through
mimalloc source-level owner selection.

## Decision

No new fast path opens from this row.

Return to mimalloc optimization with the remaining work framed as source-level
owner refresh / `.hako` state-shape cleanup, not another helper micro-lane.

## Forbidden

- no new DirectArray member
- no helper micro-optimization
- no generic typed-field residence retry
- no RuntimeDataBox fallback widening
- no public ArrayBox handle reinterpretation
- no provider activation
- no allocator replacement
- no hook installation
- no `#[global_allocator]`

## Guard

```bash
bash tools/checks/k2_wide_phase296x_post_directarray_remaining_direct_path_surface_check_guard.sh
```
