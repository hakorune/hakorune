---
Status: Landed
Date: 2026-05-23
Scope: select the next owner-local production exact `usize` field group after the page lifecycle observer counter migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-102-HAKO-ALLOC-USIZE-PAGE-LIFECYCLE-OBSERVER-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/abandoned_reclaim_inventory_box.hako
  - tools/checks/k2_wide_hako_alloc_abandoned_reclaim_inventory_guard.sh
---

# 294x-103 Hako Alloc Usize Abandoned Reclaim Inventory Counter Selection

## Decision

Select the owner-local monotonic counters in
`HakoAllocAbandonedReclaimInventory` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-125`:

- `classify_count`
- `candidate_count`
- `reject_count`
- `missing_backing_reject_count`
- `owner_active_reject_count`
- `remote_pending_reject_count`
- `decommitted_reject_count`
- `abandoned_live_count`
- `abandoned_retired_count`
- `purge_forward_candidate_count`

These fields count M213 abandoned/reclaim inventory classifications and
read-only reject/candidate outcomes. They do not carry page identity, thread
identity, reason vocabulary, backing bytes, or execution state.

## Stop Line

This selection does not migrate:

- `HakoAllocAbandonedReclaimDecision` fields, because they are status, reason,
  page/thread id, flag, and byte payload vocabulary;
- `last_page_id`, because it uses the `-1` signed sentinel;
- `last_reason`, because it is reason vocabulary;
- reclaim scheduling/execution, atomics, remote-free draining, page-source
  calls, OSVM byte/pointer payloads, provider / hook / global-allocator rows,
  TLS, or `#[global_allocator]`.

## Verification

Selection-only row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
