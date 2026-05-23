---
Status: Landed
Date: 2026-05-23
Scope: select the next owner-local production exact `usize` field group after the decommit-side purge marker counter migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-94-HAKO-ALLOC-USIZE-PURGE-DECOMMIT-MARKER-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/purge_decommit_state_marker_box.hako
  - tools/checks/k2_wide_hako_alloc_recommit_marker_transition_guard.sh
---

# 294x-95 Hako Alloc Usize Purge Recommit Marker Counter Selection

## Decision

Select the recommit-side owner-local monotonic counters in
`HakoAllocPurgeDecommitStateMarker` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-117`:

- `recommit_attempt_count`
- `recommitted_count`
- `recommit_reject_count`
- `duplicate_recommit_count`
- `missing_recommit_report_count`
- `not_recommitted_count`
- `recommit_widened_reject_count`
- `unmarked_recommit_reject_count`

These fields count M204 recommit marker transition attempts, accepted
transitions, and reject reasons. They do not carry page ids, marker arrays,
report flags, or page-source / heap execution state.

## Stop Line

This selection does not migrate:

- `marked_page_ids` or `recommitted_page_ids`, because they are handle-backed
  marker state;
- `last_page_id`, because it is a signed page-id seam with `-1`;
- `HakoAllocPurgeRecommitStateMarkReport` fields, because they are status /
  page-id / flag report vocabulary;
- decommit-side marker counters, because they were closed by
  `HAKO-ALLOC-USIZE-FIELD-GROUP-115`;
- page-source calls, heap/page mutation, OSVM byte/pointer payloads, provider /
  hook / global-allocator rows, TLS, atomics, or `#[global_allocator]`.

## Verification

Selection-only row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
