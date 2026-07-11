# 293x-455 MIMAP-030B Post-Huge-Decommit-Failfast Row Selection

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-030B` is a planning-only row. It selects exactly one next allocator
behavior row after the landed MIMAP-030A duplicate/stale huge decommit
fail-fast diagnostics route.

It must not land code.

The next row should be chosen from the smallest behavior that advances the
allocator lifecycle without opening provider activation or host allocator
replacement:

```text
candidate:
  MIMAP-031A OSVM unreserve capability inventory / planning row
candidate:
  facade huge decommit state-marker reuse / verifier-contract row
candidate:
  post-decommit stats/diagnostic observer row
candidate:
  explicit park row if unreserve/recommit still needs more substrate inventory
```

## Selection Criteria

The selected row must:

- build directly on MIMAP-029A and MIMAP-030A evidence
- keep allocator-provider activation, hooks, host allocator replacement, and
  `#[global_allocator]` inactive
- keep backend `.inc` matcher shortcuts forbidden
- avoid adding unreserve/recommit behavior unless the row is explicitly scoped
  as an inventory / planning row
- keep duplicate/stale decommit fail-fast allocator-side, not page-source-owned

## Candidate Template

The closeout for this card should fill in:

```text
row:
  MIMAP-031A OSVM unreserve capability inventory / planning row
owner:
  docs/development/current/main/phases/phase-293x/293x-456-MIMAP-031A-OSVM-UNRESERVE-CAPABILITY-INVENTORY.md
proof app:
  none in MIMAP-030B; MIMAP-031A decides whether a proof app is needed
guard:
  bash tools/checks/current_state_pointer_guard.sh
  tools/checks/dev_gate.sh quick
reused owners:
  HakoAllocPageSourcePolicy
  OsVmCoreBox
  HakoAllocObjectLifecycleFacadeHugeDecommitRoute
  HakoAllocObjectLifecycleFacadeHugeDecommitFailfastRoute
primary proof:
  select inventory/planning before adding OSVM unreserve behavior
stop lines:
  no unreserve implementation
  no recommit implementation
  no provider activation
  no host allocator replacement / hook / #[global_allocator]
  no backend .inc matcher shortcut
```

If the next row needs a compiler/language sidecar, this card must name the
sidecar and keep allocator implementation parked until the sidecar is green.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `030B.1` | Review MIMAP-029A / MIMAP-030A closeout evidence. | Next row does not repeat already-landed decommit success/fail-fast proof. | no code |
| `030B.2` | Pick exactly one next allocator row. | Owner/proof/guard/stop lines are named. | no broad provider work |
| `030B.3` | Update current pointers. | Current state moves to the selected next row. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Return Condition

This row closes when one next allocator behavior row is selected with clear
owner/proof/guard names and provider/host allocator replacement still inactive.

## Selection Result

`MIMAP-030B` selects `MIMAP-031A`.

Rationale:

- `MIMAP-029A` and `MIMAP-030A` prove decommit success and duplicate/stale
  fail-fast rejection, but neither opens unreserve/recommit/provider behavior.
- OSVM unreserve is the next broad lifecycle concept, so it needs an inventory
  / planning row before implementation.
- The row must decide whether unreserve belongs behind the existing page-source
  owner, a new adapter, a verifier contract, or a later provider capability.

Closeout:

```text
current blocker moves to MIMAP-031A.
```
