---
Status: Landed
Date: 2026-05-23
Scope: select the next owner-local production exact `usize` field group after the page-source decommit adapter counter migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-92-HAKO-ALLOC-USIZE-PAGE-SOURCE-DECOMMIT-ADAPTER-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/purge_decommit_state_marker_box.hako
  - tools/checks/k2_wide_hako_alloc_purge_decommit_state_marker_guard.sh
---

# 294x-93 Hako Alloc Usize Purge Decommit Marker Counter Selection

## Decision

Select the decommit-side owner-local monotonic counters in
`HakoAllocPurgeDecommitStateMarker` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-115`:

- `attempt_count`
- `marked_count`
- `reject_count`
- `duplicate_count`
- `missing_report_count`
- `not_decommitted_count`
- `release_field_reject_count`

These fields count decommit marker attempts, accepted marks, and reject
reasons. They do not carry page ids, marker arrays, recommit transition state,
status vocabulary, or heap/page payload.

## Stop Line

This selection does not migrate:

- `marked_page_ids` or `recommitted_page_ids`, because they are handle-backed
  marker state;
- `last_page_id`, because it is a signed page-id seam with `-1`;
- `HakoAllocPurgeDecommitStateMarkReport` fields, because they are status /
  page-id / flag report vocabulary;
- recommit-side marker counters, because they belong to the M204 transition
  path and need their own field-group row;
- page-source calls, heap/page mutation, OSVM byte/pointer payloads, provider /
  hook / global-allocator rows, TLS, atomics, or `#[global_allocator]`.

## Verification

Selection-only row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
