# 293x-814 HAKO-ALLOC-USIZE-FIELD-GROUP-006 Allocation-Ledger Byte/Capacity Migration

Status: landed
Date: 2026-05-19

## Decision

Migrate the modeled allocation-ledger report byte/capacity owner group to exact
`usize`.

This row follows the release-candidate report and diagnostic mirror migrations
by moving back to the owner-local allocation-ledger report that feeds the
release-candidate family.

## Scope

Change only these fields on
`HakoAllocSegmentArenaBackingModeledAllocationLedgerReport`:

```text
source_capacity
source_committed_bytes
source_uncommitted_bytes
padded_bytes
slot_capacity
planned_backing_bytes
planned_committed_bytes
applied_backing_bytes
applied_committed_bytes
remaining_source_bytes
```

These are non-negative modeled byte/capacity facts copied from accepted
allocation-apply facts. They are already proven downstream on the
release-candidate report.

## Stop Lines

- No allocation-apply or allocation-plan migration in this row.
- No diagnostic mirror migration in this row.
- No migration of ledger counters.
- No migration of reason/status/flag vocabularies.
- No migration of token/id/generation fields.
- No migration of `row_index` or any `-1` sentinel-bearing field.
- No new backend route or `.inc` owner-name matcher.
- No runtime sum materialization.
- No cross-function `Result` direct ABI.
- No real raw pointer residence, arena backing execution, segment-map mutation,
  atomic bitmap execution, OSVM/page-source execution, provider activation,
  hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_diagnostics_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Notes

- Migrated only the allocation-ledger report byte/capacity fields to exact
  `usize`.
- Extended the MIMAP-276A guard to verify exact `usize` storage for the migrated
  fields and explicit `i64` storage for reason, row-index, and ledger-token
  fields.
- Extended the MIMAP-277A diagnostics guard to keep downstream diagnostic mirror
  bytes on `i64` for this row.
- Updated the historical MIMAP-278A closeout guard so it accepts the already
  landed MIMAP-279A row selection instead of requiring that old row to remain
  `selected current`.
- Re-ran MIMAP-276A L2, MIMAP-277A L2, and MIMAP-278A closeout L3 evidence.

## Next

Select `HAKO-ALLOC-USIZE-FIELD-GROUP-007` to close out the allocation-ledger
byte/capacity field group before selecting another allocator byte/capacity
group.
