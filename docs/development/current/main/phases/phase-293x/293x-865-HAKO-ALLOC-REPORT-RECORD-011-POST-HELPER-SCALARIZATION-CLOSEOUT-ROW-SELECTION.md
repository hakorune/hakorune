# 293x-865 HAKO-ALLOC-REPORT-RECORD-011 Post Helper-Scalarization Closeout Row Selection

Status: selected current
Date: 2026-05-20

## Decision

After closing out the first two `ReportFields` helper-argument scalarization
owners, select the next row by inventory rather than migrating another owner
opportunistically.

## Scope

- Inventory remaining scalar-only report-box owners that still copy many `i64`
  or exact-`usize` fields through ordinary report boxes.
- Separate candidates that already have local `ReportFields` records from
  candidates that first need a record-carrier pilot.
- Choose exactly one next owner or choose to pause report-record work and return
  to the active mimalloc modeled allocator lane.
- Keep validation profile bounded to L0/L2 unless a new compiler acceptance
  shape is selected.

## Stop Lines

- No new report owner migration in this selection row.
- No broad conversion from report boxes to records.
- No record return values.
- No runtime record representation, packed storage, or backend matcher.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No real raw pointer residence, real segment-map mutation, arena backing
  execution, atomic bitmap execution, OSVM/page-source execution, provider
  activation, hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Completion Criteria

- Remaining candidate owners are named with the reason they are or are not next.
- The selected next row has a single owner and a bounded validation profile.
- The taskboard and current pointer name the selected next row.
