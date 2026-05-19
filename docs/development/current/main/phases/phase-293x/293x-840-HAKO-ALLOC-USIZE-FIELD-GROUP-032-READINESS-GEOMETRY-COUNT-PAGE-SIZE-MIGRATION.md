# 293x-840 HAKO-ALLOC-USIZE-FIELD-GROUP-032 Readiness Geometry Count / Page-Size Migration

Status: selected current
Date: 2026-05-19

## Decision

Migrate the segment arena-backing readiness report geometry count / page-size
owner group to exact `usize`.

This row moves one owner upstream from requirement matrix. The group contains
non-negative modeled geometry facts that feed the already-migrated
requirement-matrix family. It is intentionally not a byte/capacity row.

## Scope

Change only these fields on
`HakoAllocSegmentArenaBackingReadinessInventoryReport`:

```text
slice_count
committed_slices
free_slices
page_size
```

These are modeled non-negative geometry count / page-size facts accepted by the
readiness classifier after local validation.

## Stop Lines

- No readiness diagnostic mirror migration in this row.
- No requirement-matrix, residence arena-binding, arena-slot, source-bridge,
  source-accounting, allocation-plan, allocation-apply, allocation-ledger, or
  release-candidate migration in this row.
- No broad `i64` to `usize` rewrite.
- No migration of readiness counters.
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

After migration, select `HAKO-ALLOC-USIZE-FIELD-GROUP-033` to close out the
readiness geometry count / page-size group before selecting another allocator
exact-`usize` field group.
