# 293x-819 HAKO-ALLOC-USIZE-FIELD-GROUP-011 Allocation-Apply Byte/Capacity Closeout

Status: selected current
Date: 2026-05-19

## Decision

Close out the allocation-apply byte/capacity exact-`usize` field group after
`HAKO-ALLOC-USIZE-FIELD-GROUP-010`.

This row should prove that the allocation-apply report byte/capacity fields are
now part of the current production `usize` storage inventory and that the
allocation-apply diagnostics and representative pure-first EXE route remain
stable.

## Scope

- Freeze the allocation-apply byte/capacity field-group migration evidence.
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

- Keep allocation-apply counters, reason/status flags, tokens, ids, and
  sentinel-bearing fields on `i64`.
- Keep the allocation-apply diagnostic mirror byte fields on `i64`; those belong
  to a separate row.
- Confirm `NUMERIC_FIELDS.md` lists this group as current production `usize`
  storage, not selected/pending storage.
- Use the existing MIMAP-272A / MIMAP-273A / MIMAP-274A guards as evidence.

## Stop Lines

- No second field-group migration in this row.
- No broad `i64` to `usize` rewrite.
- No allocation-plan migration.
- No allocation-apply diagnostic mirror migration.
- No allocation-ledger or release-candidate migration.
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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_apply_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_apply_diagnostics_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_apply_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

After this closeout, select a separate row for the allocation-apply diagnostic
mirror byte fields if the current guard evidence remains stable.
