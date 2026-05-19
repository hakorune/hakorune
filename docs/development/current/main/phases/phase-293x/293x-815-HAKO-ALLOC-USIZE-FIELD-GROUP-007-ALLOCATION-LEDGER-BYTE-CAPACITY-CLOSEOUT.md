# 293x-815 HAKO-ALLOC-USIZE-FIELD-GROUP-007 Allocation-Ledger Byte/Capacity Closeout

Status: selected current
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

## Next

After this closeout, select the next narrow allocator byte/capacity group only
if its owner-local invariant and sentinel policy are as small as this group.
