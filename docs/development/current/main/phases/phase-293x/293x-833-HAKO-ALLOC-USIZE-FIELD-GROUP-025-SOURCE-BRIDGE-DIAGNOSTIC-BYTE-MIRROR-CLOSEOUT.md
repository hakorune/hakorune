# 293x-833 HAKO-ALLOC-USIZE-FIELD-GROUP-025 Source-Bridge Diagnostic Byte Mirror Closeout

Status: selected current
Date: 2026-05-19

## Decision

Close out the source-bridge diagnostic mirror byte field group after
`HAKO-ALLOC-USIZE-FIELD-GROUP-024`.

This row should prove that the source-bridge diagnostic mirror byte fields are
now part of the current production `usize` storage inventory and that the
source-bridge closeout route remains stable.

## Scope

- Freeze the source-bridge diagnostic byte mirror migration evidence.
- Keep the group limited to:

```text
last_report_source_capacity
last_report_source_committed_bytes
```

- Keep diagnostic counters, reasons, tokens, ids, alignments, and
  sentinel-bearing fields on `i64`.
- Confirm `NUMERIC_FIELDS.md` lists this group as current production `usize`
  storage, not selected/pending storage.

## Stop Lines

- No second field-group migration in this row.
- No broad `i64` to `usize` rewrite.
- No source-bridge report migration.
- No source-accounting, allocation-plan, allocation-apply, allocation-ledger, or
  release-candidate migration.
- No migration of diagnostic counters.
- No migration of reason/status/flag vocabularies.
- No migration of token/id/generation fields.
- No migration of alignment fields.
- No migration of any sentinel-bearing field.
- No new backend route or `.inc` owner-name matcher.
- No runtime sum materialization.
- No cross-function `Result` direct ABI.
- No real raw pointer residence, arena backing execution, segment-map mutation,
  atomic bitmap execution, OSVM/page-source execution, provider activation,
  hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_bridge_diagnostics_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_bridge_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

After this closeout, select the next allocator exact-`usize` byte/capacity field
group.
