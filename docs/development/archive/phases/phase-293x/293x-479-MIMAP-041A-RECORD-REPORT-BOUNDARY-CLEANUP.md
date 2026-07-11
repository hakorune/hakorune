# 293x-479 MIMAP-041A Record Report Boundary Cleanup

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-041A` is the BoxShape cleanup selected by `MIMAP-040C`.

It targets the bounded purge/decommit scheduler report construction in:

```text
lang/src/hako_alloc/memory/purge_bounded_scheduler_box.hako
```

The current owner uses a 16-argument `report(...)` helper. This row should
replace that call boundary with an identity-free record-shaped local report
payload, while preserving the existing returned report box and proof output.

## Scope

- Add a scheduler-local report-field record declaration for the current report
  scalar group.
- Collapse the four 16-argument `me.report(...)` call sites into one local
  report-field payload after status / stop-reason selection.
- Materialize the existing `HakoAllocBoundedPurgeDecommitSchedulerReport` box
  from local record field reads.
- Update the focused guard so it rejects the old `report/16` shape and proves
  the record-report owner stays local.
- Keep the existing proof app output unchanged.

## Acceptance Shape

The record value must stay builder-local:

```text
construct local record literal
  -> read fields in the same owner
  -> copy scalars into the existing report box
  -> return the report box
```

The row must not pass, return, store, or expose the record value itself. If the
accepted record surface cannot express this shape cleanly, stop and cut a
compiler acceptance sidecar instead of adding a source workaround.

## Acceptance Sidecar

Initial proof exposed a direct MIR emit gap:

```text
RecordLiteral:
  Stage1 Program JSON lowering is live
  direct MIR builder route was missing expression support
```

`MIMAP-041A` may include the minimal direct-MIR acceptance sidecar only for
local record literal construction/read parity with the already accepted
builder-local record constructor route. The sidecar must stay allocator-neutral
and must not enable record escape, record materialization, packed ArrayBox
storage, backend lowering, or ordinary `NewBox` construction for records.

## Stop Lines

- Do not change purge/decommit scheduler behavior or proof output.
- Do not convert other report boxes in the same row.
- Do not place report records in `allocator_metadata_records.hako`; that file
  remains the packed metadata record owner.
- Do not enable record materialization, packed ArrayBox storage, backend record
  lowering, or `.inc` matchers.
- Do not mix in usize migration, rune contract promotion, or capability
  verifier work.
- Do not add provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `041A.1` | Document the row and select owner/proof/guard. | `CURRENT_STATE.toml` points at this card. | no code before docs |
| `041A.2` | Add minimal direct MIR `RecordLiteral` local-value acceptance if the proof exposes it. | Record literal lowers like the existing builder-local record constructor. | no record escape/materialization |
| `041A.3` | Implement the local record-report payload. | `report/16` is gone and field values still flow into the report box. | no record pass/return/store |
| `041A.4` | Update the focused guard and MIR/EXE proof checks. | Guard rejects old `report/16` and proof output is unchanged. | no broad report sweep |
| `041A.5` | Run the current pointer and quick gates. | Required evidence is green. | no provider/backend activation |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_bounded_purge_decommit_scheduler_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Closeout

This row closes when the bounded scheduler report boundary uses a local record
payload, the existing report box/proof output is unchanged, and current moves
to the next row selection card.

## Landed Implementation

```text
owners:
  lang/src/hako_alloc/memory/purge_bounded_scheduler_box.hako
  src/mir/builder/record_values.rs
  src/mir/builder/exprs.rs
guard:
  tools/checks/k2_wide_hako_alloc_bounded_purge_decommit_scheduler_guard.sh
proof:
  apps/hako-alloc-bounded-purge-decommit-scheduler-proof/main.hako
```

The old `HakoAllocBoundedPurgeDecommitScheduler.report/16` helper is removed.
`run/3` now selects `status` / `stop_reason`, constructs a local
`HakoAllocBoundedPurgeDecommitSchedulerReportFields` record literal, and copies
record field reads into the existing report box.

The direct MIR builder now accepts explicit `RecordLiteral` expressions as
builder-local record values, matching the existing record constructor
scalarization route. Record escape, materialization, packed storage, backend
lowering, and ordinary `NewBox` record construction remain closed.

Evidence:

```text
bash tools/checks/k2_wide_hako_alloc_bounded_purge_decommit_scheduler_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

Closeout:

```text
current blocker moves to MIMAP-041B post-record-report row selection.
```
