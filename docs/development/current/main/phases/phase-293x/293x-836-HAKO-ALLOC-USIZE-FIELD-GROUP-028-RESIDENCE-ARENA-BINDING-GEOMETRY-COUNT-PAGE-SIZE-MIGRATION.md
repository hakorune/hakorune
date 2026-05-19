# 293x-836 HAKO-ALLOC-USIZE-FIELD-GROUP-028 Residence Arena-Binding Geometry Count / Page-Size Migration

Status: landed
Date: 2026-05-19

## Decision

Migrate the modeled residence arena-binding report geometry count / page-size
owner group to exact `usize`.

This row moves one owner upstream from arena-slot. The group contains
non-negative modeled geometry facts that feed the already-migrated arena-slot
family. It is intentionally not a byte/capacity row.

## Scope

Change only these fields on
`HakoAllocSegmentArenaBackingModeledResidenceArenaBindingReport`:

```text
slice_count
committed_slices
free_slices
page_size
```

These are modeled non-negative geometry count / page-size facts copied from the
accepted scalar requirement matrix after local validation.

## Stop Lines

- No residence arena-binding diagnostic mirror migration in this row.
- No arena-slot, source-bridge, source-accounting, allocation-plan,
  allocation-apply, allocation-ledger, or release-candidate migration in this
  row.
- No broad `i64` to `usize` rewrite.
- No migration of residence arena-binding counters.
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

## Landed Notes

- Migrated only the
  `HakoAllocSegmentArenaBackingModeledResidenceArenaBindingReport` geometry
  count / page-size group to exact `usize` storage.
- Kept residence arena-binding counters, reasons, tokens, ids, generations,
  alignments, row index, and sentinel-bearing fields on `i64`.
- Strengthened the MIMAP-252A guard to assert exact `usize` typed-object storage
  for the geometry count / page-size fields and to assert `required_alignment`
  and `row_index` remain `i64`.
- Re-ran the MIMAP-253A diagnostics guard and the downstream MIMAP-256A
  arena-slot L2 guard after the migration.

## Next

Select `HAKO-ALLOC-USIZE-FIELD-GROUP-029` to close out the residence
arena-binding geometry count / page-size group before selecting another
allocator exact-`usize` field group.
