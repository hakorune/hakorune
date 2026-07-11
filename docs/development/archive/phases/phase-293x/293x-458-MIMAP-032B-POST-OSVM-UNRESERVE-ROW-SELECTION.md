# 293x-458 MIMAP-032B Post-OSVM-Unreserve Row Selection

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-032B` is a planning-only row. It selects exactly one next allocator
behavior row after the landed MIMAP-032A OSVM unreserve substrate route.

It must not land code.

The next row should be chosen from the smallest behavior that advances allocator
lifecycle ownership without opening provider activation or host allocator
replacement:

```text
candidate:
  page-source unreserve adapter / contract row
candidate:
  facade huge unreserve-after-decommit route
candidate:
  unreserve fail-fast / stale backing diagnostics row
candidate:
  explicit park row if page-source ownership needs more contract work first
```

## Selection Criteria

The selected row must:

- build directly on MIMAP-029A / MIMAP-030A / MIMAP-032A evidence
- keep allocator-provider activation, hooks, host allocator replacement, and
  `#[global_allocator]` inactive
- keep backend `.inc` matcher shortcuts forbidden
- keep unreserve behind an explicit owner, not an app/box-name route
- avoid adding recommit, purge, remote-free, TLS cache, or provider behavior
  unless the row explicitly scopes that capability

## Candidate Template

The closeout for this card should fill in:

```text
row:
  MIMAP-033A <selected owner / behavior>
owner:
  <new or reused owner path>
proof app:
  <proof app path or none>
guard:
  <focused guard>
reused owners:
  OsVmCoreBox.unreserve_bytes_i64
  HakoAllocPageSourcePolicy
  HakoAllocObjectLifecycleFacadeHugeDecommitRoute
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

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `032B.1` | Review MIMAP-029A / MIMAP-030A / MIMAP-032A closeout evidence. | Next row does not repeat success/fail-fast/substrate proof. | no code |
| `032B.2` | Pick exactly one next allocator row. | Owner/proof/guard/stop lines are named. | no broad provider work |
| `032B.3` | Update current pointers. | Current state moves to the selected next row. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Return Condition

This row closes when one next allocator behavior row is selected with clear
owner/proof/guard names and provider/host allocator replacement still inactive.

## Selection Result

`MIMAP-032B` selects `MIMAP-033A`.

Rationale:

- `MIMAP-032A` proved `OsVmCoreBox.unreserve_bytes_i64`, but allocator owners
  still do not consume it.
- The smallest allocator behavior is the page-source adoption row, not facade
  huge unreserve. This mirrors the existing decommit split:

```text
HakoAllocPageSourcePolicy.decommitPage
  -> HakoAllocPageSourceDecommitAdapter
  -> facade huge decommit route later
```

- The selected row should add the analogous unreserve page-source owner /
  adapter, prove reserve/commit/decommit/unreserve through a scalar proof, and
  still stop before facade huge unreserve, recommit, provider activation, hooks,
  host allocator replacement, or backend matcher shortcuts.

Closeout:

```text
current blocker moves to MIMAP-033A page-source unreserve adapter.
```
