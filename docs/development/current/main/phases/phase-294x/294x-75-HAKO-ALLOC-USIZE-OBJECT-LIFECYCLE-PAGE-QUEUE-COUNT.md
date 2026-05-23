---
Status: Landed
Date: 2026-05-23
Scope: object-lifecycle page-queue count/page-count exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-74-HAKO-ALLOC-USIZE-OBJECT-LIFECYCLE-PAGE-QUEUE-COUNT-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/object_lifecycle_page_queue_box.hako
  - apps/mimalloc-object-lifecycle-queue-proof/main.hako
  - apps/mimalloc-facade-object-lifecycle-queue-proof/main.hako
  - tools/checks/k2_wide_mimalloc_object_lifecycle_queue_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_facade_object_lifecycle_queue_exe_guard.sh
---

# 294x-75 Hako Alloc Usize Object Lifecycle Page Queue Count

## Decision

Migrate only the `HakoAllocObjectLifecyclePageQueue` count/page-count group to
exact `usize` storage:

- `page_count`
- `add_count`
- `request_count`
- `select_count`
- `reuse_select_count`
- `active_select_count`
- `decommitted_skip_count`
- `retired_skip_count`
- `unavailable_skip_count`
- `miss_count`
- `reject_count`

The queue now reuses exact `page_count` as the selection-loop bound, while the
selection observers and reject seams stay signed. The proof apps and EXE guards
now publish/assert the full migrated counter surface so the monotonic queue
state is exercised through both the direct queue route and the facade route.

## Stop Line

This row does not migrate:

- `last_selected_index`, `last_selected_page_id`, or `last_selected_kind`;
- the `addPage()` `-1` reject seam or any page-id/index failure vocabulary;
- `HakoAllocObjectLifecycleFacade` result/report state;
- `HakoAllocOsVmBackedFastPathHeap.next_page_id` because `addFreshPage()`
  still emits the signed `next_page_id - 1` seam;
- page/block identity payloads, pointer-like fields, provider activation, hooks,
  TLS, atomics, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_object_lifecycle_queue_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_object_lifecycle_queue_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
