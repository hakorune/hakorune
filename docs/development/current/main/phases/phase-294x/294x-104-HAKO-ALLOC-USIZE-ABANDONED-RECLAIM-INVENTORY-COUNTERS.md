---
Status: Landed
Date: 2026-05-23
Scope: abandoned/reclaim inventory owner-local counter exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-103-HAKO-ALLOC-USIZE-ABANDONED-RECLAIM-INVENTORY-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/abandoned_reclaim_inventory_box.hako
  - apps/hako-alloc-abandoned-reclaim-inventory-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_abandoned_reclaim_inventory_guard.sh
---

# 294x-104 Hako Alloc Usize Abandoned Reclaim Inventory Counters

## Decision

Migrate only the selected `HakoAllocAbandonedReclaimInventory` owner-local
monotonic counters to exact `usize` storage:

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

The M213 abandoned/reclaim inventory guard now asserts these fields are exact
`usize` in the typed-object plan.

## Stop Line

This row does not migrate:

- `HakoAllocAbandonedReclaimDecision` fields, because they are status, reason,
  page/thread id, flag, and byte payload vocabulary;
- `last_page_id`, because it uses the `-1` signed sentinel;
- `last_reason`, because it is reason vocabulary;
- reclaim scheduling/execution, atomics, remote-free draining, page-source
  calls, OSVM byte/pointer payloads, provider / hook / global-allocator rows,
  TLS, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_abandoned_reclaim_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
