# 293x-822 HAKO-ALLOC-USIZE-FIELD-GROUP-014 Allocation-Plan Byte/Capacity Migration

Status: selected current
Date: 2026-05-19

## Decision

Migrate the modeled allocation-plan report byte/capacity owner group to exact
`usize`.

This row continues moving upstream from allocation-apply to the allocation-plan
owner that feeds it. The group contains non-negative modeled byte/capacity facts
only; reason/status/token/id/sentinel fields remain signed.

## Scope

Change only these fields on
`HakoAllocSegmentArenaBackingModeledAllocationPlanReport`:

```text
source_capacity
source_committed_bytes
source_uncommitted_bytes
padded_bytes
slot_capacity
planned_backing_bytes
planned_committed_bytes
remaining_source_bytes
```

These are non-negative modeled byte/capacity facts copied from accepted
source-accounting facts and allocation-plan route inputs. They are already
proven downstream on the allocation-apply, allocation-ledger, and
release-candidate families.

## Stop Lines

- No allocation-plan diagnostic mirror migration in this row.
- No allocation-apply, allocation-ledger, or release-candidate migration in this
  row.
- No migration of plan counters.
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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_plan_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_plan_diagnostics_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_plan_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

After this migration, close out the allocation-plan byte/capacity field group
before selecting another allocator byte/capacity group.
