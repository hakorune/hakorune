---
Status: Landed
Date: 2026-05-23
Scope: decommit-side purge marker owner-local counter exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-93-HAKO-ALLOC-USIZE-PURGE-DECOMMIT-MARKER-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/purge_decommit_state_marker_box.hako
  - apps/hako-alloc-purge-decommit-state-marker-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_purge_decommit_state_marker_guard.sh
---

# 294x-94 Hako Alloc Usize Purge Decommit Marker Counters

## Decision

Migrate only the selected decommit-side
`HakoAllocPurgeDecommitStateMarker` owner-local monotonic counters to exact
`usize` storage:

- `attempt_count`
- `marked_count`
- `reject_count`
- `duplicate_count`
- `missing_report_count`
- `not_decommitted_count`
- `release_field_reject_count`

The M198 purge decommit state marker guard now asserts these fields are exact
`usize` in the typed-object plan.

## Stop Line

This row does not migrate:

- `marked_page_ids` or `recommitted_page_ids`, because they are handle-backed
  marker state;
- `last_page_id`, because it is a signed page-id seam with `-1`;
- decommit/recommit report fields, because they are status / page-id / flag
  report vocabulary;
- recommit-side marker counters, because they belong to the M204 transition
  path and need their own field-group row;
- page-source calls, heap/page mutation, OSVM byte/pointer payloads, provider /
  hook / global-allocator rows, TLS, atomics, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_purge_decommit_state_marker_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
