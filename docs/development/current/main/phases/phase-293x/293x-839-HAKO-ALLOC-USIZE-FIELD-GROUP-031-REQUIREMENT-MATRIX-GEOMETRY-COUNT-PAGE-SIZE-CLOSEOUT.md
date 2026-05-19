# 293x-839 HAKO-ALLOC-USIZE-FIELD-GROUP-031 Requirement-Matrix Geometry Count / Page-Size Closeout

Status: landed
Date: 2026-05-19

## Decision

Close out the requirement-matrix geometry count / page-size exact-`usize` field
group after `HAKO-ALLOC-USIZE-FIELD-GROUP-030`.

This row should prove that the requirement-matrix geometry count / page-size
fields are now part of the current production `usize` storage inventory and
that the requirement-matrix closeout route plus immediate residence
arena-binding consumer remain stable.

## Scope

- Freeze the requirement-matrix geometry count / page-size field-group
  migration evidence.
- Keep the group limited to:

```text
slice_count
committed_slices
free_slices
page_size
```

- Keep counters, reasons, ids, alignments, requirement flags, blocker counts,
  and sentinel-bearing fields on `i64`.
- Confirm `NUMERIC_FIELDS.md` lists this group as current production `usize`
  storage, not selected/pending storage.

## Stop Lines

- No second field-group migration in this row.
- No broad `i64` to `usize` rewrite.
- No residence arena-binding, arena-slot, source-bridge, source-accounting,
  allocation-plan, allocation-apply, allocation-ledger, or release-candidate
  migration.
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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_requirement_matrix_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_requirement_matrix_diagnostics_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_requirement_matrix_closeout_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_residence_arena_binding_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Notes

- Re-ran the MIMAP-240A requirement-matrix L2 guard after the geometry count /
  page-size migration.
- Re-ran the MIMAP-241A diagnostics L2 guard and the MIMAP-242A closeout L3
  guard to keep the representative exact-MIR evidence green.
- Re-ran the downstream MIMAP-252A residence arena-binding L2 guard because
  residence arena-binding consumes the requirement-matrix geometry facts.
- Confirmed `NUMERIC_FIELDS.md` lists the requirement-matrix geometry count /
  page-size fields as current production `usize` storage.
- Kept counters, reasons, ids, alignments, requirement flags, blocker counts,
  and sentinel-bearing fields on `i64`.

## Next

Select `HAKO-ALLOC-USIZE-FIELD-GROUP-032` for the readiness inventory geometry
count / page-size group that feeds the already-migrated requirement-matrix
family. This is intentionally not a byte/capacity row.
