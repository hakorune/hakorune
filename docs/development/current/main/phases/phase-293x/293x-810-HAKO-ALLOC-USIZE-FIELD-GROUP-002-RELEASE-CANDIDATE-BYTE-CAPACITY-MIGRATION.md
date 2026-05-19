# 293x-810 HAKO-ALLOC-USIZE-FIELD-GROUP-002 Release-Candidate Byte/Capacity Migration

Status: landed
Date: 2026-05-19

## Decision

Migrate only the selected release-candidate report byte/capacity field group to
exact `usize`.

Selected owner:

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_candidate_box.hako
```

Selected fields:

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

## Scope

- Update `NUMERIC_FIELDS.md` for this field group.
- Change only the selected fields in the release-candidate `ReportFields`
  record and returned report box from `i64` to `usize`.
- Keep all reason/status/token/id/generation/sentinel fields on `i64`.
- Extend the existing MIMAP-280A guard to require exact `usize` metadata for the
  selected fields and `i64` for the non-selected fields.
- Keep the backend structural sidecar limited to exact field-set continuation
  PHI predecessor remapping. This is not a new route or owner-name matcher; it
  preserves valid LLVM CFG after exact `usize` field-set traps insert
  continuation labels.

## Stop Lines

- No broad `i64` to `usize` rewrite.
- No migration of reason/status/flag vocabularies.
- No migration of token/id/generation fields.
- No migration of `row_index` or any `-1` sentinel-bearing field.
- No runtime sum materialization.
- No cross-function `Result` direct ABI.
- No new backend route or `.inc` owner-name matcher.
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

- Migrated only the selected release-candidate `ReportFields` record fields and
  report box stored fields to exact `usize`.
- Kept `reason`, status flags, tokens, `row_index`, and sentinel-bearing fields
  on `i64`.
- Extended MIMAP-280A MIR checks to require `usize` storage/declared type for
  the selected fields and `i64` for non-selected reason/token/index fields.
- Fixed the pure-first C shim so exact field-set trap continuation labels are
  used as PHI incoming predecessors when a block later branches to successor
  blocks. This kept the representative MIMAP-282A closeout L3 EXE route green
  without a `.hako` workaround.

## Next

Select `HAKO-ALLOC-USIZE-FIELD-GROUP-003` to close this first field group before
choosing the next narrow byte/capacity group.
