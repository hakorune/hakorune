# 293x-823 HAKO-ALLOC-USIZE-FIELD-GROUP-015 Allocation-Plan Byte/Capacity Closeout

Status: landed
Date: 2026-05-19

## Decision

Close out the allocation-plan byte/capacity exact-`usize` field group after
`HAKO-ALLOC-USIZE-FIELD-GROUP-014`.

This row should prove that the allocation-plan report byte/capacity fields are
now part of the current production `usize` storage inventory and that the
allocation-plan diagnostics and representative pure-first EXE route remain
stable.

## Scope

- Freeze the allocation-plan byte/capacity field-group migration evidence.
- Keep the group limited to:

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

- Keep allocation-plan counters, reason/status flags, tokens, ids, and
  sentinel-bearing fields on `i64`.
- Keep the allocation-plan diagnostic mirror byte fields on `i64`; those belong
  to a separate row.
- Confirm `NUMERIC_FIELDS.md` lists this group as current production `usize`
  storage, not selected/pending storage.
- Use the existing MIMAP-268A / MIMAP-269A / MIMAP-270A guards as evidence.

## Stop Lines

- No second field-group migration in this row.
- No broad `i64` to `usize` rewrite.
- No allocation-plan diagnostic mirror migration.
- No allocation-apply, allocation-ledger, or release-candidate migration.
- No migration of counters.
- No migration of reason/status/flag vocabularies.
- No migration of token/id/generation fields.
- No migration of any sentinel-bearing field.
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

## Landed Notes

- Re-ran the MIMAP-268A allocation-plan L2 guard after the byte/capacity
  migration.
- Re-ran the MIMAP-269A allocation-plan diagnostics L2 guard, confirming the
  diagnostic mirror byte fields still remain `i64`.
- Re-ran the MIMAP-270A allocation-plan closeout guard, including
  representative exact MIR -> pure-first EXE evidence for the allocation-plan
  diagnostics proof app.
- Confirmed `NUMERIC_FIELDS.md` lists the allocation-plan byte/capacity group as
  current production `usize` storage.

## Next

Select `HAKO-ALLOC-USIZE-FIELD-GROUP-016` to migrate the allocation-plan
diagnostic mirror byte fields that copy already-migrated allocation-plan byte
facts.
