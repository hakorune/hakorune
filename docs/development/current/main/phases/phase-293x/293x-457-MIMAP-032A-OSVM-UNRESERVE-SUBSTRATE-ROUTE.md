# 293x-457 MIMAP-032A OSVM Unreserve Substrate Route

Status: selected current
Date: 2026-05-16

## Decision

`MIMAP-032A` is the behavior row selected by `MIMAP-031A`.

It adds the narrow OSVM substrate unreserve route. The route is a runtime /
backend capability row only; allocator page-source and facade owners must not
consume it in this row.

## Scope

- Add a `hako_osvm_unreserve_bytes_i64(base, len_bytes)` substrate route.
- Add `OsVmCoreBox.unreserve_bytes_i64(base, len_bytes)`.
- Add route metadata / VM / native backend wiring consistent with the existing
  reserve/commit/decommit rows.
- Add proof app:
  `apps/mimalloc-osvm-unreserve-proof/main.hako`.
- Add guard:
  `tools/checks/k2_wide_mimalloc_osvm_unreserve_exe_guard.sh`.

## Stop Lines

- Do not add `HakoAllocPageSourcePolicy.unreservePage` in this row.
- Do not add facade huge unreserve, provider activation, hooks, host allocator
  replacement, or `#[global_allocator]`.
- Do not add recommit, purge, allocator release policy, page-map behavior, or
  duplicate/stale decommit changes.
- Do not add app/box-name backend classifiers.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `032A.1` | Add substrate route vocabulary and runtime shim. | `hako_osvm_unreserve_bytes_i64` is exported consistently with existing OSVM rows. | no allocator owner |
| `032A.2` | Add `OsVmCoreBox` route. | `.hako` substrate facade can call the new extern. | no page-source API |
| `032A.3` | Add MIR/backend/VM route wiring. | Existing route metadata style recognizes the new OSVM row. | no name classifier |
| `032A.4` | Add proof app and guard. | EXE guard proves reserve/commit/decommit/unreserve sequence. | no provider activation |
| `032A.5` | Close current pointers. | Current state moves to the next selected row. | no facade use |

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_osvm_unreserve_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Return Condition

This row closes when the OSVM unreserve substrate route is live and proven, but
allocator/page-source/facade owners still do not consume it.
