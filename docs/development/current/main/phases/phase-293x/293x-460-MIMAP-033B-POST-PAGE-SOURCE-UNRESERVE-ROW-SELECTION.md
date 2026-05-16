# 293x-460 MIMAP-033B Post-Page-Source-Unreserve Row Selection

Status: selected current
Date: 2026-05-16

## Decision

`MIMAP-033B` is a planning-only row. It selects exactly one next allocator
behavior row after the landed MIMAP-033A page-source unreserve adapter.

It must not land code.

The next row should be chosen from the smallest behavior that advances the
huge-page lifecycle without opening provider activation or host allocator
replacement:

```text
candidate:
  facade huge unreserve-after-decommit success route
candidate:
  facade huge unreserve fail-fast / stale backing diagnostics
candidate:
  page-source unreserve contract hardening if facade ownership still needs
  a smaller seam
candidate:
  explicit park row if a compiler/language sidecar is exposed
```

## Selection Criteria

The selected row must:

- build on MIMAP-029A / MIMAP-030A / MIMAP-032A / MIMAP-033A evidence
- keep unreserve behind `HakoAllocPageSourceUnreserveAdapter`, not a direct
  facade or backend matcher shortcut
- keep allocator-provider activation, hooks, host allocator replacement, and
  `#[global_allocator]` inactive
- avoid adding recommit, purge scheduler, remote-free, TLS cache, or provider
  behavior unless the row explicitly scopes that capability
- name proof app / guard / stop lines before implementation starts

## Candidate Template

The closeout for this card should fill in:

```text
row:
  MIMAP-034A <selected owner / behavior>
owner:
  <new or reused owner path>
proof app:
  <proof app path or none>
guard:
  <focused guard>
reused owners:
  HakoAllocObjectLifecycleFacadeHugeDecommitRoute
  HakoAllocObjectLifecycleFacadeHugeDecommitFailfastRoute
  HakoAllocPageSourceUnreserveAdapter
  HakoAllocPageSourcePolicy.unreservePage
  OsVmCoreBox.unreserve_bytes_i64
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
| `033B.1` | Review MIMAP-029A / MIMAP-030A / MIMAP-032A / MIMAP-033A closeout evidence. | Next row does not repeat success/fail-fast/substrate/adapter proof. | no code |
| `033B.2` | Pick exactly one next allocator row. | Owner/proof/guard/stop lines are named. | no broad provider work |
| `033B.3` | Update current pointers. | Current state moves to the selected next row. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Return Condition

This row closes when one next allocator behavior row is selected with clear
owner/proof/guard names and provider/host allocator replacement still inactive.
