# 293x-810 HAKO-ALLOC-USIZE-FIELD-GROUP-002 Release-Candidate Byte/Capacity Migration

Status: selected current
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

## Stop Lines

- No broad `i64` to `usize` rewrite.
- No migration of reason/status/flag vocabularies.
- No migration of token/id/generation fields.
- No migration of `row_index` or any `-1` sentinel-bearing field.
- No runtime sum materialization.
- No cross-function `Result` direct ABI.
- No new backend route or `.inc` matcher.
- No real raw pointer residence, arena backing execution, segment-map mutation,
  atomic bitmap execution, OSVM/page-source execution, provider activation,
  hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

After this migration, close the field group or select the next narrow
byte/capacity group only if the evidence stays small.
