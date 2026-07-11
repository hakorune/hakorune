---
Status: Landed
Date: 2026-05-23
Scope: select object-lifecycle page-queue count/page-count exact `usize` row.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/object_lifecycle_page_queue_box.hako
---

# 294x-74 Hako Alloc Usize Object Lifecycle Page Queue Count Selection

## Decision

Select the `HakoAllocObjectLifecyclePageQueue` count/page-count group as
`HAKO-ALLOC-USIZE-FIELD-GROUP-097`.

This owner is queue-local, monotonic, and non-negative across the V2 small-path
proof route. `page_count`, `add_count`, `request_count`, `select_count`,
`reuse_select_count`, `active_select_count`, `decommitted_skip_count`,
`retired_skip_count`, `unavailable_skip_count`, `miss_count`, and
`reject_count` are owner-local counters that do not carry pointer-like or
negative semantics, while the signed selected-index/page-id observers stay on
their existing seam.

## Stop Line

The follow-on row must not migrate:

- `last_selected_index`, `last_selected_page_id`, or `last_selected_kind`;
- the `addPage()` `-1` reject seam or any other signed selection vocabulary;
- `HakoAllocObjectLifecycleFacade` result/report state;
- `HakoAllocOsVmBackedFastPathHeap.next_page_id` because `addFreshPage()`
  still feeds the signed `next_page_id - 1` seam;
- page/block identity payloads, pointer-like fields, provider activation, hooks,
  TLS, atomics, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
