---
Status: Landed
Date: 2026-05-28
Scope: apply the .hako reason-local bind keeper for objectLifecycleSmallAlloc failure returns.
Blocker: HAKO-MIMALLOC-SMALL-ALLOC-HAKO-REASON-BIND-KEEPER-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-122-HAKO-MIMALLOC-SMALL-ALLOC-HAKO-REASON-BIND-PROBE.md
---

# 296x-123 Hako Mimalloc Small Alloc Hako Reason Bind Keeper

## Purpose

Row122 proved the temporary `.hako` reason-local bind removes duplicate reason
calls:

```text
before_reason_call_count=10
after_reason_call_count=5
after_duplicate_reason_call_count=0
next_action=apply_hako_reason_bind_keeper
```

Land the smallest source keeper in
`lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako`.

## Required Output

```text
output_contract=hako-mimalloc-small-alloc-hako-reason-bind-keeper-v0
input_contract=hako-mimalloc-small-alloc-hako-reason-bind-probe-v0
selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
reason_call_count=5
duplicate_reason_call_count=0
semantic_summary=ok
next_action=post_hako_reason_bind_measurement
summary=ok
```

## Stop Line

Do not add MIR CSE, reason singleton lowering, or another allocator keeper in
this row.

## Evidence

Report:

```text
output_contract=hako-mimalloc-small-alloc-hako-reason-bind-keeper-v0
input_contract=hako-mimalloc-small-alloc-hako-reason-bind-probe-v0
selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
keeper=small_alloc_hako_reason_bind
keeper_kind=box_count
reason_call_count=5
duplicate_reason_call_count=0
allocation_count=524288
free_count=524288
select_page_single_fast_path_count=524288
release_known_page_fast_path_count=524288
semantic_summary=ok
next_action=post_hako_reason_bind_measurement
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_small_alloc_hako_reason_bind_keeper_guard.sh
```
