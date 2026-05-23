---
Status: Landed
Date: 2026-05-23
Scope: select the next owner-local production exact `usize` field group after the bounded decommit policy counter migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-98-HAKO-ALLOC-USIZE-BOUNDED-DECOMMIT-POLICY-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/heap_reuse_priority_box.hako
  - tools/checks/k2_wide_hako_alloc_heap_reuse_priority_policy_guard.sh
---

# 294x-99 Hako Alloc Usize Heap Reuse Priority Counter Selection

## Decision

Select the owner-local monotonic counters in
`HakoAllocHeapReusePriorityPolicy` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-121`:

- `select_count`
- `active_pick_count`
- `recommitted_pick_count`
- `retired_pick_count`
- `fresh_pick_count`
- `decommitted_skip_count`
- `missing_skip_count`

These fields count M208 heap reuse priority policy selections and read-only
skip outcomes. They do not carry route vocabulary, page identity, lifecycle
state, candidate snapshot report fields, or mutation/execution state.

## Stop Line

This selection does not migrate:

- `HakoAllocHeapReusePriorityDecision` fields, because they are route/status,
  page-id, lifecycle-state, flag, and candidate snapshot report vocabulary;
- `last_route`, because it is route vocabulary;
- `last_page_id`, because it uses the `-1` signed sentinel;
- page lifecycle observer counters, heap/page queues, page-source adapters,
  heap/page mutation, OSVM byte/pointer payloads, provider / hook /
  global-allocator rows, TLS, atomics, or `#[global_allocator]`.

## Verification

Selection-only row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
