# 293x-811 HAKO-ALLOC-USIZE-FIELD-GROUP-003 Release-Candidate Byte/Capacity Closeout

Status: landed
Date: 2026-05-19

## Decision

Close out the first allocator exact-`usize` stored field group after
`HAKO-ALLOC-USIZE-FIELD-GROUP-002`.

This row should prove that the release-candidate byte/capacity field group is
now the current production `usize` group and that the surrounding diagnostics
and representative pure-first EXE route still work.

## Scope

- Freeze the release-candidate byte/capacity field-group migration evidence.
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

- Keep reason/status/token/id/generation/sentinel fields on `i64`.
- Confirm `NUMERIC_FIELDS.md` lists this group as current production `usize`
  storage, not selected/pending storage.
- Use the existing MIMAP-280A / MIMAP-281A / MIMAP-282A guards as evidence; add
  a dedicated thin closeout guard only if this row needs an extra static
  contract.

## Stop Lines

- No second field-group migration in this row.
- No broad `i64` to `usize` rewrite.
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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_diagnostics_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Notes

- Re-ran the MIMAP-280A inventory L2 guard after the exact `usize` field
  migration.
- Re-ran the MIMAP-281A diagnostics L2 guard after the exact `usize` field
  migration.
- Re-ran the MIMAP-282A closeout L3 guard, including exact MIR -> pure-first
  EXE evidence for the release-candidate diagnostics proof app.
- Confirmed `NUMERIC_FIELDS.md` lists the release-candidate byte/capacity group
  as current production `usize` storage.

## Next

Select `HAKO-ALLOC-USIZE-FIELD-GROUP-004` to migrate the observer-only
release-candidate diagnostic mirror byte fields:

```text
last_report_applied_backing_bytes
last_report_applied_committed_bytes
last_report_remaining_source_bytes
```

The migration stays downstream of the already-migrated release-candidate report
group and keeps reason/status/token/id/count fields on `i64`.
