# 293x-884 HAKO-ALLOC-REPORT-RECORD-029 Bounded Purge Scheduler ReportFields Helper Scalarization Pilot

Status: selected current
Date: 2026-05-20

## Decision

Apply record-local helper-argument scalarization to the remaining allocator
report `ReportFields` owner:

```text
HakoAllocBoundedPurgeDecommitSchedulerReportFields
```

SSOT:

```text
docs/development/current/main/design/record-local-scalarization-ssot.md
```

## Scope

- Add one same-owner helper that accepts the local `ReportFields` record and
  builds the existing ordinary scheduler report box.
- Use the helper from the bounded scheduler `run(...)` construction path.
- Keep scheduler scan/state logic unchanged.
- Keep the record carrier builder-local only.
- Keep validation at the existing M212 guard profile.

## Stop Lines

- No broad conversion from report boxes to records.
- No record return values.
- No record storage in box fields, ArrayBox, MapBox, or globals.
- No scheduler behavior change.
- No direct M197/M195/page-source/OSVM release seams.
- No backend `.inc` owner-name matchers.
- No packed ArrayBox residence or inline-record storage.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No real raw pointer residence, real segment-map mutation, arena backing
  execution, atomic bitmap execution, provider activation, hooks, host
  allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/k2_wide_allocator_record_construction_read_guard.sh
bash tools/checks/k2_wide_hako_alloc_bounded_purge_decommit_scheduler_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Completion Criteria

- The scheduler report construction path calls the same-owner helper with the
  local `ReportFields` record.
- The target guard stays green and continues proving M212 behavior.
- No other owner is migrated.
