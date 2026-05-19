# 293x-830 HAKO-ALLOC-USIZE-FIELD-GROUP-022 Source-Bridge Byte/Capacity Migration

Status: selected current
Date: 2026-05-19

## Decision

Migrate the modeled source-bridge report byte/capacity owner group to exact
`usize`.

This row moves one owner upstream from source-accounting. The group contains
non-negative modeled byte/capacity facts that feed the already-migrated
source-accounting family.

## Scope

Change only these fields on
`HakoAllocSegmentArenaBackingModeledSourceBridgeReport`:

```text
source_capacity
source_committed_bytes
requested_bytes
padded_bytes
slot_capacity
```

These are modeled byte/capacity facts copied from accepted arena-slot/source
inputs after local validation.

## Stop Lines

- No source-bridge diagnostic mirror migration in this row.
- No source-accounting, allocation-plan, allocation-apply, allocation-ledger, or
  release-candidate migration in this row.
- No broad `i64` to `usize` rewrite.
- No migration of source-bridge counters.
- No migration of reason/status/flag vocabularies.
- No migration of token/id/generation fields.
- No migration of alignment fields.
- No migration of `row_index` or any `-1` sentinel-bearing field.
- No new backend route or `.inc` owner-name matcher.
- No runtime sum materialization.
- No cross-function `Result` direct ABI.
- No real raw pointer residence, arena backing execution, segment-map mutation,
  atomic bitmap execution, OSVM/page-source execution, provider activation,
  hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_bridge_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_bridge_diagnostics_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_bridge_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

After this migration, select a closeout row for the source-bridge byte/capacity
field group before selecting another allocator byte/capacity group.
