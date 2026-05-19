# 293x-818 HAKO-ALLOC-USIZE-FIELD-GROUP-010 Allocation-Apply Byte/Capacity Migration

Status: landed
Date: 2026-05-19

## Decision

Migrate the modeled allocation-apply report byte/capacity owner group to exact
`usize`.

This row follows the allocation-ledger report and diagnostic mirror migrations
by moving one step upstream to the allocation-apply report that feeds the
allocation-ledger family.

## Scope

Change only these fields on
`HakoAllocSegmentArenaBackingModeledAllocationApplyReport`:

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
allocation-plan facts and apply-route inputs. They are already proven
downstream on the allocation-ledger and release-candidate families.

## Stop Lines

- No allocation-plan migration in this row.
- No allocation-apply diagnostic mirror migration in this row.
- No allocation-ledger or release-candidate migration in this row.
- No migration of apply counters.
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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_apply_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_apply_diagnostics_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_apply_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Notes

- Migrated only the
  `HakoAllocSegmentArenaBackingModeledAllocationApplyReport` byte/capacity
  owner group to exact `usize` storage.
- Kept counters, reasons, tokens, ids, and the `row_index = -1` sentinel on
  `i64`.
- Strengthened the MIMAP-272A guard to assert exact `usize` typed-object
  storage for the allocation-apply byte/capacity fields.
- Strengthened the MIMAP-273A diagnostics guard to prove the allocation-apply
  diagnostic mirror byte fields remain `i64` in this row.
- Re-ran the MIMAP-274A closeout guard after fixing its historical MIMAP-275A
  status expectation from `selected current` to `landed`.

## Next

Select `HAKO-ALLOC-USIZE-FIELD-GROUP-011` to close out the allocation-apply
byte/capacity field group before selecting another allocator byte/capacity
group.
