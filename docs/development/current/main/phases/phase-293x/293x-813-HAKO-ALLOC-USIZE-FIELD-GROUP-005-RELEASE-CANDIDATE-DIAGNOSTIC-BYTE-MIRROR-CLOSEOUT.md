# 293x-813 HAKO-ALLOC-USIZE-FIELD-GROUP-005 Release-Candidate Diagnostic Byte Mirror Closeout

Status: landed
Date: 2026-05-19

## Decision

Close out the release-candidate diagnostic byte mirror exact-`usize` field
group after `HAKO-ALLOC-USIZE-FIELD-GROUP-004`.

This row should prove that the observer-only diagnostic mirror fields are now
part of the current production `usize` storage inventory and that the existing
diagnostic and representative pure-first EXE route remain stable.

## Scope

- Freeze the diagnostic mirror field-group migration evidence.
- Keep the group limited to:

```text
last_report_applied_backing_bytes
last_report_applied_committed_bytes
last_report_remaining_source_bytes
```

- Keep diagnostic counters, reason/status flags, tokens, ids, and sentinels on
  `i64`.
- Confirm `NUMERIC_FIELDS.md` lists this group as current production `usize`
  storage, not selected/pending storage.
- Use the existing MIMAP-281A / MIMAP-282A guards as evidence; add a dedicated
  thin closeout guard only if this row needs an extra static contract.

## Stop Lines

- No second field-group migration in this row.
- No broad `i64` to `usize` rewrite.
- No migration of diagnostic counters.
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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_diagnostics_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Notes

- Re-ran the MIMAP-281A diagnostics L2 guard after the diagnostic mirror field
  migration.
- Re-ran the MIMAP-282A closeout L3 guard after the diagnostic mirror field
  migration.
- Confirmed `NUMERIC_FIELDS.md` lists the diagnostic mirror byte group as
  current production `usize` storage.

## Next

Select `HAKO-ALLOC-USIZE-FIELD-GROUP-006` to migrate the allocation-ledger
byte/capacity owner group:

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

This returns migration to the owner-local report that feeds the release
candidate family, instead of migrating more observer-only mirrors first.
