# 293x-837 HAKO-ALLOC-USIZE-FIELD-GROUP-029 Residence Arena-Binding Geometry Count / Page-Size Closeout

Status: selected current
Date: 2026-05-19

## Decision

Close out the residence arena-binding geometry count / page-size exact-`usize`
field group after `HAKO-ALLOC-USIZE-FIELD-GROUP-028`.

This row should prove that the residence arena-binding geometry count /
page-size fields are now part of the current production `usize` storage
inventory and that the residence arena-binding closeout route plus immediate
arena-slot consumer remain stable.

## Scope

- Freeze the residence arena-binding geometry count / page-size field-group
  migration evidence.
- Keep the group limited to:

```text
slice_count
committed_slices
free_slices
page_size
```

- Keep counters, reasons, tokens, ids, generations, alignments, row index, and
  sentinel-bearing fields on `i64`.
- Confirm `NUMERIC_FIELDS.md` lists this group as current production `usize`
  storage, not selected/pending storage.

## Stop Lines

- No second field-group migration in this row.
- No broad `i64` to `usize` rewrite.
- No arena-slot, source-bridge, source-accounting, allocation-plan,
  allocation-apply, allocation-ledger, or release-candidate migration.
- No migration of counters.
- No migration of reason/status/flag vocabularies.
- No migration of token/id/generation fields.
- No migration of alignment fields, including `required_alignment`.
- No migration of `row_index` or any `-1` sentinel-bearing field.
- No new backend route or `.inc` owner-name matcher.
- No runtime sum materialization.
- No cross-function `Result` direct ABI.
- No real raw pointer residence, arena backing execution, segment-map mutation,
  atomic bitmap execution, OSVM/page-source execution, provider activation,
  hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_residence_arena_binding_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_residence_arena_binding_diagnostics_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_residence_arena_binding_closeout_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_arena_slot_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

After this closeout, select the next allocator exact-`usize` field group.
