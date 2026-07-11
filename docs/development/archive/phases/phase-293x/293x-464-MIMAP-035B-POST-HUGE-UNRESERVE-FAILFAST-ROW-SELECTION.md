# 293x-464 MIMAP-035B Post-Huge-Unreserve-Failfast Row Selection

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-035B` is a planning-only row. It selects exactly one next allocator
behavior row after the landed MIMAP-035A facade huge unreserve duplicate/stale
fail-fast diagnostics.

It must not land code.

The next row should be chosen from the smallest behavior that advances the
post-unreserve lifecycle without opening provider activation or host allocator
replacement:

```text
candidate:
  next lifecycle row after successful decommit/unreserve
candidate:
  explicit park row if MIMAP-035A evidence exposes a compiler sidecar
candidate:
  state/report helper cleanup only if another facade huge diagnostics row
  repeats the same report boilerplate
candidate:
  provider/host allocator replacement remains parked unless explicitly reopened
```

## Selection Criteria

The selected row must:

- build on MIMAP-029A / MIMAP-030A / MIMAP-034A / MIMAP-035A evidence
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
  MIMAP-036A <selected owner / behavior>
owner:
  <new or reused owner path>
proof app:
  <proof app path or none>
guard:
  <focused guard>
reused owners:
  HakoAllocObjectLifecycleFacadeHugeUnreserveRoute
  HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastRoute
  HakoAllocPageSourceUnreserveAdapter
primary proof:
  <smallest scalar proof>
stop lines:
  no provider activation
  no host allocator replacement / hook / #[global_allocator]
  no backend .inc matcher shortcut
  no broad recommit / purge / remote-free / TLS behavior unless selected
```

If the next row needs a compiler/language sidecar, this card must name the
sidecar and keep allocator implementation parked until the sidecar is green.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `035B.1` | Review MIMAP-035A closeout evidence. | Next row does not repeat fail-fast proof. | no code |
| `035B.2` | Pick exactly one next allocator row. | Owner/proof/guard/stop lines are named. | no broad provider work |
| `035B.3` | Update current pointers. | Current state moves to the selected next row. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Return Condition

This row closes when one next allocator behavior row is selected with clear
owner/proof/guard names and provider/host allocator replacement still inactive.

## Selection Result

`MIMAP-035B` selects `MIMAP-036A`.

Rationale:

- `MIMAP-034A` and `MIMAP-035A` close facade huge unreserve success plus
  duplicate/stale diagnostics.
- Existing post-M213 inventory docs still describe unreserve as inactive, so
  the next clean step is a closeout/sync guard before any new allocator
  behavior.
- Recommit, purge scheduling, remote-free/TLS behavior, provider activation,
  hooks, host allocator replacement, and `#[global_allocator]` remain parked.

Selected row:

```text
row:
  MIMAP-036A post-huge-unreserve closeout guard
owner:
  docs/development/current/main/design/mimalloc-post-huge-unreserve-closeout-ssot.md
proof app:
  none
guard:
  tools/checks/k2_wide_mimalloc_post_huge_unreserve_closeout_guard.sh
reused owners:
  HakoAllocObjectLifecycleFacadeHugeUnreserveRoute
  HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastRoute
  HakoAllocPageSourceUnreserveAdapter
primary proof:
  docs/guard inventory that the huge unreserve lane is closed through
  MIMAP-035A and the remaining surfaces are explicit inactive future rows
stop lines:
  no allocator behavior
  no provider activation
  no host allocator replacement / hook / #[global_allocator]
  no backend .inc matcher shortcut
  no recommit / purge / remote-free / TLS behavior
```

Closeout:

```text
current blocker moves to MIMAP-036A post-huge-unreserve closeout guard.
```
