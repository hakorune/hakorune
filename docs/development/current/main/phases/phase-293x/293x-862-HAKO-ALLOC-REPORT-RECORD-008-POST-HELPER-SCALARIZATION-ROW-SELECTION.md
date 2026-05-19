# 293x-862 HAKO-ALLOC-REPORT-RECORD-008 Post Helper Scalarization Row Selection

Status: selected current
Date: 2026-05-19

## Decision

Select the next narrow allocator-side use of `RECORD-VALUE-HELPER-001`.

The compiler acceptance is now present for same-owner helper argument
scalarization of local `ReportFields` record carriers. The next allocator row
should apply it to exactly one existing `ReportFields` owner.

## Candidate Order

Recommended next implementation row:

```text
HAKO-ALLOC-REPORT-RECORD-009
  allocation-ledger release-candidate ReportFields helper scalarization pilot
```

Rationale:

- `HAKO-ALLOC-REPORT-RECORD-005` already introduced the release-candidate
  `ReportFields` record.
- The owner has repeated report-copy boilerplate similar to the just-landed
  local-free reuse ledger release-apply owner.
- The migration exercises the same compiler shape without opening a new runtime
  representation.

## Scope For The Selected Next Row

- Add one same-owner helper that accepts the existing local `ReportFields`
  record type.
- Replace repeated field-copy blocks in that owner with the helper call.
- Keep the returned ordinary report box.
- Keep direct local record field-read scalarization green.

## Stop Lines

- No broad migration across all `ReportFields` owners.
- No record return values.
- No record storage in box fields, ArrayBox, MapBox, or globals.
- No backend `.inc` owner-name matchers.
- No packed ArrayBox residence or inline-record storage.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No real raw pointer residence, real segment-map mutation, arena backing
  execution, atomic bitmap execution, OSVM/page-source execution, provider
  activation, hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/k2_wide_allocator_record_construction_read_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The selected implementation row must add or reuse its owner guard.
