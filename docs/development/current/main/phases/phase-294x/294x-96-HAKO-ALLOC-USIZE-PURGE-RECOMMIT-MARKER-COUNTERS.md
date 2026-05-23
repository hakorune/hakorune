---
Status: Landed
Date: 2026-05-23
Scope: recommit-side purge marker owner-local counter exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-95-HAKO-ALLOC-USIZE-PURGE-RECOMMIT-MARKER-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/purge_decommit_state_marker_box.hako
  - apps/hako-alloc-recommit-marker-transition-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_recommit_marker_transition_guard.sh
---

# 294x-96 Hako Alloc Usize Purge Recommit Marker Counters

## Decision

Migrate only the selected recommit-side
`HakoAllocPurgeDecommitStateMarker` owner-local monotonic counters to exact
`usize` storage:

- `recommit_attempt_count`
- `recommitted_count`
- `recommit_reject_count`
- `duplicate_recommit_count`
- `missing_recommit_report_count`
- `not_recommitted_count`
- `recommit_widened_reject_count`
- `unmarked_recommit_reject_count`

The M204 recommit marker transition guard now asserts these fields are exact
`usize` in the typed-object plan.

## Stop Line

This row does not migrate:

- `marked_page_ids` or `recommitted_page_ids`, because they are handle-backed
  marker state;
- `last_page_id`, because it is a signed page-id seam with `-1`;
- decommit/recommit report fields, because they are status / page-id / flag
  report vocabulary;
- page-source calls, heap/page mutation, OSVM byte/pointer payloads, provider /
  hook / global-allocator rows, TLS, atomics, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_recommit_marker_transition_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
