---
Status: Landed
Date: 2026-05-23
Scope: heap reuse priority owner-local counter exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-99-HAKO-ALLOC-USIZE-HEAP-REUSE-PRIORITY-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/heap_reuse_priority_box.hako
  - apps/hako-alloc-heap-reuse-priority-policy-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_heap_reuse_priority_policy_guard.sh
---

# 294x-100 Hako Alloc Usize Heap Reuse Priority Counters

## Decision

Migrate only the selected `HakoAllocHeapReusePriorityPolicy` owner-local
monotonic counters to exact `usize` storage:

- `select_count`
- `active_pick_count`
- `recommitted_pick_count`
- `retired_pick_count`
- `fresh_pick_count`
- `decommitted_skip_count`
- `missing_skip_count`

The M208 heap reuse priority guard now asserts these fields are exact `usize`
in the typed-object plan.

## Stop Line

This row does not migrate:

- `HakoAllocHeapReusePriorityDecision` fields, because they are route/status,
  page-id, lifecycle-state, flag, and candidate snapshot report vocabulary;
- `last_route`, because it is route vocabulary;
- `last_page_id`, because it uses the `-1` signed sentinel;
- page lifecycle observer counters, heap/page queues, page-source adapters,
  heap/page mutation, OSVM byte/pointer payloads, provider / hook /
  global-allocator rows, TLS, atomics, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_heap_reuse_priority_policy_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
