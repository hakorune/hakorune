---
Status: Landed
Date: 2026-05-28
Scope: select the next owner after block-local typed-field residence proved non-feasible.
Blocker: CFG-RESIDENCE-OR-RUNTIME-OWNER-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-189-TYPED-OBJECT-HELPER-LOCK-COST-PROBE.md
  - docs/development/current/main/phases/phase-296x/296x-192-TYPED-OBJECT-RUNTIME-FAST-LANE-KEEPER-MEASUREMENT.md
  - docs/development/current/main/phases/phase-296x/296x-197-MIR-TYPED-FIELD-RESIDENCE-ERASURE-FEASIBILITY.md
---

# 296x-198 CFG Residence Or Runtime Owner Selection

## Purpose

Choose the next large owner after row196/197 showed that block-local typed-field
residence cannot erase helper calls for `HakoAllocPageModel.acquire_usize/1`.
This row is selection-only. It does not edit compiler/runtime code.

## Evidence Chain

```text
row189:
  dominant_helper_subowner=lock_global_slab
  recommended_next=runtime_single_thread_fast_lane

row192:
  runtime_fast_lane_keeper=1
  keeper_effect=accepted

row194:
  selected_method=HakoAllocPageModel.acquire_usize/1
  selected_method_dynamic_eligible_estimate=9961472

row196:
  attempted_shape=block_local_residence_with_block_end_writeback
  erased_helper_call_count=0
  keeper_effect=no_effect
  implementation_landed=0

row197:
  net_helper_call_delta=0
  block_local_residence_feasible=0
  implementation_recommendation=do_not_implement_block_local_residence
```

## Decision

```text
Decision: accepted

selected_next_owner=cfg_aware_typed_field_residence_design
selection_reason=runtime_fast_lane_already_accepted_but_helper_calls_remain_large
rejected_next_owner=retry_block_local_typed_field_residence
rejected_next_owner_reason=net_helper_call_delta_is_zero
runtime_owner_status=single_thread_exact_store_landed_as_floor
transform_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Boundary

The next compiler-side owner must be a real CFG-aware residence design, not a
second block-local rewrite. The design must explicitly own:

```text
- field load initialization
- dirty field writeback
- branch/merge behavior
- PHI interaction
- unknown call barriers
- return barriers
- fallback to typed-object helper ABI
```

Do not implement the transform until that ownership is written as an SSOT and
an inventory/plan guard proves the selected method has positive net erasure
under the CFG-aware policy.

## Rejected

```text
retry_block_local_residence:
  rejected because row197 proves net_helper_call_delta=0

typed_object_runtime_retry:
  rejected for this row because SingleThreadExactStore is already the runtime
  floor and the remaining C parity gap needs helper-call erasure, not another
  storage backend guess

ArrayBox optimization:
  rejected for this row because the active evidence chain selected typed-field
  helper calls first
```

## Next

```text
row199:
  cfg_aware_typed_field_residence_ssot

Goal:
  define CFG-aware residence ownership and fail-fast barriers before any
  compiler transform.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_cfg_residence_or_runtime_owner_selection_guard.sh
```
