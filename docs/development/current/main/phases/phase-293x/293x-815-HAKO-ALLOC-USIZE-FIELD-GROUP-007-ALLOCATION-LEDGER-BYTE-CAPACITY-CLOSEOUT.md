# 293x-815 HAKO-ALLOC-USIZE-FIELD-GROUP-007 Allocation-Ledger Byte/Capacity Closeout

Status: landed
Date: 2026-05-19

## Decision

Close out the allocation-ledger byte/capacity exact-`usize` field group after
`HAKO-ALLOC-USIZE-FIELD-GROUP-006`.

This row should prove that the allocation-ledger report byte/capacity group is
now part of the current production `usize` storage inventory and that the
existing allocation-ledger diagnostics and representative pure-first EXE route
remain stable.

## Scope

- Freeze the allocation-ledger byte/capacity field-group migration evidence.
- Keep the group limited to:

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

- Keep allocation-ledger counters, reason/status flags, tokens, ids, generation,
  and sentinels on `i64`.
- Keep downstream allocation-ledger diagnostic mirror fields on `i64` until
  their own row selects them.
- Confirm `NUMERIC_FIELDS.md` lists this group as current production `usize`
  storage, not selected/pending storage.

## Stop Lines

- No second field-group migration in this row.
- No broad `i64` to `usize` rewrite.
- No allocation-plan or allocation-apply migration.
- No diagnostic mirror migration.
- No migration of counters.
- No migration of reason/status/flag vocabularies.
- No migration of token/id/generation fields.
- No migration of `row_index` or any sentinel-bearing field.
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

- Re-ran the MIMAP-276A allocation-ledger L2 guard after the exact `usize`
  field migration.
- Re-ran the MIMAP-277A diagnostics L2 guard after the exact `usize` field
  migration while keeping diagnostic mirror bytes on `i64`.
- Re-ran the MIMAP-278A closeout L3 guard, including exact MIR -> pure-first EXE
  evidence for the allocation-ledger diagnostics proof app.
- Confirmed `NUMERIC_FIELDS.md` lists the allocation-ledger byte/capacity group
  as current production `usize` storage.

## Next

Select `HAKO-ALLOC-USIZE-FIELD-GROUP-008` to migrate the observer-only
allocation-ledger diagnostic mirror byte fields:

```text
last_report_applied_backing_bytes
last_report_applied_committed_bytes
last_report_remaining_source_bytes
```

The migration stays downstream of the already-migrated allocation-ledger report
group and keeps reason/status/token/id/count fields on `i64`.
