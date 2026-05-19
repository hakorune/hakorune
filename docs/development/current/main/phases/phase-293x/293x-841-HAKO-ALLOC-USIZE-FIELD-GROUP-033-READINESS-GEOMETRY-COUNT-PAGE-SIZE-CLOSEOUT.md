# 293x-841 HAKO-ALLOC-USIZE-FIELD-GROUP-033 Readiness Geometry Count / Page-Size Closeout

Status: selected current
Date: 2026-05-19

## Decision

Close out the readiness geometry count / page-size exact-`usize` field group
after `HAKO-ALLOC-USIZE-FIELD-GROUP-032`.

This row should prove that the readiness geometry count / page-size fields are
now part of the current production `usize` storage inventory and that the
readiness closeout route plus immediate requirement-matrix consumer remain
stable.

## Scope

- Freeze the readiness geometry count / page-size field-group migration
  evidence.
- Keep the group limited to:

```text
slice_count
committed_slices
free_slices
page_size
```

- Keep counters, reasons, ids, alignments, flags, and sentinel-bearing fields on
  `i64`.
- Confirm `NUMERIC_FIELDS.md` lists this group as current production `usize`
  storage, not selected/pending storage.

## Stop Lines

- No second field-group migration in this row.
- No broad `i64` to `usize` rewrite.
- No requirement-matrix, residence arena-binding, arena-slot, source-bridge,
  source-accounting, allocation-plan, allocation-apply, allocation-ledger, or
  release-candidate migration.
- No migration of counters.
- No migration of reason/status/flag vocabularies.
- No migration of token/id/generation fields.
- No migration of alignment fields, including `required_alignment`.
- No new backend route or `.inc` owner-name matcher.
- No runtime sum materialization.
- No cross-function `Result` direct ABI.
- No real raw pointer residence, arena backing execution, segment-map mutation,
  atomic bitmap execution, OSVM/page-source execution, provider activation,
  hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_readiness_inventory_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_readiness_diagnostics_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_readiness_closeout_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_requirement_matrix_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

After this closeout, select the next allocator exact-`usize` field group.
