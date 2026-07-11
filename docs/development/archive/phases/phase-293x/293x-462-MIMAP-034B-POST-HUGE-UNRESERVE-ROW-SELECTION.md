# 293x-462 MIMAP-034B Post-Huge-Unreserve Row Selection

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-034B` is a planning-only row. It selects exactly one next allocator
behavior row after the landed MIMAP-034A facade huge unreserve success route.

It must not land code.

The next row should be chosen from the smallest behavior that hardens the new
unreserve lifecycle without opening provider activation or host allocator
replacement:

```text
candidate:
  facade huge unreserve duplicate/stale fail-fast diagnostics
candidate:
  explicit park row if success route evidence exposes a compiler sidecar
candidate:
  next lifecycle row only if unreserve diagnostics are not needed yet
```

## Selection Criteria

The selected row must:

- build on MIMAP-029A / MIMAP-030A / MIMAP-033A / MIMAP-034A evidence
- keep unreserve behind `HakoAllocPageSourceUnreserveAdapter`
- name the facade owner, proof app, guard, and stop lines before implementation
- keep allocator-provider activation, hooks, host allocator replacement, and
  `#[global_allocator]` inactive
- avoid recommit, purge scheduler, remote-free, TLS cache, or provider behavior
  unless the row explicitly scopes that capability

## Candidate Template

The closeout for this card should fill in:

```text
row:
  MIMAP-035A <selected owner / behavior>
owner:
  <new or reused owner path>
proof app:
  <proof app path or none>
guard:
  <focused guard>
reused owners:
  HakoAllocObjectLifecycleFacadeHugeUnreserveRoute
  HakoAllocPageSourceUnreserveAdapter
  HakoAllocObjectLifecycleFacadeHugeDecommitFailfastRoute
primary proof:
  <smallest scalar proof>
stop lines:
  no provider activation
  no host allocator replacement / hook / #[global_allocator]
  no backend .inc matcher shortcut
  no broad recommit / purge / remote-free / TLS behavior
```

If the next row needs a compiler/language sidecar, this card must name the
sidecar and keep allocator implementation parked until the sidecar is green.

## Pre-Selection Cleanup Queue

This row may do docs/task cleanup before selecting the next allocator behavior
row. Cleanup must stay behavior-neutral and must not change allocator runtime
semantics.

| Item | Status | Action | Stop line |
| --- | --- | --- | --- |
| `DOCS-034B.0` | required | Deduplicate the MIMAP taskboard so sidecar rows and allocator rows each have one SSOT table. | no code |
| `GUARD-CLEANUP-CANDIDATE` | parked | Consider a pure-first EXE guard helper/manifest row if another guard repeats the same emit/preflight/build/run skeleton. | separate BoxShape row only |
| `STATE-REPORT-CANDIDATE` | parked | Consider a small facade huge state/report helper only if duplicate/stale unreserve diagnostics repeat the decommit/unreserve report boilerplate. | do not pre-factor before the next behavior proves it |

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `034B.0` | Finish behavior-neutral docs/taskboard cleanup. | Taskboard sidecar table no longer repeats allocator row sequence. | no code |
| `034B.1` | Review MIMAP-034A closeout evidence. | Next row does not repeat success-route proof. | no code |
| `034B.2` | Pick exactly one next allocator row. | Owner/proof/guard/stop lines are named. | no broad provider work |
| `034B.3` | Update current pointers. | Current state moves to the selected next row. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Return Condition

This row closes when one next allocator behavior row is selected with clear
owner/proof/guard names and provider/host allocator replacement still inactive.

## Selection Result

`MIMAP-034B` selects `MIMAP-035A`.

Rationale:

- `MIMAP-034A` proves the facade huge unreserve-after-decommit success route.
- The nearest safety gap is duplicate/stale unreserve diagnostics, mirroring the
  landed `MIMAP-030A` huge-decommit fail-fast pattern.
- Provider activation, host allocator replacement, and recommit/purge behavior
  still remain outside the current lane.

Selected row:

```text
row:
  MIMAP-035A facade huge unreserve fail-fast diagnostics
owner:
  lang/src/hako_alloc/memory/object_lifecycle_facade_huge_unreserve_failfast_box.hako
proof app:
  apps/mimalloc-facade-huge-unreserve-failfast-proof/main.hako
guard:
  tools/checks/k2_wide_mimalloc_facade_huge_unreserve_failfast_exe_guard.sh
reused owners:
  HakoAllocObjectLifecycleFacadeHugeUnreserveRoute
  HakoAllocPageSourceUnreserveAdapter
primary proof:
  allocate/decommit/unreserve one page-source-backed huge handle, record the
  successful backing range, then reject duplicate and stale unreserve attempts
  before a second page-source unreserve adapter call
stop lines:
  no direct page-source / OSVM call from the fail-fast owner
  no recommit / purge scheduler / remote-free / TLS behavior
  no provider activation
  no host allocator replacement / hook / #[global_allocator]
  no backend .inc matcher shortcut
```

Closeout:

```text
current blocker moves to MIMAP-035A facade huge unreserve fail-fast diagnostics.
```
